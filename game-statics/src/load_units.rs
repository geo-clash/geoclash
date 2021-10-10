use super::unit_data::UnitType;
use nanoserde::DeRon;

fn load_units() -> Vec<UnitType> {
	let units: Vec<UnitType> = DeRon::deserialize_ron(include_str!("../assets/Units.ron")).unwrap();
	units
}

lazy_static! {
	pub static ref UNIT_TYPES: Vec<UnitType> = load_units();
}
