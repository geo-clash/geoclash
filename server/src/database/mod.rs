mod player_data;

pub use player_data::PlayerData;

use std::sync::{Arc, Mutex};

/// The in-memory database shared amongst all clients.
///
/// This database will be shared via `Arc`, so to mutate the internal data we're
/// going to use a `Mutex` for interior mutability.
#[derive(Debug)]
pub struct Database {
    pub players: Mutex<Vec<PlayerData>>,
}
impl Database {
    pub fn construct() -> Arc<Database> {
        let x = Arc::new(Database {
            players: Mutex::new(Vec::new()),
        });
        x
    }
}
