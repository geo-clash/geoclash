use bevy::{pbr::AmbientLight, prelude::*};
mod camera;
mod city;
mod connect_ui;
mod info;
mod units;
mod world;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
	Loading,
	Account,
	Playing,
}

fn main() {
	App::new()
		.add_state(GameState::Loading)
		.insert_resource(WindowDescriptor {
			title: "Cerium".to_string(),
			..Default::default()
		})
		.insert_resource(Msaa { samples: 4 })
		.add_plugins(DefaultPlugins)
		.add_plugin(camera::CameraPlugin)
		.add_plugin(world::WorldPlugin)
		.add_plugin(city::CityPlugin)
		.add_plugin(units::UnitPlugin)
		.add_plugin(info::InfoPlugin)
		.add_plugin(connect_ui::ConnectUIPlugin)
		.add_startup_system(setup)
		.run();
}

/// set up a simple 3D scene
fn setup(mut ambient_light: ResMut<AmbientLight>) {
	// light
	ambient_light.brightness = 0.5;
}
