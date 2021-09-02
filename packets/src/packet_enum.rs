macro_rules! packet_enum {
	($enum_name:ident; $($varient_name:ident: $varient_index:expr),* ) => {
		#[derive(Debug, PartialEq, Eq)]
		pub enum $enum_name{
			// block to be repeated
			$(
				$varient_name = $varient_index,
			)*
		}
		impl $enum_name{
			pub fn from_index(index: u16) -> Result<$enum_name, crate::ReadValueError>{
				match index{
					// block to be repeated
					$(
						$varient_index => Ok($enum_name::$varient_name),
					)*
					_ => Err(crate::ReadValueError::InvalidPacketIndex(index))
				}
			}
		}
	};
}
