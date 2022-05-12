mod model_construction;
mod utils;

use model_construction::catmull_clark::catmull_clark_super;
use model_construction::catmull_clark::Face;
use model_construction::catmull_clark::Vertex;
use model_construction::raster::Raster;

use std::fs::File;
use std::io::Write;

pub fn main() {
	gen_box();

	let file_name = String::from("box_output.obj");
	let mut file = File::create(file_name).unwrap();
	let mut verts: String = String::new();
	let mut faces: String = String::new();

	// OBJ GENERATION
	//VERTEX LINE:  v x y z
	//FACE TRIANGLE: f [v1] [v2] [v3]

	let mut raster = Raster::new(0.0, 0.0, 0, 0);
	let mut is_sharp: Vec<Vec<bool>> = Vec::new();

	let (vs, fs) = catmull_clark_super(1, &is_sharp, &mut raster).expect("catumull broke");

	for v in vs {
		verts.push_str(&format!("v {a} {c} {b} \n", a = v.x, b = v.y, c = v.z));
	}
	for f in fs {
		faces.push_str(&format!("f {a} {c} {b} \n", a = f.points[0] + 1, b = f.points[1] + 1, c = f.points[3] + 1));
		faces.push_str(&format!("f {a} {c} {b} \n", a = f.points[0] + 1, b = f.points[2] + 1, c = f.points[3] + 1));
	}

	verts.push_str(&faces);
	file.write_all(verts.as_bytes());
}

pub fn gen_box() {
	let file_name = String::from("box_input.obj");
	let mut file = File::create(file_name).unwrap();

	let mut points = Vec::new();
	let mut fs: Vec<Face> = Vec::new();

	points.push(Vertex {
		x: 0.0,
		y: 0.0,
		z: 0.0,
		is_sharp: false,
	});
	points.push(Vertex {
		x: 0.0,
		y: 5.0,
		z: 0.0,
		is_sharp: false,
	});
	points.push(Vertex {
		x: 5.0,
		y: 0.0,
		z: 0.0,
		is_sharp: false,
	});
	points.push(Vertex {
		x: 5.0,
		y: 5.0,
		z: 0.0,
		is_sharp: false,
	});

	points.push(Vertex {
		x: 0.0,
		y: 0.0,
		z: 5.0,
		is_sharp: false,
	});
	points.push(Vertex {
		x: 0.0,
		y: 5.0,
		z: 5.0,
		is_sharp: false,
	});
	points.push(Vertex {
		x: 5.0,
		y: 0.0,
		z: 5.0,
		is_sharp: false,
	});
	points.push(Vertex {
		x: 5.0,
		y: 5.0,
		z: 5.0,
		is_sharp: false,
	});

	fs.push(Face { points: vec![0, 1, 2, 3] });
	fs.push(Face { points: vec![0, 1, 4, 5] });
	fs.push(Face { points: vec![0, 2, 4, 6] });
	fs.push(Face { points: vec![1, 3, 5, 7] });
	fs.push(Face { points: vec![2, 3, 6, 7] });
	fs.push(Face { points: vec![4, 5, 6, 7] });

	file.write_all(make_obj(points, fs).as_bytes());
}

pub fn make_obj(vs: Vec<Vertex>, fs: Vec<Face>) -> String {
	let mut verts: String = String::new();
	let mut faces: String = String::new();

	for v in vs {
		verts.push_str(&format!("v {a} {c} {b} \n", a = v.x, b = v.y, c = v.z));
	}
	for f in fs {
		faces.push_str(&format!("f {a} {c} {b} \n", a = f.points[0] + 1, b = f.points[1] + 1, c = f.points[3] + 1));
		faces.push_str(&format!("f {a} {c} {b} \n", a = f.points[0] + 1, b = f.points[2] + 1, c = f.points[3] + 1));
	}

	verts.push_str(&faces);
	verts
}
