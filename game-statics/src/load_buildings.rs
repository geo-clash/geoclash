use super::building_data::BuildingData;
use nanoserde::DeRon;

fn load_buildings() -> Vec<BuildingData> {
	let buildings: Vec<BuildingData> = DeRon::deserialize_ron(include_str!("../assets/Buildings.ron")).unwrap();
	buildings
}

lazy_static! {
	pub static ref BUILDINGS: Vec<BuildingData> = load_buildings();
}
