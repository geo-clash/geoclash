use super::camera::MainCamera;
use bevy::{prelude::*, render::camera::Camera};
use game_statics::COUNTRIES;

pub struct CityPlugin;

impl Plugin for CityPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(setup).add_system(hover_city);
	}
}

#[derive(Debug)]
struct City {
	pub id: usize,
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	const RADIUS: f32 = 2.;
	for (id, country) in COUNTRIES.iter().enumerate() {
		let lat: f32 = f32::to_radians(country.lat);
		let lon: f32 = -f32::to_radians(country.long);

		let sphere = meshes.add(Mesh::from(shape::Icosphere {
			radius: 0.03,
			subdivisions: 2,
		}));
		let material = materials.add(StandardMaterial {
			base_color: Color::RED,
			roughness: 0.7,
			..Default::default()
		});
		commands
			.spawn_bundle(PbrBundle {
				mesh: sphere,
				material: material,
				transform: Transform::from_xyz(
					-RADIUS * lat.cos() * lon.cos(),
					RADIUS * lat.cos() * lon.sin(),
					RADIUS * lat.sin(),
				),
				..Default::default()
			})
			.insert(City { id });
	}
}

fn hover_city(
	windows: Res<Windows>,
	mut cursor_moved_events: EventReader<CursorMoved>,
	mut city_query: Query<(&GlobalTransform, &City)>,
	camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
	for cursor in cursor_moved_events.iter() {
		for (camera, camera_transform) in camera_query.iter() {
			let mut closest_index: Option<usize> = None;
			let mut closest_distance: f32 = f32::MAX;

			// Build a transform to convert from world to camera space
			let window = (&windows).get(camera.window).unwrap();
			let window_size = Vec2::new(window.width(), window.height());
			let world_to_ndc: Mat4 =
				camera.projection_matrix * camera_transform.compute_matrix().inverse();

			// Iterate through cities
			for (transform, city) in city_query.iter_mut() {
				// Make sure the city is not on the other side of the world
				if camera_transform.translation.dot(transform.translation) > 0. {
					// Project world to camera space
					let ndc_space_coords: Vec3 = world_to_ndc.project_point3(transform.translation);
					// discard the z element and rescale x/y to fit the screen
					let screen_space_coords =
						(ndc_space_coords.truncate() + Vec2::ONE) / 2.0 * window_size;
					// Calculate distance from city in screen space to the cursor
					let distance = (screen_space_coords - cursor.position).length();
					if distance < closest_distance {
						closest_distance = distance;
						closest_index = Some(city.id);
					}
				}
			}
			trace!(
				"closest {}  name {}",
				closest_index.unwrap(),
				COUNTRIES[closest_index.unwrap()].name
			);
		}
	}
}
