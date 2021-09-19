use bevy::{math::Vec3A, prelude::*, render::pipeline::PrimitiveTopology};

use crate::{city::add_cities, GameState};

pub struct WorldTexture {
	pub handle: Handle<Texture>,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(setup)
			.add_system_set(SystemSet::on_update(GameState::Loading).with_system(load_world));
	}

	fn name(&self) -> &str {
		std::any::type_name::<Self>()
	}
}

pub struct HeightmapSampler {
	pub radius: f32,
	pub height_radius: f32,
}
impl HeightmapSampler {
	fn sphere_uv(d: &Vec3A) -> Vec2 {
		Vec2::new(
			0.5 + f32::atan2(d.x, d.z) / (std::f32::consts::PI * 2.),
			0.5 - d.y.asin() / std::f32::consts::PI,
		)
	}

	pub fn height(&self, d: &Vec3A, texture: &Texture) -> u8 {
		let sphere_uv = Self::sphere_uv(d);

		if sphere_uv.x > 1. && sphere_uv.y > 1. {
			error!("More than one {:?}", sphere_uv);
		}
		let dims = texture.size;
		let pixel_coord =
			(Vec2::new((dims.width - 1) as f32, (dims.height - 1) as f32) * sphere_uv).as_uvec2();

		let pixel_index = ((dims.width) * (pixel_coord.y) + (pixel_coord.x)) as usize * 4;
		texture.data[pixel_index]
	}
	pub fn sample(&self, d: impl Into<Vec3A>, texture: &Texture) -> Vec3A {
		let d = d.into();
		d * ((self.height(&d, texture) as f32 / u8::MAX as f32) * self.height_radius + self.radius)
	}
	fn get_mesh(&self, order: usize, texture: &Texture) -> [Mesh; 2] {
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
					let upper_left =
						next_row_pos_b.lerp(next_row_pos_c, x as f32 / next_row_divisor);

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
		let len = subdivided.len();

		let mut land: (Vec<[f32; 3]>, Vec<[f32; 2]>) = (Vec::new(), Vec::new());
		let mut water: (Vec<[f32; 3]>, Vec<[f32; 2]>) = (Vec::new(), Vec::new());

		for index in (0..subdivided.len()).step_by(3) {
			let mut max_tri_height = 0;
			let positions = subdivided[index..index + 3]
				.iter()
				.map(|pos| {
					let normalized = pos.normalize();
					let height = self.height(&normalized, texture);
					max_tri_height = max_tri_height.max(height);
					(normalized
						* ((height as f32 / u8::MAX as f32) * self.height_radius + self.radius))
						.to_array()
				})
				.collect::<Vec<[f32; 3]>>();

			if max_tri_height > 0 {
				land.0.extend(positions);
				land.1.extend([[0., 0.]; 3]);
			} else {
				water.0.extend(positions);
				water.1.extend([[0., 0.]; 3]);
			}
		}

		let mut land_mesh = Mesh::new(PrimitiveTopology::TriangleList);
		land_mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, land.0);
		land_mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, land.1);
		land_mesh.compute_flat_normals();

		let mut water_mesh = Mesh::new(PrimitiveTopology::TriangleList);
		water_mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, water.0);
		water_mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, water.1);
		water_mesh.compute_flat_normals();

		info!("Generated mesh with {} verticies", len);

		[land_mesh, water_mesh]
	}
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
	let texture_handle: Handle<Texture> = asset_server.load("textures/height_map.png");
	commands.insert_resource(WorldTexture {
		handle: texture_handle,
	});
}

fn load_world(
	world_texture: Option<Res<WorldTexture>>,
	textures: Res<Assets<Texture>>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut state: ResMut<State<GameState>>,
) {
	if let Some(world_texture) = world_texture {
		if let Some(height_map) = textures.get(&world_texture.handle) {
			info!("Dims {:?}, len {}", height_map.size, height_map.data.len());
			let sampler = HeightmapSampler {
				radius: 2.,
				height_radius: 0.3,
			};
			let [land, ocean] = sampler.get_mesh(70, height_map);
			commands.spawn_bundle(PbrBundle {
				mesh: meshes.add(land),
				material: materials.add(StandardMaterial {
					base_color: Color::rgb_u8(51, 189, 62),
					roughness: 0.1,
					..Default::default()
				}),
				transform: Transform::from_xyz(0.0, 0.0, 0.0),
				..Default::default()
			});
			commands.spawn_bundle(PbrBundle {
				mesh: meshes.add(ocean),
				material: materials.add(StandardMaterial {
					base_color: Color::rgb_u8(66, 135, 245),
					roughness: 0.1,
					..Default::default()
				}),
				transform: Transform::from_xyz(0.0, 0.0, 0.0),
				..Default::default()
			});

			add_cities(&mut commands, meshes, materials, &sampler, height_map);

			state.set(GameState::Account).unwrap();
			commands.insert_resource(sampler);
		}
	}
}
