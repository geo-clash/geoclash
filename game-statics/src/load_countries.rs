use super::country_data::CountryData;
use nanoserde::DeRon;

fn load_countries() -> Vec<CountryData> {
	let countries: Vec<CountryData> =
		DeRon::deserialize_ron(include_str!("../assets/Countries.ron")).unwrap();

	countries
}

lazy_static! {
	pub static ref COUNTRIES: Vec<CountryData> = load_countries();
}
