use bevy::prelude::*;
use bevy_egui::{
	egui::{self, Label},
	EguiContext, EguiPlugin,
};
use client_net::*;
use egui::widgets::{Button, Checkbox, TextEdit};

use crate::GameState;

pub struct ConnectUIPlugin;

impl Plugin for ConnectUIPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugin(ClientNetworkPlugin)
			.add_plugin(EguiPlugin)
			.init_resource::<ConnectUIState>()
			.net_listen::<{ ServerPacket::Connect as u16 }>()
			.net_listen::<{ ServerPacket::InvalidAuth as u16 }>()
			.net_listen::<{ ServerPacket::SuccessfulAuth as u16 }>()
			.add_system_set(
				SystemSet::on_update(GameState::Account)
					.with_system(connect_ui)
					.with_system(on_connect)
					.with_system(on_auth_error)
					.with_system(on_sucessful_auth)
					.with_system(on_net_error),
			);
	}
}

struct ConnectUIState {
	new_user: bool,
	sending_new_user: bool,
	server_address: String,
	username: String,
	password: String,
	status: Option<String>,
}
impl Default for ConnectUIState {
	fn default() -> Self {
		Self {
			new_user: true,
			sending_new_user: false,
			server_address: String::from("127.0.0.1:2453"),
			username: String::from("IEEE"),
			password: String::from("I777777"),
			status: Default::default(),
		}
	}
}

fn connect_ui(
	egui_context: ResMut<EguiContext>,
	mut state: ResMut<ConnectUIState>,
	mut commands: Commands,
) {
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
		ui.add(Checkbox::new(&mut state.new_user, "Register a new account"));

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
				info!("Some required fields are missing");
				state.status = Some("Some required fields are missing.".to_string());
			} else {
				state.status = Some("Connecting...".to_string());
				state.sending_new_user = state.new_user;
				let net_client = NetworkClient::new(state.server_address.clone());
				commands.insert_resource(net_client);
			}
		}
	});
}

fn on_connect(
	mut events: EventReader<EventReadBuffer<{ ServerPacket::Connect as u16 }>>,
	mut state: ResMut<ConnectUIState>,
	mut writer: EventWriter<WriteBuf>,
) {
	for _ in events.iter() {
		if state.new_user {
			writer.send(
				WriteBuf::new_client_packet(ClientPacket::SignUp).push(Authentication {
					username: state.username.clone(),
					password: state.password.clone(),
				}),
			);
		} else {
			writer.send(
				WriteBuf::new_client_packet(ClientPacket::Login).push(Authentication {
					username: state.username.clone(),
					password: state.password.clone(),
				}),
			);
		}

		state.status = Some("Connected!".to_string());
	}
}

fn on_net_error(mut events: EventReader<NetworkError>, mut state: ResMut<ConnectUIState>) {
	for e in events.iter() {
		state.status = Some(format!("Error {}", e));
	}
}

fn on_auth_error(
	mut invalid_auth_event: EventReader<EventReadBuffer<{ ServerPacket::InvalidAuth as u16 }>>,
	mut state: ResMut<ConnectUIState>,
) {
	for _ in invalid_auth_event.iter() {
		if state.sending_new_user {
			state.status = Some("Invalid Signup :(".to_string());
			info!("Invalid signup");
		} else {
			state.status = Some("Invalid Login :(".to_string());
			info!("Invalid login");
		}
	}
}

fn on_sucessful_auth(
	mut sucessful_auth_event: EventReader<EventReadBuffer<{ ServerPacket::SuccessfulAuth as u16 }>>,
	mut state: ResMut<ConnectUIState>,
	mut game_state: ResMut<State<GameState>>,
) {
	for _ in sucessful_auth_event.iter() {
		if state.sending_new_user {
			state.status = Some("Signed up!".to_string());
			info!("Signed up!");
		} else {
			state.status = Some("Logged in!".to_string());
			info!("Logged In!");
		}
		game_state.set(GameState::Playing).unwrap();
	}
}
