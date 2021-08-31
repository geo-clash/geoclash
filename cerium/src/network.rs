use bevy::prelude::*;
use client_net::*;

use crate::info;

pub fn send_client_network_events(
	client_server: ResMut<NetworkClient>,
	mut client_network_events: EventWriter<ClientNetworkEvent>,
) {
	client_network_events.send_batch(client_server.network_events.receiver.try_iter());
}

pub fn handle_connection_event(
	mut net_res: ResMut<NetworkClient>,
	mut events: EventWriter<ClientNetworkEvent>,
) {
	let (connection, peer_addr) = match net_res.connection_events.receiver.try_recv() {
		Ok(event) => event,
		Err(_err) => {
			return;
		}
	};

	let (read_socket, send_socket) = connection.into_split();
	//let recv_message_map = net_res.recv_message_map.clone();
	let (send_message, recv_message) = unbounded_channel();
	let network_event_sender = net_res.network_events.sender.clone();
	let network_event_sender_two = net_res.network_events.sender.clone();

	net_res.server_connection = Some(ServerConnection {
		peer_addr,
		send_task: net_res.runtime.spawn(async move {
			let mut recv_message = recv_message;
			let mut send_socket = send_socket;

			debug!("Starting new server connection, sending task");

			while let Some(message) = recv_message.recv().await {
				let encoded = SerBin::serialize_bin(&message);

				let len = encoded.len();
				info!("Sending a new message of size: {}", len);

				match send_socket.write_u32(len as u32).await {
					Ok(_) => (),
					Err(err) => {
						error!("Could not send packet length: {:?}: {}", len, err);
						break;
					}
				}

				match send_socket.write_all(&encoded).await {
					Ok(_) => (),
					Err(err) => {
						error!("Could not send packet: {:?}: {}", message, err);
						break;
					}
				}

				trace!("Succesfully written all!");
			}

			let _ = network_event_sender_two.send(ClientNetworkEvent::Disconnected);
		}),
		receive_task: net_res.runtime.spawn(async move {
			let mut read_socket = read_socket;
			let mut buffer: Vec<u8> = vec![0; 1000];
			// let recv_message_map = recv_message_map;

			loop {
				let length = match read_socket.read_u32().await {
					Ok(len) => len as usize,
					Err(err) => {
						if err.kind() == std::io::ErrorKind::UnexpectedEof {
							error!("Server forcably disconnected");
							return;
						}
						error!(
							"Encountered error while fetching length [{}]: {}",
							peer_addr, err
						);
						break;
					}
				};

				match read_socket.read_exact(&mut buffer[..length]).await {
					Ok(_) => (),
					Err(err) => {
						error!(
							"Encountered error while fetching stream of length {} [{}]: {}",
							length, peer_addr, err
						);
						break;
					}
				}

				let packet: ServerPackets = match DeBin::deserialize_bin(&buffer[..length]) {
					Ok(packet) => packet,
					Err(err) => {
						error!(
							"Failed to decode network packet from [{}]: {}",
							peer_addr, err
						);
						break;
					}
				};

				/*match recv_message_map.get_mut(&packet.kind[..]) {
					Some(mut packets) => packets.push(packet.data),
					None => {
						error!(
							"Could not find existing entries for message kinds: {:?}",
							packet
						);
					}
				}*/
				info!("Received message from: {}:    {:?}", peer_addr, packet);
			}

			let _ = network_event_sender.send(ClientNetworkEvent::Disconnected);
		}),
		send_message,
	});

	events.send(ClientNetworkEvent::Connected);
}

fn connect(mut net_res: ResMut<NetworkClient>) {
	info!("connecting....!");
	net_res.connect("127.0.0.1:2453");
}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(NetworkClient::new())
			.add_event::<ClientNetworkEvent>()
			.add_system_to_stage(CoreStage::PreUpdate, send_client_network_events)
			.add_system_to_stage(CoreStage::PreUpdate, handle_connection_event)
			.add_startup_system(connect);
	}
}
