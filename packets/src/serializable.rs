use glam::Vec3;

use crate::error::ReadValueError;
use std::convert::TryInto;

pub struct ReadBuffer {
	buffer: Vec<u8>,
	position: usize,
}

impl<'a> ReadBuffer {
	pub fn new(buffer: Vec<u8>) -> Self {
		Self {
			buffer,
			position: 0,
		}
	}
	pub fn read_array<const COUNT: usize>(
		&mut self,
		type_name: &'static str,
	) -> Result<[u8; COUNT], ReadValueError> {
		self.position += COUNT;
		match self.buffer[self.position - COUNT..self.position].try_into() {
			Ok(x) => Ok(x),
			Err(_) => Err(ReadValueError::BufferToShort(COUNT, type_name)),
		}
	}
	pub fn read_vec(
		&mut self,
		bytes: usize,
		type_name: &'static str,
	) -> Result<Vec<u8>, ReadValueError> {
		if self.position + bytes > self.buffer.len() {
			Err(ReadValueError::BufferToShort(bytes, type_name))
		} else {
			self.position += bytes;
			Ok(self.buffer[self.position - bytes..self.position].to_vec())
		}
	}
	pub fn get_pos(&self) -> usize {
		self.position
	}
}

pub trait Serializable {
	fn serialize(&self, buf: &mut Vec<u8>);
	fn deserialize(buf: &mut ReadBuffer) -> Result<Self, ReadValueError>
	where
		Self: Sized;
}

macro_rules! impl_num_serializable {
	($($struct_name:ident),*) => {
		// block to be repeated
		$(
			impl Serializable for $struct_name {
				fn serialize(&self, buf: &mut Vec<u8>) {
					buf.extend_from_slice(&self.to_be_bytes())
				}
				fn deserialize(buf: &mut ReadBuffer) -> Result<Self, ReadValueError>{
					Ok($struct_name::from_be_bytes(buf.read_array(std::any::type_name::<$struct_name>())?))
				}
			}
		)*
	};
}

impl_num_serializable! { u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64 }

impl Serializable for String {
	fn serialize(&self, buf: &mut Vec<u8>) {
		Serializable::serialize(&(self.len() as u16), buf);
		buf.extend_from_slice(self.as_bytes())
	}
	fn deserialize(buf: &mut ReadBuffer) -> Result<Self, ReadValueError> {
		let len = u16::from_be_bytes(buf.read_array("String len (u16)")?) as usize;
		String::from_utf8(buf.read_vec(len, "String value")?)
			.map_err(|_| ReadValueError::StringParseError)
	}
}

impl Serializable for Vec3 {
	fn serialize(&self, buf: &mut Vec<u8>) {
		buf.extend_from_slice(&self.x.to_be_bytes());
		buf.extend_from_slice(&self.y.to_be_bytes());
		buf.extend_from_slice(&self.z.to_be_bytes());
	}
	fn deserialize(buf: &mut ReadBuffer) -> Result<Self, ReadValueError> {
		Ok(Vec3::new(
			f32::deserialize(buf)?,
			f32::deserialize(buf)?,
			f32::deserialize(buf)?,
		))
	}
}
