use bevy::prelude::*;
use client_net::*;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
	fn build(&self, app: &mut App) {
		app.net_listen::<{ ServerPacket::ServerInfo as u16 }>()
			.add_system(on_server_info)
			.net_listen::<{ ServerPacket::InvalidLogin as u16 }>()
			.add_system(on_invalid_login);
	}
}

fn on_server_info(
	mut events: EventReader<EventReadBuffer<{ ServerPacket::ServerInfo as u16 }>>,
	mut writer: EventWriter<WriteBuf>,
) {
	for event in events.iter() {
		let mut x = event.read();
		info!(
			"got server msg \n{}",
			ServerInfo::deserialize(&mut x).unwrap()
		);
		writer.send(
			WriteBuf::new_client_packet(ClientPacket::Login).push(Authentication {
				username: "test".to_string(),
				password: "testy".to_string(),
			}),
		);
		info!("Sent login request");
	}
}

fn on_invalid_login(
	mut events: EventReader<EventReadBuffer<{ ServerPacket::InvalidLogin as u16 }>>,
) {
	for _ in events.iter() {
		info!("Invalid Login !!! :( 😕")
	}
}
