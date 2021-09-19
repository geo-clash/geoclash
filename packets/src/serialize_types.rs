use glam::{Quat, Vec3};

use crate::Serializable;

use std::fmt;

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

#[derive(Clone, Debug, PartialEq)]
pub struct Unit {
	pub start: Vec3,
	pub end: Vec3,
	pub time: f32,
}

impl Unit {
	pub fn get_position(&self) -> Vec3 {
		let start = Quat::from_scaled_axis(self.start);
		let end = Quat::from_scaled_axis(self.end);
		start.lerp(end, self.time).to_axis_angle().0
	}
}

impl Serializable for Unit {
	fn serialize(&self, buf: &mut Vec<u8>) {
		self.start.serialize(buf);
		self.end.serialize(buf);
		self.time.serialize(buf);
	}

	fn deserialize(buf: &mut crate::ReadBuffer) -> Result<Self, crate::error::ReadValueError> {
		Ok(Self {
			start: Vec3::deserialize(buf)?,
			end: Vec3::deserialize(buf)?,
			time: f32::deserialize(buf)?,
		})
	}
}

#[test]
fn test_get_position() {
	let japan = Vec3::new(0.52484196, 0.5836691, -0.6195735);
	let germany = Vec3::new(0.14106606, 0.79356587, 0.59190667);
	let mut unit = Unit {
		start: japan,
		end: germany,
		time: 0.,
	};
	assert_eq!(unit.get_position(), japan);
	unit.time = 1.;
	assert_eq!(unit.get_position(), germany);
}
