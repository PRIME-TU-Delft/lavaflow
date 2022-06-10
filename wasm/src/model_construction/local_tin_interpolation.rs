use crate::objects::{level_curves::LevelCurve, level_curves::LevelCurveSet, point::Point, triangle::Triangle};
use miette::{miette, Result};

// pub struct LevelCurveSet {
// 	altitude_step: f32,
// 	level_curves: Vec<LevelCurve>,
// }

// TODO: I'm not a big fan of the error handing in this function. It's a very long function, with loads of things that can throw an error, with varying consequences. For a future refactor, it would be good to split this into several different functions that each have their own error handling.

/// Extension of the LevelCurveSet class
impl LevelCurveSet {
	/// Method: local_tin_interpolate
	///
	/// # Arguments
	/// * 'self': The LevelCurveSet instance to apply this method to
	/// * 'p':    The Point to interpolate
	pub fn local_tin_interpolate(&self, p: &Point) -> Result<f32> {
		// If this set contains less than two level-curves, interpolation is not possible. Return 'p.z'
		if self.level_curves.len() < 2 {
			return Err(miette!("Set contains less than two level curves"));
		}

		//
		// 1. Find the two closest levelcurves to this point
		//
		let mut closest_two_levelcurves: (&LevelCurve, &LevelCurve) = (&self.level_curves[0], &self.level_curves[1]);
		let mut closest_two_distances: (f32, f32) = (self.level_curves[0].dist_to_point(p), self.level_curves[1].dist_to_point(p));

		// Go over every level curve in this set
		for i in 2..self.level_curves.len() {
			// Compute the distance from this level curve to this point
			let current_dist = self.level_curves[i].dist_to_point(p);

			// If this distance is smaller than both current level-curves, shift everything over
			if current_dist < closest_two_distances.0 && current_dist < closest_two_distances.1 {
				closest_two_levelcurves.1 = closest_two_levelcurves.0;
				closest_two_levelcurves.0 = &self.level_curves[i];

				closest_two_distances.1 = closest_two_distances.0;
				closest_two_distances.0 = current_dist;
			}
			// Else, if the distance is greater than the first level curve but smaller than the second, insert the new one
			else if current_dist >= closest_two_distances.0 && current_dist < closest_two_distances.1 {
				closest_two_levelcurves.1 = &self.level_curves[i];

				closest_two_distances.1 = current_dist;
			}

			// Else, the distance is larger than both current level curves, skip this one
		}

		//
		// 2. Compute the triangles between these two level-curves
		//

		let points_closest_0 = closest_two_levelcurves.0.get_points();
		let points_closest_1 = closest_two_levelcurves.1.get_points();

		// Additional checking constraint: if one of these level-curves has no points, return 'p.z'
		if points_closest_0.is_empty() || points_closest_1.is_empty() {
			return Err(miette!("One of the closest two level curves has no points"));
		}

		// Create a vector of triangles (which are triples)
		let mut triangles: Vec<Triangle> = Vec::new();

		// Select a random point from the first closest curve and find the closest point on the other curve
		let mut point_a = 0;
		let mut point_b = closest_two_levelcurves
			.1
			.find_closest_point_with_index_on_level_curve(closest_two_levelcurves.0.get_point(0).ok_or_else(|| miette!("Error when extracting point from level-curve."))?)
			.0
			.ok_or_else(|| miette!("Error when extracting point-index from level-curve."))?;

		// Store both points for later reference
		let initial_points: (usize, usize) = (point_a, point_b);

		// We will now start computing the triangles
		loop {
			// We will consider the following two points for a new triangle.
			// Level curves are always closed (that's a requirement for this application), therefore
			// the first point in the array comes right after the last point in the array.
			let mut next_point_a = point_a + 1;
			let mut next_point_b = point_b + 1;

			// Following up on this: if the next_point_a or next_point_b first fall outside of the array,
			// set them equal to zero (essentially closing the circle)
			if next_point_a >= closest_two_levelcurves.0.get_points().len() {
				next_point_a = 0;
			}
			if next_point_b >= closest_two_levelcurves.1.get_points().len() {
				next_point_b = 0;
			}

			// Compute minimal angle of triangle 1
			let triangle_1 = Triangle::new(
				closest_two_levelcurves.0.get_point(point_a).ok_or_else(|| miette!("Error when extracting point from level-curve."))?,
				closest_two_levelcurves.1.get_point(point_b).ok_or_else(|| miette!("Error when extracting point from level-curve."))?,
				closest_two_levelcurves
					.0
					.get_point(next_point_a)
					.ok_or_else(|| miette!("Error when extracting point from level-curve."))?,
			);
			let min_angle_t1 = triangle_1.minimal_internal_angle();

			// Compute minimal angle of triangle 2
			let triangle_2 = Triangle::new(
				closest_two_levelcurves.0.get_point(point_a).ok_or_else(|| miette!("Error when extracting point from level-curve."))?,
				closest_two_levelcurves.1.get_point(point_b).ok_or_else(|| miette!("Error when extracting point from level-curve."))?,
				closest_two_levelcurves
					.1
					.get_point(next_point_b)
					.ok_or_else(|| miette!("Error when extracting point from level-curve."))?,
			);
			let min_angle_t2 = triangle_2.minimal_internal_angle();

			// If the first triangle has a larger minimal interior angle, select that one
			if min_angle_t1 > min_angle_t2 {
				// Select the first triangle
				triangles.push(triangle_1);

				// The point on the first level curve has now shifted one place further (A)
				point_a = next_point_a;
			}
			// Else, select the other one
			else {
				// Select the second triangle
				triangles.push(triangle_2);

				// The point on the second level curve has now shifted one place further (B)
				point_b = next_point_b;
			}

			// The loop is complete if either point a or point b is equal to the initial point
			if point_a == initial_points.0 || point_b == initial_points.1 {
				break;
			}
		}

		//
		// 3. Determine the triangle that 'p' falls in
		//

		// Check: If the list of triangles is empty, return 'p.z'
		if triangles.is_empty() {
			return Err(miette!("List of interpolation triangles is empty"));
		}

		let mut triangle_around_p: &Triangle = &triangles[0];

		for tri in &triangles {
			if tri.contains_point(p) {
				triangle_around_p = tri;
				break;
			}
		}

		//
		// 4. Compute the interpolated value
		//

		// Compute the distance from p to triangle.a
		let dist_p_a = Point::xy_dist(p, triangle_around_p.a);

		// Compute the distance from p to triangle.b
		let dist_p_b = Point::xy_dist(p, triangle_around_p.b);

		// Compute the distance from p to triangle.c
		let dist_p_c = Point::xy_dist(p, triangle_around_p.c);

		// Compute the sum of these distances
		let dist_sum = 1.0 / dist_p_a + 1.0 / dist_p_b + 1.0 / dist_p_c;

		// Compute the inverse weighted average over these distances and the z-values of the triangle-corners.
		// Return this value
		Ok(((triangle_around_p.a.z / dist_p_a) + (triangle_around_p.b.z / dist_p_b) + (triangle_around_p.c.z / dist_p_c)) / dist_sum)
	}
}
