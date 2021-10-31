use net::{
	packets::*,
	tokio::{self, net::TcpListener},
	ReadResponse, RemoteConnection,
};

use async_channel::Sender;

/// Runs the server, sending back read buffers and write buffer senders for each new client message
pub async fn server(
	addr: &'static str,
	read_buf_sender: Sender<((ReadBuffer, usize), Sender<WriteBuf>)>,
) -> Result<(), Box<std::io::Error>> {
	let listener = TcpListener::bind(addr).await?;
	info!("Started server on {}...", addr);

	let mut client_id = 0;

	loop {
		let (socket, other_addr) = listener.accept().await?;

		let (mut socket_read, mut socket_write) = socket.into_split();

		let buf_sender = read_buf_sender.clone();

		let (write_buf_sender, write_buf_reciever) = async_channel::unbounded::<WriteBuf>();

		tokio::spawn(async move {
			info!("Client connected from {}", other_addr);

			if let Err(_) = buf_sender
				.send((
					(ReadBuffer::new(vec![0, 0]), client_id),
					write_buf_sender.clone(),
				))
				.await
			{
				info!("receiver dropped");
				return;
			}

			let mut buffer = vec![0; 10000];

			'read: loop {
				match RemoteConnection::read_length(&mut buffer, &mut socket_read).await {
					ReadResponse::Ok(len) => {
						info!("Recieved message: {:?}", &buffer[0..len]);
						if let Err(_) = buf_sender
							.send((
								(ReadBuffer::new(buffer[0..len].to_vec()), client_id),
								write_buf_sender.clone(),
							))
							.await
						{
							info!("receiver dropped");
							return;
						}
					}
					ReadResponse::Disconnected => break 'read,
					ReadResponse::Error => break 'read,
					ReadResponse::PacketLengthTooLong => {
						error!("Packet length more than buffer length");
						write_buf_sender.clone().send(WriteBuf::new_server_packet(
							ServerPacket::PacketLengthInvalid,
						).push("Packet length (first 2 bytes) exeeds maximum allowed on this server".to_string())).await.unwrap();
					}
				}
			}

			if let Err(_) = buf_sender
				.send((
					(ReadBuffer::new(vec![0, 1]), client_id),
					write_buf_sender.clone(),
				))
				.await
			{
				info!("receiver dropped");
				return;
			}
		});

		tokio::spawn(async move {
			while let Ok(mut x) = write_buf_reciever.recv().await {
				RemoteConnection::socket_write(&mut x, &mut socket_write).await;
			}
		});

		client_id += 1;
	}
}
