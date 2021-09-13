use async_channel::{Receiver, Sender};
use bevy::prelude::*;
pub use net::packets::*;
use net::tokio::sync::{Mutex, MutexGuard};
use net::{
	tokio::net::{
		lookup_host,
		tcp::{OwnedReadHalf, OwnedWriteHalf},
		TcpStream,
	},
	RemoteConnection, Runtime,
};
use std::collections::BTreeMap;
use std::net::SocketAddr;

pub struct ClientNetworkPlugin;

impl Plugin for ClientNetworkPlugin {
	fn build(&self, app: &mut App) {
		app.add_system_to_stage(CoreStage::PreUpdate, check_connect)
			.add_event::<WriteBuf>()
			.add_system_to_stage(CoreStage::PostUpdate, handle_write)
			.add_event::<NetworkError>();
	}
}

#[derive(thiserror::Error, Debug)]
pub enum NetworkError {
	#[error("Could not find server address: {0}")]
	AddressNotFound(std::io::Error),
	#[error("Could not find connection with id: {0}")]
	ConnectionNotFound(SocketAddr),
	#[error("Connection closed with id: {0}")]
	ChannelClosed(SocketAddr),
	#[error("Not connected to any server")]
	NotConnected,
	#[error("An error occured when trying to connect: {0}")]
	Connection(std::io::Error),
	#[error("{0}, Could not find address '{1}'")]
	ParseAddressError(std::io::Error, String),
	#[error("Could not find address '{0}'")]
	ParseAddress(String),
	#[error("An error occured when trying to send data between threads")]
	SendData,
}

#[derive(Debug)]
pub enum ClientNetworkEvent {
	Connected,
	Disconnected,
	Error(NetworkError),
}

// TODO: Convert to using enum once const generics support them
pub struct EventReadBuffer<const INDEX: u16> {
	pub read_buffer: Mutex<ReadBuffer>,
}

impl<const INDEX: u16> EventReadBuffer<INDEX> {
	pub fn read(&self) -> MutexGuard<ReadBuffer> {
		self.read_buffer.try_lock().unwrap()
	}
}

struct NetThreadConnection {
	pub send_write_buf: Sender<WriteBuf>,
	pub reciever: BTreeMap<u16, Receiver<ReadBuffer>>,
}

pub struct NetworkClient {
	pub runtime: Runtime,
	pub connection_event_reciever: Receiver<Result<TcpStream, NetworkError>>,
	net_thread_connection: Option<NetThreadConnection>,
}

#[derive(Default)]
pub struct RegisteredRecievers {
	registered_recievers: Option<Vec<u16>>,
}

async fn connect(
	addr: String,
	connect_sender: Sender<Result<TcpStream, NetworkError>>,
) -> Result<(), NetworkError> {
	let mut iter = match lookup_host(&addr)
		.await
		.map_err(|e| NetworkError::ParseAddressError(e, addr.clone()))
	{
		Ok(x) => x,
		Err(e) => {
			return connect_sender
				.send(Err(e))
				.await
				.map_err(|_| NetworkError::SendData)
		}
	};
	let socket = match iter.next().ok_or(NetworkError::ParseAddress(addr.clone())) {
		Ok(x) => x,
		Err(e) => {
			return connect_sender
				.send(Err(e))
				.await
				.map_err(|_| NetworkError::SendData)
		}
	};

	let stream = match TcpStream::connect(socket)
		.await
		.map_err(|e| NetworkError::Connection(e))
	{
		Ok(x) => x,
		Err(e) => {
			return connect_sender
				.send(Err(e))
				.await
				.map_err(|_| NetworkError::SendData)
		}
	};

	let other_addr = match stream
		.peer_addr()
		.map_err(|e| NetworkError::AddressNotFound(e))
	{
		Ok(x) => x,
		Err(e) => {
			return connect_sender
				.send(Err(e))
				.await
				.map_err(|_| NetworkError::SendData)
		}
	};

	info!("Connected to: {:?}", other_addr);

	connect_sender
		.send(Ok(stream))
		.await
		.map_err(|_| NetworkError::SendData)
}

async fn dispatch_messages(mut connection: OwnedWriteHalf, reciever: Receiver<WriteBuf>) {
	while let Ok(mut message) = reciever.recv().await {
		RemoteConnection::socket_write(&mut message, &mut connection).await;
	}
}

async fn read_messages(mut connection: OwnedReadHalf, sender: BTreeMap<u16, Sender<ReadBuffer>>) {
	let b = ReadBuffer::new(vec![0, 0]);
	if let Some(sender) = sender.get(&0) {
		sender.send(b).await.unwrap();
	}

	info!("recieving messages");
	let mut buffer = vec![0; 1000];
	loop {
		use net::ReadResponse;
		match RemoteConnection::read_length(&mut buffer, &mut connection).await {
			ReadResponse::Ok(len) => {
				info!("Recieved message: {:?}", &buffer[0..len]);
				let mut b = ReadBuffer::new(buffer[0..len].to_vec());
				match b.read_server_packet() {
					Ok(packet) => {
						let x = packet as u16;
						if let Some(sender) = sender.get(&x) {
							sender.send(b).await.unwrap();
						}
					}
					Err(e) => {
						error!("Error when reading server packet: {}", e);
						continue;
					}
				}
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
	pub fn new(addr: String) -> Self {
		let (connect_sender, connect_reciever) =
			async_channel::unbounded::<Result<TcpStream, NetworkError>>();
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

fn check_connect(
	client: Option<ResMut<NetworkClient>>,
	registered_recievers: Option<ResMut<RegisteredRecievers>>,
	mut net_errors: EventWriter<NetworkError>,
) {
	if let Some(mut c) = client {
		if let Ok(x) = c.connection_event_reciever.try_recv() {
			match x {
				Ok(stream) => {
					if let Some(registered_recievers) = registered_recievers {
						if let Some(packets) = registered_recievers.registered_recievers.clone() {
							let (socket_reader, socket_writer) = stream.into_split();

							let (send_write_buf, recieve_write_buf) =
								async_channel::unbounded::<WriteBuf>();

							let mut sender = BTreeMap::new();
							let mut reciever = BTreeMap::new();
							for packet in packets {
								let (send_read_buf, recieve_read_buf) =
									async_channel::unbounded::<ReadBuffer>();
								sender.insert(packet, send_read_buf);
								reciever.insert(packet, recieve_read_buf);
							}

							c.runtime.spawn(read_messages(socket_reader, sender));
							c.runtime
								.spawn(dispatch_messages(socket_writer, recieve_write_buf));

							c.net_thread_connection = Some(NetThreadConnection {
								send_write_buf,
								reciever,
							})
						}
					}
				}
				Err(e) => net_errors.send(e),
			}
		}
	}
}

pub fn check_read<const INDEX: u16>(
	client: Option<ResMut<NetworkClient>>,
	mut resources: EventWriter<EventReadBuffer<INDEX>>,
) {
	if let Some(c) = client {
		if let Some(connection) = &(&c).net_thread_connection {
			while let Ok(x) = connection.reciever.get(&INDEX).unwrap().try_recv() {
				resources.send(EventReadBuffer {
					read_buffer: Mutex::new(x),
				});
			}
		}
	}
}

pub fn handle_write(client: Option<ResMut<NetworkClient>>, mut resources: EventReader<WriteBuf>) {
	if let Some(c) = client {
		if let Some(connection) = &(&c).net_thread_connection {
			for i in resources.iter() {
				// TODO: Remove copy here
				connection.send_write_buf.try_send(i.to_owned()).unwrap();
			}
		}
	}
}

/// A utility trait on [`App`] to easily register [`ServerPacket`]s
pub trait AppNetworkClientMessage {
	/// Register a server packet
	///
	/// To use:
	/// ```rust
	/// app.listen_for_client_message::<{ ServerPacket::ServerInfo as u16 }>().add_system(on_server_info);
	///
	/// fn on_server_info(mut events: EventReader<EventReadBuffer<{ ServerPacket::CountryInfo as u16 }>>) {
	/// 	for event in events.iter() {
	///			...
	///		}
	///	}
	/// ```
	fn net_listen<const INDEX: u16>(&mut self) -> &mut Self;
}

impl AppNetworkClientMessage for App {
	fn net_listen<const INDEX: u16>(&mut self) -> &mut Self {
		info!("Registered a new server packet: {}", INDEX);
		self.add_event::<EventReadBuffer<INDEX>>();

		self.add_system_to_stage(CoreStage::PreUpdate, check_read::<INDEX>);

		self.init_resource::<RegisteredRecievers>();

		if let Some(mut network_client) = self.world.get_resource_mut::<RegisteredRecievers>() {
			if let Some(registered_recievers) = &mut network_client.registered_recievers {
				registered_recievers.push(INDEX);
			} else {
				network_client.registered_recievers = Some(vec![INDEX]);
			}
		} else {
			error!("Failed to register server packet because NetworkClient was not yet inserted.");
		}

		self
	}
}
