use bevy::{
	math::Vec3A,
	prelude::*,
	render::{
		pipeline::{PipelineDescriptor, PrimitiveTopology, RenderPipeline},
		shader::{ShaderStage, ShaderStages},
	},
};

const VERTEX_SHADER: &str = r#"
#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Color;
layout(location = 0) out vec3 v_color;
layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
void main() {
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
    v_color = Vertex_Color;
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 450
layout(location = 0) out vec4 o_Target;
layout(location = 0) in vec3 v_color;
void main() {
    o_Target = vec4(v_color, 1.0);
}
"#;

struct WorldTexture(Handle<Texture>);

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(setup).add_system(load_world);
	}

	fn name(&self) -> &str {
		std::any::type_name::<Self>()
	}
}

fn sphere_uv(d: &Vec3A) -> Vec2 {
	Vec2::new(
		0.5 + f32::atan2(d.x, d.z) / (std::f32::consts::PI * 2.),
		0.5 - d.y.asin() / std::f32::consts::PI,
	)
}

fn height(d: &Vec3A, texture: &Texture) -> u8 {
	let sphere_uv = sphere_uv(d);

	if sphere_uv.x > 1. && sphere_uv.y > 1. {
		error!("More than one {:?}", sphere_uv);
	}
	let dims = texture.size;
	let pixel_coord =
		(Vec2::new((dims.width - 1) as f32, (dims.height - 1) as f32) * sphere_uv).as_u32();

	let pixel_index = ((dims.width) * (pixel_coord.y) + (pixel_coord.x)) as usize * 4;
	texture.data[pixel_index]
}

fn get_mesh(order: usize, radius: f32, height_map: &Texture, height_radius: f32) -> Mesh {
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
	let len = subdivided.len();

	let mut points: Vec<[f32; 3]> = Vec::new();
	let mut colours: Vec<[f32; 3]> = Vec::new();
	let mut uvs: Vec<[f32; 2]> = Vec::new();
	points.reserve_exact(len);
	colours.reserve_exact(len);
	uvs.reserve_exact(len);

	let mut max_tri_height = 0;

	for (index, pos) in subdivided.iter().enumerate() {
		let normalized = pos.normalize();
		let height = height(&normalized, height_map);
		points.push(
			(normalized * ((height as f32 / u8::MAX as f32) * height_radius + radius)).into(),
		);
		max_tri_height = max_tri_height.max(height);
		if index % 3 == 2 {
			let colour = if max_tri_height > 0 {
				[0.19, 0.65, 0.32]
			} else {
				[0.30, 0.38, 0.87]
			};
			colours.extend([colour; 3]);
			uvs.extend([[0., 0.]; 3]);
			max_tri_height = 0;
		}
	}

	let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
	//mesh.set_indices(Some(indices));
	mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, points);
	mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, colours);
	mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

	mesh.compute_flat_normals();

	info!("Generated mesh with {} verticies", len);

	mesh
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
	let texture_handle: Handle<Texture> = asset_server.load("textures/height_map.png");
	commands.insert_resource(WorldTexture(texture_handle));
}

fn load_world(
	world_texture: Option<Res<WorldTexture>>,
	textures: Res<Assets<Texture>>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut pipelines: ResMut<Assets<PipelineDescriptor>>,
	mut shaders: ResMut<Assets<Shader>>,
) {
	if let Some(world_texture) = world_texture {
		if let Some(height_map) = textures.get(&world_texture.0) {
			// Create a new shader pipeline
			let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
				vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
				fragment: Some(
					shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER)),
				),
			}));
			info!("Dims {:?}, len {}", height_map.size, height_map.data.len());
			commands.spawn_bundle(PbrBundle {
				mesh: meshes.add(get_mesh(50, 2., height_map, 0.3)),
				render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
					pipeline_handle,
				)]),

				transform: Transform::from_xyz(0.0, 0.0, 0.0),
				..Default::default()
			});
			commands.remove_resource::<WorldTexture>();
		}
	}
}
