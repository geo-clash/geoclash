use async_channel::{Receiver, Sender};
use net::{packets::*, Runtime};
mod server_net;
use server_net::server;
mod database;
use database::*;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
use simple_logger::SimpleLogger;

#[derive(PartialEq, Eq, Hash)]
struct Country {
	owner: UserId,
}

async fn handle_packet(
	read_buffer: &mut ReadBuffer,
	write_buf_sender: Sender<WriteBuf>,
	db: &mut Database,
) -> Result<(), ReadValueError> {
	let packet = read_buffer.read_client_packet()?;
	info!("Recieved packet {:?} from client", packet);

	match packet {
		ClientPacket::Connect => {
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
		ClientPacket::SignUp => {
			let auth = Authentication::deserialize(read_buffer)?;
			info!("signup Auth {:?}", auth);
			let response = if PlayerData::pass_secure(&auth) {
				if db.get_player_by_username(&auth.username).is_some() {
					ServerPacket::InvalidSignup
				} else {
					db.players.push(PlayerData::new(auth));
					ServerPacket::SucessfulSignup
				}
			} else {
				ServerPacket::InvalidSignup
			};
			write_buf_sender
				.send(WriteBuf::new_server_packet(response))
				.await
				.unwrap();
		}
		ClientPacket::Login => {
			let auth = Authentication::deserialize(read_buffer)?;
			info!("login Auth {:?}", auth);
			let response = if let Some(user) = db.get_player_by_username(&auth.username) {
				if user.check_pass(auth.password) {
					ServerPacket::SucessfulLogin
				} else {
					ServerPacket::InvalidLogin
				}
			} else {
				ServerPacket::InvalidLogin
			};
			write_buf_sender
				.send(WriteBuf::new_server_packet(response))
				.await
				.unwrap();
		}
		ClientPacket::RequestCountryInfo => todo!(),
	}
	Ok(())
}

async fn handle_packets<'a>(read_buf_reciever: Receiver<(ReadBuffer, Sender<WriteBuf>)>) {
	let mut db = Database::construct();
	while let Ok(mut data) = read_buf_reciever.recv().await {
		handle_packet(&mut data.0, data.1, &mut db)
			.await
			.unwrap_or_else(|e| error!("error reading data from client {}", e))
	}
}

fn main() {
	SimpleLogger::new().init().unwrap();

	info!("Spawning runtime");

	let rt = Runtime::new().unwrap();
	let (read_buf_sender, read_buf_reciever) =
		async_channel::unbounded::<(ReadBuffer, Sender<WriteBuf>)>();

	rt.spawn(server("0.0.0.0:2453", read_buf_sender));

	rt.block_on(handle_packets(read_buf_reciever));
}
