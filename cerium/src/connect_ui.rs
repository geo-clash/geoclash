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
			.net_listen::<{ ServerPacket::InvalidLogin as u16 }>()
			.net_listen::<{ ServerPacket::InvalidSignup as u16 }>()
			.add_system(on_auth_error)
			.net_listen::<{ ServerPacket::SucessfulLogin as u16 }>()
			.net_listen::<{ ServerPacket::SucessfulSignup as u16 }>()
			.add_system(on_sucessful_auth)
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
					let net_client = NetworkClient::new(state.server_address.clone());
					commands.insert_resource(net_client);
				}
			}
		});
	}
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
	mut invalid_login_event: EventReader<EventReadBuffer<{ ServerPacket::InvalidLogin as u16 }>>,
	mut invalid_signup_event: EventReader<EventReadBuffer<{ ServerPacket::InvalidSignup as u16 }>>,
	mut state: ResMut<ConnectUIState>,
) {
	for _ in invalid_login_event.iter() {
		state.status = Some("Invalid Login :(".to_string());
		info!("Invalid login");
	}
	for _ in invalid_signup_event.iter() {
		state.status = Some("Invalid Signup :(".to_string());
		info!("Invalid signup");
	}
}

fn on_sucessful_auth(
	mut sucessful_login_event: EventReader<
		EventReadBuffer<{ ServerPacket::SucessfulLogin as u16 }>,
	>,
	mut sucessful_signup_event: EventReader<
		EventReadBuffer<{ ServerPacket::SucessfulSignup as u16 }>,
	>,
	mut state: ResMut<ConnectUIState>,
) {
	for _ in sucessful_login_event.iter() {
		state.status = Some("Logged in!".to_string());
		info!("Logged In!");
	}
	for _ in sucessful_signup_event.iter() {
		state.status = Some("Signed up!".to_string());
		info!("Signed up!");
	}
}
