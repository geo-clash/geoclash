use server_net::CountryId;

// Argon2 with default params (Argon2id v19)
lazy_static! {
    static ref ARGON: argon2::Argon2<'static> = argon2::Argon2::default();
}

#[derive(Debug)]
pub struct PlayerData {
    pub name: String,
    pub argon: String,
    pub countries: Vec<CountryId>,
}
impl PlayerData {
    pub fn new(user: String, pass: String) -> Self {
        Self {
            name: user,
            argon: {
                use argon2::{password_hash::SaltString, PasswordHasher};
                use rand_core::OsRng;
                let salt = SaltString::generate(&mut OsRng);

                ARGON
                    .hash_password_simple(pass.as_bytes(), &salt)
                    .unwrap()
                    .to_string()
            },
            countries: Vec::new(),
        }
    }
    pub fn check_pass(&self, pass: String) -> bool {
        use argon2::{PasswordHash, PasswordVerifier};
        let parsed_hash = PasswordHash::new(&pass).unwrap();
        ARGON
            .verify_password("hi".to_string().as_bytes(), &parsed_hash)
            .is_ok()
    }
}
