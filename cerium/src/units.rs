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
		app.add_startup_system(insert_materials)
			.add_system_set(
				SystemSet::on_exit(GameState::Loading)
					.with_system(add_unit)
					.with_system(add_box_selection),
			)
			.add_system_set(
				SystemSet::on_update(GameState::Account)
					.with_system(update_units)
					.with_system(select_units)
					.with_system(move_units),
			);
	}
}

#[derive(Component)]
struct UnitComponent(Unit);

struct UnitMaterials {
	selected: Handle<StandardMaterial>,
	standard: Handle<StandardMaterial>,
}
fn insert_materials(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
	commands.insert_resource(UnitMaterials {
		selected: materials.add(StandardMaterial {
			base_color: Color::CRIMSON,
			..Default::default()
		}),
		standard: materials.add(StandardMaterial {
			base_color: Color::hex("ffd891").unwrap(),
			..Default::default()
		}),
	})
}

struct Drag {
	start: IVec2,
}
struct SelectionData {
	drag: Option<Drag>,
}
#[derive(Component)]
struct SelectedUnit;
impl SelectionData {
	fn drag(
		&mut self,
		cursor_position: IVec2,
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
				i.size = Size::new(
					Val::Px((cursor_position.x - start.x).abs() as f32),
					Val::Px((cursor_position.y - start.y).abs() as f32),
				);
				i.position_type = PositionType::Absolute;
				i.position = Rect {
					left: Val::Px(cursor_position.x.min(start.x) as f32),
					bottom: Val::Px(cursor_position.y.min(start.y) as f32),
					..Default::default()
				}
			}
		}
	}
	fn end_drag(
		&mut self,
		selection_widget_query: &mut Query<&mut Visible, With<SelectionWidget>>,
		mut unit_query: Query<
			(Entity, &GlobalTransform, &mut Handle<StandardMaterial>),
			With<UnitComponent>,
		>,
		mut commands: Commands,
		windows: Res<Windows>,
		camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
		unit_materials: Res<UnitMaterials>,
	) {
		const MAX_CLICK_DIST_TO_UNIT_SQRD: f32 = 2500.;
		const MAX_CLICK_SQRD: f32 = 2500.;
		if let Some(Drag { start }) = self.drag {
			let projection = match camera_query.iter().next() {
				Some(c) => Projection::new(&windows, c),
				None => return,
			};
			let cursor_position = match windows.get_primary().unwrap().cursor_position() {
				Some(x) => x,
				None => return,
			};
			let start = start.as_vec2();
			let is_click = MAX_CLICK_SQRD > start.distance_squared(cursor_position);
			let min = cursor_position.min(start);
			let max = cursor_position.max(start);

			for (entity, transform, mut mat) in unit_query.iter_mut() {
				if let Some(screen_space) = projection.project_from_world(&transform) {
					if (is_click
						&& screen_space.distance_squared(cursor_position)
							< MAX_CLICK_DIST_TO_UNIT_SQRD)
						|| (!is_click
							&& screen_space.x > min.x && screen_space.x < max.x
							&& screen_space.y > min.y && screen_space.y < max.y)
					{
						info!("Unit selected");
						commands.entity(entity).insert(SelectedUnit);
						*mat = unit_materials.selected.clone();
					} else {
						commands.entity(entity).remove::<SelectedUnit>();
						*mat = unit_materials.standard.clone();
					}
				}
			}
			self.drag = None;
			for mut i in selection_widget_query.iter_mut() {
				i.is_visible = false;
			}
		}
	}
}
#[derive(Component)]
struct SelectionWidget;
#[derive(Component)]
struct SelectionRect;

fn add_box_selection(mut commands: Commands, mut colour_materials: ResMut<Assets<ColorMaterial>>) {
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

fn add_unit(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	unit_materials: Res<UnitMaterials>,
) {
	let japan = Vec3::new(0.52484196, 0.5836691, -0.6195735);
	let germany = Vec3::new(0.14106606, 0.79356587, 0.59190667);
	commands
		.spawn_bundle(PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
			material: unit_materials.standard.clone(),
			..Default::default()
		})
		.insert(UnitComponent(Unit::new(japan, germany, 0)));
	commands
		.spawn_bundle(PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
			material: unit_materials.standard.clone(),
			..Default::default()
		})
		.insert(UnitComponent(Unit::new(japan, germany, 1)));
}

fn update_units(
	mut query: Query<(&mut UnitComponent, &mut GlobalTransform)>,
	heightmap_sampler: Option<Res<HeightmapSampler>>,
	texture_handle: Option<Res<WorldTexture>>,
	textures: Res<Assets<Texture>>,
) {
	if let (Some(heightmap_sampler), Some(texture_handle)) = (heightmap_sampler, texture_handle) {
		if let Some(height_map) = textures.get(&texture_handle.handle) {
			for (unit, mut transform) in query.iter_mut() {
				transform.translation = heightmap_sampler
					.sample(unit.0.get_position(), &height_map)
					.into();
			}
		}
	}
}

fn select_units(
	unit_query: Query<
		(Entity, &GlobalTransform, &mut Handle<StandardMaterial>),
		With<UnitComponent>,
	>,
	commands: Commands,
	mut selection_rect_query: Query<&mut Style, With<SelectionRect>>,
	mut selection_widget_query: Query<&mut Visible, With<SelectionWidget>>,
	mut selection_data: ResMut<SelectionData>,
	windows: Res<Windows>,
	camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
	mouse_input: Res<Input<MouseButton>>,
	unit_materials: Res<UnitMaterials>,
) {
	if mouse_input.pressed(MouseButton::Left) {
		let cursor_position = match windows.get_primary().unwrap().cursor_position() {
			Some(x) => x,
			None => return,
		};
		selection_data.drag(
			cursor_position.as_ivec2(),
			&mut selection_rect_query,
			&mut selection_widget_query,
		);
	} else {
		selection_data.end_drag(
			&mut selection_widget_query,
			unit_query,
			commands,
			windows,
			camera_query,
			unit_materials,
		);
	}
}

fn move_units(
	mut unit_query: Query<&mut UnitComponent, With<SelectedUnit>>,
	mouse_input: Res<Input<MouseButton>>,
	windows: Res<Windows>,
	camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
	if mouse_input.just_pressed(MouseButton::Right) {
		let projection = match camera_query.iter().next() {
			Some(c) => Projection::new(&windows, c),
			None => return,
		};
		let cursor_position = match windows.get_primary().unwrap().cursor_position() {
			Some(x) => x,
			None => return,
		};
		let ray = projection.project_from_screen(&cursor_position);
		let point = match Projection::intersect(ray, Vec3::ZERO, 2.) {
			Some(x) => x,
			None => return,
		};

		for mut unit in unit_query.iter_mut() {
			unit.0.set_destination(&point);
		}
	}
}
