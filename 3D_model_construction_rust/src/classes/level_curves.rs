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

	pub fn add_point(&mut self, a: Point) {
		self.points.push(a);
	}

	pub fn get_points(&self) -> &Vec<Point> {
		&self.points
	}

	pub fn find_closest_point_and_distance_on_level_curve(&self, a: &Point) -> (Option<&Point>, f64) {
		if self.points.is_empty() {
			return (None, f64::INFINITY);
		}

		// Get the distance to the first point in the list, as a starting point.
		let mut min_dist_sqr: f64 = Point::dist_sqr(&self.points[0], a);
		let mut min_dist_sqr_point: &Point = &self.points[0];

		// Loop over every point in the list and find the smallest distance.
		// You don't have to keep track of which point had this smallest distance.
		for p in &self.points {
			let current_dist_sqr = Point::dist_sqr(p, a);

			if current_dist_sqr < min_dist_sqr {
				min_dist_sqr = current_dist_sqr;
				min_dist_sqr_point = p;
			}
		}

		// Return the smallest distance found
		(Some(min_dist_sqr_point), f64::sqrt(min_dist_sqr))
	}

	pub fn find_closest_point_on_level_curve(&self, a: &Point) -> Option<&Point> {
		return self.find_closest_point_and_distance_on_level_curve(a).0;
	}

	pub fn dist_to_point(&self, a: &Point) -> f64 {
		return self.find_closest_point_and_distance_on_level_curve(a).1;
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

	/// transforms pixelStructure [TODO DEFINE PROPERLY] to levelCurveMap structure, while reducing amount of total points from pixelStructure
    ///
    /// # Arguments
    ///
    /// * `pixels` - structure containing info from scanning step
    /// * `altitude_step` - increase in height per contour line
    /// * `desired_dist` - minimum desired distance between points in final conout map
    /// 
    /// 
	pub fn transform_to_LevelCurveMap(pixels : &Vec< (usize, Vec<Vec<(f64, f64)>> ) > ,  altitude_step: f64, desired_dist: f64 ) -> LevelCurveMap{
		let mut ret: LevelCurveMap = LevelCurveMap::new(altitude_step);

		for level in pixels {
			let level_height = level.0 as f64 * altitude_step;

			//for every curve in pixel structire, make a new Level curve 
			for curve in &level.1 {

				let mut level_curve: LevelCurve = LevelCurve::new(level_height);

				level_curve.add_point(   Point { x: curve[0].0, y: curve[1].1, z: level_height }) ;

				let mut last_point: &(f64, f64)  = &curve[0];
				
				//reduce amount of points in curve such that distance between points is at least desired_dist
				for current_point in curve {
					if(dist(last_point, current_point) >=  desired_dist){
						level_curve.add_point( Point {x: current_point.0 , y: current_point.1 , z: level_height}  );
						last_point = &current_point;
					}
				}
				ret.add_level_curve(level_curve);	
			}

		}
		ret
	}
}



fn dist(a : &(f64, f64) , b: &(f64, f64)) -> f64 {
	( (a.0 -b.0).powi(2) + (a.1 - b.1).powi(2) ).sqrt()
}