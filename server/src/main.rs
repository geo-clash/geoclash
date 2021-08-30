use server_net::*;
mod database;
use database::*;
use std::sync::{Arc};
#[macro_use]
extern crate lazy_static;

#[derive(PartialEq, Eq, Hash)]
struct Country {
    owner: UserId,
}

pub fn evaluate(packet: ClientPackets, db: &Arc<Database>) -> ServerPackets {
    use ClientPackets::*;
    match packet {
        Connect => {
            let mut players = db.players.lock().unwrap();
            players.push(PlayerData::new("hi".to_string(), "lo".to_string()));
            println!("Players {:?}",players);
            ServerPackets::CountryInfo {
                name: "hey there ".to_string(),
            }
        }
        RequestServerInfo => ServerPackets::ServerInfo {
            name: "Alpha server".to_string(),
            description: "The testing server".to_string(),
            host: "James".to_string(),
        },
        Login { username:_, password:_ } => todo!(),
        SignUp { username:_, password:_ } => todo!(),
        RequestCountryInfo { country:_ } => todo!(),
    }
}

fn main() {
    let db = Database::construct();

    let rt = Runtime::new().unwrap();
    rt.block_on(server("127.0.0.1:8080", evaluate, db)).unwrap();
}
