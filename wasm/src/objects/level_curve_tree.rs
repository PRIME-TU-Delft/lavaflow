//
// Level Curve Tree
//
// The level curve tree is used to represent the information that was received from the OpenCV library.
// This step is necessary, since it allows us to efficiently store all information that OpenCV outputs.
// This includes information that could be required by other algorithms.
//

use std::collections::HashSet;

use super::{
	level_curves::{LevelCurve, LevelCurveSet},
	point::Point,
};
use miette::{miette, Result};

#[derive(Debug)]
pub struct LevelCurveTree<'a> {
	pixels_per_curve: &'a Vec<Vec<(u64, u64)>>,
	parent_relations: &'a Vec<Option<usize>>,
	// own_index refers to the own perspective of this tree. Since an internal
	// datastructure is used that matches OpenCV, this perspective index is required.
	// The perspective of a Tree instance specified 'which child this instance represents'.
	// This information is required in order to be able to determine what the parent of this
	// node is and what the children are, using the parent_relations array.
	own_index: usize,
}

impl<'a> LevelCurveTree<'a> {
	//
	// CONSTRUCTORS
	//

	/// Dynamic constructor: From `OpenCV` datastructure
	pub fn from_open_cv(pixels_per_curve: &'a Vec<Vec<(u64, u64)>>, parent_relations: &'a Vec<Option<usize>>) -> Self {
		let mut own_index = 0;

		// 1. Find the node that has no parent (this is the root of the tree)
		for (index, &relation) in parent_relations.iter().enumerate() {
			if relation.is_none() {
				// We've found the root!
				own_index = index;
			}
		}

		Self {
			pixels_per_curve,
			parent_relations,
			own_index,
		}
	}

	/// Private Dynamic constructor: From existing tree, with different index
	// Note: not really according to Rust convention, though I guess it sorta makes sense in context? Might wanna change this later.
	#[allow(clippy::wrong_self_convention)]
	fn from_perspective_index(&self, index: usize) -> Self {
		Self {
			pixels_per_curve: self.pixels_per_curve,
			parent_relations: self.parent_relations,
			own_index: index,
		}
	}

	//
	// PRIVATE METHODS
	//

	//
	// PUBLIC METHODS
	//

	/// Method: `get_children`
	pub fn get_children(&self) -> Vec<LevelCurveTree> {
		let mut result: Vec<LevelCurveTree> = Vec::new();

		// Add all trees of whom this instance is the parent
		for (index, parent) in self.parent_relations.iter().enumerate() {
			if parent == &Some(self.own_index) {
				// The node at index i is a child of this instance
				// Add it to the list
				result.push(self.from_perspective_index(index));
			}
		}

		result
	}

	/// Method: Retrieve pixels in curve at current top of tree
	pub fn get_pixels_for_curve(&self) -> Option<&Vec<(u64, u64)>> {
		self.pixels_per_curve.get(self.own_index)
	}
	/// HELPER
	///
	/// transforms `levelCurveTree` to `levelCurveSet` structure, while reducing amount of total points from tree structure
	///
	/// # Arguments
	///
	/// * `tree` - `levelCurveTree` datastructure containing information from scanning step
	/// * `altitude_step` - increase in height per contour line
	/// * `desired_dist` - minimum desired distance between points in final contour map
	/// * `current_height` - used to track height when traversing tree recursively, initial call should start with 1
	///
	///
	#[allow(non_snake_case)]
	pub fn transform_to_LevelCurveSet_helper(&'a self, altitude_step: f32, mut desired_dist: f32, current_height: usize) -> Result<LevelCurveSet> {
		let mut result: LevelCurveSet = LevelCurveSet::new(altitude_step);
		let mut current_level_curve = LevelCurve::new(altitude_step * current_height as f32);

		//get pixels from tree
		let pixels = self.get_pixels_for_curve().ok_or_else(|| miette!("Could not get pixels in tree"))?;
		let first_pixel: &(u64, u64) = pixels.get(0).ok_or_else(|| miette!("Could not get first pixel"))?;

		// If there are only 50 pixels, select all of them
		// TODO: 50 is somewhat arbitrarily chosen, there's probably a better way to do this
		if pixels.len() < 50 {
			desired_dist = 0.0;
		}

		//add first point to level curve and store as most recently added
		current_level_curve.add_point(Point {
			x: first_pixel.0 as f32,
			y: first_pixel.1 as f32,
			z: current_level_curve.altitude,
		});
		let mut last_added = first_pixel;

		//keep track of traversed pixels
		let mut visited: HashSet<&(u64, u64)> = HashSet::with_capacity(pixels.len());
		visited.insert(first_pixel);

		//loop to add rest of pixels to level curve
		for pixel in pixels {
			if !visited.contains(pixel) {
				//check if next pixel is desired distance from last saved pixel , if so add to current level curve
				if pixel_dist(&(pixel.0, pixel.1), last_added) >= desired_dist {
					current_level_curve.add_point(Point {
						x: pixel.0 as f32,
						y: pixel.1 as f32,
						z: current_level_curve.altitude,
					});
					last_added = pixel;
				}
			}
		}

		// If self doesn't have any children, current_level_curve must be a mountain-top. Update the administration
		if self.get_children().is_empty() {
			current_level_curve.is_mountain_top = true;
		}

		result.add_level_curve(current_level_curve);

		// if there are no children to traverse return current set
		if self.get_children().is_empty() {
			return Ok(result);
		}

		//if current node has children find their level curves recursively
		for child in &self.get_children() {
			let child_set = child.transform_to_LevelCurveSet_helper(altitude_step, desired_dist, current_height + 1)?;
			//TODO: is this bad space wise?
			for curve in child_set.level_curves {
				result.add_level_curve(curve);
			}
		}

		Ok(result)
	}

	///
	///
	/// transforms `levelCurveTree` to `levelCurveSet` structure, while reducing amount of total points from tree structure
	///
	/// # Arguments
	///
	/// * `tree` - `levelCurveTree` datastructure containing information from scanning step
	/// * `altitude_step` - increase in height per contour line
	/// * `desired_dist` - minimum desired distance between points in final contour map
	/// * `current_height` - used to track height when traversing tree recursively, initial call should start with 1
	///
	///
	#[allow(non_snake_case)]
	pub fn transform_to_LevelCurveSet(&'a self, altitude_step: f32, desired_dist: f32, current_height: usize) -> Result<LevelCurveSet> {
		// 1. Make a collection of all level curves that are a root (parent None)
		let mut collection_of_roots: Vec<usize> = Vec::new();
		for (i, relation) in self.parent_relations.iter().enumerate() {
			if relation.is_none() {
				// This node has a parent of None, meaning it has no parent (it's a root)
				collection_of_roots.push(i);
			}
		}

		// 2. Run the helper function on all of these roots
		let mut resulting_level_curve_sets: Vec<LevelCurveSet> = Vec::new();
		for i in collection_of_roots {
			resulting_level_curve_sets.push(self.from_perspective_index(i).transform_to_LevelCurveSet_helper(altitude_step, desired_dist, current_height)?);
		}

		// 3. Combine the level-curves into one level curve set
		let mut result: LevelCurveSet = LevelCurveSet::new(altitude_step);
		for (_, lc_set) in resulting_level_curve_sets.iter().enumerate() {
			for (_, lc) in lc_set.level_curves.iter().enumerate() {
				result.add_level_curve(lc.clone());
			}
		}

		Ok(result)
	}
}

fn pixel_dist(a: &(u64, u64), b: &(u64, u64)) -> f32 {
	((a.0 as f32 - b.0 as f32).powi(2) + (a.1 as f32 - b.1 as f32).powi(2)).sqrt()
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
		for _ in 0..4 {
			// Add a vector to the pixels_per_curve array and add arbitrary pixels
			let mut pixels: Vec<(u64, u64)> = Vec::new();

			for j in 0..10 {
				pixels.push((j, 2 * j));
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

		parent_relations.push(None); // index 0 has no parent
		parent_relations.push(Some(0)); // index 1 has parent 0
		parent_relations.push(Some(1)); // index 2 has parent 1
		parent_relations.push(Some(1)); // index 3 has parent 1

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
		let tree = construct_tree(&mut pixels_per_curve, &mut parent_relations);

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
		assert!(tree.get_parent().is_some());

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
		let tree = construct_tree(&mut pixels_per_curve, &mut parent_relations);

		// Get the list of children from this parent
		let children = tree.get_children();

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
		let children = tree.get_children();

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
