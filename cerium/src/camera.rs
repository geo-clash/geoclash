use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(setup)
			.add_system(camera_input)
			.add_system(camera_movement);
	}
}

fn setup(mut commands: Commands) {
	// camera
	commands
		.spawn_bundle(PerspectiveCameraBundle::default())
		.insert(MovableCamera {
			old_cursor_position: None,
			velocity: Vec2::ZERO,
			rotation: Quat::IDENTITY,
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
	keys: Res<Input<KeyCode>>,
	windows: Res<Windows>,
	mut query: Query<&mut MovableCamera>,
) {
	for mut camera in query.iter_mut() {
		if (mouse_input.pressed(MouseButton::Left)
			&& (keys.pressed(KeyCode::RControl)
				|| keys.pressed(KeyCode::LControl)
				|| keys.pressed(KeyCode::RAlt)
				|| keys.pressed(KeyCode::LAlt)))
			|| mouse_input.pressed(MouseButton::Middle)
		{
			camera.velocity = Vec2::ZERO;
			let cursor_position = windows.get_primary().unwrap().cursor_position().unwrap();
			if let Some(old_cursor_position) = camera.old_cursor_position {
				camera.velocity +=
					(cursor_position - old_cursor_position) / time.delta_seconds() / 200.;
			}
			camera.old_cursor_position = Some(cursor_position);
		} else {
			camera.old_cursor_position = None;
		}
		let direction = Vec2::new(
			(keys.pressed(KeyCode::Left) || keys.pressed(KeyCode::A)) as u8 as f32,
			0.,
		) - Vec2::new(
			(keys.pressed(KeyCode::Right) || keys.pressed(KeyCode::D)) as u8 as f32,
			0.,
		) + Vec2::new(
			0.,
			(keys.pressed(KeyCode::Down) || keys.pressed(KeyCode::S)) as u8 as f32,
		) - Vec2::new(
			0.,
			(keys.pressed(KeyCode::Up) || keys.pressed(KeyCode::W)) as u8 as f32,
		);
		if direction != Vec2::ZERO {
			camera.velocity = direction * time.delta_seconds() * 100.;
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
