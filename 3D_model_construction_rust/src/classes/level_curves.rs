#![allow(dead_code)]

use std::thread::current;

//
// Class: LevelCurves
//
use super::{point::Point, constructor};


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
	///
	/// transforms levelCurveTree to levelCurveMap structure, while reducing amount of total points from pixelStructure
    ///
    /// # Arguments
    ///
    /// * `tree` - levelCurveTree datastructure containing information from scanning step
    /// * `altitude_step` - increase in height per contour line
    /// * `desired_dist` - minimum desired distance between points in final conout map
    /// * `current_height` - to track height when traversing tree, initial call should start with 1
    /// 
	pub fn transform_to_LevelCurveMap<'a>(&self, tree : &'a mut LevelCurveTree<'a> ,  altitude_step: f64, desired_dist: f64 , current_height : usize) -> LevelCurveMap {
		
		let mut ret: LevelCurveMap = LevelCurveMap::new(altitude_step);

		let mut current_level_curve = LevelCurve::new(altitude_step * current_height as f64);

		//TODO: dont use unwrap
		let first_pixel = tree.get_first_pixel().unwrap();
		let mut last_saved = first_pixel;
		let mut last_visited = first_pixel;
		let mut current_pixel = first_pixel;

		//untill we rencounter the first pixel, search direct neightborhood (directly adjacent pixels) of current pixel for next pixel
		//Assumption: there are no breaks in the line
		//break for loop in line 165

		loop {
			//Assumption: pixels have directly connected neighbors (diagonals do not count as adjacent)
			//Assumption: line is exactly 1 pixel wide
			//TODO; check if in actual input every pixel has adjacent pixel
			let neighbors = vec![(current_pixel.0 - 1, current_pixel.1),
												(current_pixel.0 + 1, current_pixel.1),
												(current_pixel.0, current_pixel.1 - 1),
												(current_pixel.0, current_pixel.1 + 1)];
			for (x, y) in neighbors {

					//TODO: check how this holds for corner cases
					if (x , y) != current_pixel && ( x , y) != last_visited && tree.contains_pixel(x, y) {
	
						if tree.contains_pixel(x, y) {
							//if dist to last saved and current pixel is desired length, save current pixel, else move on
							if pixel_dist( &(x, y), &last_saved  ) >= desired_dist {

								current_level_curve.add_point( Point{x: x as f64 , y: y as f64, z: current_level_curve.altitude} );
								last_saved = (x, y);

							}

							last_visited = current_pixel;
							current_pixel= (x, y);

						}

					}
				}
				if(current_pixel == first_pixel){
					break;
				}
			
		}
		
		//for every child get levelcurvemap and add to ret

		for mut child in tree.get_children(){
			let childmap = self.transform_to_LevelCurveMap(&mut child, altitude_step, desired_dist, current_height + 1 );
			for curve in childmap.level_curves {
				ret.add_level_curve(curve);
			}
		}
		 
		ret
	}
}
//TODO: find better method
fn pixel_dist(a : &(u64, u64) , b: &(u64, u64)) -> f64 {
		( (a.0 as f64 -b.0 as f64).powi(2) + (a.1 as f64 - b.1 as f64).powi(2) ).sqrt()
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
	// own_index refers to the own perspective of this tree. Since an internal
	// datastructure is used that matches OpenCV, this perspective index is required.
	// The perspective of a Tree instance specified 'which child this instance represents'.
	// This information is required in order to be able to determine what the parent of this
	// node is and what the children are, using the parent_relations array.
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

	/// Method: Get the current perspective index (testing purposes)
	pub fn get_current_perspective(&self) -> usize {
		self.own_index
	}

	/// Method: Set the current perspective index (testing purposes)
	pub fn set_current_perspective(&mut self, perspective: usize) {
		self.own_index = perspective;
	}

	/// Method: Get the parent of this node
	pub fn get_parent(&'a self) -> Option<LevelCurveTree> {
		if self.parent_relations[self.own_index].is_none() {
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
	pub fn get_children(&'a self) -> Vec<LevelCurveTree> {

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

	/// Method: Retrieve the first pixel in this level-curve
	pub fn get_first_pixel(&self) -> Option<(u64, u64)> {
		if self.pixels_per_curve[self.own_index].is_empty() {
			return None;
		}

		Some(self.pixels_per_curve[self.own_index][0])
	}
}





//
// UNIT TESTS
//

#[cfg(test)]
mod tests {

	use super::LevelCurveTree;

	//
	// Unit tests for the LevelCurveTree class
	//

	fn construct_tree<'a>(pixels_per_curve: &'a mut Vec<Vec<(u64, u64)>>, parent_relations: &'a mut Vec<Option<usize>>) -> LevelCurveTree<'a> {

		// We will create a level-curve with 4 layers
		for i in 0..4 {
			// Add a vector to the pixels_per_curve array and add arbitrary pixels
			let mut pixels: Vec<(u64, u64)> = Vec::new();

			for j in 0..10 {
				pixels.push((j, 2*j));
			}

			pixels_per_curve.push(pixels);
		}

		// Add the parent-relations as follows:
		/*					0
							|
							1
						  /  \
						 2    3
		*/

		parent_relations.push(None);		// index 0 has no parent
		parent_relations.push(Some(0));		// index 1 has parent 0
		parent_relations.push(Some(1));     // index 2 has parent 1
		parent_relations.push(Some(1));		// index 3 has parent 1

		// Pass these vectors to the constuctor
		LevelCurveTree::from_open_cv(pixels_per_curve, parent_relations)
	}

	#[test]
	fn level_curve_tree_constructor_right_own_index() {

		// Construct the arrays, as one would receive them from OpenCV
		let mut pixels_per_curve: Vec<Vec<(u64, u64)>> = Vec::new();
		let mut parent_relations: Vec<Option<usize>> = Vec::new();

		// Fill the arrays with information, as one would receive from OpenCV
		let tree = construct_tree(&mut pixels_per_curve, &mut parent_relations);

		// Assert the root to be 0
		assert_eq!(tree.own_index, 0);

	}

	#[test]
	fn level_curve_tree_get_parent_of_root() {

		// Construct the arrays, as one would receive them from OpenCV
		let mut pixels_per_curve: Vec<Vec<(u64, u64)>> = Vec::new();
		let mut parent_relations: Vec<Option<usize>> = Vec::new();

		// Fill the arrays with information, as one would receive from OpenCV
		let mut tree = construct_tree(&mut pixels_per_curve, &mut parent_relations);

		// Assert the root to be 0
		assert!(tree.get_parent().is_none());

	}


	#[test]
	fn level_curve_tree_get_parent_of_nonroot() {

		// Construct the arrays, as one would receive them from OpenCV
		let mut pixels_per_curve: Vec<Vec<(u64, u64)>> = Vec::new();
		let mut parent_relations: Vec<Option<usize>> = Vec::new();

		// Fill the arrays with information, as one would receive from OpenCV
		let mut tree = construct_tree(&mut pixels_per_curve, &mut parent_relations);

		// Set the perspective of the tree to the child '1'
		tree.set_current_perspective(1);

		// Assert the parent to be 'some'
		assert!(!tree.get_parent().is_none());

		let parent_unwrapped = tree.get_parent().unwrap();

		// Assert the parent to have the right perspective index
		assert_eq!(parent_unwrapped.get_current_perspective(), 0);

	}

	#[test]
	fn level_curve_tree_get_children_count_1() {

		// Construct the arrays, as one would receive them from OpenCV
		let mut pixels_per_curve: Vec<Vec<(u64, u64)>> = Vec::new();
		let mut parent_relations: Vec<Option<usize>> = Vec::new();

		// Fill the arrays with information, as one would receive from OpenCV
		let mut tree = construct_tree(&mut pixels_per_curve, &mut parent_relations);

		// Get the list of children from this parent
		let mut children = tree.get_children();

		// Assert that the length of this vector is 1
		assert_eq!(children.len(), 1);

	}

	#[test]
	fn level_curve_tree_get_children_count_2() {

		// Construct the arrays, as one would receive them from OpenCV
		let mut pixels_per_curve: Vec<Vec<(u64, u64)>> = Vec::new();
		let mut parent_relations: Vec<Option<usize>> = Vec::new();

		// Fill the arrays with information, as one would receive from OpenCV
		let mut tree = construct_tree(&mut pixels_per_curve, &mut parent_relations);

		// Change the perspective of the tree to be that of the first node
		tree.set_current_perspective(1);

		// Get the list of children from this parent
		let mut children = tree.get_children();

		// Assert that the length of this vector is 1
		assert_eq!(children.len(), 2);

	}

	#[test]
	fn level_curve_tree_get_child() {

		// Construct the arrays, as one would receive them from OpenCV
		let mut pixels_per_curve: Vec<Vec<(u64, u64)>> = Vec::new();
		let mut parent_relations: Vec<Option<usize>> = Vec::new();

		// Fill the arrays with information, as one would receive from OpenCV
		let tree = construct_tree(&mut pixels_per_curve, &mut parent_relations);

		// Get the list of children from this parent
		let children = tree.get_children();

		// Get the first child of this node
		let child1 = &children[0];

		// Assert that the perspective of this child is correct
		assert_eq!(child1.get_current_perspective(), 1);

	}

	#[test]
	fn level_curve_tree_is_parent_of() {

		// Construct the arrays, as one would receive them from OpenCV
		let mut pixels_per_curve: Vec<Vec<(u64, u64)>> = Vec::new();
		let mut parent_relations: Vec<Option<usize>> = Vec::new();

		// Fill the arrays with information, as one would receive from OpenCV
		let tree = construct_tree(&mut pixels_per_curve, &mut parent_relations);

		// Get the list of children from this parent
		let children = tree.get_children();

		// Get the first child of this node
		let child1 = &children[0];

		// Assert that the perspective of this child is correct
		assert!(tree.is_parent_of(child1));

	}

	#[test]
	fn level_curve_tree_is_child_of() {

		// Construct the arrays, as one would receive them from OpenCV
		let mut pixels_per_curve: Vec<Vec<(u64, u64)>> = Vec::new();
		let mut parent_relations: Vec<Option<usize>> = Vec::new();

		// Fill the arrays with information, as one would receive from OpenCV
		let tree = construct_tree(&mut pixels_per_curve, &mut parent_relations);

		// Get the list of children from this parent
		let children = tree.get_children();

		// Get the first child of this node
		let child1 = &children[0];

		// Assert that the perspective of this child is correct
		assert!(child1.is_child_of(&tree));

	}

	#[test]
	fn level_curve_tree_contains_pixel_positive() {

		// Construct the arrays, as one would receive them from OpenCV
		let mut pixels_per_curve: Vec<Vec<(u64, u64)>> = Vec::new();
		let mut parent_relations: Vec<Option<usize>> = Vec::new();

		// Fill the arrays with information, as one would receive from OpenCV
		let tree = construct_tree(&mut pixels_per_curve, &mut parent_relations);

		// Assert that the tree contains the pixel (1, 2)
		assert!(tree.contains_pixel(1, 2));

	}

	#[test]
	fn level_curve_tree_contains_pixel_negative() {

		// Construct the arrays, as one would receive them from OpenCV
		let mut pixels_per_curve: Vec<Vec<(u64, u64)>> = Vec::new();
		let mut parent_relations: Vec<Option<usize>> = Vec::new();

		// Fill the arrays with information, as one would receive from OpenCV
		let tree = construct_tree(&mut pixels_per_curve, &mut parent_relations);

		// Assert that the tree does not contain the pixel (2, 2)
		assert!(!tree.contains_pixel(2, 2));

	}

}