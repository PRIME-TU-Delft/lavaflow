use gltf_json as json;

use std::mem;

use json::validation::Checked::Valid;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vertex {
	position: [f32; 3],
	color: [f32; 3],
}

fn to_padded_byte_vector<T>(vec: Vec<T>) -> Vec<u8> {
	let byte_length = vec.len() * mem::size_of::<T>();
	let byte_capacity = vec.capacity() * mem::size_of::<T>();
	let alloc = vec.into_boxed_slice();
	let ptr = Box::<[T]>::into_raw(alloc).cast::<u8>();
	let mut new_vec = unsafe { Vec::from_raw_parts(ptr, byte_length, byte_capacity) };
	while new_vec.len() % 4 != 0 {
		new_vec.push(0); // pad to multiple of four bytes
	}
	new_vec
}

pub fn generate_gltf(triangle_vertices: Vec<Vertex>) -> String {
	let triangle_vertices_len = triangle_vertices.len();

	let bin_content = to_padded_byte_vector(triangle_vertices);
	let mut bin_content_b64 = String::from("data:application/octet-stream;base64,");
	bin_content_b64.push_str(&base64::encode(bin_content));

	let buffer_length = (triangle_vertices_len * mem::size_of::<Vertex>()) as u32;
	let buffer = json::Buffer {
		byte_length: buffer_length,
		extensions: Default::default(),
		extras: Default::default(),
		name: None,
		uri: Some(bin_content_b64),
	};
	let buffer_view = json::buffer::View {
		buffer: json::Index::new(0),
		byte_length: buffer.byte_length,
		byte_offset: None,
		byte_stride: Some(mem::size_of::<Vertex>() as u32),
		extensions: Default::default(),
		extras: Default::default(),
		name: None,
		target: Some(Valid(json::buffer::Target::ArrayBuffer)),
	};
	let positions = json::Accessor {
		buffer_view: Some(json::Index::new(0)),
		byte_offset: 0,
		count: triangle_vertices_len as u32,
		component_type: Valid(json::accessor::GenericComponentType(json::accessor::ComponentType::F32)),
		extensions: Default::default(),
		extras: Default::default(),
		type_: Valid(json::accessor::Type::Vec3),
		min: Some(json::Value::from(vec![-0.5f32, -0.5f32, 0.0f32])),
		max: Some(json::Value::from(vec![0.5f32, 0.5f32, 0.0f32])),
		name: None,
		normalized: false,
		sparse: None,
	};
	let colors = json::Accessor {
		buffer_view: Some(json::Index::new(0)),
		byte_offset: (3 * mem::size_of::<f32>()) as u32,
		count: triangle_vertices_len as u32,
		component_type: Valid(json::accessor::GenericComponentType(json::accessor::ComponentType::F32)),
		extensions: Default::default(),
		extras: Default::default(),
		type_: Valid(json::accessor::Type::Vec3),
		min: None,
		max: None,
		name: None,
		normalized: false,
		sparse: None,
	};

	let primitive = json::mesh::Primitive {
		attributes: {
			let mut map = std::collections::HashMap::new();
			map.insert(Valid(json::mesh::Semantic::Positions), json::Index::new(0));
			map.insert(Valid(json::mesh::Semantic::Colors(0)), json::Index::new(1));
			map
		},
		extensions: Default::default(),
		extras: Default::default(),
		indices: None,
		material: None,
		mode: Valid(json::mesh::Mode::Triangles),
		targets: None,
	};

	let mesh = json::Mesh {
		extensions: Default::default(),
		extras: Default::default(),
		name: None,
		primitives: vec![primitive],
		weights: None,
	};

	let node = json::Node {
		camera: None,
		children: None,
		extensions: Default::default(),
		extras: Default::default(),
		matrix: None,
		mesh: Some(json::Index::new(0)),
		name: None,
		rotation: None,
		scale: None,
		translation: None,
		skin: None,
		weights: None,
	};

	let root = json::Root {
		accessors: vec![positions, colors],
		buffers: vec![buffer],
		buffer_views: vec![buffer_view],
		meshes: vec![mesh],
		nodes: vec![node],
		scenes: vec![json::Scene {
			extensions: Default::default(),
			extras: Default::default(),
			name: None,
			nodes: vec![json::Index::new(0)],
		}],
		..Default::default()
	};

	json::serialize::to_string_pretty(&root).expect("Serialization error")
}
