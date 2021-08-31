use crossbeam_channel::{unbounded, Receiver, Sender};
use log::*;
pub use nanoserde::{DeBin, SerBin};
pub use packets::{ClientPackets, ServerPackets};
use std::net::SocketAddr;
pub use tokio::{
	io::{AsyncReadExt, AsyncWriteExt},
	sync::mpsc::unbounded_channel,
};
use tokio::{net::TcpStream, runtime::Runtime, sync::mpsc::UnboundedSender, task::JoinHandle};

#[derive(thiserror::Error, Debug)]
pub enum NetworkError {
	#[error("An error occured when accepting a new connnection: {0}")]
	Accept(std::io::Error),
	#[error("Could not find connection with id: {0}")]
	ConnectionNotFound(SocketAddr),
	#[error("Connection closed with id: {0}")]
	ChannelClosed(SocketAddr),
	#[error("Not connected to any server")]
	NotConnected,
	#[error("An error occured when trying to start listening for new connections: {0}")]
	Listen(std::io::Error),
	#[error("An error occured when trying to connect: {0}")]
	Connection(std::io::Error),
}

pub struct SyncChannel<T> {
	pub sender: Sender<T>,
	pub receiver: Receiver<T>,
}

impl<T> SyncChannel<T> {
	fn new() -> Self {
		let (sender, receiver) = unbounded();

		SyncChannel { sender, receiver }
	}
}

#[derive(Debug)]
pub enum ClientNetworkEvent {
	Connected,
	Disconnected,
	Error(NetworkError),
}

pub struct ServerConnection {
	pub peer_addr: SocketAddr,
	pub receive_task: JoinHandle<()>,
	pub send_task: JoinHandle<()>,
	pub send_message: UnboundedSender<ClientPackets>,
}

impl ServerConnection {
	fn stop(self) {
		self.receive_task.abort();
		self.send_task.abort();
	}
}

pub struct NetworkClient {
	pub runtime: Runtime,
	pub server_connection: Option<ServerConnection>,
	pub network_events: SyncChannel<ClientNetworkEvent>,
	pub connection_events: SyncChannel<(TcpStream, SocketAddr)>,
}

impl NetworkClient {
	pub fn new() -> Self {
		Self {
			runtime: Runtime::new().expect("Could not create a tokio runtime"),
			server_connection: None,
			network_events: SyncChannel::new(),
			connection_events: SyncChannel::new(),
		}
	}
	pub fn connect(&mut self, addr: &'static str) {
		let network_error_sender = self.network_events.sender.clone();
		let connection_event_sender = self.connection_events.sender.clone();
		self.runtime.spawn(async move {
			let stream = match TcpStream::connect(addr).await {
				Ok(stream) => stream,
				Err(error) => {
					match network_error_sender
						.send(ClientNetworkEvent::Error(NetworkError::Connection(error)))
					{
						Ok(_) => (),
						Err(err) => {
							error!("Could not send error event: {}", err);
						}
					}

					return;
				}
			};
			println!("stream.");

			let addr = stream
				.peer_addr()
				.expect("Could not fetch peer_addr of existing stream");

			println!("addr {}.", addr);

			match connection_event_sender.send((stream, addr)) {
				Ok(_) => (),
				Err(err) => {
					error!("Could not initiate connection: {}", err);
				}
			}

			println!("Connected to: {:?}", addr);
		});
	}
	pub fn disconnect(&mut self) {
		if let Some(conn) = self.server_connection.take() {
			conn.stop();

			let _ = self
				.network_events
				.sender
				.send(ClientNetworkEvent::Disconnected);
		}
	}

	pub fn send_message(&self, message: ClientPackets) -> Result<(), NetworkError> {
		debug!("Sending message to server");
		let server_connection = match self.server_connection.as_ref() {
			Some(server) => server,
			None => return Err(NetworkError::NotConnected),
		};

		match server_connection.send_message.send(message) {
			Ok(_) => (),
			Err(err) => {
				error!("Server disconnected: {}", err);
				return Err(NetworkError::NotConnected);
			}
		}

		Ok(())
	}
}
