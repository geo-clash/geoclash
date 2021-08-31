#[macro_use]
extern crate lazy_static;

mod country_data;
pub use country_data::CountryData;

mod load_countries;
pub use load_countries::COUNTRIES;

mod building_data;
pub use building_data::{BuildingData, BuildingEffects};

mod load_buildings;
pub use load_buildings::BUILDINGS;
