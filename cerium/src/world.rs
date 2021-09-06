use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(setup);
	}
}
fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	asset_server: Res<AssetServer>,
) {
	let texture_handle: Handle<Texture> = asset_server.load("textures/colour_map.jpg");
	commands.spawn_bundle(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::UVSphere {
			radius: 2.,
			sectors: 50,
			stacks: 50,
		})),
		material: materials.add(StandardMaterial {
			base_color_texture: Some(texture_handle.clone()),
			roughness: 0.7,
			..Default::default()
		}),
		transform: Transform::from_xyz(0.0, 0.0, 0.0),
		..Default::default()
	});
}
