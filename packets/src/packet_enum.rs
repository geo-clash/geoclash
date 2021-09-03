macro_rules! packet_enum {
	($enum_name:ident; $($varient_name:ident),* ) => {
		#[derive(Debug, PartialEq, Eq)]
		pub enum $enum_name{
			// block to be repeated
			$(
				$varient_name,
			)*
		}
		impl $enum_name{
			pub fn from_index(index: u16) -> Result<$enum_name, crate::ReadValueError>{
				let mut i: u16 = 0;
				// block to be repeated
				$(
					i = i + 1;
					if index == i - 1 {
						return Ok($enum_name::$varient_name);
					}
				)*
				Err(crate::ReadValueError::InvalidPacketIndex(index))
			}
		}
	};
}
