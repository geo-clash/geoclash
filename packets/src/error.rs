//! Contains byte parsing errors

use thiserror::Error;

/// Errors when parsing byte data
#[derive(Error, Debug, PartialEq)]
pub enum ReadValueError {
	#[error("Attempted to read {0} bytes when parsing {1}")]
	BufferToShort(usize, &'static str),
	#[error("Could not parse utf8 string")]
	StringParseError,
	#[error("Invalid packet index: {0}")]
	InvalidPacketIndex(u16),
}
