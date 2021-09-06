use bevy::prelude::*;
use bevy_egui::{
	egui::{self, Label},
	EguiContext, EguiPlugin,
};
use egui::widgets::{Button, TextEdit};

pub struct ConnectUIPlugin;

impl Plugin for ConnectUIPlugin {
	fn build(&self, app: &mut App) {
		app.add_system(ui_example.system())
			.add_plugin(EguiPlugin)
			.init_resource::<ConnectUIState>();
	}
}

#[derive(Default)]
struct ConnectUIState {
	server_address: String,
	username: String,
	password: String,
	status: Option<String>,
	sent: bool,
}

fn ui_example(egui_context: ResMut<EguiContext>, mut state: ResMut<ConnectUIState>) {
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
					state.status = Some("Connecting...".to_string());
				}
			}
		});
	}
}
