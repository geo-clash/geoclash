use server_net::*;
mod database;
use database::*;
use std::sync::Arc;
#[macro_use]
extern crate lazy_static;

#[derive(PartialEq, Eq, Hash)]
struct Country {
	owner: UserId,
}

pub fn evaluate(packet: ClientPackets, db: &Arc<Database>) -> ServerPackets {
	use ClientPackets::*;
	match packet {
		Connect => ServerPackets::ServerInfo(ServerInfo {
			name: "Alpha server".to_string(),
			description: "The testing server".to_string(),
			host: "James".to_string(),
		}),
		Login { username, password } => {
			let mut players = db.players.lock().unwrap();
			let username = username.to_lowercase();
			if let Some(player_data) = players
				.iter_mut()
				.find(|p| p.name.to_lowercase() == username)
			{
				if player_data.check_pass(password) {
					ServerPackets::SucessfulLogin
				} else {
					ServerPackets::InvalidLogin {
						error: "Hashed passwords do not match".to_string(),
					}
				}
			} else {
				ServerPackets::InvalidLogin {
					error: "Username not found".to_string(),
				}
			}
		}
		SignUp { username, password } => {
			let mut players = db.players.lock().unwrap();
			let username = username.to_lowercase();
			let taken = players
				.iter()
				.find(|p| p.name.to_lowercase() == username)
				.is_some();
			if taken {
				ServerPackets::InvalidSignup {
					error: "Username taken".to_string(),
				}
			} else if PlayerData::pass_secure(&password) {
				ServerPackets::InvalidSignup {
					error: "Password too short".to_string(),
				}
			} else {
				players.push(PlayerData::new(username, password));
				ServerPackets::SucessfulSignup
			}
		}
		RequestCountryInfo { country: _ } => todo!(),
	}
}

fn main() {
	let db = Database::construct();

	let rt = Runtime::new().unwrap();
	rt.block_on(server("127.0.0.1:2453", evaluate, db)).unwrap();
}
