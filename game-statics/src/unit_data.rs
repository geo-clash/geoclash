use nanoserde::DeRon;

use crate::Money;

#[derive(DeRon, Debug)]
pub enum UnitClass {
	Light,
	Heavy,
}

#[derive(DeRon, Debug)]
pub struct UnitType {
	pub name: String,
	pub description: String,
	pub class: UnitClass,
	pub cost: Money,
	pub speed: f32,
	pub range: f32,
	pub health: f32,
	pub health_recharge_sec: f32,
	pub dps_light: f32,
	pub dps_heavy: f32,
}
