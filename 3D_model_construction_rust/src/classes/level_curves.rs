#![allow(dead_code)]

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
}



//
// Level Curve Tree
//
// The level curve tree is used to represent the information that was received from the OpenCV library.
// This step is necessary, since it allows us to efficiently store all information that OpenCV outputs.
// This includes information that could be required by other algorithms.
//

pub struct LevelCurveTree<'a> {
	pixels_per_curve: &'a Vec<Vec<(u64, u64)>>,
	parent_relations: &'a Vec<Option<usize>>,
	own_index: usize
}

impl<'a> LevelCurveTree<'a> {


	//
	// CONSTRUCTORS
	//

	/// Dynamic constructor: From OpenCV datastructure
	pub fn from_open_cv(pixels_per_curve: &'a Vec<Vec<(u64, u64)>>, parent_relations: &'a Vec<Option<usize>>) -> Self {
		let mut own_index = 0;

		// 1. Find the node that has no parent (this is the root of the tree)
		for i in 0..parent_relations.len() {
			if parent_relations[i] == None {
				// We've found the root!
				own_index = i;
			}
		}

		Self {
			pixels_per_curve,
			parent_relations,
			own_index: own_index
		}
	}

	/// Private Dynamic constructor: From existing tree, with different index
	fn from_perspective_index(&'a self, index: usize) -> Self {
		Self {
			pixels_per_curve: self.pixels_per_curve,
			parent_relations: self.parent_relations,
			own_index: index
		}
	}

	//
	// PRIVATE METHODS
	//




	//
	// PUBLIC METHODS
	//

	/// Method: Get the parent of this node
	pub fn get_parent(&'a mut self) -> Option<LevelCurveTree> {
		if self.parent_relations[self.own_index] == None {
			return None
		}

		let result: LevelCurveTree = self.from_perspective_index(self.parent_relations[self.own_index]?);

		Some(result)
	}

	/// Method: isParentOf
	/// Similar to: isChildOf
	pub fn is_parent_of(&self, child: &LevelCurveTree) -> bool {
		let parent = self.parent_relations[child.own_index];
		match parent {
			None => false,
			Some(p) => p == self.own_index
		}
	}

	/// Method: isChildOf
	/// Similar to: isParentOf
	pub fn is_child_of(&self, parent: &LevelCurveTree) -> bool {
		let child = self.parent_relations[self.own_index];
		match child {
			None => false,
			Some(c) => c == parent.own_index
		}
	}

	/// Method: getChildren
	pub fn get_children(&'a mut self) -> Vec<LevelCurveTree> {

		let mut result: Vec<LevelCurveTree> = Vec::new();

		// Add all trees of whom this instance is the parent
		for i in 0..self.parent_relations.len() {
			if self.parent_relations[i] == Some(self.own_index) {
				// The node at index i is a child of this instance
				// Add it to the list
				result.push(self.from_perspective_index(i));
			}
		}

		result

	}

	/// Method: Check whether a certain point is in the set
	///
	pub fn contains_pixel(&self, x: u64, y: u64) -> bool {
		for p in &self.pixels_per_curve[self.own_index] {
			if p.0 == x && p.1 == y {
				return true;
			}
		}
		return false;
	}

}