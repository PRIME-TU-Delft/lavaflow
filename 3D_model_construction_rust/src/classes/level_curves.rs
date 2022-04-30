//
// Class: LevelCurves
//

use super::point::Point;

#[derive(Debug)]
pub struct LevelCurve {
	altitude: f64,
	points: Vec<Point>
}


impl LevelCurve {
	pub fn new(altitude: f64) -> Self {
		Self { altitude, points: Vec::new() }
	}

	pub fn addPoint(&mut self, a: Point) {
		self.points.push(a);
	}

	pub fn getPoints(&self) -> &Vec<Point> {
		return &self.points;
	}

	pub fn findClosestPointAndDistanceOnLevelCurve(&self, a: &Point) -> (Option<&Point>, f64) {
		if (self.points.len() == 0) {
			return (None, f64::INFINITY);
		}

		// Get the distance to the first point in the list, as a starting point.
		let mut min_dist_sqr: f64 = Point::distSqr(&self.points[0], a);
		let mut min_dist_sqr_point: &Point = &self.points[0];

		// Loop over every point in the list and find the smallest distance.
		// You don't have to keep track of which point had this smallest distance.
		for p in &self.points {
			let current_dist_sqr = Point::distSqr(p, a);

			if (current_dist_sqr < min_dist_sqr) {
				min_dist_sqr = current_dist_sqr;
				min_dist_sqr_point = p;
			}
		}

		// Return the smallest distance found
		return (Some(min_dist_sqr_point), f64::sqrt(min_dist_sqr));
	}

	pub fn findClosestPointOnLevelCurve(&self, a: &Point) -> Option<&Point> {
		return self.findClosestPointAndDistanceOnLevelCurve(a).0;
	}

	pub fn distToPoint(&self, a: &Point) -> f64 {
		return self.findClosestPointAndDistanceOnLevelCurve(a).1;
	}
}


//
// Class: LevelCurveMap
// This class gathers multiple level curves and provides functionality for working with
// the system as a whole
//

#[derive(Debug)]
pub struct LevelCurveMap {
	altitude_step: f64,
	level_curves: Vec<LevelCurve>
}

impl LevelCurveMap {
	// Construct a new LevelCurveMap, by specifying the altitude per level
	pub fn new(altitude_step: f64) -> Self {
		Self {
			altitude_step,
			level_curves: Vec::new()
		}
	}

	// Add a new level curve to the map
	pub fn addLevelCurve(&mut self, a: LevelCurve) {
		self.level_curves.push(a);
	}

	// Retrieve the list of level curves
	pub fn getLevelCurves(&self) -> &Vec<LevelCurve> {
		return &self.level_curves;
	}

	// Finding the closest point on any level curve that's stored in this map
	pub fn findClosestPointOnLevelCurve(&self, a: &Point) -> Option<&Point> {
		// If this map doesn't contain any level-curves, return None
		if self.level_curves.len() == 0 {
			return None;
		}

		// Find the baseline tuple, storing a (Point, Distance)
		let mut min_dist = self.level_curves[0].findClosestPointAndDistanceOnLevelCurve(a);

		// Loop over every level-curve, find the point that lies closest to the specified point a
		for lc in &self.level_curves {
			let current_dist = lc.findClosestPointAndDistanceOnLevelCurve(a);

			if current_dist.1 < min_dist.1 {
				min_dist = current_dist;
			}
		}

		// Return the point
		return min_dist.0;
	}
}
