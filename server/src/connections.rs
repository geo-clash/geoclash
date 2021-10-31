//! Contains the [`Connections`] type for each socket connection to the server

use async_channel::Sender;
use net::packets::WriteBuf;

/// Data stored temporerly about each socket.
pub struct Connection {
	pub user_id: Option<usize>,
	pub write_buf_sender: Option<Sender<WriteBuf>>,
}

/// A collection of [`Connection`]s for all the sockets.
pub type Connections = Vec<Connection>;
