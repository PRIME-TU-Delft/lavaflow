//
// Class: LevelCurves
//
use crate::objects::point::Point;

#[derive(Debug)]
pub struct LevelCurve {
	pub altitude: f32,
	pub points: Vec<Point>,
	pub is_mountain_top: bool,
}

impl LevelCurve {
	pub fn new(altitude: f32) -> Self {
		Self {
			altitude,
			points: Vec::new(),
			is_mountain_top: false,
		}
	}

	pub fn add_point(&mut self, a: Point) {
		self.points.push(a);
	}

	pub fn get_point(&self, index: usize) -> Option<&Point> {
		if index < self.points.len() {
			return Some(&self.points[index]);
		}
		None
	}

	pub fn get_points(&self) -> &Vec<Point> {
		&self.points
	}

	pub fn find_closest_point_with_index_and_distance_on_level_curve(&self, a: &Point) -> (Option<usize>, Option<&Point>, f32) {
		if self.points.is_empty() {
			return (None, None, f32::INFINITY);
		}

		// Get the distance to the first point in the list, as a starting point.
		// let mut min_dist_sqr: f32 = Point::dist_sqr(&self.points[0], a);
		let mut min_dist_sqr: f32 = Point::xy_dist_sqr(&self.points[0], a);
		let mut min_dist_sqr_point: &Point = &self.points[0];
		let mut min_dist_sqr_index: usize = 0;

		// Loop over every point in the list and find the smallest distance.
		// You don't have to keep track of which point had this smallest distance.
		for (index, point) in self.points.iter().enumerate() {
			// let current_dist_sqr = Point::dist_sqr(p, a);
			let current_dist_sqr = Point::xy_dist_sqr(point, a);

			if current_dist_sqr < min_dist_sqr {
				min_dist_sqr = current_dist_sqr;
				min_dist_sqr_point = point;
				min_dist_sqr_index = index;
			}
		}

		// Return the smallest distance found
		(Some(min_dist_sqr_index), Some(min_dist_sqr_point), f32::sqrt(min_dist_sqr))
	}

	pub fn find_closest_point_and_distance_on_level_curve(&self, a: &Point) -> (Option<&Point>, f32) {
		let result = self.find_closest_point_with_index_and_distance_on_level_curve(a);
		(result.1, result.2)
	}

	pub fn find_closest_point_with_index_on_level_curve(&self, a: &Point) -> (Option<usize>, Option<&Point>) {
		let result = self.find_closest_point_with_index_and_distance_on_level_curve(a);
		(result.0, result.1)
	}

	pub fn dist_to_point(&self, a: &Point) -> f32 {
		return self.find_closest_point_and_distance_on_level_curve(a).1;
	}
}

impl Clone for LevelCurve {
	fn clone(&self) -> LevelCurve {
		let mut points_clone: Vec<Point> = Vec::new();

		for p in &self.points {
			points_clone.push(Point { x: p.x, y: p.y, z: p.z });
		}

		LevelCurve {
			altitude: self.altitude,
			points: points_clone,
			is_mountain_top: self.is_mountain_top,
		}
	}
}

//
// Class: LevelCurveMap
// This class gathers multiple level curves and provides functionality for working with
// the system as a whole
//

#[derive(Debug)]
pub struct LevelCurveSet {
	pub altitude_step: f32,
	pub level_curves: Vec<LevelCurve>,
}

impl LevelCurveSet {
	// Construct a new LevelCurveMap, by specifying the altitude per level
	pub fn new(altitude_step: f32) -> Self {
		Self {
			altitude_step,
			level_curves: Vec::new(),
		}
	}

	// Add a new level curve to the map
	pub fn add_level_curve(&mut self, a: LevelCurve) {
		self.level_curves.push(a);
	}

	// Retrieve the list of level curves
	pub fn get_level_curves(&self) -> &Vec<LevelCurve> {
		&self.level_curves
	}

	// Finding the closest point on any level curve that's stored in this map
	pub fn find_closest_point_on_level_curve(&self, a: &Point) -> Option<&Point> {
		// If this map doesn't contain any level-curves, return None
		if self.level_curves.is_empty() {
			return None;
		}

		// Find the baseline tuple, storing a (Point, Distance)
		let mut min_dist = self.level_curves[0].find_closest_point_and_distance_on_level_curve(a);

		// Loop over every level-curve, find the point that lies closest to the specified point a
		for lc in &self.level_curves {
			let current_dist = lc.find_closest_point_and_distance_on_level_curve(a);

			if current_dist.1 < min_dist.1 {
				min_dist = current_dist;
			}
		}

		// Return the point
		min_dist.0
	}
}
