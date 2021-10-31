//! Contains the [`Unit`] type, a server's representation of a unit

use game_statics::{UnitClassId, UnitId};
use glam::{Quat, Vec3};
use net::packets::InitialUnit;
use std::time::{SystemTime, UNIX_EPOCH};

/// Contains the server's representation of a unit
#[derive(Clone, Debug, PartialEq)]
pub struct Unit {
	start: Quat,
	end: Quat,
	start_time: u128,
	duration: u128,
	class: UnitClassId,
	pub owner: usize,
}

impl Unit {
	pub fn time() -> u128 {
		SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap()
			.as_millis()
	}
	fn duration(start: Quat, end: Quat, class: UnitClassId) -> u128 {
		(start.angle_between(end) / game_statics::UNIT_TYPES[class as usize].speed * 1000.) as u128
	}
	pub fn new(start: Vec3, end: Vec3, class: UnitClassId, owner: usize) -> Self {
		let (start, end) = (
			Quat::from_axis_angle(start, 1.),
			Quat::from_axis_angle(end, 1.),
		);
		Self {
			start,
			end,
			start_time: Self::time(),
			duration: Self::duration(start, end, class),
			class,
			owner,
		}
	}
	fn get_quat_position(&self) -> Quat {
		let time = ((Self::time() - self.start_time) as f32) / self.duration as f32;
		if time > 1. {
			self.end
		} else {
			self.start.lerp(self.end, time)
		}
	}
	pub fn set_destination(&mut self, end: &Quat) -> (Quat, u128) {
		self.start = self.get_quat_position();
		self.end = *end;
		self.start_time = Self::time();
		self.duration = Self::duration(self.start, self.end, self.class);
		info!(
			"Duration {} start {:?} end {:?}",
			self.duration, self.start, self.end
		);
		(self.start, self.start_time)
	}
	pub fn to_initial_unit(&self, id: UnitId) -> InitialUnit {
		InitialUnit {
			id,
			start: self.start,
			end: self.end,
			start_time: self.start_time,
			class: self.class,
		}
	}
}
