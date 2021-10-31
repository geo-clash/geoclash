use bevy::prelude::*;
use client_net::{InitialUnit, SetDestination};
use game_statics::{UnitClassId, UnitId};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, PartialEq)]
pub struct Unit {
	pub id: UnitId,
	start: Quat,
	end: Quat,
	start_time: u128,
	duration: u128,
	class: UnitClassId,
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
	#[allow(dead_code)]
	pub fn new(start: Vec3, end: Vec3, class: UnitClassId) -> Self {
		let (start, end) = (
			Quat::from_axis_angle(start, 0.),
			Quat::from_axis_angle(end, 0.),
		);
		Self {
			id: 0,
			start,
			end,
			start_time: Self::time(),
			duration: Self::duration(start, end, class),
			class,
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
	pub fn get_position(&self) -> Vec3 {
		self.get_quat_position().to_axis_angle().0
	}
	#[allow(dead_code)]
	pub fn set_destination(&mut self, end: &Vec3) {
		self.start = self.get_quat_position();
		self.end = Quat::from_axis_angle(*end, 0.);
		self.start_time = Self::time();
		self.duration = Self::duration(self.start, self.end, self.class);
	}
	pub fn set_destination_net(&mut self, destination: &SetDestination) {
		self.start = destination.current_position;
		self.end = destination.destination;
		self.start_time = destination.start_time;
		self.duration = Self::duration(self.start, self.end, self.class);
		info!(
			"Duration {} start {:?} end {:?}",
			self.duration, self.start, self.end
		);
	}
	pub fn from_initial_unit(initial_unit: &InitialUnit) -> Self {
		let (start, end) = (initial_unit.start, initial_unit.end);
		Self {
			id: initial_unit.id,
			start,
			end,
			start_time: initial_unit.start_time,
			duration: Self::duration(start, end, initial_unit.class),
			class: initial_unit.class,
		}
	}
}
