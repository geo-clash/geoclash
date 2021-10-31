#[macro_use]
extern crate lazy_static;

pub type Money = i64;
pub type UnitClassId = u8;
pub type UnitId = u32;

mod country_data;
pub use country_data::*;

mod load_countries;
pub use load_countries::*;

mod building_data;
pub use building_data::*;

mod load_buildings;
pub use load_buildings::*;

mod unit_data;
pub use unit_data::*;

mod load_units;
pub use load_units::*;
