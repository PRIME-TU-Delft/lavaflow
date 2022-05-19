use super::catmull_clark::{Edge, Vertex};
/// Finds set of lava paths for a given model, from a specified starting vertex on the model.
///
/// # Arguments
///
/// * `start_index` - Index of first vertex in lava path.
/// * `length` - Desired amount of points in lava path.
/// * `fork_val` - Value that determines amount of forking. If two potential next points have a smaller difference than fork_val, an additional path will be added to the set.
/// * `vs` - List of all points of the model.
/// * `es` - List of neighbors indexes per point in  the model.
///
/// # Return
/// *  `Result<Vec<Vec< &'a Vertex>>, String>` - Result of list of lava paths. A lava path is a list of
///
pub fn get_lava_paths_super<'a>(highest_points: &Vec<usize>, length: usize, fork_val: f32, min_altitude: f32, vs: &'a Vec<Vertex>, es: &'a Vec<Vec<usize>>) -> Result<Vec<Vec<&'a Vertex>>, String> {
	let mut paths = LavaPathSet { all_paths: Vec::new() };
	let start_point = get_start(highest_points, vs, es)?;
	paths.get_lava_path(start_point, length, fork_val, min_altitude, vs, es)?;
	Ok(paths.all_paths)
}

struct LavaPathSet<'a> {
	all_paths: Vec<Vec<&'a Vertex>>,
}

impl<'a> LavaPathSet<'a> {
	/// Gets a singled lava path of a specified length starting at a specfied point in the model. A lava path follows the edges witht the steepest gradient in the z direction.
	///
	/// # Arguments
	///
	/// * `start` - The index of the first point in the lava path.
	/// * `length` - Amount of points to be added to the lava path.
	/// * `fork_val` - Value that determines amount of forking. If two potential next points have a smaller difference than fork_val, an additional path will be added to the set.
	/// * `vs` - List of all points of the model.
	/// * `es` - List of neighbors indexes per point in  the model.
	///
	/// # Return
	/// *  `Result<Vec< &'a Vertex>, String>` - Result of list of vertexes in lava path
	///
	fn get_lava_path(&mut self, start_index: usize, length: usize, fork_val: f32, min_altitude: f32, vs: &'a Vec<Vertex>, es: &'a Vec<Vec<usize>>) -> Result<(), String> {
		let mut path = Vec::with_capacity(length);

		path.push(vs.get(start_index).ok_or(String::from("start point for lava does not exist in vertex list"))?);

		//index vertex pair of current point in parth
		let mut cur = (start_index, vs.get(start_index).ok_or(String::from("start point for lava does not exist in vertex list"))?);

		while (path.len() < length) {
			//get neighbors
			//neighbor is (index, Vertex)
			let mut neighbors: Vec<(usize, &Vertex)> = Vec::new();

			for i in &es[cur.0] {
				neighbors.push((*i, vs.get(*i).ok_or(format!("lava_path: index {i} not found in vertex list"))?));
			}

			//per neighbor calculate gradient and find maximum
			let mut max = cur;
			let mut max_g = f32::MIN;

			//keep track of next best for forking
			let mut second_best = cur;
			let mut second_best_g = f32::MIN;

			for n in neighbors {
				let new_g = gradient_between_points(cur.1, n.1);
				if max_g < new_g {
					max = n;
					max_g = new_g;
				} else if second_best_g < new_g {
					second_best = n;
					second_best_g = new_g;
				}
			}

			//break loop if next point has low altitude (avoid path going to edge of map)
			if (max.1.z <= min_altitude) {
				break;
			}
			//add steepest neighbor to path
			path.push(max.1);

			//if diffence is smaller than given value, start another path with half length at second best neighbor
			if (max_g - second_best_g) < fork_val {
				self.get_lava_path(second_best.0, length / 2, fork_val, min_altitude, vs, es)?;
			}

			//mark steepest neighbor as next point
			cur = max;
		}
		self.all_paths.push(path);
		Ok(())
	}
}
/// Calculates the gradient between two points.
/// Gradient is currently defined as: z difference divided by absolute distance.
/// Positive gradient goes down mountain.
///
/// # Arguments
///
/// * `from` - 1st vertex
/// * `to` -2nd vertex
///
/// # Return
/// *  `f32` - gradient
///
fn gradient_between_points(from: &Vertex, to: &Vertex) -> f32 {
	//(sqr(from.x-to.x) + sqr(from.y-to.y) + sqr(from.z - to.z)).sqrt() * ((from.z - to.z) / (from.z - to.z))

	//gradient = diff in z
	//(from.z - to.z)
	//gradient = z diff divided by length of edge
	(from.z - to.z) / (sqr(from.x - to.x) + sqr(from.y - to.y) + sqr(from.z - to.z)).sqrt()
}

fn get_start(highest_points : &Vec<usize>, vs : &Vec<Vertex>, es : &Vec<Vec<usize>>) -> Result<usize, String> {

	let mut store = (highest_points[0], f32::MIN);


	for i in highest_points {

		let mut neighbors: Vec<(usize, &Vertex)> = Vec::new();

		//per neighbor calculate gradient and find maximum

		for j in &es[*i] {
			neighbors.push((*j, vs.get(*j).ok_or(format!("lava_path: index {i} not found in vertex list"))?));
		}

		let cur: (&usize, &Vertex) = (i, &vs[*i]);

		for n in neighbors {
			let new_g = gradient_between_points(cur.1, n.1);
			if store.1 > new_g {
				store = (*cur.0, new_g);
			}
		}
		
	} 

	Ok(store.0)
}

fn sqr(a: f32) -> f32 {
	a * a
}
