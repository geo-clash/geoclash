mod player_data;

pub use player_data::PlayerData;

/// The in-memory database shared amongst all clients.
#[derive(Debug)]
pub struct Database {
	pub players: Vec<PlayerData>,
}
impl Database {
	pub fn construct() -> Database {
		Database {
			players: Vec::new(),
		}
	}
}

impl Database {
	pub fn get_player_by_username(&self, username: &String) -> Option<&PlayerData> {
		let target = username.to_lowercase();
		self.players
			.iter()
			.find(|f| f.username.to_lowercase() == target)
	}
}
