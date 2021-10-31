use async_channel::{Receiver, Sender};
use connections::Connections;
use net::{packets::*, Runtime};
mod server_net;
use server_net::server;
mod database;
use database::*;

mod connections;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
use simple_logger::SimpleLogger;

use crate::connections::Connection;

#[derive(PartialEq, Eq, Hash)]
struct Country {
	owner: UserId,
}

async fn handle_packet(
	read_buffer: &mut ReadBuffer,
	client_id: usize,
	write_buf_sender: Sender<WriteBuf>,
	db: &mut Database,
	connections: &mut Connections,
) -> Result<(), ReadValueError> {
	let packet = read_buffer.read_client_packet()?;
	info!("Recieved packet {:?} from client", packet);

	match packet {
		ClientPacket::Connect => {
			assert_eq!(client_id, connections.len() as usize);
			connections.push(Connection {
				user_id: None,
				write_buf_sender: Some(write_buf_sender.clone()),
			});
			write_buf_sender
				.send(
					WriteBuf::new_server_packet(ServerPacket::ServerInfo).push(ServerInfo {
						name: "Alpha server".to_string(),
						description: "The testing server".to_string(),
						host: "James".to_string(),
					}),
				)
				.await
				.unwrap();
		}
		ClientPacket::Disconnect => {
			info!("Client disconnected.");
			connections[client_id].write_buf_sender = None;
		}
		ClientPacket::SignUp => {
			if connections[client_id].user_id.is_some() {
				info!("Sign up request by already authenticated user");
				write_buf_sender
					.send(WriteBuf::new_server_packet(
						ServerPacket::AlreadyAuthenticated,
					))
					.await
					.unwrap();
			}
			let auth = Authentication::deserialize(read_buffer)?;
			info!("signup Auth {:?}", auth);
			let success = if PlayerData::pass_secure(&auth) {
				db.get_player_by_username(&auth.username).is_none()
			} else {
				false
			};
			write_buf_sender
				.send(if success {
					WriteBuf::new_server_packet(ServerPacket::InitialUnits).push(db.initial_state())
				} else {
					WriteBuf::new_server_packet(ServerPacket::InvalidAuth)
				})
				.await
				.unwrap();
			if success {
				let player_id = db.new_player(auth, connections);
				connections[client_id].user_id = Some(player_id);
				write_buf_sender
					.send(WriteBuf::new_server_packet(ServerPacket::SuccessfulAuth))
					.await
					.unwrap();
			}
		}
		ClientPacket::Login => {
			if connections[client_id].user_id.is_some() {
				info!("log in request by already authenticated user");
				write_buf_sender
					.send(WriteBuf::new_server_packet(
						ServerPacket::AlreadyAuthenticated,
					))
					.await
					.unwrap();
			}
			let auth = Authentication::deserialize(read_buffer)?;
			info!("login Auth {:?}", auth);
			let success = if let Some((player_id, user)) = db.get_player_by_username(&auth.username)
			{
				if user.check_pass(auth.password) {
					connections[client_id].user_id = Some(player_id);
					true
				} else {
					false
				}
			} else {
				false
			};
			if success {
				write_buf_sender
					.send(WriteBuf::new_server_packet(ServerPacket::SuccessfulAuth))
					.await
					.unwrap();
			}
			write_buf_sender
				.send(if success {
					WriteBuf::new_server_packet(ServerPacket::InitialUnits).push(db.initial_state())
				} else {
					WriteBuf::new_server_packet(ServerPacket::InvalidAuth)
				})
				.await
				.unwrap();
		}
		ClientPacket::MoveUnit => {
			let user_id = match connections[client_id].user_id {
				Some(x) => x,
				None => {
					info!("Move unit request from non authenticated user");
					write_buf_sender
						.send(WriteBuf::new_server_packet(ServerPacket::NotAuthenticated))
						.await
						.unwrap();
					return Ok(());
				}
			};
			let move_request = MoveUnit::deserialize(read_buffer)?;
			let unit = match db.get_unit(move_request.unit) {
				Some(x) => x,
				None => {
					info!("Client tried to move non existant unit");
					write_buf_sender
						.send(WriteBuf::new_server_packet(
							ServerPacket::UnitNotControllable,
						))
						.await
						.unwrap();
					return Ok(());
				}
			};
			if unit.owner != user_id {
				info!("Client tried to move opponent's unit!");
				write_buf_sender
					.send(WriteBuf::new_server_packet(
						ServerPacket::UnitNotControllable,
					))
					.await
					.unwrap();
				return Ok(());
			}
			let (current_position, start_time) = unit.set_destination(&move_request.destination);
			let dispatch =
				WriteBuf::new_server_packet(ServerPacket::SetDestination).push(SetDestination {
					unit: move_request.unit,
					current_position,
					destination: move_request.destination,
					start_time,
				});
			for connection in connections {
				if let Some(sender) = connection.write_buf_sender.clone() {
					if let Err(_) = sender.try_send(dispatch.clone()) {
						connection.write_buf_sender = None;
					}
				}
			}
		}
		ClientPacket::RequestCountryInfo => todo!(),
	}
	Ok(())
}

async fn handle_packets<'a>(read_buf_reciever: Receiver<((ReadBuffer, usize), Sender<WriteBuf>)>) {
	let mut db = Database::construct();
	let mut connections = Vec::new();
	while let Ok(mut data) = read_buf_reciever.recv().await {
		handle_packet(
			&mut (data.0).0,
			(data.0).1,
			data.1,
			&mut db,
			&mut connections,
		)
		.await
		.unwrap_or_else(|e| error!("error reading data from client {}", e))
	}
}

fn main() {
	SimpleLogger::new().init().unwrap();

	info!("Spawning runtime");

	let rt = Runtime::new().unwrap();
	let (read_buf_sender, read_buf_reciever) =
		async_channel::unbounded::<((ReadBuffer, usize), Sender<WriteBuf>)>();

	rt.spawn(server("0.0.0.0:2453", read_buf_sender));

	rt.block_on(handle_packets(read_buf_reciever));
}
