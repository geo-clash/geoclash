pub use packets::*;
pub use tokio::runtime::Runtime;
pub use tokio::sync::mpsc;
use tokio::{
	io::{AsyncReadExt, AsyncWriteExt},
	net::{TcpListener, TcpStream},
	sync::mpsc::Sender,
};
#[macro_use]
extern crate log;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

pub struct ClientConnection {
	socket: Arc<Mutex<TcpStream>>,
	address: SocketAddr,
}
impl Clone for ClientConnection {
	fn clone(&self) -> Self {
		Self {
			socket: Arc::clone(&self.socket),
			address: self.address.clone(),
		}
	}
}

enum ReadResponse {
	Ok,
	Disconnected,
	Error,
	PacketLengthTooLong,
}

impl ClientConnection {
	// Write data back to the client
	pub async fn socket_write(&mut self, buf: &mut WriteBuf) {
		let mut socket = self.socket.lock().await;
		let buffer = buf.inner();
		let len = buffer.len() as u32;
		info!("buf {}  {:?}", len, buffer);
		socket.write_u32(len).await.unwrap();
		if let Err(e) = socket.write_all(buffer).await {
			error!("failed to write to socket; err = {:?}", e);
			return;
		}
	}
	async fn read_length(&mut self, buf: &mut Vec<u8>) -> ReadResponse {
		let mut socket = self.socket.lock().await;
		let length = match socket.read_u32().await {
			Ok(len) => len as usize,
			Err(err) => {
				if err.kind() == std::io::ErrorKind::UnexpectedEof {
					info!("Client disconnected");
					return ReadResponse::Disconnected;
				}
				error!("Encountered error while fetching length: {}", err);
				return ReadResponse::Error;
			}
		};

		if length > buf.len() {
			return ReadResponse::PacketLengthTooLong;
		}
		match socket.read_exact(&mut buf[..length]).await {
			// socket closed
			Ok(n) if n == 0 => return ReadResponse::Disconnected,
			Ok(n) => n,
			Err(e) => {
				error!("failed to read from socket; err = {:?}", e);
				return ReadResponse::Error;
			}
		};
		ReadResponse::Ok
	}
}

// Handle a packet frm the client

pub async fn server(
	addr: &'static str,
	read_buf_sender: Sender<(ReadBuffer, ClientConnection)>,
) -> Result<(), Box<std::io::Error>> {
	let listener = TcpListener::bind(addr).await?;
	println!("Started server on {}...", addr);

	loop {
		let (socket, other_addr) = listener.accept().await?;

		let buf_sender = read_buf_sender.clone();

		tokio::spawn(async move {
			println!("Client connected from {}", other_addr);

			let mut client_connection = ClientConnection {
				socket: Arc::new(Mutex::new(socket)),
				address: other_addr,
			};

			if let Err(_) = buf_sender
				.send((ReadBuffer::new(vec![0, 0]), client_connection.clone()))
				.await
			{
				println!("receiver dropped");
				return;
			}

			//handle_client_packet(&mut socket, ClientPacket::Connect, &db, &evaluate).await;

			let mut buf = vec![0; 10000];

			// In a loop, read data from the socket and write the data back.
			loop {
				match client_connection.read_length(&mut buf).await {
					ReadResponse::Ok => {}
					ReadResponse::Disconnected => break,
					ReadResponse::Error => return,
					ReadResponse::PacketLengthTooLong => {
						error!("Packet length more than buffer length");
						client_connection.socket_write(WriteBuf::new_server_packet(
							ServerPacket::PacketLengthInvalid,
						).push("Packet length (first 2 bytes) exeeds maximum allowed on this server".to_string())).await;
					}
				}
			}
		});
	}
}
