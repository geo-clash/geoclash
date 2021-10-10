use nanoserde::DeRon;

use crate::Money;

#[derive(DeRon, Debug)]
pub struct CountryData {
	pub name: String,
	pub gdp: Money,
	pub population: i64,
	pub long: f32,
	pub lat: f32,
}
