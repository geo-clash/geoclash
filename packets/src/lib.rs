pub type CountryId = u16;
pub type UserId = u32;

mod serializable;
pub use error::ReadValueError;
pub use serializable::ReadBuffer;
pub use serializable::Serializable;
mod serialize_types;
pub use serialize_types::*;

mod error;

#[macro_use]
mod packet_enum;

packet_enum! { ClientPacket;
	Connect,
	Login,
	SignUp,
	RequestCountryInfo
}

packet_enum! { ServerPacket;
	Connect,
	PacketLengthInvalid,
	ServerInfo,
	InvalidLogin,
	InvalidSignup,
	SucessfulLogin,
	SucessfulSignup,
	CountryInfo
}

#[derive(Clone, Debug, Default)]
pub struct WriteBuf(Vec<u8>);

impl WriteBuf {
	pub fn new_server_packet(packet: ServerPacket) -> Self {
		Self(u16::to_be_bytes(packet as u16).to_vec())
	}
	pub fn new_client_packet(packet: ClientPacket) -> Self {
		Self(u16::to_be_bytes(packet as u16).to_vec())
	}
	pub fn push(mut self, value: impl Serializable) -> Self {
		value.serialize(&mut self.0);
		self
	}
	pub fn inner(&self) -> &[u8] {
		&self.0
	}
	pub fn inner_mut(&mut self) -> &mut Vec<u8> {
		&mut self.0
	}
}

impl ReadBuffer {
	pub fn read_server_packet(&mut self) -> Result<ServerPacket, ReadValueError> {
		let index: u16 = Serializable::deserialize(self)?;
		ServerPacket::from_index(index)
	}
	pub fn read_client_packet(&mut self) -> Result<ClientPacket, ReadValueError> {
		let index: u16 = Serializable::deserialize(self)?;
		ClientPacket::from_index(index)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn serialize_enum() {
		assert_eq!(
			WriteBuf::new_server_packet(ServerPacket::ServerInfo).inner(),
			&[0, 2]
		);
	}
	#[test]
	fn serialize_u8() {
		assert_eq!(
			WriteBuf::new_server_packet(ServerPacket::ServerInfo)
				.push(6_u8)
				.inner(),
			&[0, 2, 6]
		);
	}
	#[test]
	fn serialize_i128() {
		assert_eq!(
			WriteBuf::new_server_packet(ServerPacket::ServerInfo)
				.push(-90_i128)
				.inner(),
			&[
				0, 2, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
				166
			]
		);
	}

	#[test]
	fn serialize_str() {
		assert_eq!(
			WriteBuf::new_server_packet(ServerPacket::ServerInfo)
				.push(String::from("hello"))
				.inner(),
			&[0, 2, 0, 5, 104, 101, 108, 108, 111]
		);
	}

	#[test]
	fn deserialize_enum() {
		assert_eq!(
			ReadBuffer::new(vec![0, 3]).read_server_packet().unwrap(),
			ServerPacket::InvalidLogin
		);
	}
	#[test]
	fn deserialize_u8() {
		assert_eq!(
			u8::deserialize(&mut ReadBuffer::new(vec![6])).unwrap(),
			6_u8
		);
	}
	#[test]
	fn deserialize_i128() {
		assert_eq!(
			i128::deserialize(&mut ReadBuffer::new(vec![
				255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 166
			]))
			.unwrap(),
			-90_i128
		);
	}

	#[test]
	fn deserialize_str() {
		assert_eq!(
			String::deserialize(&mut ReadBuffer::new(vec![0, 5, 104, 101, 108, 108, 111])).unwrap(),
			String::from("hello")
		);
	}

	#[test]
	fn server_info() {
		let server_info = ServerInfo {
			name: "test".to_string(),
			description: "test descrip".to_string(),
			host: "test_host".to_string(),
		};
		let f = WriteBuf::new_server_packet(ServerPacket::CountryInfo).push(server_info.clone());

		let mut reader = ReadBuffer::new(f.0);

		assert_eq!(
			reader.read_server_packet().unwrap(),
			ServerPacket::CountryInfo
		);

		assert_eq!(ServerInfo::deserialize(&mut reader).unwrap(), server_info);
	}

	#[test]
	fn str_too_long() {
		assert_eq!(
			String::deserialize(&mut ReadBuffer::new(vec![0, 6, 104, 101, 108, 108, 111])),
			Err(ReadValueError::BufferToShort(6, "String value"))
		);
	}
}
