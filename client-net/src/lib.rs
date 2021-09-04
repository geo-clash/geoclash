use async_channel::{Receiver, Sender};
use bevy::prelude::*;
use net::{
	tokio::net::{
		tcp::{OwnedReadHalf, OwnedWriteHalf},
		TcpStream,
	},
	ReadBuffer, RemoteConnection, Runtime, WriteBuf,
};
use std::net::SocketAddr;

pub struct ClientNetworkPlugin;

impl Plugin for ClientNetworkPlugin {
	fn build(&self, app: &mut App) {
		app.add_system_to_stage(CoreStage::PreUpdate, check_connect);
	}
}

#[derive(thiserror::Error, Debug)]
pub enum NetworkError {
	#[error("Could not find server adress: {0}")]
	AdressNotFound(std::io::Error),
	#[error("Could not find connection with id: {0}")]
	ConnectionNotFound(SocketAddr),
	#[error("Connection closed with id: {0}")]
	ChannelClosed(SocketAddr),
	#[error("Not connected to any server")]
	NotConnected,
	#[error("An error occured when trying to connect: {0}")]
	Connection(std::io::Error),
	#[error("An error occured when trying to send data between threads")]
	SendData,
}

#[derive(Debug)]
pub enum ClientNetworkEvent {
	Connected,
	Disconnected,
	Error(NetworkError),
}

struct NetThreadConnection {
	pub send_write_buf: Sender<WriteBuf>,
	pub recieve_read_buf: Receiver<ReadBuffer>,
}

pub struct NetworkClient {
	pub runtime: Runtime,
	pub connection_event_reciever: Receiver<TcpStream>,
	net_thread_connection: Option<NetThreadConnection>,
}

async fn connect(
	addr: &'static str,
	connect_sender: Sender<TcpStream>,
) -> Result<(), NetworkError> {
	let stream = TcpStream::connect(addr)
		.await
		.map_err(|e| NetworkError::Connection(e))?;

	let other_addr = stream
		.peer_addr()
		.map_err(|e| NetworkError::AdressNotFound(e))?;

	info!("Connected to: {:?}", addr);

	connect_sender
		.send(stream)
		.await
		.map_err(|_| NetworkError::SendData)
}

async fn dispatch_messages(mut connection: OwnedWriteHalf, reciever: Receiver<WriteBuf>) {
	while let Ok(mut message) = reciever.recv().await {
		RemoteConnection::socket_write(&mut message, &mut connection).await;
	}
}

async fn read_messages(mut connection: OwnedReadHalf, sender: Sender<ReadBuffer>) {
	info!("recieving messages");
	let mut buffer = vec![0; 1000];
	loop {
		use net::ReadResponse;
		match RemoteConnection::read_length(&mut buffer, &mut connection).await {
			ReadResponse::Ok(len) => {
				info!("Recieved message: {:?}", &buffer[0..len]);
				let b = ReadBuffer::new(buffer[0..len].to_vec());
				sender.send(b).await.unwrap();
			}
			ReadResponse::Disconnected => break,
			ReadResponse::Error => return,
			ReadResponse::PacketLengthTooLong => {
				error!("Packet length more than buffer length");
			}
		}
	}
}

impl NetworkClient {
	pub fn new(addr: &'static str) -> Self {
		let (connect_sender, connect_reciever) = async_channel::bounded::<TcpStream>(1);
		let client = Self {
			runtime: Runtime::new().expect("Could not create a tokio runtime"),
			connection_event_reciever: connect_reciever,
			net_thread_connection: None,
		};
		info!("Connecting...");
		client.runtime.spawn(connect(addr, connect_sender));
		client
	}
}

fn check_connect(client: Option<ResMut<NetworkClient>>) {
	if let Some(mut c) = client {
		if let Ok(x) = c.connection_event_reciever.try_recv() {
			let (socket_reader, socket_writer) = x.into_split();

			let (send_write_buf, recieve_write_buf) = async_channel::unbounded::<WriteBuf>();

			let (send_read_buf, recieve_read_buf) = async_channel::unbounded::<ReadBuffer>();
			c.runtime.spawn(read_messages(socket_reader, send_read_buf));
			c.runtime
				.spawn(dispatch_messages(socket_writer, recieve_write_buf));

			c.net_thread_connection = Some(NetThreadConnection {
				send_write_buf,
				recieve_read_buf,
			})
		}
	}
}
