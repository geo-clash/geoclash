use net::packets::*;

// Argon username: (), password: ()  username: (), password: ()  username: (), password: () 2 with default params (Argon2id v19)
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
	pub fn pass_secure(authentication: &Authentication) -> bool {
		if authentication.password.len() > 5 {
			for i in 0..10 {
				if authentication
					.password
					.chars()
					.collect::<Vec<char>>()
					.contains(&char::from_digit(i, 10).unwrap())
				{
					if authentication.password != authentication.username {
						if authentication.password != authentication.password.to_lowercase() {
							return true;
						}
					}
				}
			}
		}
		false
	}
	pub fn check_pass(&self, pass: String) -> bool {
		use argon2::{PasswordHash, PasswordVerifier};
		let parsed_hash = PasswordHash::new(&self.argon).unwrap();
		ARGON.verify_password(pass.as_bytes(), &parsed_hash).is_ok()
	}
}

#[test]
fn test_pass() {
	assert_eq!(
		PlayerData::pass_secure(&Authentication {
			password: "J e f f r y 1 ".to_string(),
			username: "Jeffry2".to_string()
		}),
		true
	);
	assert_eq!(
		PlayerData::pass_secure(&Authentication {
			password: "aaaaaa".to_string(),
			username: "Jeffry2".to_string()
		}),
		false
	);
	assert_eq!(
		PlayerData::pass_secure(&Authentication {
			password: "abcd2A".to_string(),
			username: "abcd2A".to_string()
		}),
		false
	);
	assert_eq!(
		PlayerData::pass_secure(&Authentication {
			password: "hello1".to_string(),
			username: "mynameisbob".to_string()
		}),
		false
	);
	assert_eq!(
		PlayerData::pass_secure(&Authentication {
			password: "素敵なパスワード123A".to_string(),
			username: "japanesepasswordsRcool".to_string()
		}),
		true
	);
	assert_eq!(
		PlayerData::pass_secure(&Authentication {
			password: "Jeffry1".to_string(),
			username: "Jeffry2".to_string()
		}),
		true
	);
	assert_eq!(
		PlayerData::pass_secure(&Authentication {
			password: "A1b".to_string(),
			username: "Jeffry2".to_string()
		}),
		false
	);
}
