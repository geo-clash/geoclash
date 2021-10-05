use super::camera::MainCamera;
use crate::{screenspace::Projection, world::HeightmapSampler};
use bevy::{math::Vec3A, prelude::*, render::camera::Camera};
use game_statics::COUNTRIES;

pub struct CityPlugin;

impl Plugin for CityPlugin {
	fn build(&self, app: &mut App) {
		app.add_system(hover_city);
	}
}

#[derive(Debug)]
struct City {
	pub id: usize,
}

pub fn add_cities(
	commands: &mut Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	height_map: &HeightmapSampler,
	texture: &Texture,
) {
	const RADIUS: f32 = 2.;
	for (id, country) in COUNTRIES.iter().enumerate() {
		let lat: f32 = -f32::to_radians(country.lat);
		let lon: f32 = f32::to_radians(country.long);

		let sphere = meshes.add(Mesh::from(shape::Icosphere {
			radius: 0.03,
			subdivisions: 2,
		}));
		let material = materials.add(StandardMaterial {
			base_color: Color::RED,
			roughness: 0.7,
			..Default::default()
		});
		let position = Vec3A::new(
			RADIUS * lat.cos() * lon.sin(),
			-RADIUS * lat.sin(),
			RADIUS * lat.cos() * lon.cos(),
		)
		.normalize();
		println!("{} Pos: {}", country.name, position);
		let height = height_map.height(&position, texture);
		let position = position
			* (height_map.radius + height as f32 / u8::MAX as f32 * height_map.height_radius);
		commands
			.spawn_bundle(PbrBundle {
				mesh: sphere,
				material: material,
				transform: Transform::from_translation(position.into()),
				..Default::default()
			})
			.insert(City { id });
	}
}

fn hover_city(
	windows: Res<Windows>,
	mut city_query: Query<(&GlobalTransform, &City)>,
	camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
	if let Some(cursor_position) = windows.get_primary().unwrap().cursor_position() {
		for camera in camera_query.iter() {
			let mut closest_index: Option<usize> = None;
			let mut closest_distance: f32 = f32::MAX;

			// Build a transform to convert from world to camera space
			let projection = Projection::new(&windows, camera);

			// Iterate through cities
			for (transform, city) in city_query.iter_mut() {
				// Make sure the city is not on the other side of the world
				if let Some(screen_space_coords) = projection.project_from_world(transform) {
					// Calculate distance from city in screen space to the cursor
					let distance = (screen_space_coords - cursor_position).length();
					if distance < closest_distance {
						closest_distance = distance;
						closest_index = Some(city.id);
					}
				}
			}
			if let Some(ind) = closest_index {
				trace!("closest {}  name {}", ind, COUNTRIES[ind].name);
			}
		}
	}
}
