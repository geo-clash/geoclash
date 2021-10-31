//! Contains shared client and server net code.
//!
//! Re-exports some tokio types as well as [`packets`]

pub extern crate packets;
pub use tokio::runtime::Runtime;
pub extern crate tokio;
pub use tokio::sync::mpsc;
use tokio::{
	io::{AsyncReadExt, AsyncWriteExt},
	net::{
		tcp::{OwnedReadHalf, OwnedWriteHalf},
		TcpStream,
	},
};
#[macro_use]
extern crate log;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

/// Wraps around a tokio socket & adress. Provides a read and write method taking [`packets::WriteBuf`]s and byte vecs.
pub struct RemoteConnection {
	pub socket: Arc<Mutex<TcpStream>>,
	pub address: SocketAddr,
}
impl Clone for RemoteConnection {
	fn clone(&self) -> Self {
		Self {
			socket: Arc::clone(&self.socket),
			address: self.address.clone(),
		}
	}
}

/// Contains the possible outputs from reading from the socket, Ok returns the bytes read.
pub enum ReadResponse {
	Ok(usize),
	Disconnected,
	Error,
	PacketLengthTooLong,
}

impl RemoteConnection {
	// Write data back to the client
	pub async fn socket_write(buf: &packets::WriteBuf, socket: &mut OwnedWriteHalf) {
		let buffer = buf.inner();
		let len = buffer.len() as u32;
		info!("buf {}  {:?}", len, buffer);
		socket.write_u32(len).await.unwrap();
		if let Err(e) = socket.write_all(buffer).await {
			error!("failed to write to socket; err = {:?}", e);
			return;
		}
	}
	pub async fn read_length<'a>(buf: &mut Vec<u8>, socket: &mut OwnedReadHalf) -> ReadResponse {
		let length = match socket.read_u32().await {
			Ok(len) => len as usize,
			Err(err) => {
				if err.kind() == std::io::ErrorKind::UnexpectedEof {
					info!("Remote disconnected");
					return ReadResponse::Disconnected;
				}
				error!("Encountered error while fetching length: {}", err);
				return ReadResponse::Error;
			}
		};
		if length > buf.len() {
			return ReadResponse::PacketLengthTooLong;
		}
		match socket.read_exact(&mut buf[..length]).await {
			// socket closed
			Ok(n) if n == 0 => return ReadResponse::Disconnected,
			Ok(n) => ReadResponse::Ok(n),
			Err(e) => {
				error!("failed to read from socket; err = {:?}", e);
				return ReadResponse::Error;
			}
		}
	}
}
