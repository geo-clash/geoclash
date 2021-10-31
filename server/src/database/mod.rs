//! Provides the [`Database`] type

mod player_data;
mod unit;

use std::collections::HashMap;

use game_statics::UnitId;
use glam::Vec3;
use net::packets::{Authentication, InitialState, ServerPacket, WriteBuf};
pub use player_data::PlayerData;

use crate::connections::{ Connections};

use self::unit::Unit;

/// The in-memory database shared amongst all clients.
///
/// Contains the players and units.
#[derive(Debug)]
pub struct Database {
	pub players: Vec<PlayerData>,
	pub units: HashMap<UnitId, Unit>,
	unit_id: UnitId,
}
impl Database {
	pub fn construct() -> Database {
		Database {
			players: Vec::new(),
			units: HashMap::new(),
			unit_id: 0,
		}
	}
}

impl Database {
	pub fn get_player_by_username(
		&self,
		username: &String,
	) -> Option<(usize, &player_data::PlayerData)> {
		let target = username.to_lowercase();
		self.players
			.iter()
			.enumerate()
			.find(|(_, playerdata)| playerdata.username.to_lowercase() == target)
	}
	pub fn new_player(&mut self, auth: Authentication, connections: &mut Connections) -> usize {
		let player_id = self.players.len();
		self.players.push(PlayerData::new(auth));
		let new_unit = Unit::new(
			Vec3::new(0., 0., 1.),
			Vec3::new(0., 1., 0.),
			(self.unit_id % 2) as u8,
			player_id,
		);
		let dispatch = WriteBuf::new_server_packet(ServerPacket::NewUnit)
			.push(new_unit.to_initial_unit(self.unit_id));
		for connection in connections {
			if let Some(sender) = connection.write_buf_sender.clone() {
				if let Err(_) = sender.try_send(dispatch.clone()) {
					connection.write_buf_sender = None;
				}
			}
		}
		self.units.insert(self.unit_id, new_unit);
		self.unit_id += 1;

		player_id
	}
	pub fn initial_state(&self) -> InitialState {
		InitialState {
			time: Unit::time(),
			units: self
				.units
				.iter()
				.map(|(id, unit)| unit.to_initial_unit(*id))
				.collect(),
		}
	}
	pub fn get_unit(&mut self, id: UnitId) -> Option<&mut Unit> {
		self.units.get_mut(&id)
	}
}
