use bevy::{
	diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
	prelude::*,
};

pub struct InfoPlugin;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

impl Plugin for InfoPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(setup)
			.add_plugin(FrameTimeDiagnosticsPlugin::default())
			.add_system(text_update_system);
	}
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn_bundle(UiCameraBundle::default());

	commands.spawn_bundle(TextBundle {
		style: Style {
			align_self: AlignSelf::FlexEnd,
			position_type: PositionType::Absolute,
			position: Rect {
				bottom: Val::Px(5.0),
				right: Val::Px(5.0),
				..Default::default()
			},
			..Default::default()
		},
		text: Text {
			sections: vec![
				TextSection {
					value: "Version: ".to_string(),
					style: TextStyle {
						font: asset_server.load("fonts/FiraSans-Bold.ttf"),
						font_size: 18.0,
						color: Color::WHITE,
					},
				},
				TextSection {
					value: VERSION.unwrap_or("Release").to_string(),
					style: TextStyle {
						font: asset_server.load("fonts/FiraSans-SemiBold.ttf"),
						font_size: 20.0,
						color: Color::GOLD,
					},
				},
			],
			..Default::default()
		},
		..Default::default()
	});

	commands
		.spawn_bundle(TextBundle {
			style: Style {
				align_self: AlignSelf::FlexEnd,
				position_type: PositionType::Absolute,
				position: Rect {
					top: Val::Px(5.0),
					right: Val::Px(5.0),
					..Default::default()
				},
				..Default::default()
			},
			text: Text {
				sections: vec![
					TextSection {
						value: "FPS: ".to_string(),
						style: TextStyle {
							font: asset_server.load("fonts/FiraSans-Bold.ttf"),
							font_size: 18.0,
							color: Color::WHITE,
						},
					},
					TextSection {
						value: "-".to_string(),
						style: TextStyle {
							font: asset_server.load("fonts/FiraSans-SemiBold.ttf"),
							font_size: 20.0,
							color: Color::GOLD,
						},
					},
				],
				..Default::default()
			},
			..Default::default()
		})
		.insert(FpsText);
}

struct FpsText;
fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
	for mut text in query.iter_mut() {
		if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
			if let Some(average) = fps.average() {
				// Update the value of the second section
				text.sections[1].value = format!("{:.2}", average);
			}
		}
	}
}
