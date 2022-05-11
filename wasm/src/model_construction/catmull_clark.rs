use super::raster::Raster;

/// a face is a square of references to points
/// should only ever have 4 points
struct Face {
	points: Vec<usize>,
}

#[derive(Clone)]
struct Vertex {
	x: f32,
	y: f32,
	z: f32,
	is_sharp: bool,
}

#[derive(Clone)]
struct Edge {
	p1: usize,
	p2: usize,
	f1: usize,
	f2: Option<usize>,
	center: Vertex,
}

///
/// TODO: figure out input and return in relation to GLTF : list(triangle(point, point, point))
///
pub fn catmull_clark_super(iterations: usize, is_sharp: &Vec<Vec<bool>>, raster: &mut Raster) {
	// transform raster to list of faces and vertices
	let (mut f, mut v) = raster_to_faces(raster, is_sharp);

	// call catmull clark i times
	for i in 0..iterations {
		catmull_clark(&f, &v);
	}
}

//TODO IMPLEMENT
fn raster_to_faces(raster: &mut Raster, is_sharp: &Vec<Vec<bool>>) -> (Vec<Face>, Vec<Vertex>) {
	(
		vec![],
		vec![Vertex {
			x: 0.0,
			y: 1.0,
			z: 2.0,
			is_sharp: false,
		}],
	)
}

// implemented using : https://rosettacode.org/wiki/Catmull%E2%80%93Clark_subdivision_surface
//TODO modify implementation such that sharp values are not modified
//TODO dont copy input lists
fn catmull_clark(f: &Vec<Face>, v: &Vec<Vertex>) -> Result<(), String> {
	// 1. for each face, a face point is created which is the average of all the points of the face.
	// each entry in the returned list is a vertex.

	let face_points = get_face_points(v, f);

	// get list of edges with 1 or 2 adjacent faces
	// [pointnum_1, pointnum_2, facenum_1, facenum_2, center] or
	// [pointnum_1, pointnum_2, facenum_1, None, center]

	let edges = get_edges_faces(v, f);

	// get edge points, a list of points
	let edge_points = get_edge_points(v, &edges?, &face_points?);

	// per point: the average of the face points of the faces the point belongs to (avg_face_points)
	let avg_face_points = get_average_face_points(v, f, &face_points?);

	//per point:  the average of the centers of edges the point belongs to (avg_mid_edges)
	let avg_mid_edges = get_average_edges(v, &edges?);

	// how many faces a point belongs to
	let points_faces = get_faces_per_point(v, f);

	//find out new locations of exisitng points in mesh
	//TODO sharp points should not ever move in y direction
	let new_points = get_new_points(v, &points_faces, &avg_face_points, &avg_mid_edges);

	Ok(())
	// // add face points to new_points

	// let face_point_nums = [];

	// // point num after next append to new_points
	// let next_pointnum = len(new_points);

	// for face_point in face_points{
	//     new_points.append(face_point);
	//     face_point_nums.append(next_pointnum);
	//     next_pointnum += 1;
	// }

	// // add edge points to new_points

	// let edge_point_nums = dict()

	// for edgenum in range(len(edges_faces)){
	//     let pointnum_1 = edges_faces[edgenum][0];
	//     let pointnum_2 = edges_faces[edgenum][1];
	//     let  edge_point = edge_points[edgenum];
	//     let new_points.append(edge_point);
	//     let edge_point_nums[(pointnum_1, pointnum_2)] = next_pointnum;
	//     let next_pointnum += 1;

	//     let new_faces =[];
	// }
	// for oldfacenum in range(len(f)){
	//     let oldface = f[oldfacenum];
	//     // 4 point face
	//     if len(oldface) == 4 {
	//         a = oldface[0];
	//         b = oldface[1];
	//         c = oldface[2];
	//         d = oldface[3];
	//         face_point_abcd = face_point_nums[oldfacenum]
	//         edge_point_ab = edge_point_nums[switch_nums((a, b))];
	//         edge_point_da = edge_point_nums[switch_nums((d, a))];
	//         edge_point_bc = edge_point_nums[switch_nums((b, c))];
	//         edge_point_cd = edge_point_nums[switch_nums((c, d))];
	//         new_faces.append((a, edge_point_ab, face_point_abcd, edge_point_da));
	//         new_faces.append((b, edge_point_bc, face_point_abcd, edge_point_ab));
	//         new_faces.append((c, edge_point_cd, face_point_abcd, edge_point_bc));
	//         new_faces.append((d, edge_point_da, face_point_abcd, edge_point_cd));
	//     }
	// }
	// return new_points, new_faces
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
fn get_edges_faces(v: &Vec<Vertex>, f: &Vec<Face>) -> Result<Vec<Edge>, String> {
	// will have [pointnum_1, pointnum_2, facenum]

	let mut edges: Vec<Edge> = Vec::new();

	// get edges from each face

	for facenum in 0..f.len() {
		let face = f.get(facenum).ok_or("get_edges_faces: vertex at index does not exist")?;
		let num_points = f.len();
		// loop over index into face
		for pointindex in 0..num_points {
			// if not last point then edge is curr point and next point
			let mut other_index = pointindex + 1;
			if !pointindex < (num_points - 1) {
				other_index = 0;
			}

			//fins indexes of points in face
			let i1 = *face.points.get(pointindex).ok_or("get_edges_faces: vertex at index does not exist")?;
			let i2 = *face.points.get(other_index).ok_or("get_edges_faces: vertex at index does not exist")?;

			//find points using verts
			let p1 = v.get(i1).ok_or("get_edges_faces: vertex at index does not exist")?;
			let p2 = v.get(i2).ok_or("get_edges_faces: vertex at index does not exist")?;

			// order points in edge by lowest point number
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
					p1: i1,
					p2: i2,
					f1: facenum,
					f2: None,
					center: center_point(p1, p2),
				});
			}
		}
	}

	// sort edges by pointnum_1, pointnum_2, facenum, for merge step
	//edges = sorted(edges);

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
			x: (AF.x + ME.x) / 2.0,
			y: (AF.y + ME.y) / 2.0,
			z: (AF.z + ME.z) / 2.0,
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

//Move each original point to the new vertex point (F + 2R + (n-3)*v)/n
//v				//n								//F							//R
fn get_new_points(vs: &Vec<Vertex>, f_per_v: &Vec<usize>, avg_face_points: &Vec<Vertex>, avg_mid_edges: &Vec<Vertex>) -> Result<(), String> {
	let mut new_vertices: Vec<Vertex> = Vec::new();

	for i in 0..vs.len() {
		let v = vs.get(i).ok_or("get average face points : could not find face")?;
		let n = f_per_v.get(i).ok_or("get average face points : could not find face")?;
		let F = avg_face_points.get(i).ok_or("get average face points : could not find face")?;
		let R = avg_mid_edges.get(i).ok_or("get average face points : could not find face")?;

		let x = (v.x * (*n as f32 - 3.0) + (2.0 * R.x) + F.x);
		let y = (v.y * (*n as f32 - 3.0) + (2.0 * R.y) + F.y);
		let z = (v.y * (*n as f32 - 3.0) + (2.0 * R.y) + F.y);

		new_vertices.push(Vertex { x, y, z, is_sharp: false })
	}

	Ok(())
}
