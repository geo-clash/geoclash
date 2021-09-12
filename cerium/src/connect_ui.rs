use std::net::SocketAddr;

use bevy::prelude::*;
use bevy_egui::{
	egui::{self, Label},
	EguiContext, EguiPlugin,
};
use client_net::*;
use egui::widgets::{Button, Checkbox, TextEdit};

pub struct ConnectUIPlugin;

impl Plugin for ConnectUIPlugin {
	fn build(&self, app: &mut App) {
		app.add_system(connect_ui)
			.add_plugin(ClientNetworkPlugin)
			.add_plugin(EguiPlugin)
			.init_resource::<ConnectUIState>()
			.net_listen::<{ ServerPacket::Connect as u16 }>()
			.add_system(on_connect)
			.add_system(on_net_error);
	}
}

#[derive(Default)]
struct ConnectUIState {
	new_user: bool,
	server_address: String,
	username: String,
	password: String,
	status: Option<String>,
	sent: bool,
}

fn connect_ui(
	egui_context: ResMut<EguiContext>,
	mut state: ResMut<ConnectUIState>,
	mut commands: Commands,
) {
	if !state.sent {
		egui::SidePanel::left("my_side_panel").show(egui_context.ctx(), |ui| {
			ui.heading("Login");
			if state.server_address.as_str() == "" {
				state.server_address = String::from("127.0.0.1:2453");
			}

			ui.add_space(10.);
			ui.label("Server address");
			ui.add(TextEdit::singleline(&mut state.server_address));

			ui.add_space(10.);
			ui.label("Username");
			ui.add(TextEdit::singleline(&mut state.username));

			ui.add_space(10.);
			ui.label("Password");
			ui.add(TextEdit::singleline(&mut state.password).password(true));

			ui.add_space(10.);
			if ui
				.add(Checkbox::new(&mut state.new_user, "Register a new account"))
				.clicked()
			{
				state.new_user = true;
			} else {
				state.new_user = false;
			}

			if let Some(status) = &mut state.status {
				ui.add_space(5.);
				ui.add(Label::new(status));
				ui.add_space(5.);
			} else {
				ui.add_space(20.);
			}

			if ui
				.add(Button::new("Connect").text_style(egui::TextStyle::Heading))
				.clicked()
			{
				if state.username.as_str() == "" || state.password.as_str() == "" {
					println!("Some required fields are missing");
					state.status = Some("Some required fields are missing.".to_string());
				} else {
					if let Ok(remote) = state.server_address.parse::<SocketAddr>() {
						state.status = Some("Connecting...".to_string());
						let net_client = NetworkClient::new(remote);
						commands.insert_resource(net_client);
					} else {
						state.status = Some(format!("Invalid address '{}'", state.server_address));
					}
				}
			}
		});
	}
}

fn on_connect(
	mut events: EventReader<EventReadBuffer<{ ServerPacket::Connect as u16 }>>,
	mut state: ResMut<ConnectUIState>,
) {
	for _ in events.iter() {
		state.status = Some("Connected!".to_string());
	}
}

fn on_net_error(mut events: EventReader<NetworkError>, mut state: ResMut<ConnectUIState>) {
	for e in events.iter() {
		state.status = Some(format!("Error {}", e));
	}
}
