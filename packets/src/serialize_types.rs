use glam::{Quat, Vec3};

use crate::Serializable;

use std::{
	fmt,
	time::{SystemTime, UNIX_EPOCH},
};

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
	start: Quat,
	end: Quat,
	start_time: u128,
	duration: u128,
	class: usize,
}

impl Unit {
	fn time() -> u128 {
		SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap()
			.as_millis()
	}
	fn duration(start: Quat, end: Quat, class: usize) -> u128 {
		(start.angle_between(end) / game_statics::UNIT_TYPES[class as usize].speed * 1000.) as u128
	}
	pub fn new(start: Vec3, end: Vec3, class: usize) -> Self {
		let (start, end) = (Quat::from_scaled_axis(start), Quat::from_scaled_axis(end));
		Self {
			start,
			end,
			start_time: Self::time(),
			duration: Self::duration(start, end, class),
			class,
		}
	}
	pub fn get_position(&self) -> Vec3 {
		let time = ((Self::time() - self.start_time) as f32) / self.duration as f32;
		let time = time % 1.;
		self.start.lerp(self.end, time).to_axis_angle().0
	}
}

impl Serializable for Unit {
	fn serialize(&self, buf: &mut Vec<u8>) {
		self.start.serialize(buf);
		self.end.serialize(buf);
		self.start_time.serialize(buf);
		(self.class as u32).serialize(buf);
	}

	fn deserialize(buf: &mut crate::ReadBuffer) -> Result<Self, crate::error::ReadValueError> {
		let (start, end) = (Quat::deserialize(buf)?, Quat::deserialize(buf)?);
		let start_time = u128::deserialize(buf)?;
		let class = u32::deserialize(buf)? as usize;
		Ok(Self {
			start,
			end,
			start_time,
			duration: Self::duration(start, end, class),
			class,
		})
	}
}

#[test]
fn test_get_position() {
	let japan = Vec3::new(0.52484196, 0.5836691, -0.6195735);
	let germany = Vec3::new(0.14106606, 0.79356587, 0.59190667);
	let mut unit = Unit::new(japan, germany, 0);
	assert_eq!(unit.duration, 1255);
	assert_eq!(unit.get_position(), japan);
	unit.start_time -= unit.duration;
	assert_eq!(unit.get_position(), germany);
}
