use bevy::{prelude::*, render::camera::Camera};

#[derive(Debug)]
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
	pub fn project_from_screen(&self, screen_space: &Vec2) -> (Vec3, Vec3) {
		// Rescale pixel coords to ndc
		let normalized_screen_space = *screen_space * 2. / self.window_size - Vec2::ONE;

		// project near and far planes
		let near = self.world_to_ndc.inverse().project_point3(Vec3::new(
			normalized_screen_space.x,
			normalized_screen_space.y,
			-1.,
		));
		let far = self.world_to_ndc.inverse().project_point3(Vec3::new(
			normalized_screen_space.x,
			normalized_screen_space.y,
			1.,
		));
		(near, far)
	}
	pub fn intersect(ray: (Vec3, Vec3), sphere: Vec3, radius: f32) -> Option<Vec3> {
		let ray_direction = (ray.1 - ray.0).normalize();
		let ray_start_distance_to_sphere = sphere - ray.0;
		let time_centre_intersection = ray_start_distance_to_sphere.dot(ray_direction);
		if time_centre_intersection < 0. {
			return None;
		}
		let distance_sphere_ray =
			(time_centre_intersection * ray_direction - ray_start_distance_to_sphere).length();
		if distance_sphere_ray > radius {
			return None;
		}
		let half_time_between_intersections = (radius.powi(2) - distance_sphere_ray.powi(2)).sqrt();
		let time = time_centre_intersection - half_time_between_intersections;
		Some(ray_direction * time + ray.0)
	}
}
