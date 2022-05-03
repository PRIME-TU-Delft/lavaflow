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
	pixels: Vec<(u64, u64)>,
	parent: Option<&'a LevelCurveTree<'a>>,
	children: Vec<&'a LevelCurveTree<'a>>
}


impl<'a> LevelCurveTree<'a> {

	/// Constructor
	/// 
	/// This constructor creates a completely empty level-curve-tree
	pub fn new() -> Self {
		Self {
			pixels: Vec::new(),
			parent: None,
			children: Vec::new()
		}
	}

	/// Dynamic constructor: Set parent
	/// 
	pub fn withParent(mut self, p: &'a LevelCurveTree<'a>) -> Self {
		self.parent = Some(p);
		self
	}

	/// Dynamic constructor: From OpenCV datastructure
	pub fn fromOpenCV(mut self, pixels_per_curve: Vec<Vec<(u64, u64)>>, parent_per_node: &Vec<usize>) -> Self {

		// 1. Transform the list of parents into a list of LevelCurveTrees
		let mut trees: Vec<LevelCurveTree> = Vec::new();

		for p in parent_per_node {
			trees.push(LevelCurveTree::new());
		}

		// 2. Connect the right children to the right parent
		for (i, mut t) in trees.iter().enumerate() {
			// t.setParent(trees[parent_per_node[i]]);
			// trees[parent_per_node[i]].addChildNode(t);
		}

		self
	}

	/// Method: Set the parent of this node
	pub fn setParent(&mut self, p: &'a LevelCurveTree<'a>) {
		self.parent = Some(p);
	}

	// Method: Remove the parent of this node
	pub fn removeParent(&mut self) {
		self.parent = None;
	}

	/// Method: Add point to this node in the tree
	pub fn addPixel(&mut self, x: u64, y: u64) {
		self.pixels.push((x, y));
	}

	/// Method: Check whether a certain point is in the set
	/// 
	pub fn containsPixel(&self, x: u64, y: u64) -> bool {
		for p in &self.pixels {
			if p.0 == x && p.1 == y {
				return true;
			}
		}
		return false;
	}

	/// Method: addChildNode
	/// 
	/// * 'c': The new child LevelCurveTree
	pub fn addChildNode(&mut self, c: &'a LevelCurveTree<'a>) {
		self.children.push(c);
	}

	/// Method: getChildren
	pub fn getChildren(&self) -> &Vec<&'a LevelCurveTree<'a>> {
		return &self.children;
	}



}