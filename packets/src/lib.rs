pub type CountryId = u16;
pub type UserId = u32;


use nanoserde::{DeBin, SerBin};

#[derive(Debug, SerBin, DeBin)]
pub enum ClientPackets {
    Connect,

    RequestServerInfo,

    Login { username: String, password: String },
    SignUp { username: String, password: String },

    RequestCountryInfo { country: CountryId },
}

#[derive(Debug, SerBin, DeBin)]
pub enum ServerPackets {
    ServerInfo {
        name: String,
        description: String,
        host: String,
    },

    InvalidLogin {
        error: String,
    },
    InvalidSignup {
        error: String,
    },
    SucessfulLogin,
    SucessfulSignup,

    CountryInfo {
        name: String,
    },
}
