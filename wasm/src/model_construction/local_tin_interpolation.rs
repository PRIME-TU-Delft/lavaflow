use super::level_curves::{LevelCurve, LevelCurveSet};
use super::point::Point;

// pub struct LevelCurveSet {
// 	altitude_step: f32,
// 	level_curves: Vec<LevelCurve>,
// }

// The Triangle struct is used for computations that are related to the triangulation step
struct Triangle<'a> {
	pub a: &'a Point,
	pub b: &'a Point,
	pub c: &'a Point,
}

impl<'a> Triangle<'a> {
	/// Constructor
	pub fn new(a: &'a Point, b: &'a Point, c: &'a Point) -> Self {
		Self { a, b, c }
	}

	/// Compute the interior angle at point a
	fn angle_at_a(&self) -> f32 {
		// let a_side = self.b.xy_dist(self.c);
		// let b_side = self.a.xy_dist(self.c);
		// let c_side = self.a.xy_dist(self.b);

		let a_side = Point::xy_dist(self.b, self.c);
		let b_side = Point::xy_dist(self.a, self.c);
		let c_side = Point::xy_dist(self.a, self.b);

		f32::acos((f32::powi(b_side, 2) + f32::powi(c_side, 2) - f32::powi(a_side, 2)) / 2.0 * b_side * c_side) as f32
	}

	/// Compute the minimal internal angle of this triangle
	pub fn minimal_internal_angle(&self) -> f32 {
		// Compute angle at a
		let a_angle = self.angle_at_a();

		// Compute angle at b
		let b_angle = Triangle::new(self.b, self.a, self.c).angle_at_a();

		// Compute angle at c
		let c_angle = Triangle::new(self.c, self.a, self.b).angle_at_a();

		// Return the minimal angle
		f32::min(f32::min(a_angle, b_angle), c_angle)
	}

	/// Method: Does this tringle contain this point?
	pub fn contains_point(&self, p: &Point) -> bool {
		// To make this function more readable, we temporarily store self.a, self.b and self.c into a, b and c
		let a = self.a;
		let b = self.b;
		let c = self.c;

		// Determine whether p lies in this triangle
		let w1_upper = p.x * (c.y - a.y) - p.y * (c.x - a.x) - a.x * (c.y - a.y) + a.y * (c.x - a.x);
		let w1_lower = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);

		let w1 = w1_upper / w1_lower;

		let w2_upper = p.y - a.y - w1 * (b.y - a.y);
		let w2_lower = c.y - a.y;

		let w2 = w2_upper / w2_lower;

		// If w1 and w2 are both >= 0 and w1+w2 <= 1, then this point lies within the triangle. Otherwise not.
		w1 >= 0.0 && w2 >= 0.0 && w1 + w2 <= 1.0
	}
}

/// Extension of the LevelCurveSet class
impl LevelCurveSet {
	/// Method: local_tin_interpolate
	///
	/// # Arguments
	/// * 'self': The LevelCurveSet instance to apply this method to
	/// * 'p':    The Point to interpolate
	pub fn local_tin_interpolate(&self, p: &Point) -> f32 {
		// If this set contains less than two level-curves, interpolation is not possible. Return 'p.z'
		if self.level_curves.is_empty() {
			return p.z;
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
			return p.z;
		}

		// Create a vector of triangles (which are triples)
		let mut triangles: Vec<Triangle> = Vec::new();

		// Select a random point from the first closest curve and find the closest point on the other curve
		let mut point_a = 0;
		let mut point_b = closest_two_levelcurves
			.1
			.find_closest_point_with_index_on_level_curve(closest_two_levelcurves.0.get_point(0)
			.ok_or("Error when extracting point from level-curve.")?)
			.0
			.ok_or("Error when extracting point-index from level-curve.")?;

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
				closest_two_levelcurves.0.get_point(point_a).ok_or("Error when extracting point from level-curve.")?,
				closest_two_levelcurves.1.get_point(point_b).ok_or("Error when extracting point from level-curve.")?,
				closest_two_levelcurves.0.get_point(next_point_a).ok_or("Error when extracting point from level-curve.")?,
			);
			let min_angle_t1 = triangle_1.minimal_internal_angle();

			// Compute minimal angle of triangle 2
			let triangle_2 = Triangle::new(
				closest_two_levelcurves.0.get_point(point_a).ok_or("Error when extracting point from level-curve.")?,
				closest_two_levelcurves.1.get_point(point_b).ok_or("Error when extracting point from level-curve.")?,
				closest_two_levelcurves.1.get_point(next_point_b).ok_or("Error when extracting point from level-curve.")?,
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
			return p.z;
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
		((triangle_around_p.a.z / dist_p_a) + (triangle_around_p.b.z / dist_p_b) + (triangle_around_p.c.z / dist_p_c)) / dist_sum
	}
}
