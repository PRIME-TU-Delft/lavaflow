use super::gltf_conversion::generate_gltf;
use std::{cmp::Ordering, collections::HashMap, fs::File, io::Write, usize};
use wasm_bindgen::JsValue;

use super::{
	point::Point,
	raster::{self, Raster},
};

/// a face is a square of references to points
/// should only ever have 4 points
pub struct Face {
	pub points: Vec<usize>,
}

/// Struct representing a point in 3d space and its sharpness. A vertex is sharp if it is on a contour line or strictly between points on a contour line. If a point is sharp it should not move in the z direction.
/// 

#[derive(Clone, PartialEq)]
pub struct Vertex {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub is_sharp: bool,
	pub half_sharp: bool,
}

/// Struct representing an edge in a vertex-face representation of a model.
/// 
/// # Fields
///  
/// * `p1 : usize` - index of first point at end of edge.
/// * `p2 : usize`- index of second point at end of edge.
/// * `f1 : usize` - index of face edge is adjacent to.  
/// * `f2 : Optional<usize>` - optional index of face edge is adjacent to. None if egde is on 'end' of model.
/// * `middle : Vertex`  - Point in the middle of p1 and p2.
/// 
#[derive(Clone)]
pub struct Edge {
	p1: usize,
	p2: usize,
	f1: usize,
	f2: Option<usize>,
	middle: Vertex,
}
	/// Applies the catmull clark surface subdivision algorithm over a given raster. Wrapper to use algoritm on raster, see method catmull_clark() for algorithm over only vetices and faces.
	/// Returns a model in the form of a list of vertices and faces, a face being 4 indexes corresponding to the list of vertexes.
	///
	/// # Arguments
	///
	/// * `iterations` - The number of subdivisions you want to occur. Setting to zero returns an unchanged model.
	/// * `is_sharp` -  Matrix of booleans defining 'sharp' points. A point that is sharp will not move in z direction during subdivision.
	/// * `raster` - The raster representing the model to be subdivided. 
	/// * `keep_heights` - If set to false sharpness of points will be ignored.
	///
	/// # Return
	/// * `Result<(Vec<Vertex>, Vec<Face>), String>` - Result containing vertex list and face list.
	///
pub fn catmull_clark_super(iterations: usize, is_sharp: &Vec<Vec<bool>>, raster: &Raster, keep_heights : bool) -> Result<(Vec<Vertex>, Vec<Face>), String> {

	//transform raster to list of faces and vertices
	let (mut vs, mut fs) = raster_to_faces(raster, is_sharp, keep_heights);

	// call catmull clark i times
	for _ in 0..iterations {
		(vs, fs) = catmull_clark(&fs, &vs)?;
	}

	if(vs.len() <= 0 ) {
		return Err(String::from("surface subdivision returns empty vertex list"));
	}
	if(fs.len() <=0 ) {
		return Err(String::from("surface subdivision returns empty face list"));
	}

	Ok((vs, fs))
}
	/// Transforms Raster to face -vertex representation.
	/// 
	/// Transformation of a cell in a raster in the ith column in the jth row:
	///
	/// 	(i, j) -- (i+ 1, j)
	/// 		|		|			--> Face ( points ((i, j) , (i+ 1, j) , (i, j + 1) , (i+1, j+1)) )
	/// 		|		|
	/// 	(i, j + 1) --(i+1, j+1)
	///
	/// # Arguments
	///
	/// * `raster` - The raster representing the model to be subdivided.
	/// * `is_sharp` -  Matrix of booleans defining 'sharp' points. A point that is sharp will not move in z direction during subdivision.
	/// * `keep_heights` - If set to false sharpness of points will be ignored, all is_sharp values in vertices is set to zero
	///
	///
	///
fn raster_to_faces(raster: &Raster, is_sharp: &Vec<Vec<bool>>, keep_heights : bool) -> (Vec<Vertex>, Vec<Face>) {
	let mut vs = Vec::new();
	let mut fs = Vec::new();

	let heights = &raster.altitudes;

	let mut next_index = 0;

	//TODO REFACTOR : think about how iteration over rows makes checking for duplicates in vs easier
	//TODO REFACTOR : SHOULD BE X = X* COLUMN WIDTH, Y = Y*ROW HEIGHT -> GIVES WRONG VISUAL RESULT (refactoring must happen in eithor constuctor.rs or in raster.rs)
	//TODO REFACTOR : rows and columns in for loop must also be swapped when raster problem is fixed
	for x in 0..raster.columns - 1 {
		for y in 0..raster.rows- 1 {
			//indexes of face vertices
			let mut ps = Vec::new();

			// calc corner vertices
			//0,0
			let a = Vertex {
				x: (x as f32 * raster.column_width),
				y: (( y) as f32 * raster.row_height),
				z: heights[y][x].unwrap(),
				is_sharp: if(keep_heights){is_sharp[y][x]} else {false},
				half_sharp: false,
			};
			//0,1
			let b = Vertex {
				x: (x as f32 * raster.column_width),
				y: (( y + 1) as f32) * raster.row_height,
				z: heights[y +1][x ].unwrap(),
				is_sharp: if(keep_heights){is_sharp[y][x + 1 ]} else {false},
				half_sharp: false,
			};
			//1, 0
			let c = Vertex {
				x: ((x + 1) as f32 * raster.column_width),
				y: (( y) as f32) * raster.row_height,
				z: heights[y ][x + 1].unwrap(),
				is_sharp: if(keep_heights){is_sharp[y ][x + 1 ]} else {false},
				half_sharp: false,
			};
			//1,1
			let d = Vertex {
				x: ((x + 1) as f32 * raster.column_width),
				y: (( y + 1) as f32) * raster.row_height,
				z: heights[y + 1][x + 1].unwrap(),
				is_sharp: if(keep_heights){is_sharp[y + 1][x + 1]} else {false},
				half_sharp: false,
			};

			
			//per vertex check if they are already in list of vertices before adding, if it exists, push exisitng index instead of duplicating
			//ORDERING isIMPORTANT DO NOT CHANGE! : (0,0),(0,1),(1,1),(1,0) (has to coincide with an order of edges)
			//TODO REFACTOR: code duplication

			//(0,0)
			if vs.contains(&a) {
				//if point already in list store its index
				ps.push(
					vs.iter()
						.position(|x| x == &a)
						.unwrap(),
				);
			} else {
				ps.push(next_index);
				vs.push(a);
				next_index += 1;
			}

			//(0,1)
			if vs.contains(&b) {
				//if point already in list store index to it
				ps.push(
					vs.iter()
						.position(|x| x == &b)
						.unwrap(),
				);
			} else {
				ps.push(next_index);
				vs.push(b);
				next_index += 1;
			}
			//(1,1)
			if vs.contains(&d) {
				//if point already in list store index to it
				ps.push(
					vs.iter()
						.position(|x| x == &d)
						.unwrap(),
				);
			} else {
				ps.push(next_index);
				vs.push(d);
				next_index += 1;
			}
			//(0 , 1)
			if vs.contains(&c) {
				//if point already in list store index to it
				ps.push(
					vs.iter()
						.position(|x| x == &c)
						.unwrap(),
				);
			} else {
				ps.push(next_index);
				vs.push(c);
				next_index += 1;
			}
			//add face

			fs.push(Face { points: ps });
		}
	}
	(vs, fs)
}
	/// Applies the catmull clark surface subdivision algorithm to a a list of vertices and faces, a face being 4 indexes corresponding to the list of vertexes.
	/// Implemented using https://rosettacode.org/wiki/Catmull%E2%80%93Clark_subdivision_surface and  https://en.wikipedia.org/wiki/Catmull%E2%80%93Clark_subdivision_surface as reference.
	/// Vertexes contain a bool feild is_sharp, when this is set to true its z position will be less affected by the algorithm. (To keep height of points on a contour line consistent).
	/// 
	/// ## Arguments
	///
	/// * `vs` - list of vertices.
	/// * `fs` - list of faces.
	///
	/// ## Return
	/// * `Result<(Vec<Vertex>, Vec<Face>), String>` - Result containing vertex list and face list.
	///
fn catmull_clark(fs: &Vec<Face>, vs: &Vec<Vertex>) -> Result<(Vec<Vertex>, Vec<Face>), String> {
	//
	// STEP 1 FINDING ALL NEW POINTS
	//
	let vs_len = vs.len();

	// for each face, a face point is created which is the average of all the points of the face.
	let face_points = get_face_points(vs, fs)?;

	// get list of all edges and their middle points (edge.middle)
	let edges = get_edges_faces(vs, fs)?;

	// per edge get an edge point, = (average of face points + edge center)/2
	let edge_points = get_edge_points(vs, &edges, &face_points)?;

	// per original point: find the average of the face points of the faces the point belongs to
	let avg_face_points = get_average_face_points(vs, fs, &face_points)?;

	// per original point: the average of the middles of its incidental edges  
	let avg_mid_edges = get_average_edges(vs_len, &edges)?;

	// per point find n : how many faces a point belongs to
	let points_faces = get_faces_per_point(vs_len, fs)?;

	//Using previously calculated points :
		//calculate new locations of original points 
		//sharp points move less in z direction
	let mut new_points = get_new_points(vs, &points_faces, &avg_face_points, &avg_mid_edges)?;

	//
	// STEP 2. ADDING NEW POINTS TO USABLE LISTS
	//

	// add face points to new_points
	// and keep track of indexes of newly added points
	let mut face_point_index: Vec<usize> = Vec::new();
	let mut next_index = new_points.len();

	for face_point in face_points {
		new_points.push(face_point);
		face_point_index.push(next_index);
		next_index += 1;
	}

	// add edge points to new_points, using hash so you can find index per edge
	let mut edge_index_map: HashMap<(usize, usize), usize> = HashMap::with_capacity(edges.len());

	for (i, edge) in edges.iter().enumerate() {
        let edge_point = edge_points.get(i).ok_or("catmull clark: edge does not exist at index")?.clone();
		new_points.push(edge_point);
		edge_index_map.insert(smallest_first(edge.p1, edge.p2), next_index);
		next_index += 1;
    }

	//
	//  STEP 3. CREATE NEW FACES USING NEW POINTS
	//

	let mut new_faces: Vec<Face> = Vec::new();

	//split each face in to 4 : connect moved original points, to edge points, to face points.
	for (i, f) in fs.iter().enumerate() {
		// 4 point required in face
		if f.points.len() == 4 {
			let a = f.points[0];
			let b = f.points[1];
			let c = f.points[2];
			let d = f.points[3];

			let face_point_abcd = face_point_index[i];

			let edge_point_ab = *edge_index_map.get(&smallest_first(a, b)).ok_or(format!("catmull : egde ab not found in map: {a} , {b}"))?;
			let edge_point_da = *edge_index_map.get(&smallest_first(d, a)).ok_or(format!("catmull : da de ab not found in map: {d} , {a}"))?;
			let edge_point_bc = *edge_index_map.get(&smallest_first(b, c)).ok_or(format!("catmull : egde ab not found in map: {b} , {c}"))?;
			let edge_point_cd = *edge_index_map.get(&smallest_first(c, d)).ok_or(format!("catmull : egde ab not found in map: {c} , {d}"))?;

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

		} else {
			return Err(String::from("catmull ; face found with less than 4 vertices"));
		}
	}

	Ok((new_points, new_faces))
}

///
/// HELPER METHODS
///

//if both are sharp, center is sharp
//if either, center is half sharp
fn center_point(p1: &Vertex, p2: &Vertex) -> Vertex {
	Vertex {
		x: (p1.x + p2.x) / 2.0,
		y: (p1.y + p2.y) / 2.0,
		z: (p1.z + p2.z) / 2.0,
		is_sharp: p1.is_sharp && p2.is_sharp,
		half_sharp: p1.is_sharp || p2.is_sharp,
	}
}

fn add(p1: &Vertex, p2: &Vertex) -> Vertex {
	Vertex {
		x: (p1.x + p2.x),
		y: (p1.y + p2.y),
		z: (p1.z + p2.z),
		is_sharp: false,
		half_sharp: false,
	}
}

fn average_of_points(xs: Vec<Vertex>) -> Vertex {
	let n = xs.len() as f32;
	let mut agr = Vertex {
		x: 0.0,
		y: 0.0,
		z: 0.0,
		is_sharp: false,
		half_sharp: false,
	};
	for x in xs {
		agr = add(&agr, &x);
	}
	Vertex {
		x: agr.x / n,
		y: agr.y / n,
		z: agr.z / n,
		is_sharp: false,
		half_sharp: false,
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
		face_points.push(Vertex {
			x: x / 4.0,
			y: y / 4.0,
			z: z / 4.0,
			is_sharp: false,
			half_sharp: false,
		});
	}
	if (f.len() != face_points.len()) {
		return Err(String::from("number face points generated does not match number of faces"));
	}

	Ok(face_points)
}

//gets all edges between points represented as : incedent points, adjacent faces, and center of edge
fn get_edges_faces(vs: &Vec<Vertex>, fs: &Vec<Face>) -> Result<Vec<Edge>, String> {
	let mut edges: Vec<Edge> = Vec::new();

	// get edges from each face

	for (face_index, f) in fs.iter().enumerate() {
		
		if  f.points.len() < 4 {
			return Err(String::from("get_edges_faces: face does not have enough points"));
		}

		// hardcoded tuples of points that make face edges
		let es = vec![(f.points[0], f.points[1]), (f.points[1], f.points[2]), (f.points[2], f.points[3]), (f.points[3], f.points[0])];

		for (i1, i2) in es {
			let p1 = vs.get(i1).ok_or(format!("get_edges_faces: vertex at index {i1} does not exist"))?;
			let p2 = vs.get(i2).ok_or(format!("get_edges_faces: vertex at index {i2} does not exist"))?;

			// order points in edge by lowest point number
			// and define center of edges
				edges.push(Edge {
					p1: smallest_first(i1, i2).0,
					p2: smallest_first(i1, i2).1,
					f1: face_index,
					f2: None,
					middle: center_point(p1, p2),
				});
		}
	}

	//sort edges by p1, p2, f1, to be able to detect duplicates in merge step
	edges.sort_by_key(|a| (a.p1, a.p2, a.f1));

	//if two faces share an edge, the edge is currently duplicate in edges, so we can combine them to one edge with 2 faces
	// results in :
	// [pointnum_1, pointnum_2, facenum_1, facenum_2] or
	// [pointnum_1, pointnum_2, facenum_1, None] (if edge is not duplciate)

	let num_edges = edges.len();
	let mut e_index = 0;
	let mut merged_edges: Vec<Edge> = Vec::new();

	while e_index < num_edges {
		let e1 = edges.get(e_index).ok_or(format!("get_edges_faces: edge at index {e_index} does not exist"))?;
		
		// check if not last edge
		if e_index < num_edges - 1 {
			let e2 = edges.get(e_index + 1).ok_or(format!("get_edges_faces: edge at index {a} does not exist", a = e_index + 1))?;
			if e1.p1 == e2.p1 && e1.p2 == e2.p2 {
				merged_edges.push(Edge {
					p1: e1.p1,
					p2: e1.p2,
					f1: e1.f1,
					f2: Some(e2.f1),
					middle: e1.middle.clone(),
				});
				e_index += 2
			} else {
				merged_edges.push(Edge {
					p1: e1.p1,
					p2: e1.p2,
					f1: e1.f1,
					f2: None,
					middle: e1.middle.clone(),
				});
				e_index += 1
			}

		//if last edge no e2 and no neighboring edge
		} else {
			merged_edges.push(Edge {
				p1: e1.p1,
				p2: e1.p2,
				f1: e1.f1,
				f2: None,
				middle: e1.middle.clone(),
			});
			e_index += 1
		}
	}

	Ok(merged_edges)
}

//For each edge, add an edge point.
//Set each edge point to be the average of the two neighbouring face points (AF) and the midpoint of the edge (ME)
// = (AF + ME)/ 2
//Handling sharpness : if ME is sharp, so is corresponding edge point. edge point moves less in z dir if ME is sharp
fn get_edge_points(v: &Vec<Vertex>, edges: &Vec<Edge>, face_points: &Vec<Vertex>) -> Result<Vec<Vertex>, String> {
	let mut edge_points: Vec<Vertex> = Vec::new();

	for edge in edges {
		let f1 = face_points.get(edge.f1).ok_or("get edge points: no face point")?;
		//if edge has no second face, count single face twice
		//TODO : not correct, should count once (reason border is ugly)
		let f2 = match edge.f2 {
			Some(x) => face_points.get(edge.f2.unwrap()).ok_or("get edge points: no face point")?,
			None => f1,
		};

		let AF = add(f1, f2);
		let ME = &edge.middle;
		edge_points.push(Vertex {
			x: (AF.x + ME.x) / 3.0,
			y: (AF.y + ME.y) / 3.0 ,
			//if sharp, z less impacted
			z: if ME.is_sharp { (AF.z + (ME.z * 4.0)) / 6.0  } else { (AF.z + ME.z) / 3.0 },
			//if ME is sharp, so is corresponding edge point.
			is_sharp: ME.is_sharp,
			half_sharp: ME.half_sharp,
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
		for (j, f) in fs.iter().enumerate() {
			//if p is part of face
			if f.points.contains(&i) {
				//add face point to list
				let fp = face_points.get(j).ok_or("get average face points : could not find face point")?.clone();
				adjacents.push(fp);
			}
		}
		//average of adjacent face points
		averages.push(average_of_points(adjacents));
	}

	Ok(averages)
}

//For each original point (P), the average (R) of all n edge midpoints for original edges touching P, where each edge midpoint is the average of its two endpoint vertices
fn get_average_edges(vs_len: usize, es: &Vec<Edge>) -> Result<Vec<Vertex>, String> {
	
	let mut averages: Vec<Vertex> = Vec::new();

	//for each vertex
	for p in 0..vs_len {
		let mut adjacents: Vec<Vertex> = Vec::new();

		//for each edge
		for e in es {
			//if egde is incedent on vertex add its middle to list
			if e.p1 == p || e.p2 == p {
				adjacents.push(e.middle.clone());
			}
		}
		//ret average 
		averages.push(average_of_points(adjacents));
	}

	Ok(averages)
}

fn get_faces_per_point(vs_len: usize, fs: &Vec<Face>) -> Result<Vec<usize>, String> {
	let mut faces_per_point = vec![0; vs_len];

	for f in fs {
		for v in &f.points {
			if (v > &vs_len ){
				return Err(String::from("get_faces_per_point: invalid vertex number"));
			}
			faces_per_point[*v] += 1;
		}
	}
	Ok(faces_per_point)
}

//Move each original point to the new vertex point (f + 2r + (n-3)*v)/n
//if a vertex is sharp, its edge points and original point weigh twice as much as usual -> moves less towards faces and stays towards height original edge
//					//v				//n								//f							//r
fn get_new_points(vs: &Vec<Vertex>, f_per_v: &Vec<usize>, avg_face_points: &Vec<Vertex>, avg_mid_edges: &Vec<Vertex>) -> Result<(Vec<Vertex>), String> {
	let mut new_vertices: Vec<Vertex> = Vec::new();

	for i in 0..vs.len() {
		//get all vertices to derive new point from
		let v = vs.get(i).ok_or("get average face points : could not find face")?;
		let n = *f_per_v.get(i).ok_or("get average face points : could not find face")? as f32;
		let f = avg_face_points.get(i).ok_or("get average face points : could not find face")?;
		let r = avg_mid_edges.get(i).ok_or("get average face points : could not find face")?;

		//calculate new coordinates : new point =(f + 2r + (n-3)*v)/n
		let x = ((v.x * (n - 3.0)) + (2.0 * r.x) + f.x) / n;

		let y = ((v.y * (n - 3.0)) + (2.0 * r.y) + f.y) / n;

		//if v is sharp :  new point =(f + 4r + (n-1)*v)/ (n + 4)
		let z = if v.is_sharp { ((v.z * (n - 1.0)) + (4.0 * r.z) + f.z) / (n + 4.0)} else { ((v.z * (n - 3.0)) + (2.0 * r.z) + f.z) / n };


		new_vertices.push(Vertex {
			x,
			y,
			z,
			is_sharp: false,
			half_sharp: false,
		})
	}

	Ok(new_vertices)
}
