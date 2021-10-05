use nanoserde::DeRon;

use crate::Money;

#[derive(DeRon, Debug)]
pub enum BuildingEffects {
	MultiplyGdp(f64),
	AddTroopProduction(u64),
}

#[derive(DeRon, Debug)]
pub struct BuildingData {
	pub name: String,
	pub description: String,
	pub cost: Money,
	pub effects: Vec<BuildingEffects>,
}
