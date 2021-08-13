use bevy::{pbr::AmbientLight, prelude::*};
mod camera;
mod city;
mod info;
mod world;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Cerium".to_string(),
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(world::WorldPlugin)
        .add_plugin(city::CityPlugin)
        .add_plugin(info::InfoPlugin)
        .add_startup_system(setup.system())
        .run();
}

/// set up a simple 3D scene
fn setup(mut ambient_light: ResMut<AmbientLight>) {
    // light
    ambient_light.brightness = 0.5;
}
