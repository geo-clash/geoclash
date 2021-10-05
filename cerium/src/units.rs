use bevy::prelude::*;
use client_net::Unit;

use crate::{
	world::{HeightmapSampler, WorldTexture},
	GameState,
};
pub struct UnitPlugin;

impl Plugin for UnitPlugin {
	fn build(&self, app: &mut App) {
		app.add_system_set(SystemSet::on_exit(GameState::Loading).with_system(add_unit))
			.add_system_set(SystemSet::on_update(GameState::Account).with_system(update_units));
	}
}

fn add_unit(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let japan = Vec3::new(0.52484196, 0.5836691, -0.6195735);
	let germany = Vec3::new(0.14106606, 0.79356587, 0.59190667);
	commands
		.spawn_bundle(PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
			material: materials.add(StandardMaterial {
				base_color: Color::hex("ffd891").unwrap(),
				..Default::default()
			}),
			..Default::default()
		})
		.insert(Unit::new(japan, germany, 0));
	commands
		.spawn_bundle(PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
			material: materials.add(StandardMaterial {
				base_color: Color::hex("ffd891").unwrap(),
				..Default::default()
			}),
			..Default::default()
		})
		.insert(Unit::new(japan, germany, 1));
}

fn update_units(
	mut query: Query<(&mut Unit, &mut GlobalTransform)>,
	heightmap_sampler: Option<Res<HeightmapSampler>>,
	texture_handle: Option<Res<WorldTexture>>,
	textures: Res<Assets<Texture>>,
	windows: Res<Windows>,
) {
	if let (Some(heightmap_sampler), Some(texture_handle)) = (heightmap_sampler, texture_handle) {
		if let Some(height_map) = textures.get(&texture_handle.handle) {
			if let Some(_cursor_position) = windows.get_primary().unwrap().cursor_position() {
				for (unit, mut transform) in query.iter_mut() {
					transform.translation = heightmap_sampler
						.sample(unit.get_position(), &height_map)
						.into();
				}
			}
		}
	}
}
