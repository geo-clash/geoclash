use nanoserde::DeRon;

#[derive(DeRon, Debug)]
pub struct CountryData {
    pub name: String,
    pub gdp: i64,
    pub population: i64,
    pub long: f32,
    pub lat: f32,
}