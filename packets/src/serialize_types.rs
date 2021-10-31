//! Contains types specific to the game that are sent over the network.

use glam::Quat;

use crate::Serializable;
use game_statics::{UnitClassId, UnitId};

use std::fmt;

/// Sent by the server to give the client information about the server to display to the user.
///
/// Contains a `name`, a `description` and a `host` string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServerInfo {
	pub name: String,
	pub description: String,
	pub host: String,
}

impl fmt::Display for ServerInfo {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"\n{}:\n    About: {}\n    Hosted by {}.",
			self.name, self.description, self.host
		)
	}
}

impl Serializable for ServerInfo {
	fn serialize(&self, buf: &mut Vec<u8>) {
		self.name.serialize(buf);
		self.description.serialize(buf);
		self.host.serialize(buf);
	}

	fn deserialize(buf: &mut crate::ReadBuffer) -> Result<Self, crate::error::ReadValueError> {
		Ok(Self {
			name: String::deserialize(buf)?,
			description: String::deserialize(buf)?,
			host: String::deserialize(buf)?,
		})
	}
}

/// Used by client for submitting sign up and log in requests
///
/// Contains a username and a password string
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Authentication {
	pub username: String,
	pub password: String,
}

impl Serializable for Authentication {
	fn serialize(&self, buf: &mut Vec<u8>) {
		self.username.serialize(buf);
		self.password.serialize(buf);
	}

	fn deserialize(buf: &mut crate::ReadBuffer) -> Result<Self, crate::error::ReadValueError> {
		Ok(Self {
			username: String::deserialize(buf)?,
			password: String::deserialize(buf)?,
		})
	}
}

/// Contains all data about a unit necessary for a client. Id, start and end locations, start time and class.
///
/// This is used for giving a client some initial data only.
#[derive(Clone, Debug, PartialEq)]
pub struct InitialUnit {
	pub id: UnitId,
	pub start: Quat,
	pub end: Quat,
	pub start_time: u128,
	pub class: UnitClassId,
}

impl Serializable for InitialUnit {
	fn serialize(&self, buf: &mut Vec<u8>) {
		self.id.serialize(buf);
		self.start.serialize(buf);
		self.end.serialize(buf);
		self.start_time.serialize(buf);
		self.class.serialize(buf);
	}

	fn deserialize(buf: &mut crate::ReadBuffer) -> Result<Self, crate::error::ReadValueError> {
		let id = UnitId::deserialize(buf)?;
		let (start, end) = (Quat::deserialize(buf)?, Quat::deserialize(buf)?);
		let start_time = u128::deserialize(buf)?;
		let class = UnitClassId::deserialize(buf)?;
		Ok(Self {
			id,
			start,
			end,
			start_time,
			class,
		})
	}
}

/// Data sent to client to init world state from blank.
///
/// Contains the current server time and a list of [`InitialUnit`]
#[derive(Clone, Debug, PartialEq)]
pub struct InitialState {
	pub time: u128,
	pub units: Vec<InitialUnit>,
}

impl Serializable for InitialState {
	fn serialize(&self, buf: &mut Vec<u8>) {
		self.time.serialize(buf);
		self.units.serialize(buf);
	}

	fn deserialize(buf: &mut crate::ReadBuffer) -> Result<Self, crate::error::ReadValueError> {
		Ok(Self {
			time: u128::deserialize(buf)?,
			units: Vec::deserialize(buf)?,
		})
	}
}

/// Packet send by the client when the user clicks to move a unit.
///
/// Contains simply the unit id and the destination. Time & current position are calculated by the server to prevent cheating.
#[derive(Clone, Debug, PartialEq)]
pub struct MoveUnit {
	pub unit: UnitId,
	pub destination: Quat,
}
impl Serializable for MoveUnit {
	fn serialize(&self, buf: &mut Vec<u8>) {
		self.unit.serialize(buf);
		self.destination.serialize(buf);
	}

	fn deserialize(buf: &mut crate::ReadBuffer) -> Result<Self, crate::error::ReadValueError> {
		Ok(Self {
			unit: u32::deserialize(buf)?,
			destination: Quat::deserialize(buf)?,
		})
	}
}

/// Packet sent by server to update all clients about a unit move.
/// Contains the unit id, destination and start time
#[derive(Clone, Debug, PartialEq)]
pub struct SetDestination {
	pub unit: UnitId,
	pub current_position: Quat,
	pub destination: Quat,
	pub start_time: u128,
}
impl Serializable for SetDestination {
	fn serialize(&self, buf: &mut Vec<u8>) {
		self.unit.serialize(buf);
		self.current_position.serialize(buf);
		self.destination.serialize(buf);
		self.start_time.serialize(buf);
	}

	fn deserialize(buf: &mut crate::ReadBuffer) -> Result<Self, crate::error::ReadValueError> {
		Ok(Self {
			unit: u32::deserialize(buf)?,
			current_position: Quat::deserialize(buf)?,
			destination: Quat::deserialize(buf)?,
			start_time: u128::deserialize(buf)?,
		})
	}
}
