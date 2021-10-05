use bevy::{prelude::*, render::camera::Camera};

pub struct Projection {
	window_size: Vec2,
	world_to_ndc: Mat4,
	camera_translation: Vec3,
}
impl Projection {
	pub fn new(
		windows: &Res<Windows>,
		(camera, camera_transform): (&Camera, &GlobalTransform),
	) -> Self {
		// Build a transform to convert from world to camera space
		let window = (&windows).get(camera.window).unwrap();
		let window_size = Vec2::new(window.width(), window.height());
		let world_to_ndc: Mat4 =
			camera.projection_matrix * camera_transform.compute_matrix().inverse();
		let camera_translation = camera_transform.translation;
		Projection {
			window_size,
			world_to_ndc,
			camera_translation,
		}
	}
	pub fn project_from_world(&self, other: &GlobalTransform) -> Option<Vec2> {
		if self.camera_translation.dot(other.translation) > 0. {
			// Project world to camera space
			let ndc_space_coords: Vec3 = self.world_to_ndc.project_point3(other.translation);
			// discard the z element and rescale x/y to fit the screen

			Some((ndc_space_coords.truncate() + Vec2::ONE) / 2.0 * self.window_size)
		} else {
			None
		}
	}
}
