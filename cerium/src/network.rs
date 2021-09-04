use bevy::prelude::*;
use client_net::*;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugin(ClientNetworkPlugin)
			.insert_resource(NetworkClient::new("127.0.0.1:2453"));
	}
}
