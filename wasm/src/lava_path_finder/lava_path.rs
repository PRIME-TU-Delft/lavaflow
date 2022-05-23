use crate::objects::point::Point;

/// Finds set of lava paths for a given model, from a specified starting point on the model.
///
/// # Arguments
///
/// * `start_index` - Index of first point in lava path.
/// * `length` - Desired amount of points in lava path.
/// * `fork_val` - Value that determines amount of forking. If two potential next points have a smaller difference than fork_val, an additional path will be added to the set.
/// * `vs` - List of all points of the model.
/// * `es` - List of neighbors indexes per point in  the model.
///
/// # Return
/// *  `Result<Vec<Vec< &'a Point>>, String>` - Result of list of lava paths. A lava path is a list of
///
pub fn get_lava_paths_super<'a>(highest_points: &[usize], length: usize, fork_val: f32, min_altitude: f32, vs: &'a [Point], es: &'a Vec<Vec<usize>>) -> Result<Vec<Vec<&'a Point>>, String> {
	let mut paths = LavaPathSet { all_paths: Vec::new() };
	let start_point = get_start(highest_points, vs, es)?;
	paths.get_lava_path(start_point, length, fork_val, min_altitude, vs, es)?;
	Ok(paths.all_paths)
}

struct LavaPathSet<'a> {
	all_paths: Vec<Vec<&'a Point>>,
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
	/// *  `Result<Vec< &'a Point>, String>` - Result of list of points in lava path
	///
	fn get_lava_path(&mut self, start_index: usize, length: usize, fork_val: f32, min_altitude: f32, vs: &'a [Point], es: &'a Vec<Vec<usize>>) -> Result<(), String> {
		let mut path = Vec::with_capacity(length);

		let start_point = vs.get(start_index).ok_or_else(|| String::from("start point for lava does not exist in point list"))?;
		path.push(start_point);

		//index point pair of current point in parth
		let mut cur = (start_index, start_point);

		while path.len() < length {
			//get neighbors
			//neighbor is (index, Point)
			let mut neighbors: Vec<(usize, &Point)> = Vec::new();

			for i in &es[cur.0] {
				neighbors.push((*i, vs.get(*i).ok_or(format!("lava_path: index {i} not found in point list"))?));
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
			if max.1.z <= min_altitude {
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
/// * `from` - 1st point
/// * `to` -2nd point
///
/// # Return
/// *  `f32` - gradient
///
fn gradient_between_points(from: &Point, to: &Point) -> f32 {
	//(sqr(from.x-to.x) + sqr(from.y-to.y) + sqr(from.z - to.z)).sqrt() * ((from.z - to.z) / (from.z - to.z))

	//gradient = diff in z
	//(from.z - to.z)
	//gradient = z diff divided by length of edge
	(from.z - to.z) / ((from.x - to.x).powf(2.0) + (from.y - to.y).powf(2.0) + (from.z - to.z).powf(2.0)).sqrt()
}
/// Given list of indexes (Desired is list of index of points that can be the top of the model), find the correct start point. The correct start point is the upper point of the steepest edge in the set of possible start points.
///
/// # Arguments
///
/// * `highest_points` - Indeces of points that could possibly first start point.
/// * `vs` - List of all points of the model.
/// * `es` - List of neighbors indexes per point in  the model.
///
/// # Return
/// *  `Result<usize>, String>` - Result of index of start point of lava path.
///
fn get_start(highest_points: &[usize], vs: &[Point], es: &[Vec<usize>]) -> Result<usize, String> {
	if highest_points.is_empty() {
		return Err(String::from("lava path: cannot find highest point because no points exist above top contour line."));
	}
	let mut store = (0, f32::MIN);

	for i in highest_points {
		let mut neighbors: Vec<(usize, &Point)> = Vec::new();

		//per neighbor calculate gradient and find maximum

		for j in &es[*i] {
			neighbors.push((*j, vs.get(*j).ok_or(format!("lava_path: index {i} not found in point list"))?));
		}

		let cur: (&usize, &Point) = (i, &vs[*i]);

		for n in neighbors {
			let new_g = gradient_between_points(cur.1, n.1);
			if store.1 < new_g {
				store = (*cur.0, new_g);
			}
		}
	}

	Ok(store.0)
}
