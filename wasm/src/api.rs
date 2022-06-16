// Imports for WASM
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

// Imports for miette
use miette::Result;

// External imports
use serde::{Deserialize, Serialize};

// Internal imports
use crate::gltf_conversion::generate_gltf;
use crate::model_construction::constructor::ModelConstructor;
use crate::model_construction::smoother::Smoother;
use crate::objects::level_curve_tree::LevelCurveTree;
use crate::objects::point::{Point, Vector};
use crate::objects::raster::Raster;
use crate::objects::triangle::Triangle;
use crate::utils::log;

// Create a trait that will be used for the procedural macro 'SmoothingOperation'
pub trait SmoothingOperation {
	fn apply(&self, smoother: &mut Smoother) -> Result<()>;
}

/// API Struct: OpenCV Tree
///
/// This API is used for transferring OpenCV information from the main web applicaiton to this pre-compiled Rust code.
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug)]
pub struct OpenCVTree {
	pixels_per_curve: Vec<Vec<(u64, u64)>>,
	parent_relations: Vec<isize>,
}
#[wasm_bindgen]
impl OpenCVTree {
	#[wasm_bindgen(constructor)]
	pub fn new(val: &JsValue) -> Result<OpenCVTree, JsValue> {
		val.into_serde().map_err(|_| JsValue::from("Could not parse input from JavaScript as a valid OpenCVTree"))
	}

	pub fn debug(&self) -> Result<JsValue, JsValue> {
		JsValue::from_serde(self).map_err(|_| JsValue::from("Could not serialize OpenCVTree"))
	}
}

/// API Struct: ModelConstructionResult
/// This api struct is used to return the computed result to JavaScript.
/// It's useful, since this rust code will return multiple components: the 3D model and the lava-paths, for example.
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug)]
pub struct ModelConstructionResult {
	gltf: String,
	lava_paths: Vec<Vec<(f32, f32, f32)>>,
	craters: Vec<(f32, f32)>,
}

#[wasm_bindgen]
impl ModelConstructionResult {
	#[wasm_bindgen(constructor)]
	pub fn new(val: &JsValue) -> Result<ModelConstructionResult, JsValue> {
		val.into_serde().map_err(|_| JsValue::from("Could not parse input from JavaScript as a valid ModelConstructionResult"))
	}

	pub fn debug(&self) -> String {
		format!("{self:?}")
	}

	pub fn to_js(&self) -> Result<JsValue, JsValue> {
		JsValue::from_serde(self).map_err(|_| JsValue::from("Could not serialize ModelConstructionResult"))
	}
}

/// API Struct: AltitudeGradientPair
/// This API struct is used to return the result of the get_altitude_and_gradient_for_point function,
/// that is implemented in the ModelConstructionApi struct.
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug)]
pub struct AltitudeGradientPair {
	x: f32,
	y: f32,
	altitude: f32,
	gradient: (f32, f32, f32),
}

#[wasm_bindgen]
impl AltitudeGradientPair {
	#[wasm_bindgen(constructor)]
	pub fn new(val: &JsValue) -> Result<AltitudeGradientPair, JsValue> {
		val.into_serde().map_err(|_| JsValue::from("Could not parse input from JavaScript as a valid AltitudeGradientPair"))
	}

	pub fn debug(&self) -> String {
		format!("{self:?}")
	}

	pub fn to_js(&self) -> Result<JsValue, JsValue> {
		JsValue::from_serde(self).map_err(|_| JsValue::from("Could not serialize AltitudeGradientPair"))
	}
}

/// Main API
///
/// This struct represents the main API towards WASM and can be used for all communication with the library.
///
/// STRUCT
#[wasm_bindgen]
pub struct ModelConstructionApi {
	// API properties
	pub rows: usize,
	pub columns: usize,
	pub width: f32,
	pub height: f32,
	pub altitude_step: f32,
	pub curve_point_separation: f32,
	pub svc_distance: f32,
	pub catmull_clark_iterations: usize,
	pub lava_path_length: usize,
	pub lava_path_fork_val: f32,

	// Private properties
	open_cv_tree: OpenCVTree,
	smoothing_operations_queue: Vec<Box<dyn SmoothingOperation>>,
	computed_model_raster: Option<Raster>,
}

/// Main API
///
/// This struct represents the main API towards WASM and can be used for all communication with the library.
///
/// IMPLEMENTATION
#[wasm_bindgen]
impl ModelConstructionApi {
	//
	// PUBLIC API
	//

	/// # API Function
	/// ## Constructor
	///
	/// Setup the properties for the ModelConstructionApi and give them a default value.
	/// If the user of the API wants, the parameters for the algorithm can be changed by calling other methods afterwards.
	#[wasm_bindgen(constructor)]
	pub fn new() -> Self {
		crate::utils::set_panic_hook();
		// The presented values below are the default values for the different parameters
		Self {
			rows: 10,
			columns: 10,
			width: 100.0,
			height: 100.0,
			altitude_step: 25.0,
			curve_point_separation: 10.0,
			svc_distance: 10.0,
			catmull_clark_iterations: 0,
			lava_path_length: 10,
			lava_path_fork_val: 0.0,
			open_cv_tree: OpenCVTree {
				pixels_per_curve: Vec::new(),
				parent_relations: Vec::new(),
			},
			smoothing_operations_queue: Vec::new(),
			computed_model_raster: None,
		}
	}

	/// # API Function
	/// ## Parameter Setup: basic 3D construction algorithm
	///
	/// The parameters that are of interest for this algorithm are:
	/// - The number of raster rows
	/// - The number of raster columns
	/// - The width in pixels of the raster
	/// - The height in pixels of the raster
	pub fn set_basic_parameters(&mut self, rows: usize, columns: usize, width: f32, height: f32) {
		self.rows = rows;
		self.columns = columns;
		self.width = width;
		self.height = height;
	}

	/// # API Function
	/// ## Parameter Setup: SVC algorithm
	///
	/// The parameters that are of interest for this algorithm are:
	/// - The svc distance, also referred to as contour-margin.
	pub fn set_svc_parameters(&mut self, svc_distance: f32) {
		self.svc_distance = svc_distance;
	}

	/// # API Function
	/// ## Parameter Setup: Catmull Clark surface subdivision
	///
	/// The parameters tht are of interest for this algorithm are:
	/// - The number of iterations that should be performed.
	pub fn set_catmull_clark_parameters(&mut self, catmull_clark_iterations: usize) {
		self.catmull_clark_iterations = catmull_clark_iterations;
	}

	/// # API Function
	/// ## Parameter Setup: Lava Path
	///
	/// The parameters tht are of interest for this algorithm are:
	/// - The length of the lava path in number of steps
	/// - The forking threshold
	pub fn set_lava_path_parameters(&mut self, lava_path_length: usize, lava_path_fork_val: f32) {
		self.lava_path_length = lava_path_length;
		self.lava_path_fork_val = lava_path_fork_val;
	}

	/// # API Function
	/// ## Base: The API-user passes the OpenCV tree from the web-application
	pub fn base(&mut self, open_cv_tree: OpenCVTree, curve_point_separation: f32) {
		self.open_cv_tree = open_cv_tree;
		self.curve_point_separation = curve_point_separation;
	}

	/// # API Function
	/// ## Build: Perform the complete 3D model construction and return the GLTF file as result.
	///
	/// Before calling this method, the user should have setup all the desired parameters already.
	pub fn build(&mut self) -> Result<ModelConstructionResult, JsValue> {
		// Transform the array of parent relations from <isize> into Option<usize>
		let transformed_parent_relations = &self.open_cv_tree.parent_relations.iter().map(|&e| if e < 0 { None } else { Some(e as usize) }).collect();

		// Transform the information we received from OpenCV earlier into a LevelCurveTree
		let level_curve_tree = LevelCurveTree::from_open_cv(&self.open_cv_tree.pixels_per_curve, transformed_parent_relations);

		// Transform this LevelCurveTree into a LevelCurveSet
		let level_curve_set = LevelCurveTree::transform_to_LevelCurveSet(&level_curve_tree, self.altitude_step, self.curve_point_separation, 1).map_err(|e| e.to_string())?;

		//create raster based on level curve model and desired rows and columns
		let mut raster = Raster::new(self.width, self.height, self.rows, self.columns);

		// create new modelConstructor (module containing 3D-model construction algorithm)
		let mut model_constructor = ModelConstructor::new(&mut raster, self.svc_distance, &level_curve_set);

		// determine heights
		model_constructor.construct().map_err(|e| e.to_string())?;

		log!("construction complete");

		// Construct smoother instance
		let mut smoother = Smoother::new(&mut model_constructor).map_err(|e| e.to_string())?;

		// Apply smoothing
		for operation in &self.smoothing_operations_queue {
			operation.apply(&mut smoother).map_err(|e| e.to_string())?;
		}
		log!("smoothing complete");
		//get max alt before normalization, to be used later
		let max_altitude = *smoother.raster.get_highest_altitude();

		// Apply raster normalisation, so it will be contained within a 100x100x100 pixel box
		let num_layers = level_curve_set.get_level_curves().len() as f32;
		let max_height_after_normalisation = f32::min(self.altitude_step * num_layers, 100.0);
		smoother.raster.normalise(100.0, 100.0, max_height_after_normalisation).map_err(|e| e.to_string())?;

		//
		// All smoothing operations have been applied, thereofore the final raster has been computed.
		// Store it as a state-variable.
		//
		self.computed_model_raster = Some(smoother.raster.clone());

		//apply surface subdivision
		let (vs, fs, edge_map) = crate::surface_subdivision::catmull_clark::catmull_clark_super(self.catmull_clark_iterations, smoother.raster)?;

		log!("surface subdivision complete");

		//for lava path generation : find point index of the highest point in the model
		let mut top_height = f32::MIN;

		//find max height in normalized model:
		let normalized_max = *Raster::map(Some(max_altitude), 0.0, max_altitude, 0.0, 100.0).get_or_insert(max_altitude);

		for curve in &level_curve_set.level_curves {
			//use normalized curve height to determine max
			let normalized_curve_height = *Raster::map(Some(curve.altitude), 0.0, max_altitude, 0.0, 100.0).get_or_insert(curve.altitude);
			if normalized_curve_height > top_height {
				top_height = normalized_curve_height;
			}
		}

		top_height -= *Raster::map(Some(level_curve_set.altitude_step), 0.0, max_altitude, 0.0, 100.0).get_or_insert(top_height);

		//for lava path generation : get list of indexes of points above or on highest level curve
		let mut highest_points = Vec::new();
		for (i, v) in vs.iter().enumerate() {
			if v.z >= top_height {
				highest_points.push(i);
			}
		}

		//find lava path from the highest point of the model

		//min alt determines at which alitude a lava path stops
		let min_altitude = *Raster::map(Some(level_curve_set.altitude_step), 0.0, max_altitude, 0.0, 100.0).get_or_insert(level_curve_set.altitude_step) / 2.0;

		//fork factor should be between 0.5 and 0. (0.1 reccommended), 0 = no forking
		//0.1 is nice for thic path, 0.02 for thin, 0.0 for one path
		let (computed_lava_paths, lava_start_points): (Vec<Vec<&Point>>, Vec<&Point>) =
			crate::lava_path_finder::lava_path::get_lava_paths_super(&highest_points, self.lava_path_length, self.lava_path_fork_val, min_altitude, &vs, &edge_map)?;

		// Extract the crater by selecting the start points in the lava-paths
		let mut lava_craters: Vec<(f32, f32)> = Vec::new();
		for p in &lava_start_points {
			lava_craters.push((p.x, p.y));
		}

		// Transform these lava-paths to an array that can be returned towards JavaScript
		let mut lava_path_triples: Vec<Vec<(f32, f32, f32)>> = Vec::new();

		// Transform every point to a tuple of three floats: (x, y, z)
		for (i, arr) in computed_lava_paths.iter().enumerate() {
			lava_path_triples.push(Vec::new());
			for p in arr {
				lava_path_triples[i].push((p.x, p.y, p.z));
			}
		}

		//Turn faces of model into triangles
		let mut final_points: Vec<([f32; 3], [f32; 3])> = Vec::new();
		for f in fs {
			if f.points.len() != 4 {
				return Err(JsValue::from("surface subdivision returns face with incorrect amount of points"));
			}
			//get points of face
			let p0 = vs.get(f.points[0]).ok_or(format!("point list does not contain point {} ", f.points[0]))?;
			let p1 = vs.get(f.points[1]).ok_or(format!("point list does not contain point {} ", f.points[1]))?;
			let p2 = vs.get(f.points[2]).ok_or(format!("point list does not contain point {} ", f.points[2]))?;
			let p3 = vs.get(f.points[3]).ok_or(format!("point list does not contain point {} ", f.points[3]))?;

			//rgb green = 0, 153, 51
			//rgb orange = 255, 153, 51

			let tri00 = ([p0.x, p0.z, p0.y], self.color_for_altitude(0.0, 100.0, p0.z, p0.x, p0.y, &lava_craters));
			let tri10 = ([p3.x, p3.z, p3.y], self.color_for_altitude(0.0, 100.0, p3.z, p3.x, p3.y, &lava_craters));
			let tri01 = ([p1.x, p1.z, p1.y], self.color_for_altitude(0.0, 100.0, p1.z, p1.x, p1.y, &lava_craters));
			let tri11 = ([p2.x, p2.z, p2.y], self.color_for_altitude(0.0, 100.0, p2.z, p2.x, p2.y, &lava_craters));

			// Add the first triangle
			final_points.push(tri00);

			final_points.push(tri01);

			final_points.push(tri11);

			// Add the second triangle
			final_points.push(tri00);

			final_points.push(tri11);

			final_points.push(tri10);
		}

		// Return the result in the form of a ModelConstructionResult
		Ok(ModelConstructionResult {
			gltf: generate_gltf(&final_points).map_err(|e| e.to_string())?,
			lava_paths: lava_path_triples,
			craters: lava_craters,
		})
	}

	/// # API Function
	/// ## After Build: get the altitude and gradient of a specified x and y in pixels.
	///
	/// This function will use the raster that was created after the smoothing operations were completed.
	/// This means that any effect on the altitude and gradient, due to catmull clark, will **not** be incorportated
	/// in this computation. The reason for this is that having to compute a continuous altitude/gradient on a set of
	/// faces and vertices is much harder than to interpolate the points on a raster.
	///
	/// ### Parameters
	/// - `x` (f32): The x-coordinate of the point
	/// - `y` (f32): The y-coordinate of the point
	///
	/// ### Returns
	/// This method returns a tuple (f32, f32). The first entry is the altitude and the second the gradient.
	///
	/// ### Methodology
	///	We distinguish two triangles in each rectangle in the raster:
	///
	///	A --- B
	///	|   / |
	///	| /   |
	///	C --- D
	///
	///	As you can see in the shape above, we have the two triangles:
	///	1. ABC
	///	2. DCB
	///
	///	We need to take two vectors a and b that describe the plane that the triangle lies on, in such
	///	a way that the normal of this plane is always pointing outwards/upwards.
	///
	///	In the case of triangle ABC:
	///	- vector a = vector AC
	///	- vector b = vector AB
	///
	///	In the case of triangle DCB:
	///	- vector a = vector DB
	///	- vector b = vector DC
	///
	///	The normal vector of this plane can now be computed with a cross-product a x b
	pub fn get_altitude_and_gradient_for_point(&self, x: f32, y: f32) -> Result<AltitudeGradientPair, JsValue> {
		let dx: isize = 1;
		let dy: isize = 1;

		// Prebase: convert (x, y) to a Point instance, with z=0
		let query_point = Point { x, y, z: 0.0 };

		// 1. If the model has not yet been computed, return an error
		let raster = self
			.computed_model_raster
			.as_ref()
			.ok_or_else(|| JsValue::from("Cannot compute altitude and gradient before rendering the model"))?;

		// Compute the row/column pair that corresponds to this point
		let row_col_pair = raster.get_row_col(x, y);
		let row = row_col_pair.0 as isize;
		let col = row_col_pair.1 as isize;

		// Collect the altitude/gradient computations for every neighbour
		let mut neighbour_results: Vec<AltitudeGradientPair> = Vec::new();

		for i in -dx..dx {
			for j in -dy..dy {
				neighbour_results.push(self.get_altitude_and_gradient_for_point_helper(col + i, row + j, &query_point)?);
			}
		}

		// Compute the average of all attributes
		let mut altitude: f32 = 0.0;
		let mut gradient: (f32, f32, f32) = (0.0, 0.0, 0.0);

		for n in &neighbour_results {
			altitude += n.altitude;
			gradient.0 += n.gradient.0;
			gradient.1 += n.gradient.1;
			gradient.2 += n.gradient.2;
		}

		altitude /= neighbour_results.len() as f32;
		gradient.0 /= neighbour_results.len() as f32;
		gradient.1 /= neighbour_results.len() as f32;
		gradient.2 /= neighbour_results.len() as f32;

		// Return the obtained result
		Ok(AltitudeGradientPair { x, y, altitude, gradient })
	}
	fn get_altitude_and_gradient_for_point_helper(&self, col: isize, row: isize, query_point: &Point) -> Result<AltitudeGradientPair, JsValue> {
		// 1. If the model has not yet been computed, return an error
		let raster = self
			.computed_model_raster
			.as_ref()
			.ok_or_else(|| JsValue::from("Cannot compute altitude and gradient before rendering the model"))?;

		// Check: if x or y fall outside of the raster, return all zeroes
		if col < 0 || row < 0 || col > (raster.columns as isize) || row > (raster.rows as isize) {
			return Ok(AltitudeGradientPair {
				x: 0.0,
				y: 0.0,
				altitude: 0.0,
				gradient: (0.0, 0.0, 0.0),
			});
		}

		// 2. Determine in which raster-rectangle this point lies
		let (a_row, a_col) = (row as usize, col as usize);
		let (b_row, b_col) = (a_row, a_col + 1);
		let (c_row, c_col) = (a_row + 1, a_col);
		let (d_row, d_col) = (a_row + 1, a_col + 1);

		// Transform these indices to points
		let a_point = raster.get_point(a_row, a_col).map_err(|e| JsValue::from("Cannot get point at index in raster"))?;
		let b_point = raster.get_point(b_row, b_col).map_err(|e| JsValue::from("Cannot get point at index in raster"))?;
		let c_point = raster.get_point(c_row, c_col).map_err(|e| JsValue::from("Cannot get point at index in raster"))?;
		let d_point = raster.get_point(d_row, d_col).map_err(|e| JsValue::from("Cannot get point at index in raster"))?;

		// Construct the two triangles ABC and DCB
		let abc_triangle = Triangle::new(&a_point, &b_point, &c_point);
		let dcb_triangle = Triangle::new(&d_point, &c_point, &b_point);

		// 3. Determine in which triangle of this rectangle this point lies
		// 4. Compute vectors a and b, as described above the signature of this function
		let a_vector: Vector;
		let b_vector: Vector;

		// Also create three vectors that reference the three corners of the triangle.
		// These are later needed to determine the altitude of this point, using barycentric coordinates
		let v0: &Point;
		let v1: &Point;
		let v2: &Point;

		// Determine which triangle contains the query point and set a and b according to the RustDoc above this function
		if abc_triangle.contains_point(query_point) {
			a_vector = Vector::from_point_to_point(&a_point, &c_point);
			b_vector = Vector::from_point_to_point(&a_point, &b_point);

			v0 = &a_point;
			v1 = &b_point;
			v2 = &c_point;
		} else {
			a_vector = Vector::from_point_to_point(&d_point, &b_point);
			b_vector = Vector::from_point_to_point(&d_point, &c_point);

			v0 = &d_point;
			v1 = &c_point;
			v2 = &b_point;
		}

		// 5. Compute the normal vector of the plane through this triangle, by computing
		//	  The cross product a x b
		let a_cross_b = Vector::cross_product(&a_vector, &b_vector);

		// 6. Determine the angle of rotation, according to this normal vector (rot-x, rot-y, rot-z)
		let rotation_angle_x = f32::atan(a_cross_b.y / a_cross_b.z);
		let rotation_angle_y = f32::atan(a_cross_b.z / a_cross_b.x);
		let rotation_angle_z = f32::atan(a_cross_b.y / a_cross_b.x);

		// 7. Use vectors a and b to determine the altitude at point (x, y)
		// Reference 'query_point' to make the formulas easier to understand
		let p = &query_point;

		let triangle_area = Vector::cross_product(&a_vector, &b_vector).len() / 2.0;
		let factor_a = Vector::cross_product(&Vector::from_point_to_point(p, v1), &Vector::from_point_to_point(p, v2)).len() / (2.0 * triangle_area);
		let factor_b = Vector::cross_product(&Vector::from_point_to_point(p, v2), &Vector::from_point_to_point(p, v0)).len() / (2.0 * triangle_area);
		let factor_c = Vector::cross_product(&Vector::from_point_to_point(p, v1), &Vector::from_point_to_point(p, v0)).len() / (2.0 * triangle_area);

		// Compute the altitude: weighted average of these three weights and the altitudes of the three points
		let altitude = (v0.z * factor_a + v1.z * factor_b + v2.z * factor_b) / (factor_a + factor_b + factor_c);

		Ok(AltitudeGradientPair {
			x: 0.0,
			y: 0.0,
			altitude,
			gradient: (rotation_angle_x, rotation_angle_y, rotation_angle_z),
		})
	}

	//
	// PRIVATE FUNCTIONS
	//

	/// Private: Enqueue Smoothing Operation
	fn enqueue_smoothing_operation(&mut self, operation: Box<dyn SmoothingOperation>) {
		self.smoothing_operations_queue.push(operation);
	}
}

// Implement the 'Default' trait for ModelConstructionApi
impl Default for ModelConstructionApi {
	fn default() -> Self {
		Self::new()
	}
}

//
// Structs that are used by the ModelConstructionApi for enqueueing operations
//

use smoothing_operation_derive::SmoothingOperation;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, SmoothingOperation)]
pub struct SmoothingOperationApplySmoothToLayer {
	pub layer: usize,
	pub strength: f32,
	pub coverage: usize,
	pub svc_weight: usize,
	pub allow_svc_change: bool,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, SmoothingOperation)]
pub struct SmoothingOperationSetAltitudeForLayer {
	pub layer: usize,
	pub altitude: f32,
	pub allow_svc_change: bool,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, SmoothingOperation)]
pub struct SmoothingOperationApplySmoothToAltitudeGroup {
	pub altitude_group: usize,
	pub strength: f32,
	pub coverage: usize,
	pub svc_weight: usize,
	pub allow_svc_change: bool,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, SmoothingOperation)]
pub struct SmoothingOperationIncreaseAltitudeForAltitudeGroup {
	pub altitude_group: usize,
	pub percentage_of_altitude_step: f32,
	pub allow_svc_change: bool,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, SmoothingOperation)]
pub struct SmoothingOperationApplySmoothToMiddleLayers {
	pub strength: f32,
	pub coverage: usize,
	pub svc_weight: usize,
	pub allow_svc_change: bool,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, SmoothingOperation)]
pub struct SmoothingOperationApplySmoothToMountainTops {
	pub strength: f32,
	pub coverage: usize,
	pub svc_weight: usize,
	pub allow_svc_change: bool,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, SmoothingOperation)]
pub struct SmoothingOperationIncreaseAltitudeForMountainTops {
	pub percentage_of_altitude_step: f32,
	pub allow_svc_change: bool,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, SmoothingOperation)]
pub struct SmoothingOperationApplySmoothToAll {
	pub strength: f32,
	pub coverage: usize,
	pub svc_weight: usize,
	pub allow_svc_change: bool,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, SmoothingOperation)]
pub struct SmoothingOperationCorrectForAltitudeConstraintsToAllLayers {}
