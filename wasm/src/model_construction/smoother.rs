use crate::objects::{point::Point, raster::Raster, triangle::Triangle};

use super::constructor::ModelConstructor;

use miette::{miette, Result};

/// # Struct: Smoother
/// The Smoother is a struct that allows for applying mountain-smoothing in a much more controlled manner.
/// For example, it enables smoothing per altitude-group of level-curves. Developers can detach specific parts of
/// the mountain, such as the mountain tops, and smooth them, separately using specific paremeters.
pub struct Smoother<'a> {
	pub raster: &'a mut Raster,
	is_svc: &'a Vec<Vec<bool>>,
	point_indices_per_layer: Vec<Vec<(usize, usize)>>,
	layer_is_top: Vec<bool>,
	altitude_of_layer: Vec<f32>,
	altitude_groups: Vec<Vec<usize>>,
	altitude_step: f32,
}

impl<'a> Smoother<'a> {
	/// ## Constructor
	/// Create a new `Smoother` instance.
	///
	/// **Usage**: Initialize a `ModelConstructor` instance and apply its mountain-construction algorithm to the level-curves.
	/// After this is completed, create a `Smoother` instance and use it to run specialised smoothing.
	///
	/// ### Parameters
	/// - `model_constructor`: Instance of a ModelConstructor, as a mutable reference with lifetime specifier.
	pub fn new(model_constructor: &'a mut ModelConstructor) -> Result<Self> {
		// Construct the list of point-indices per layer.
		// Construct the list of booleans, specifying whether a layer is a mountain-top.
		// Construct the list of altitudes per layer.
		let mut point_indices_per_layer: Vec<Vec<(usize, usize)>> = Vec::new();
		let mut layer_is_top: Vec<bool> = Vec::new();
		let mut altitude_of_layer: Vec<f32> = Vec::new();

		// Initialize the lists, by pushing empty lists for every level curve
		for _lc in 0..model_constructor.level_curve_map.level_curves.len() + 1 {
			point_indices_per_layer.push(Vec::new());
			layer_is_top.push(false);
			altitude_of_layer.push(0.0);
		}

		// Create a vector of all pixels that have not yet been assigned a place
		let mut unassigned_points: Vec<(usize, usize)> = Vec::new();
		for row in 0..model_constructor.raster.rows {
			for col in 0..model_constructor.raster.columns {
				unassigned_points.push((row, col));
			}
		}

		// 1. Triangulate all the level-curves.
		#[allow(clippy::type_complexity)]
		let mut triangles_per_level_curve: Vec<(usize, Vec<(&Point, &Point, &Point)>, bool, f32)> = Vec::new();

		// 1a. Find out which level-curve has the highest altitude
		let mut current_altitude: f32 = -10.0;
		let mut current_lc: usize = 0;

		for (i, lc) in model_constructor.level_curve_map.level_curves.iter().enumerate() {
			if lc.altitude > current_altitude {
				current_altitude = lc.altitude;
				current_lc = i;
			}
		}

		// Initialize a list of level-curves that have already been drawn and triangulated (initially empty)
		let mut drawn_lcs: Vec<usize> = Vec::new();

		// Start the triangulation loop
		// Triangulation will occur for higher layers first. The reason is explained below:
		//
		//      Imagine two level-curves, one at altitude 10 (lc1) and the other at altitude 20 (lc2). All points that triangulate
		//      into level-curve lc2 will also triangulate in lc1, because lc1 completely encapsulates lc1. The latter is a valid
		//      statement, since level-curves cannot specify a cave in a mountain.
		//      Therefore: first triangulate points on higher altitudes, then continue down the mountain.
		loop {
			// Triangulate the current level-curve and push it to the array
			triangles_per_level_curve.push((
				current_lc,
				Smoother::triangulate_level_curve(&model_constructor.level_curve_map.level_curves[current_lc])?,
				model_constructor.level_curve_map.level_curves[current_lc].is_mountain_top,
				model_constructor.level_curve_map.level_curves[current_lc].altitude,
			));

			// Mark this level-curve as drawn
			drawn_lcs.push(current_lc);

			// Set the current-altitude back to -10.
			current_altitude = -10.0;

			// While keeping track of whether we've found a new level-curve, find the level-curve that is not yet drawn
			// and that has the highest altitude.
			let mut found_new = false;

			for (i, lc) in model_constructor.level_curve_map.level_curves.iter().enumerate() {
				if lc.altitude > current_altitude && !drawn_lcs.contains(&i) {
					current_altitude = lc.altitude;
					current_lc = i;
					found_new = true;
				}
			}

			// If no new level curve was found, break out of the loop.
			if !found_new {
				break;
			}
		}

		// After the loop: all level curves have been triangulated.

		// 2. For every point in the raster, assign it to the right layer, according to this triangulation.
		// Loop over all the level curves
		for (_i, triangle_set) in triangles_per_level_curve.iter().enumerate() {
			// Loop over all unassigned points
			for j in (0..unassigned_points.len()).rev() {
				let row = unassigned_points[j].0;
				let col = unassigned_points[j].1;

				let raster_pixel = model_constructor.raster.get_pixel(row, col);

				let p = Point {
					x: raster_pixel.0,
					y: raster_pixel.1,
					z: 0.0,
				};

				// If this point is in this level-curve, assign this point to the level-curve
				if Smoother::point_in_triangle_set(&triangle_set.1, &p) {
					point_indices_per_layer[triangle_set.0 + 1].push((row, col));
					layer_is_top[triangle_set.0 + 1] = triangle_set.2;
					altitude_of_layer[triangle_set.0 + 1] = triangle_set.3;
					unassigned_points.remove(j);
				}
			}
		}

		// 4. Points that have not yet been assigned fall outside all level-curves, meaning they are in the lowest
		//    level of the mountain (the bottom of the mountain). This should be layer 0.
		for (row, col) in unassigned_points {
			point_indices_per_layer[0].push((row, col));
		}

		// 5. Now that we've defined all the points that lie in a certain layer, we will group layers, based on their altitude.
		let mut altitude_groups: Vec<Vec<usize>> = Vec::new();

		// Compute the max altitude of this mountain
		let mut max_altitude = 0.0;

		for &a in &altitude_of_layer {
			if a > max_altitude {
				max_altitude = a;
			}
		}

		// Initialize the array of altitude-groups, by computing how many layers we will have
		for _i in 0..((f32::round(max_altitude / model_constructor.level_curve_map.altitude_step) as usize) + 1) {
			altitude_groups.push(Vec::new());
		}

		// For every level-curve, assign it to the right group, according to its altitude.
		// Important: tops of this mountain will NEVER be assigned to a group.
		for (i, a) in altitude_of_layer.iter().enumerate() {
			// If this level is a top, skip
			if layer_is_top[i] {
				continue;
			}

			altitude_groups[f32::round(a / model_constructor.level_curve_map.altitude_step) as usize].push(i);
		}

		// Complete the construction and return the new instance
		Ok(Self {
			raster: model_constructor.raster,
			is_svc: &model_constructor.is_svc,
			point_indices_per_layer,
			layer_is_top,
			altitude_of_layer,
			altitude_groups,
			altitude_step: model_constructor.level_curve_map.altitude_step,
		})
	}

	/// ## Instance Method
	/// Compute the number of layers in this mountain.
	pub fn layers(&self) -> usize {
		self.point_indices_per_layer.len()
	}

	/// ## Instance Method
	/// Compute the number of altitude groups in this mountain.
	pub fn number_of_altitude_groups(&self) -> usize {
		self.altitude_groups.len()
	}

	/// ## Instance Method
	/// Get the list of points that belong to a certain level.
	///
	/// ### Parameters
	/// - `level`: usize, specifying which level to extract.
	pub fn get_point_indices_in_level(&self, level: usize) -> Result<&Vec<(usize, usize)>> {
		if level < self.point_indices_per_layer.len() {
			return Ok(&self.point_indices_per_layer[level]);
		}

		Err(miette!("Requested points of non-existing layer."))
	}

	/// ## Instance Method
	/// Triangulate all points in a single level-curve.
	///
	/// ### Parameters:
	/// - `level_curve`: The level curve to apply triangulation on.
	///
	/// ### Returns:
	/// - A triple of `Point`s, representing a triangle
	pub fn triangulate_level_curve(level_curve: &crate::objects::level_curves::LevelCurve) -> Result<Vec<(&Point, &Point, &Point)>> {
		// Prebase: If this level-curve contains less than three points, throw an exception
		if level_curve.points.len() < 3 {
			return Err(miette!("Level Curve contains less than three points."));
		}

		let mut result: Vec<(&Point, &Point, &Point)> = Vec::new();

		// Prebase: If this level-curve contains only three points, it is already triangulated.
		if level_curve.points.len() == 3 {
			result.push((&level_curve.points[0], &level_curve.points[1], &level_curve.points[2]));

			return Ok(result);
		}

		// Construct a vertex of indices that are considered 'in the level curve' for this algorithm
		let mut indices_in_polygon: Vec<usize> = Vec::new();

		for i in 0..level_curve.points.len() {
			indices_in_polygon.push(i);
		}

		// 1. Loop over every point in the level curve
		for i in (0..indices_in_polygon.len()).rev() {
			// 1a.  Extract three points:
			//      pu: The point at index i-1
			//      pv: The point at index i
			//      pw: The point at index i+1

			let mut pu_index: usize = i - 1;

			//  pv_index = i
			let pv: &Point = &level_curve.points[indices_in_polygon[i]];

			let mut pw_index: usize = i + 1;

			// if pv is the first point in the list, pu must be the last point in the list
			if i == 0 {
				pu_index = indices_in_polygon.len() - 1;
			}
			let pu: &Point = &level_curve.points[indices_in_polygon[pu_index]];

			// if pv is the last point in the list, pw must be the first point in the list
			if i == indices_in_polygon.len() - 1 {
				pw_index = 0;
			}
			let pw: &Point = &level_curve.points[indices_in_polygon[pw_index]];

			// 1b.  Determine the angle at pv
			let pv_angle = Triangle::angle_at_pv(pu, pv, pw);

			// 1c.  If the angle at pv is greater than PI, we can guarantee this triangle is not an ear
			if pv_angle > std::f32::consts::PI {
				continue;
			}

			// 1c---We can now guarantee that this triangle is a CANDIDATE for an ear
			// 1d.  Make sure no other point in the level curve is inside this triangle
			let tri: Triangle = Triangle::new(pv, pu, pw);
			let mut triangle_is_candidate = true;

			for (j, _) in indices_in_polygon.iter().enumerate() {
				// Only determine points that are not part of this triangle
				if j == pu_index || j == i || j == pw_index {
					continue;
				}

				// If this point is inside the triangle, this triangle is no longer a candidate
				if tri.contains_point(&level_curve.points[indices_in_polygon[j]]) {
					triangle_is_candidate = false;
					break;
				}
			}

			// 1d---If this triangle is no longer a candidate, skip this point
			if !triangle_is_candidate {
				continue;
			}

			// 1e   This triangle is an ear!! Add the triangle to the result vertex and remove pv from the level curve
			result.push((pu, pv, pw));
			indices_in_polygon.remove(i);

			// If there are only three points left in the list, break out of this loop
			if indices_in_polygon.len() == 3 {
				break;
			}
		}

		// After completing this triangulation, there are only three points left in the polygon.
		// Add these three points as the last index in the result
		if level_curve.points.len() == 3 {
			result.push((
				&level_curve.points[indices_in_polygon[0]],
				&level_curve.points[indices_in_polygon[1]],
				&level_curve.points[indices_in_polygon[2]],
			));
		}

		// Return the resulting vector
		Ok(result)
	}

	/// ## Private Static Method
	/// Determine whether a certain point lies within the area of any triangle in a set.
	///
	/// ### Parameters
	/// - `triangle_set`: The set of triangles.
	/// - `p`: The point to check the premise against.
	fn point_in_triangle_set(triangle_set: &[(&Point, &Point, &Point)], p: &Point) -> bool {
		// Loop over all the triangles
		for (i, _) in triangle_set.iter().enumerate() {
			let a = triangle_set[i].0;
			let b = triangle_set[i].1;
			let c = triangle_set[i].2;

			// Create a triangle from these three points
			let tri = Triangle::new(a, b, c);

			// If this triangle contains the point, set 'result' to true and break out of the loop
			if tri.contains_point(p) {
				return true;
			}
		}

		// Return false otherwise
		false
	}

	//
	// SMOOTHING ALGORITHMS: HELPER FUNCTIONS
	//

	/// ## Private Method
	/// Get the altitude of a specific neighbour.
	///
	/// ### Parameters
	/// - `row`: The row of the item of which to check the neighbours.
	/// - `col`: The column of the item of which to check the neighbours.
	/// - `alt_row`: The difference in rows between this item and the specific neighbour.
	/// - `alt_col`: The difference in columns between this item and the specific neighbour.
	///
	/// ### Returns
	/// This method returns a Result.
	///
	/// Ok - If all goes well, this method returns the altitude of the specified neighbour.
	///    - If a neighbour doesn't exist, return 0.0 as altitude. (The bottom layer of the mountain is always zero and the table is altitude zero, as specified in the requirements).
	/// Err- If something goes wrong, this method will return an error.
	fn get_neighbour_altitude(&self, row: usize, col: usize, alt_row: isize, alt_col: isize) -> Result<f32> {
		let row_n = row as isize + alt_row;
		let col_n = col as isize + alt_col;

		// If the considered neighbour falls outside of the raster, return altitude 0.0
		if row_n < 0 || col_n < 0 || row_n as usize >= self.raster.rows || col_n as usize >= self.raster.columns {
			Ok(0.0)
		}
		// Else, find the neighbour and return its altitude value
		else {
			Ok(self.raster.altitudes[row_n as usize][col_n as usize].ok_or_else(|| miette!("Altitude not present."))?)
		}
	}

	/// ## Instance Method
	/// Determine whether the specified neighbour is an svc or not.
	///
	/// ### Parameters
	/// - `row`: The row of the item of which to check the neighbours.
	/// - `col`: The column of the item of which to check the neighbours.
	/// - `alt_row`: The difference in rows between this item and the specific neighbour.
	/// - `alt_col`: The difference in columns between this item and the specific neighbour.
	///
	/// ### Returns
	/// This method returns a Result. If anything goes wrong, an error will be thrown.
	///
	/// This method returns trus iff the specified neighbour is an svc. False otherwise.
	fn neighbour_is_svc(&self, row: usize, col: usize, alt_row: isize, alt_col: isize) -> bool {
		let row_n = row as isize + alt_row;
		let col_n = col as isize + alt_col;

		// If the considered neighbour falls outside of the raster, return altitude 0.0
		if row_n < 0 || col_n < 0 || row_n as usize >= self.raster.rows || col_n as usize >= self.raster.columns {
			false
		}
		// Else, find the neighbour and return its altitude value
		else {
			self.is_svc[row_n as usize][col_n as usize]
		}
	}

	/// ## Instance Method
	/// Determine whether the specified neighbour belongs to the specified layer.
	///
	/// ### Parameters
	/// - `layer`: The layer to check this premise for.
	/// - `row`: The row of the item of which to check the neighbours.
	/// - `col`: The column of the item of which to check the neighbours.
	/// - `alt_row`: The difference in rows between this item and the specific neighbour.
	/// - `alt_col`: The difference in columns between this item and the specific neighbour.
	///
	/// ### Returns
	/// This method returns a boolean. True iff this neighbour belongs to this layer, false otherwise.
	fn neighbour_belongs_to_layer(&self, layer: usize, row: usize, col: usize, alt_row: isize, alt_col: isize) -> bool {
		// If the specified layer doesn't exist, return error
		if layer >= self.point_indices_per_layer.len() {
			return false;
		}

		let row_n = row as isize + alt_row;
		let col_n = col as isize + alt_col;

		// If the considered neighbour falls outside of the raster, return altitude 0.0
		if row_n < 0 || col_n < 0 || row_n as usize >= self.raster.rows || col_n as usize >= self.raster.columns {
			false
		}
		// Else, determine whether this neighbour is part of the specified layer.
		else {
			self.point_indices_per_layer[layer].contains(&(row_n as usize, col_n as usize))
		}
	}

	//
	// SMOOTHING ALGORITHMS
	//

	/// ## Instance Method (Smoothing Method)
	/// Apply neighbour smoothing to the points that reside in the specified layer
	///
	/// ### Parameters
	/// - `layer`: The layer to check this premise for.
	/// - `strength: f32`: The strength of the smoothing, range [0, 1]. Outside this range, undesired behaviour might occur.
	/// - `coverage: usize`: The size of the rectangle, inside which all neighbours will be incorporated in the compution. The resulting rectangle has size (2xcoverage + 1) by (2xcoverage + 1).
	/// - `svc_weight: usize`: The number of times an svc-value counts in the averaging of neighbours.
	/// - `allow_svc_change`: Boolean determining whether svc values may be assigned a new altitude as result of the smoothing.
	///
	/// ### Returns
	/// This method returns a `Result` and will throw an error if anything goes wrong. Other than that, this method returns `void`.
	pub fn apply_smooth_to_layer(&mut self, layer: usize, strength: f32, coverage: usize, svc_weight: usize, allow_svc_change: bool) -> Result<()> {
		// If the specified layer doesn't exist, return error
		if layer >= self.point_indices_per_layer.len() {
			return Err(miette!("Specified layer does not exist."));
		}

		// Iterate over all (row, col) pairs
		// TODO: Complexity can be improved
		for row in 0..self.raster.rows {
			for col in 0..self.raster.columns {
				// Skip this pair if it isn't part of this layer
				if !self.point_indices_per_layer[layer].contains(&(row, col)) {
					continue;
				}

				// This point is part of the layer

				if self.is_svc[row][col] && !allow_svc_change {
					continue;
				}

				// 1. Discover the neighbours
				let mut neighbour_altitudes: Vec<f32> = Vec::new();

				for alt_row in -(coverage as isize)..(coverage as isize) {
					for alt_col in -(coverage as isize)..(coverage as isize) {
						// Skip if this neighbour is of another layer
						if !self.neighbour_belongs_to_layer(layer, row, col, alt_row, alt_col) {
							continue;
						}

						// Compute the altitude of this particular neighbour and add it to the vector
						neighbour_altitudes.push(self.get_neighbour_altitude(row, col, alt_row, alt_col)?);

						if self.neighbour_is_svc(row, col, alt_row, alt_col) {
							for _i in 0..svc_weight - 1 {
								neighbour_altitudes.push(self.get_neighbour_altitude(row, col, alt_row, alt_col)?);
							}
						}
					}
				}

				// 2. Compute the average of the altitudes of the neighbours.
				let mut average_neighbour_altitude: f32 = 0.0;
				for neighbour in &neighbour_altitudes {
					average_neighbour_altitude += neighbour;
				}
				average_neighbour_altitude /= neighbour_altitudes.len() as f32;

				// 3. Compute the difference between this average altitude and the current altitude for this point
				let mut deviation = average_neighbour_altitude - self.raster.altitudes[row][col].ok_or_else(|| miette!("Altitude not present."))?;

				// 4. Apply the provided strength factor to this deviation and add the deviation once to the new altitude
				deviation *= strength;

				self.raster.altitudes[row][col] = Some(self.raster.altitudes[row][col].ok_or_else(|| miette!("Altitude not present."))? + deviation);
			}
		}

		Ok(())
	}

	/// ## Instance Method (Smoothing Method)
	/// Set the altitude of a specific layer
	///
	/// ### Parameters
	/// - `layer`: The layer to check this premise for.
	/// - `altitude: f32`: The new altitude to set the points in this layer to.
	/// - `allow_svc_change`: Boolean determining whether svc values may be assigned a new altitude as result of the smoothing.
	///
	/// ### Returns
	/// This method returns a `Result` and will throw an error if anything goes wrong. Other than that, this method returns `void`.
	pub fn set_altitude_for_layer(&mut self, layer: usize, altitude: f32, allow_svc_change: bool) -> Result<()> {
		// If the specified layer doesn't exist, return error
		if layer >= self.point_indices_per_layer.len() {
			return Err(miette!("Specified layer does not exist."));
		}

		// Iterate over all (row, col) pairs
		for row in 0..self.raster.rows {
			for col in 0..self.raster.columns {
				// Skip this pair if it isn't part of this layer
				if !self.point_indices_per_layer[layer].contains(&(row, col)) {
					continue;
				}

				// Skip this pair if it is an svc
				if self.is_svc[row][col] && !allow_svc_change {
					continue;
				}

				self.raster.altitudes[row][col] = Some(altitude);
			}
		}

		Ok(())
	}

	/// ## Instance Method (Smoothing Method)
	/// Apply neighbour smoothing to the points that reside in the specified altitude group.
	///
	/// ### Parameters
	/// - `altitude_group: usize`: The altitude group to apply the smoothing to.
	/// - `strength: f32`: The strength of the smoothing, range [0, 1]. Outside this range, undesired behaviour might occur.
	/// - `coverage: usize`: The size of the rectangle, inside which all neighbours will be incorporated in the compution. The resulting rectangle has size (2xcoverage + 1) by (2xcoverage + 1).
	/// - `svc_weight: usize`: The number of times an svc-value counts in the averaging of neighbours.
	/// - `allow_svc_change`: Boolean determining whether svc values may be assigned a new altitude as result of the smoothing.
	///
	/// ### Returns
	/// This method returns a `Result` and will throw an error if anything goes wrong. Other than that, this method returns `void`.
	pub fn apply_smooth_to_altitude_group(&mut self, altitude_group: usize, strength: f32, coverage: usize, svc_weight: usize, allow_svc_change: bool) -> Result<()> {
		// Apply the specified smooth to every layer that falls in this group
		for i in 0..self.altitude_groups[altitude_group].len() {
			self.apply_smooth_to_layer(self.altitude_groups[altitude_group][i], strength, coverage, svc_weight, allow_svc_change)?;
		}

		Ok(())
	}

	/// ## Instance Method (Smoothing Method)
	/// Increase the altitude of all points that reside in a certain altitude-group.
	///
	/// ### Parameters
	/// - `altitude_group: usize`: The altitude group to apply the smoothing to.
	/// - `percentage_of_altitude_step`: The percentage of the altitude-step that the points should be set higher with.
	/// - `allow_svc_change`: Boolean determining whether svc values may be assigned a new altitude as result of the smoothing.
	///
	/// ### Returns
	/// This method returns a `Result` and will throw an error if anything goes wrong. Other than that, this method returns `void`.
	pub fn increase_altitude_for_altitude_group(&mut self, altitude_group: usize, percentage_of_altitude_step: f32, allow_svc_change: bool) -> Result<()> {
		// Apply the specified altitude increase to every layer that falls in this group
		for i in 0..self.altitude_groups[altitude_group].len() {
			self.set_altitude_for_layer(
				self.altitude_groups[altitude_group][i],
				self.altitude_of_layer[self.altitude_groups[altitude_group][i]] + self.altitude_step * percentage_of_altitude_step,
				allow_svc_change,
			)?;
		}

		Ok(())
	}

	/// ## Instance Method (Smoothing Method)
	/// Apply neighbour smoothing to the points that reside in any middle altitude-layer (being not the bottom and not the mountain-tops).
	///
	/// ### Parameters
	/// - `strength: f32`: The strength of the smoothing, range [0, 1]. Outside this range, undesired behaviour might occur.
	/// - `coverage: usize`: The size of the rectangle, inside which all neighbours will be incorporated in the compution. The resulting rectangle has size (2xcoverage + 1) by (2xcoverage + 1).
	/// - `svc_weight: usize`: The number of times an svc-value counts in the averaging of neighbours.
	/// - `allow_svc_change`: Boolean determining whether svc values may be assigned a new altitude as result of the smoothing.
	///
	/// ### Returns
	/// This method returns a `Result` and will throw an error if anything goes wrong. Other than that, this method returns `void`.
	pub fn apply_smooth_to_middle_layers(&mut self, strength: f32, coverage: usize, svc_weight: usize, allow_svc_change: bool) -> Result<()> {
		for i in 0..self.layer_is_top.len() {
			if !self.layer_is_top[i] && i != 0 {
				// This layer is not a top and is also not the bottom layer
				self.apply_smooth_to_layer(i, strength, coverage, svc_weight, allow_svc_change)?;
			}
		}

		Ok(())
	}

	/// ## Instance Method (Smoothing Method)
	/// Apply neighbour smoothing to the points that reside in any of the mountain tops.
	///
	/// ### Parameters
	/// - `strength: f32`: The strength of the smoothing, range [0, 1]. Outside this range, undesired behaviour might occur.
	/// - `coverage: usize`: The size of the rectangle, inside which all neighbours will be incorporated in the compution. The resulting rectangle has size (2xcoverage + 1) by (2xcoverage + 1).
	/// - `svc_weight: usize`: The number of times an svc-value counts in the averaging of neighbours.
	/// - `allow_svc_change`: Boolean determining whether svc values may be assigned a new altitude as result of the smoothing.
	///
	/// ### Returns
	/// This method returns a `Result` and will throw an error if anything goes wrong. Other than that, this method returns `void`.
	pub fn apply_smooth_to_mountain_tops(&mut self, strength: f32, coverage: usize, svc_weight: usize, allow_svc_change: bool) -> Result<()> {
		for i in 0..self.layer_is_top.len() {
			if self.layer_is_top[i] {
				self.apply_smooth_to_layer(i, strength, coverage, svc_weight, allow_svc_change)?;
			}
		}

		Ok(())
	}

	/// ## Instance Method (Smoothing Method)
	/// Increase the altitude of the points that reside in any of the mountain-tops.
	///
	/// ### Parameters
	/// - `percentage_of_altitude_step`: The percentage of the altitude-step that the points should be set higher with.
	/// - `allow_svc_change`: Boolean determining whether svc values may be assigned a new altitude as result of the smoothing.
	///
	/// ### Returns
	/// This method returns a `Result` and will throw an error if anything goes wrong. Other than that, this method returns `void`.
	pub fn increase_altitude_for_mountain_tops(&mut self, percentage_of_altitude_step: f32, allow_svc_change: bool) -> Result<()> {
		for i in 0..self.layer_is_top.len() {
			if self.layer_is_top[i] {
				self.set_altitude_for_layer(i, self.altitude_of_layer[i] + self.altitude_step * percentage_of_altitude_step, allow_svc_change)?;
			}
		}

		Ok(())
	}

	/// ## Static Method
	/// Apply neighbour smoothing to all the points in the raster.
	///
	/// ### Parameters
	/// - `strength: f32`: The strength of the smoothing, range [0, 1]. Outside this range, undesired behaviour might occur.
	/// - `coverage: usize`: The size of the rectangle, inside which all neighbours will be incorporated in the compution. The resulting rectangle has size (2xcoverage + 1) by (2xcoverage + 1).
	/// - `svc_weight: usize`: The number of times an svc-value counts in the averaging of neighbours.
	/// - `allow_svc_change`: Boolean determining whether svc values may be assigned a new altitude as result of the smoothing.
	///
	/// ### Returns
	/// This method returns a `Result` and will throw an error if anything goes wrong. Other than that, this method returns `void`.
	pub fn apply_smooth_to_all(&mut self, strength: f32, coverage: usize, svc_weight: usize, allow_svc_change: bool) -> Result<()> {
		let mut new_altitudes: Vec<Vec<Option<f32>>> = Vec::new();

		// Iterate over all the rows and columns in this raster
		for row in 0..self.raster.rows {
			// Push a new empty vector to 'new_altitudes'
			new_altitudes.push(Vec::new());

			for col in 0..self.raster.columns {
				// Push None for this entry in the new_altitudes, to make space
				new_altitudes[row].push(None);

				// For each of these cells in the raster, do the following:

				// 1. If this cell is an svc-cell, skip this cell.
				if self.is_svc[row][col] {
					new_altitudes[row][col] = Some(self.raster.altitudes[row][col].ok_or_else(|| miette!("Altitude not present."))?);

					if !allow_svc_change {
						continue;
					}
				}

				// 2. This cell is not svc. Select the specified amount of neighbors
				let mut neighbour_altitudes: Vec<f32> = Vec::new();

				// Loop over all the neighbours that fall within the rectangle of coverage x coverage
				for alt_row in -(coverage as isize)..(coverage as isize) {
					for alt_col in -(coverage as isize)..(coverage as isize) {
						// If both alt_row and alt_col are zero, don't include the altitude. (This is the current point we're considering)
						if alt_row == 0 && alt_col == 0 {
							continue;
						}

						// Compute the altitude of this particular neighbour and add it to the vector
						neighbour_altitudes.push(self.get_neighbour_altitude(row, col, alt_row, alt_col)?);

						if self.neighbour_is_svc(row, col, alt_row, alt_col) {
							for _i in 0..svc_weight - 1 {
								neighbour_altitudes.push(self.get_neighbour_altitude(row, col, alt_row, alt_col)?);
							}
						}
					}
				}

				// 3. Compute the average of the altitudes of the neighbours.
				let mut average_neighbour_altitude: f32 = 0.0;
				for neighbour in &neighbour_altitudes {
					average_neighbour_altitude += neighbour;
				}
				average_neighbour_altitude /= neighbour_altitudes.len() as f32;

				// 4. Compute the difference between this average altitude and the current altitude for this point
				let mut deviation = average_neighbour_altitude - self.raster.altitudes[row][col].ok_or_else(|| miette!("Altitude not present."))?;

				// 5. Apply the provided strength factor to this deviation and add the deviation once to the new altitude
				deviation *= strength;

				new_altitudes[row][col] = Some(self.raster.altitudes[row][col].ok_or_else(|| miette!("Altitude not present."))? + deviation);
			}
		}

		// We have now finished computing the new altitudes, set the altitudes correctly in the model_constructor's raster
		self.raster.altitudes = new_altitudes;

		Ok(())
	}

	/// ## Instance Method
	/// Correct for the altitude constraints that are posed by the level-curves.
	/// It can never be the case that a point that resides in the layer between altitude 10.0 and 20.0 has an altitude of 5.0.
	/// Then there would have needed to be another level-curve that specified that this point would lay below altitude 10.0.
	/// This method corrects for this problem, by checking whether a specific point has this problem and then assigning the
	/// minimum or maximum altitude to it, according to the layer it resides in.
	///
	/// ### Returns
	/// This method returns a `Result` and will throw an error if anything goes wrong. Other than that, this method returns `void`.
	pub fn correct_for_altitude_constraints_to_all_layers(&mut self) -> Result<()> {
		for (i, &alt) in self.altitude_of_layer.iter().enumerate() {
			// i is the current layer we're considering
			// alt is the lower altitude of the current layer
			// alt_max is the upper altitude of the current layer
			let alt_max = alt + self.altitude_step;

			// Loop over all point-indices in this layer
			for current_index in &self.point_indices_per_layer[i] {
				let row = current_index.0;
				let col = current_index.1;

				// If the altitude of this index falls outside the lower bound
				if self.raster.altitudes[row][col].ok_or_else(|| miette!("Altitude not present."))? < alt {
					// Correct this altitude, by setting it to the minimum
					self.raster.altitudes[row][col] = Some(alt);
				}

				// If the altitude of this index falls outside the upper bound
				if self.raster.altitudes[row][col].ok_or_else(|| miette!("Altitude not present."))? > alt_max {
					// Correct this altitude, by setting it to the maximum
					self.raster.altitudes[row][col] = Some(alt_max);
				}
			}
		}

		Ok(())
	}
}
