use std::{cmp::Ordering, collections::HashMap, usize};

use super::{
	point::Point,
	raster::{self, Raster},
};

/// a face is a square of references to points
/// should only ever have 4 points
/// TODO: remove pub
pub struct Face {
	pub points: Vec<usize>,
}

//TODO: remove pub
#[derive(Clone)]
pub struct Vertex {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub is_sharp: bool,
}

//TODO: remove pub
#[derive(Clone)]
pub struct Edge {
	p1: usize,
	p2: usize,
	f1: usize,
	f2: Option<usize>,
	center: Vertex,
}

///
/// TODO: figure out input and return in relation to GLTF : list(triangle(point, point, point))
///
pub fn catmull_clark_super(iterations: usize, is_sharp: &Vec<Vec<bool>>, raster: &mut Raster) -> Result<(Vec<Vertex>, Vec<Face>), String> {
	// transform raster to list of faces and vertices
	let (mut vs, mut fs) = raster_to_faces(raster, is_sharp);

	// call catmull clark i times
	for i in 0..iterations {
		(vs, fs) = catmull_clark(&fs, &vs)?;
		println!("surface subdivision iteration {i} done!");
	}

	Ok((vs, fs))
}

//TODO IMPLEMENT
fn raster_to_faces(raster: &mut Raster, is_sharp: &Vec<Vec<bool>>) -> (Vec<Vertex>, Vec<Face>) {
	let mut points = Vec::new();
	let mut faces = Vec::new();

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

	faces.push(Face { points: vec![0, 1, 3, 2] });
	faces.push(Face { points: vec![0, 1, 5, 4] });
	faces.push(Face { points: vec![0, 2, 6, 4] });
	faces.push(Face { points: vec![1, 3, 7, 5] });
	faces.push(Face { points: vec![2, 3, 7, 6] });
	faces.push(Face { points: vec![4, 5, 7, 6] });



	(points, faces)
}

// implemented using : https://rosettacode.org/wiki/Catmull%E2%80%93Clark_subdivision_surface
//   			and : https://en.wikipedia.org/wiki/Catmull%E2%80%93Clark_subdivision_surface
//TODO modify implementation such that sharp values are not modified
//TODO dont copy input lists
fn catmull_clark(fs: &Vec<Face>, vs: &Vec<Vertex>) -> Result<(Vec<Vertex>, Vec<Face>), String> {
	//
	// FINDING ALL NEW POINTS
	//

	// 1. for each face, a face point is created which is the average of all the points of the face.
	// each entry in the returned list is a vertex.
	let face_points = get_face_points(vs, fs)?;

	// get list of edges with 1 or 2 adjacent faces
	// [pointnum_1, pointnum_2, facenum_1, facenum_2, center] or
	// [pointnum_1, pointnum_2, facenum_1, None, center]

	let edges = get_edges_faces(vs, fs)?;

	// get edge points, a list of points = (average of face points + edge center)/2
	let edge_points = get_edge_points(vs, &edges, &face_points)?;

	// per point: the average of the face points of the faces the point belongs to (avg_face_points)
	let avg_face_points = get_average_face_points(vs, fs, &face_points)?;

	//per point:  the average of the centers of edges the point belongs to (avg_mid_edges)
	let avg_mid_edges = get_average_edges(vs, &edges)?;

	// how many faces a point belongs to
	let points_faces = get_faces_per_point(vs, fs)?;

	//find out new locations of exisitng points in mesh
	//TODO sharp points should not ever move in y direction
	let mut new_points = get_new_points(vs, &points_faces, &avg_face_points, &avg_mid_edges)?;

	//
	// ADDING NEW POINTS TO USABLE LISTS
	//

	// add face points to new_points
	// and keep track of indexes of newly added points
	let mut face_point_index: Vec<usize> = Vec::new();

	//TODO is use of & and * here correct? want to avoid copying vec to get len but use literal len.
	let mut next_index = new_points.len();

	for face_point in face_points {
		new_points.push(face_point);
		face_point_index.push(next_index);
		next_index += 1;
	}

	// add edge points to new_points, using hash so you can find index per edge
	let mut edge_index_map: HashMap<(usize, usize), usize> = HashMap::with_capacity(edges.len());

	//TODO how to do index trav properly in rust ;-;
	let mut edgenum = 0;
	for edge in edges {
		let edge_point = edge_points.get(edgenum).ok_or("catmull clark: edge does not exist at index")?.clone();
		new_points.push(edge_point);
		edge_index_map.insert(smallest_first(edge.p1, edge.p2), next_index);
		next_index += 1;
		edgenum += 1;
	}
	// println!("MAP");
	// for e in &edge_index_map{
	// 	println!("edge ({a} , {b})", a = e.0.0, b = e.0.1)
		
	// }
	//println!("smallest first test : {a} {b}", a = smallest_first(2, 0).0 , b = smallest_first(2, 0).1);
	//
	//  GENERATE FACES USING NEW POINTS
	//

	let mut new_faces: Vec<Face> = Vec::new();

	//TODO how to do index trav properly in rust ;-;
	let mut facenum = 0;

	for f in fs {
		// 4 point required in face
		if f.points.len() == 4 {
			let a = *f.points.get(0).unwrap();
			let b = *f.points.get(1).unwrap();
			let c = *f.points.get(2).unwrap();
			let d = *f.points.get(3).unwrap();
			let face_point_abcd = face_point_index[facenum];
			let edge_point_ab = *edge_index_map.get(&smallest_first(a, b)).ok_or(format!("catmull : egde ab not found in map: {a} , {b}"))?;
			let edge_point_da = *edge_index_map.get(&smallest_first(d, a)).ok_or(format!("catmull : da de ab not found in map: {d} , {a}"))?;
			let edge_point_bc = *edge_index_map.get(&smallest_first(b, c)).ok_or("catmull : egde index bc not found in map")?;
			let edge_point_cd = *edge_index_map.get(&smallest_first(c, d)).ok_or("catmull : egde index cd not found in map")?;

			new_faces.push(Face {
				points: vec![a, edge_point_ab, face_point_abcd, edge_point_da],
			});
			new_faces.push(Face {
				points: vec![b, edge_point_bc, face_point_abcd, edge_point_ab],
			});
			new_faces.push(Face {
				points: vec![c, edge_point_cd, face_point_abcd, edge_point_bc],
			});
			new_faces.push(Face {
				points: vec![d, edge_point_da, face_point_abcd, edge_point_cd],
			});

			facenum += 1;
		} else {
			return Err(String::from("catmull ; face found with less than 4 vertices"));
		}
	}

	Ok((new_points, new_faces))
}

///
/// HELPER METHODS
///

//TODO what do if p is sharp
fn center_point(p1: &Vertex, p2: &Vertex) -> Vertex {
	Vertex {
		x: (p1.x + p2.x) / 2.0,
		y: (p1.y + p2.y) / 2.0,
		z: (p1.z + p2.z) / 2.0,
		is_sharp: false,
	}
}

//TODO what do if p is sharp
fn add(p1: &Vertex, p2: &Vertex) -> Vertex {
	Vertex {
		x: (p1.x + p2.x),
		y: (p1.y + p2.y),
		z: (p1.z + p2.z),
		is_sharp: false,
	}
}

fn average_of_points(xs: Vec<Vertex>) -> Vertex {
	let n = xs.len() as f32;
	let mut agr = Vertex {
		x: 0.0,
		y: 0.0,
		z: 0.0,
		is_sharp: false,
	};
	for x in xs {
		agr = add(&agr, &x);
	}
	Vertex {
		x: agr.x / n,
		y: agr.y / n,
		z: agr.z / n,
		is_sharp: false,
	}
}
fn average_of_points_b(xs: Vec<&Vertex>) -> Vertex {
	let n = xs.len() as f32;
	let mut agr = Vertex {
		x: 0.0,
		y: 0.0,
		z: 0.0,
		is_sharp: false,
	};
	for x in xs {
		agr = add(&agr, &x);
	}
	Vertex {
		x: agr.x / n,
		y: agr.y / n,
		z: agr.z / n,
		is_sharp: false,
	}
}

fn smallest_first(p1: usize, p2: usize) -> (usize, usize) {
	if (p1 <= p2) {
		(p1, p2)
	} else {
		(p2, p1)
	}
}

//for all faces find middle point on face
fn get_face_points(v: &Vec<Vertex>, f: &Vec<Face>) -> Result<Vec<Vertex>, String> {
	let mut face_points: Vec<Vertex> = Vec::new();

	// per face averace points
	for curr_face in f {
		let mut x = 0.0;
		let mut y = 0.0;
		let mut z = 0.0;

		for i in &curr_face.points {
			let curr_point: &Vertex = v.get(*i).ok_or("get all face points: vertex at index does not exist")?;
			x += curr_point.x;
			y += curr_point.y;
			z += curr_point.z;
		}
		//TODO what do if points are sharp
		face_points.push(Vertex {
			x: x / 4.0,
			y: y / 4.0,
			z: z / 4.0,
			is_sharp: false,
		});
	}
	if (f.len() != face_points.len()) {
		return Err(String::from("number face points generated does not match number of faces"));
	}

	Ok(face_points)
}

//gets all edges of faces -> for each edge the adjacent faces and center of edge
fn get_edges_faces(vs: &Vec<Vertex>, fs: &Vec<Face>) -> Result<Vec<Edge>, String> {
	// will have [pointnum_1, pointnum_2, facenum]

	let mut edges: Vec<Edge> = Vec::new();

	// get edges from each face

	for facenum in 0..fs.len() {
		let f = fs.get(facenum).ok_or(format!("get_edges_faces: vertex at index does not exist at {facenum}, f.len = {} ", fs.len()))?;
		let num_points = f.points.len();
		// hardcoded tuples of face corners that make edges
		let es = vec![(f.points[0], f.points[1]), (f.points[1], f.points[2]), (f.points[2], f.points[3]), (f.points[3], f.points[0])];
		
		for (i1, i2) in es{
				

				let p1 = vs.get(i1).ok_or("get_edges_faces: vertex at index does not exist")?;
				let p2 = vs.get(i2).ok_or("get_edges_faces: vertex at index does not exist")?;
				

				// order points in edge by lowest point number
				// and define center of edges
				if (i1 < i2) {
					edges.push(Edge {
						p1: i1,
						p2: i2,
						f1: facenum,
						f2: None,
						center: center_point(p1, p2),
					});
				} else {
					edges.push(Edge {
						p1: i2,
						p2: i1,
						f1: facenum,
						f2: None,
						center: center_point(p1, p2),
					});
				}
			}
		}


	//sort edges by pointnum_1, pointnum_2, facenum, for merge step
	edges.sort_by_key(|a|( a.p1, a.p2, a.f1) );

	//if two faces share an edge it is currently in edges list twice so
	// merge edges with 2 adjacent faces
	// results in :
	// [pointnum_1, pointnum_2, facenum_1, facenum_2] or
	// [pointnum_1, pointnum_2, facenum_1, None]

	let num_edges = edges.len();
	let mut e_index = 0;
	let mut merged_edges: Vec<Edge> = Vec::new();

	while e_index < num_edges {
		let e1 = edges.get(e_index).ok_or("get_edges_faces: edge at index does not exist")?;
		// check if not last edge, if so no e2 and no neighboring edge
		if e_index < num_edges - 1 {
			let e2 = edges.get(e_index + 1).ok_or("get_edges_faces: edge at index does not exist")?;
			if e1.p1 == e2.p1 && e1.p2 == e2.p2 {
				merged_edges.push(Edge {
					p1: e1.p1,
					p2: e1.p2,
					f1: e1.f1,
					f2: Some(e2.f1),
					center: e1.center.clone(),
				});
				e_index += 2
			} else {
				merged_edges.push(Edge {
					p1: e1.p1,
					p2: e1.p2,
					f1: e1.f1,
					f2: None,
					center: e1.center.clone(),
				});
				e_index += 1
			}
		} else {
			merged_edges.push(Edge {
				p1: e1.p1,
				p2: e1.p2,
				f1: e1.f1,
				f2: None,
				center: e1.center.clone(),
			});
			e_index += 1
		}
	}

	// println!("final edges");
	// for e in &merged_edges{
	// 	println !("e : ({e1},{e2}), face num : {e3}" , e1 =e.p1,
	// 	e2 = e.p2,
	// 	e3 = e.f1)
	// }

	Ok(merged_edges)
}

//For each edge, add an edge point.
//Set each edge point to be the average of the two neighbouring face points (AF) and the midpoint of the edge (ME)
// = (AF + ME)/ 2
fn get_edge_points(v: &Vec<Vertex>, edges: &Vec<Edge>, face_points: &Vec<Vertex>) -> Result<Vec<Vertex>, String> {
	let mut edge_points: Vec<Vertex> = Vec::new();

	for edge in edges {
		let f1 = face_points.get(edge.f1).ok_or("get edge points: no face point")?;
		let f2 = match edge.f2 {
			Some(x) => face_points.get(edge.f2.unwrap()).ok_or("get edge points: no face point")?,
			None => f1,
		};

		let AF = add(f1, f2);
		let ME = &edge.center;
		//TODO what if sharp
		edge_points.push(Vertex {
			x: (AF.x + ME.x) / 3.0,
			y: (AF.y + ME.y) / 3.0,
			z: (AF.z + ME.z) / 3.0,
			is_sharp: false,
		});
	}

	Ok(edge_points)
}

//For each original point (P), take the average (F) of all n (recently created) face points for faces touching P
fn get_average_face_points(vs: &Vec<Vertex>, fs: &Vec<Face>, face_points: &Vec<Vertex>) -> Result<Vec<Vertex>, String> {
	let mut averages: Vec<Vertex> = Vec::new();

	//for each vertex
	for i in 0..vs.len() {
		let mut adjacents: Vec<Vertex> = Vec::new();

		//for each face
		for j in 0..fs.len() {
			let f = fs.get(j).ok_or("get average face points : could not find face")?;
			//if face is adjacent
			if f.points.contains(&i) {
				//add face point to list
				let fp = face_points.get(j).ok_or("get average face points : could not find face point")?.clone();
				adjacents.push(fp);
			}
		}
		//ret average
		averages.push(average_of_points(adjacents));
	}

	Ok(averages)
}

//For each original point (P), the average (R) of all n edge midpoints for original edges touching P, where each edge midpoint is the average of its two endpoint vertices
fn get_average_edges(vs: &Vec<Vertex>, es: &Vec<Edge>) -> Result<Vec<Vertex>, String> {
	let mut averages: Vec<Vertex> = Vec::new();

	//for each vertex
	for p in 0..vs.len() {
		let mut adjacents: Vec<Vertex> = Vec::new();

		//for each edge
		for e in es {
			//if egde is adjacent
			if e.p1 == p || e.p2 == p {
				//add center of edge to list
				adjacents.push(e.center.clone());
			}
		}
		//ret average
		averages.push(average_of_points(adjacents));
	}
	Ok(averages)
}

fn get_faces_per_point(vs: &Vec<Vertex>, fs: &Vec<Face>) -> Result<Vec<usize>, String> {
	let mut faces_per_point = vec![0; vs.len()];

	for f in fs {
		for v in &f.points {
			faces_per_point[*v] += 1;
		}
	}
	Ok(faces_per_point)
}

//TODO SHARP EDGES
//Move each original point to the new vertex point (F + 2R + (n-3)*v)/n
//v				//n								//F							//R
fn get_new_points(vs: &Vec<Vertex>, f_per_v: &Vec<usize>, avg_face_points: &Vec<Vertex>, avg_mid_edges: &Vec<Vertex>) -> Result<(Vec<Vertex>), String> {
	let mut new_vertices: Vec<Vertex> = Vec::new();

	for i in 0..vs.len() {
		let v = vs.get(i).ok_or("get average face points : could not find face")?;
		let n = *f_per_v.get(i).ok_or("get average face points : could not find face")? as f32;
		let F = avg_face_points.get(i).ok_or("get average face points : could not find face")?;
		let R = avg_mid_edges.get(i).ok_or("get average face points : could not find face")?;

		let x = ((v.x * (n  - 3.0)) + (2.0 * R.x) + F.x) / n;
		let y = ((v.y * (n  - 3.0)) + (2.0 * R.y) + F.y) / n;
		let z = ((v.z * (n  - 3.0)) + (2.0 * R.z) + F.z) / n;

		new_vertices.push(Vertex { x, y, z, is_sharp: false })
	}

	Ok(new_vertices)
}
