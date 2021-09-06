use crate::Serializable;

use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServerInfo {
	pub name: String,
	pub description: String,
	pub host: String,
}

impl fmt::Display for ServerInfo {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"\n{}:\n    About: {}\n    Hosted by {}.",
			self.name, self.description, self.host
		)
	}
}

impl Serializable for ServerInfo {
	fn serialize(&self, buf: &mut Vec<u8>) {
		self.name.serialize(buf);
		self.description.serialize(buf);
		self.host.serialize(buf);
	}

	fn deserialize(buf: &mut crate::ReadBuffer) -> Result<Self, crate::error::ReadValueError> {
		Ok(Self {
			name: String::deserialize(buf)?,
			description: String::deserialize(buf)?,
			host: String::deserialize(buf)?,
		})
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Authentication {
	pub username: String,
	pub password: String,
}

impl Serializable for Authentication {
	fn serialize(&self, buf: &mut Vec<u8>) {
		self.username.serialize(buf);
		self.password.serialize(buf);
	}

	fn deserialize(buf: &mut crate::ReadBuffer) -> Result<Self, crate::error::ReadValueError> {
		Ok(Self {
			username: String::deserialize(buf)?,
			password: String::deserialize(buf)?,
		})
	}
}
