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

	pub fn add_all_points(&mut self, xs: Vec<Point>) {
		for mut p in xs {
			p.z = self.altitude;
			self.points.push(p);
		}
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

	pub fn find_closest_point_on_level_curve(&self, a: &Point) -> Option<&Point> {
		return self.find_closest_point_and_distance_on_level_curve(a).0;
	}

	pub fn find_furthest_point_and_distance_on_level_curve(&self, a: &Point) -> (Option<&Point>, f32) {
		if self.points.is_empty() {
			return (None, f32::INFINITY);
		}

		// Get the distance to the first point in the list, as a starting point.
		let mut max_dist_sqr: f32 = Point::xy_dist_sqr(&self.points[0], a);
		let mut max_dist_sqr_point: &Point = &self.points[0];

		// Loop over every point in the list and find the smallest distance.
		// You don't have to keep track of which point had this smallest distance.
		for p in &self.points {
			// let current_dist_sqr = Point::dist_sqr(p, a);
			let current_dist_sqr = Point::xy_dist_sqr(p, a);

			if current_dist_sqr > max_dist_sqr {
				max_dist_sqr = current_dist_sqr;
				max_dist_sqr_point = p;
			}
		}

		// Return the smallest distance found
		(Some(max_dist_sqr_point), f32::sqrt(max_dist_sqr))
	}

	pub fn dist_to_point(&self, a: &Point) -> f32 {
		return self.find_closest_point_and_distance_on_level_curve(a).1;
	}

	pub fn furthest_dist_to_point(&self, a: &Point) -> f32 {
		return self.find_furthest_point_and_distance_on_level_curve(a).1;
	}

	pub fn increase_point_resolution(&mut self) {
		for i in (0..self.points.len() - 1).rev() {
			let p1 = &self.points[i];
			let p2 = &self.points[i + 1];

			let p3 = Point {
				x: (p1.x + p2.x) / 2.0,
				y: (p1.y + p2.y) / 2.0,
				z: p1.z,
			};

			self.points.insert(i + 1, p3);
		}

		let p1 = &self.points[0];
		let p2 = &self.points[self.points.len() - 1];

		let p3 = Point {
			x: (p1.x + p2.x) / 2.0,
			y: (p1.y + p2.y) / 2.0,
			z: p1.z,
		};

		self.points.insert(self.points.len(), p3);
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

	// Adds new level curve of a single point that represents a peak.
	//	TODO: how to determine multiple peaks?? -> use parentage tree thing
	// Location of peak is determined using centroid formula for polygons from wikipedia
	// Height of peak = 1/2 alitude_step above top level curve.
	pub fn add_peak(&mut self) {
		//	let old_curve = self.level_curves.last().ok_or("level_curves.add_peak() : Original set has no curves to add to")?;

		//find top level curve
		let mut max_curve_height = 0.0;
		let mut top_curve: &LevelCurve = &LevelCurve::new(0.0);
		for curve in &self.level_curves {
			if curve.altitude > max_curve_height {
				max_curve_height = curve.altitude;
				top_curve = curve;
			}
		}

		let ps = &top_curve.points;

		//calculate area
		let mut a = 0.0;

		// ! THIS WOULD CAUSE A PANIC
		// ! In the last iteration, ps[p+1] will always be out of bounds
		// ! This is why I prefer using Vec::get over slicing :)
		// ! Pauline wants to keep this function for future reference, but as of now using it would be unsafe
		for p in 0..ps.len() {
			a += (ps[p].x * ps[p + 1].y) - (ps[p + 1].x * ps[p].y);
		}
		a /= 2.0;

		let mut x = 0.0;
		let mut y = 0.0;

		// ! Same here, index out of bounds
		for p in 0..ps.len() {
			let fact = ps[p].x * ps[p + 1].y - ps[p + 1].x * ps[p].y;
			x += (ps[p].x + ps[p + 1].x) * fact;
			y += (ps[p].y + ps[p + 1].y) * fact;
		}

		y /= 6.0 * a;
		x /= 6.0 * a;

		let peak_height = top_curve.altitude + 0.5 * self.altitude_step;

		self.add_level_curve(LevelCurve {
			altitude: peak_height,
			points: vec![Point { x, y, z: peak_height }],
			is_mountain_top: true,
		});
	}

	// Find points (minimum_x_cooridinate, minimum_y_coordinate) , (maximum_x_cooridinate, maximum_y_coordinate) of coordinates in levelcurveset ,
	// for the puropose of genererating a raster to cover whole area of levelcurves
	pub fn get_bounding_points(&self) -> (Point, Point) {
		let mut min = Point {
			x: std::f32::MAX,
			y: std::f32::MAX,
			z: 0.0,
		};
		let mut max = Point {
			x: std::f32::MIN,
			y: std::f32::MIN,
			z: 0.0,
		};
		for curve in &self.level_curves {
			for point in &curve.points {
				min.x = f32::min(min.x, point.x);
				min.y = f32::min(min.y, point.y);
				max.x = f32::max(max.x, point.x);
				max.y = f32::max(max.y, point.y);
			}
		}
		(min, max)
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
	///
	/// Shifts all points in level curve set such that the set is aligned with the x and y axis.
	/// Not exactly aligned, distance to keep from axes can be specified.
	///  
	/// # Arguments
	///
	/// * `min` - point with the minimal occurring x and y values
	/// * `border_x` - distance to keep model from x axis
	/// * `border_y` - distance to keep model from y axis
	pub fn align_with_origin(&mut self, min: &Point, border_x: f32, border_y: f32) {
		for curve in &mut self.level_curves {
			for p in &mut curve.points {
				p.x = p.x - min.x + border_x;
				p.y = p.y - min.y + border_y;
			}
		}
	}
}
