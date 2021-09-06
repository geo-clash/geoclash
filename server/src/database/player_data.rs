use net::packets::*;

// Argon2 with default params (Argon2id v19)
lazy_static! {
	static ref ARGON: argon2::Argon2<'static> = argon2::Argon2::default();
}

#[derive(Debug)]
pub struct PlayerData {
	pub username: String,
	pub argon: String,
	pub countries: Vec<CountryId>,
}
impl PlayerData {
	pub fn new(auth: Authentication) -> Self {
		Self {
			username: auth.username,
			argon: {
				use argon2::{password_hash::SaltString, PasswordHasher};
				use rand_core::OsRng;
				let salt = SaltString::generate(&mut OsRng);

				ARGON
					.hash_password_simple(auth.password.as_bytes(), &salt)
					.unwrap()
					.to_string()
			},
			countries: Vec::new(),
		}
	}
	// TODO: elaborate to force users to have numbers / capitals / etc
	pub fn pass_secure(pass: &str) -> bool {
		pass.len() > 5
	}
	pub fn check_pass(&self, pass: String) -> bool {
		use argon2::{PasswordHash, PasswordVerifier};
		let parsed_hash = PasswordHash::new(&self.argon).unwrap();
		ARGON.verify_password(pass.as_bytes(), &parsed_hash).is_ok()
	}
}
