use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(setup.system())
			.add_system(camera_input.system())
			.add_system(camera_movement.system());
	}
}

fn setup(mut commands: Commands) {
	// camera
	commands
		.spawn_bundle(PerspectiveCameraBundle::default())
		.insert(MovableCamera {
			old_cursor_position: None,
			velocity: Vec2::ZERO,
			rotation: Quat::from_vec4(Vec4::new(0.5, -0.5, -0.5, 0.5)),
			distance: 8.,
			friction: 0.9,
		})
		.insert(MainCamera);
}

struct MovableCamera {
	pub old_cursor_position: Option<Vec2>,
	pub velocity: Vec2,
	rotation: Quat,
	distance: f32,
	friction: f32,
}

pub struct MainCamera;

fn camera_input(
	_commands: Commands,
	time: Res<Time>,
	mouse_input: Res<Input<MouseButton>>,
	mut cursor_moved_events: EventReader<CursorMoved>,
	mut query: Query<&mut MovableCamera>,
) {
	for mut camera in query.iter_mut() {
		if mouse_input.pressed(MouseButton::Left) || mouse_input.pressed(MouseButton::Middle) {
			camera.velocity = Vec2::ZERO;
			for cursor in cursor_moved_events.iter() {
				if let Some(old_cursor_position) = camera.old_cursor_position {
					camera.velocity +=
						(cursor.position - old_cursor_position) / time.delta_seconds() / 200.;
				}
				camera.old_cursor_position = Some(cursor.position);
			}
		} else {
			camera.old_cursor_position = None;
		}
	}
}

fn camera_movement(
	_commands: Commands,
	time: Res<Time>,
	mut query: Query<(&mut MovableCamera, &mut Transform)>,
) {
	for (mut camera, mut transform) in query.iter_mut() {
		camera.rotation = camera.rotation
			* Quat::from_rotation_x(camera.velocity.y * time.delta_seconds())
			* Quat::from_rotation_y(-camera.velocity.x * time.delta_seconds());
		camera.velocity = camera.velocity * camera.friction;

		transform.translation = camera.rotation.mul_vec3(Vec3::new(0., 0., camera.distance));
		transform.rotation = camera.rotation;
	}
}
