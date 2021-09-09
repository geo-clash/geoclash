use std::{cmp, collections::BTreeMap};

use bevy::{
	asset::{Asset, LoadContext},
	math::{DVec2, Vec3A},
	prelude::*,
	render::{mesh::Indices, pipeline::PrimitiveTopology},
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(setup);
	}

	fn name(&self) -> &str {
		std::any::type_name::<Self>()
	}
}

fn get_mesh(order: usize, radius: f32) -> Mesh {
	let triangles = [
		0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 1, 5, 2, 1, 5, 3, 2, 5, 4, 3, 5, 1, 4,
	];
	let vertices = [
		// down
		Vec3A::new(0., -1., 0.),
		// forward
		Vec3A::new(0., 0., 1.),
		// left
		Vec3A::new(-1., 0., 0.),
		// back
		Vec3A::new(0., 0., -1.),
		// right
		Vec3A::new(1., 0., 0.),
		// up
		Vec3A::new(0., 1., 0.),
	];
	let vertices: [Vec3A; 24] = triangles.map(|i| vertices[i]);

	let mut subdivided: Vec<Vec3A> = Vec::new();
	subdivided.reserve_exact(order.pow(2));

	let edge_frac = 1. / order as f32;
	let half_edge_frac = 0.5 / order as f32;

	for index in (0..vertices.len()).step_by(3) {
		let (a, b, c) = (vertices[index], vertices[index + 1], vertices[index + 2]);
		for y in 0..order {
			let y_frac = y as f32 / order as f32;
			let row_pos_b = b.lerp(a, y_frac);
			let row_pos_c = c.lerp(a, y_frac);
			let row_divisor = (order - y) as f32;

			if y == order - 1 {
				subdivided.push(row_pos_b);
				subdivided.push(row_pos_c);
				subdivided.push(a);
			}

			let next_row_pos_b = b.lerp(a, y_frac + edge_frac);
			let next_row_pos_c = c.lerp(a, y_frac + edge_frac);
			let next_row_divisor = (order - y - 1) as f32;

			for x in 0..(order - y) {
				let lower_left = row_pos_b.lerp(row_pos_c, x as f32 / row_divisor);
				let lower_right = row_pos_b.lerp(row_pos_c, (x + 1) as f32 / row_divisor);
				let upper_left = next_row_pos_b.lerp(next_row_pos_c, x as f32 / next_row_divisor);

				subdivided.push(lower_left);
				subdivided.push(lower_right);
				subdivided.push(upper_left);

				if x < (order - y - 1) {
					let upper_right =
						next_row_pos_b.lerp(next_row_pos_c, (x + 1) as f32 / next_row_divisor);
					subdivided.push(lower_right);
					subdivided.push(upper_right);
					subdivided.push(upper_left);
				}
			}
		}
	}

	let colours = subdivided.clone();

	let points = subdivided
		.iter()
		.map(|&p| (p.normalize() * radius).into())
		.collect::<Vec<[f32; 3]>>();

	let colour = colours
		.iter()
		.map(|&p| [p.x / 2., 0., 0., 1.])
		.collect::<Vec<[f32; 4]>>();

	let uv = colours.iter().map(|&_| [0., 0.]).collect::<Vec<[f32; 2]>>();

	let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
	//mesh.set_indices(Some(indices));
	mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, points);
	mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, colour);
	mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uv);

	mesh.compute_flat_normals();

	info!("Generated mesh with {} verticies", colours.len());

	mesh
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	//asset_server: Res<AssetServer>,
) {
	//let texture_handle: Handle<Texture> = asset_server.load("textures/colour_map.jpg");
	//Assets::get(&self, texture_handle)
	//asset_server.get_path_asset_loader().unwrap();

	commands.spawn_bundle(PbrBundle {
		mesh: meshes.add(get_mesh(8, 2.)),
		material: materials.add(StandardMaterial {
			roughness: 0.7,
			base_color: Color::rgba_u8(5, 5, 5, 255),
			..Default::default()
		}),
		transform: Transform::from_xyz(0.0, 0.0, 0.0),
		..Default::default()
	});
}
