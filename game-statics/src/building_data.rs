use nanoserde::DeRon;

#[derive(DeRon, Debug)]
pub enum BuildingEffects {
	MultiplyGdp(f64),
	AddTroopProduction(u64),
}

#[derive(DeRon, Debug)]
pub struct BuildingData {
	pub name: String,
	pub description: String,
	pub cost: i128,
	pub effects: Vec<BuildingEffects>,
}