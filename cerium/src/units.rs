use bevy::{prelude::*, render::camera::Camera};
use client_net::Unit;

use crate::{
	camera::MainCamera,
	screenspace::Projection,
	world::{HeightmapSampler, WorldTexture},
	GameState,
};
pub struct UnitPlugin;

impl Plugin for UnitPlugin {
	fn build(&self, app: &mut App) {
		app.add_system_set(SystemSet::on_exit(GameState::Loading).with_system(add_unit))
			.add_system_set(SystemSet::on_update(GameState::Account).with_system(update_units))
			.add_system_set(SystemSet::on_update(GameState::Account).with_system(select_units));
	}
}

struct Drag {
	start: Vec2,
}
struct SelectionData {
	drag: Option<Drag>,
}
impl SelectionData {
	fn drag(
		&mut self,
		cursor_position: Vec2,
		selection_rect_query: &mut Query<&mut Style, With<SelectionRect>>,
		selection_widget_query: &mut Query<&mut Visible, With<SelectionWidget>>,
	) {
		if self.drag.is_none() {
			self.drag = Some(Drag {
				start: cursor_position,
			});
			for mut i in selection_widget_query.iter_mut() {
				i.is_visible = true;
			}
		}
		if let Some(Drag { start }) = self.drag {
			for mut i in selection_rect_query.iter_mut() {
				cursor_position.x.min(start.x);
				i.size = Size::new(
					Val::Px((cursor_position.x - start.x).abs()),
					Val::Px((cursor_position.y - start.y).abs()),
				);
				i.position_type = PositionType::Absolute;
				i.position = Rect {
					left: Val::Px(cursor_position.x.min(start.x)),
					bottom: Val::Px(cursor_position.y.min(start.y)),
					..Default::default()
				}
			}
		}
	}
	fn end_drag(
		&mut self,
		selection_widget_query: &mut Query<&mut Visible, With<SelectionWidget>>,
	) {
		if self.drag.is_some() {
			self.drag = None;
			for mut i in selection_widget_query.iter_mut() {
				i.is_visible = false;
			}
		}
	}
}
struct SelectionWidget;
struct SelectionRect;

fn add_unit(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut colour_materials: ResMut<Assets<ColorMaterial>>,
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
	commands
		.spawn_bundle(NodeBundle {
			style: Style {
				size: Size::new(Val::Percent(30.0), Val::Percent(30.0)),
				position: Rect {
					left: Val::Px(700.0),
					bottom: Val::Px(50.0),
					..Default::default()
				},
				position_type: PositionType::Absolute,
				..Default::default()
			},
			material: colour_materials.add(Color::rgba(0.2, 0.25, 0.2, 0.4).into()),
			visible: Visible {
				is_visible: false,
				is_transparent: true,
			},
			..Default::default()
		})
		.insert(SelectionWidget)
		.insert(SelectionRect)
		.with_children(|parent| {
			for i in 0..4 {
				parent
					.spawn_bundle(NodeBundle {
						style: Style {
							size: Size::new(
								if i < 2 {
									Val::Percent(100.)
								} else {
									Val::Px(2.)
								},
								if i >= 2 {
									Val::Percent(100.)
								} else {
									Val::Px(2.)
								},
							),
							position: Rect {
								right: if i != 3 { Val::Undefined } else { Val::Px(0.) },
								top: if i != 1 { Val::Undefined } else { Val::Px(0.) },
								..Default::default()
							},
							position_type: PositionType::Absolute,
							..Default::default()
						},
						material: colour_materials.add(Color::rgba(0.2, 0.25, 0.2, 1.).into()),
						visible: Visible {
							is_visible: false,
							is_transparent: false,
						},
						..Default::default()
					})
					.insert(SelectionWidget);
			}
		});
	commands.insert_resource(SelectionData { drag: None });
}

fn update_units(
	mut query: Query<(&mut Unit, &mut GlobalTransform)>,
	heightmap_sampler: Option<Res<HeightmapSampler>>,
	texture_handle: Option<Res<WorldTexture>>,
	textures: Res<Assets<Texture>>,
) {
	if let (Some(heightmap_sampler), Some(texture_handle)) = (heightmap_sampler, texture_handle) {
		if let Some(height_map) = textures.get(&texture_handle.handle) {
			for (unit, mut transform) in query.iter_mut() {
				transform.translation = heightmap_sampler
					.sample(unit.get_position(), &height_map)
					.into();
			}
		}
	}
}

fn select_units(
	mut unit_query: Query<(&GlobalTransform, &Handle<StandardMaterial>), With<Unit>>,
	mut selection_rect_query: Query<&mut Style, With<SelectionRect>>,
	mut selection_widget_query: Query<&mut Visible, With<SelectionWidget>>,
	mut selection_data: ResMut<SelectionData>,
	windows: Res<Windows>,
	camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
	mouse_input: Res<Input<MouseButton>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	if mouse_input.just_pressed(MouseButton::Left) {
		let projection = match camera_query.iter().next() {
			Some(c) => Projection::new(&windows, c),
			None => return,
		};
		let cursor_position = match windows.get_primary().unwrap().cursor_position() {
			Some(x) => x,
			None => return,
		};
		for (transform, mat) in unit_query.iter_mut() {
			info!("ITer");
			if let Some(screen_space) = projection.project_from_world(&transform) {
				info!("{:?}", screen_space);
				if screen_space.distance_squared(cursor_position) < 2500. {
					info!("Unit selected");
					//let handle = mat.();
					if let Some(atlas) = materials.get_mut(mat) {
						atlas.base_color = Color::CRIMSON;
					}
				}
			}
		}
	}
	if mouse_input.pressed(MouseButton::Left) {
		let cursor_position = match windows.get_primary().unwrap().cursor_position() {
			Some(x) => x,
			None => return,
		};
		selection_data.drag(
			cursor_position,
			&mut selection_rect_query,
			&mut selection_widget_query,
		);
	} else {
		selection_data.end_drag(&mut selection_widget_query);
	}
}
