// Imports for WASM
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

// Imports for miette
use miette::Result;

// External imports
use serde::{Deserialize, Serialize};

// Internal imports
use crate::gltf_conversion::generate_gltf;
use crate::model_construction::smoother::Smoother;
use crate::objects::level_curve_tree::LevelCurveTree;
use crate::objects::level_curves::LevelCurveSet;
use crate::objects::point::Point;
use crate::objects::raster::Raster;

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
	pub fn new(val: JsValue) -> Result<OpenCVTree, JsValue> {
		val.into_serde().map_err(|_| JsValue::from("Could not parse input from JavaScript as a valid OpenCVTree"))
	}

	pub fn debug(&self) -> String {
		format!("{self:?}")
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
}

#[wasm_bindgen]
impl ModelConstructionResult {
	#[wasm_bindgen(constructor)]
	pub fn new(val: JsValue) -> Result<ModelConstructionResult, JsValue> {
		val.into_serde().map_err(|_| JsValue::from("Could not parse input from JavaScript as a valid ModelConstructionResult"))
	}

	pub fn debug(&self) -> String {
		format!("{self:?}")
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
	pub border_size: f32,
	pub altitude_step: f32,
	pub svc_distance: f32,
	pub catmull_clark_iterations: usize,
	pub lava_path_length: usize,
	pub lava_path_fork_val: f32,

	// Private properties
	open_cv_tree: OpenCVTree,
	smoothing_operations_queue: Vec<Box<dyn SmoothingOperation>>,
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
		// The presented values below are the default values for the different parameters
		Self {
			rows: 10,
			columns: 10,
			border_size: 10.0,
			altitude_step: 10.0,
			svc_distance: 10.0,
			catmull_clark_iterations: 0,
			lava_path_length: 10,
			lava_path_fork_val: 0.0,
			open_cv_tree: OpenCVTree {
				pixels_per_curve: Vec::new(),
				parent_relations: Vec::new(),
			},
			smoothing_operations_queue: Vec::new(),
		}
	}

	/// # API Function
	/// ## Parameter Setup: basic 3D construction algorithm
	///
	/// The parameters that are of interest for this algorithm are:
	/// - The number of raster rows
	/// - The number of raster columns
	/// - The size of the border around the mountain
	pub fn set_basic_parameters(&mut self, rows: usize, columns: usize, border_size: f32) {
		self.rows = rows;
		self.columns = columns;
		self.border_size = border_size;
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
	pub fn base(&mut self, open_cv_tree: OpenCVTree) {
		self.open_cv_tree = open_cv_tree;
	}

	/// # API Function
	/// ## Build: Perform the complete 3D model construction and return the GLTF file as result.
	///
	/// Before calling this method, the user should have setup all the desired parameters already.
	pub fn build(&self) -> Result<ModelConstructionResult, JsValue> {
		// Transform the array of parent relations from <isize> into Option<usize>
		let transformed_parent_relations = &self.open_cv_tree.parent_relations.iter().map(|&e| if e < 0 { None } else { Some(e as usize) }).collect();

		// Transform the information we received from OpenCV earlier into a LevelCurveTree
		let level_curve_tree = LevelCurveTree::from_open_cv(&self.open_cv_tree.pixels_per_curve, transformed_parent_relations);

		// Transform this LevelCurveTree into a LevelCurveSet
		let mut level_curve_map = LevelCurveSet::transform_to_LevelCurveMap(&level_curve_tree, self.altitude_step, 2.0 * self.svc_distance, 1).map_err(|e| e.to_string())?;

		//find maximum and minimum cooridinates in level curve model
		let (min, max) = level_curve_map.get_bounding_points();

		// keep desired border of each axis around model
		let border_x = self.border_size * (max.x - min.x);
		let border_y = self.border_size * (max.y - min.y);

		//ensure none of the level curve points have negative coordinates , and have a 'border' distance from the axes
		level_curve_map.align_with_origin(&min, border_x, border_y);

		//create raster based on level curve model and desired rows and columns
		let mut raster = Raster::new((max.x - min.x) + (border_x * 2.0), (max.y - min.y) + (border_y * 2.0), self.rows, self.columns);

		// create new modelConstructor (module containing 3D-model construction algorithm)
		let mut model_constructor = crate::model_construction::constructor::ModelConstructor::new(&mut raster, self.svc_distance, &level_curve_map);

		// determine heights
		model_constructor.construct().map_err(|e| e.to_string())?;

		// Construct smoother instance
		let mut smoother = Smoother::new(&mut model_constructor).map_err(|e| e.to_string())?;

		// Apply smoothing
		for operation in &self.smoothing_operations_queue {
			operation.apply(&mut smoother).map_err(|e| e.to_string())?;
		}

		//apply surface subdivision
		let (vs, fs, edge_map) = crate::surface_subdivision::catmull_clark::catmull_clark_super(self.catmull_clark_iterations, &model_constructor.is_svc, model_constructor.raster, false)?;

		//for lava path generation : find point index of the highest point in the model

		let mut top_height = f32::MIN;
		for curve in &level_curve_map.level_curves {
			if curve.altitude > top_height {
				top_height = curve.altitude;
			}
		}
		//top_height -= level_curve_map.altitude_step;

		//for lava path generation : get list of indexes of points above or on highest level curve
		let mut highest_points = Vec::new();
		for (i, v) in vs.iter().enumerate() {
			if v.z >= top_height {
				highest_points.push(i);
			}
		}

		//find lava path from the highest point of the model
		//min alt determines at which alitude a lava path stops
		let min_altitude = level_curve_map.altitude_step / 2.0;
		//fork factor should be between 0.5 and 0. (0.1 reccommended), 0 = no forking
		// 0.1 is nice for thic path, 0.02 for thin, 0.0 for one path
		let computed_lava_paths: Vec<Vec<&Point>> =
			crate::lava_path_finder::lava_path::get_lava_paths_super(&highest_points, self.lava_path_length, self.lava_path_fork_val, min_altitude, &vs, &edge_map)?;

		// Transform these lava-paths to an array that can be returned towards JavaScript
		let mut lava_path_tuples: Vec<Vec<(f32, f32, f32)>> = Vec::new();

		// Transform every point to a tuple of three floats: (x, y, z)
		for (i, arr) in computed_lava_paths.iter().enumerate() {
			lava_path_tuples.push(Vec::new());
			for p in arr {
				lava_path_tuples[i].push((p.x, p.y, p.z));
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

			let tri00 = ([p0.x, p0.z, p0.y], [0.0 / 255.0, 153.0 / 255.0, 51.0 / 255.0]);
			let tri10 = ([p3.x, p3.z, p3.y], [0.0 / 255.0, 153.0 / 255.0, 51.0 / 255.0]);
			let tri01 = ([p1.x, p1.z, p1.y], [0.0 / 255.0, 153.0 / 255.0, 51.0 / 255.0]);
			let tri11 = ([p2.x, p2.z, p2.y], [0.0 / 255.0, 153.0 / 255.0, 51.0 / 255.0]);

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
			gltf: generate_gltf(final_points)?,
			lava_paths: lava_path_tuples,
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

#[derive(SmoothingOperation)]
pub struct SmoothingOperationApplySmoothToLayer {
	pub layer: usize,
	pub strength: f32,
	pub coverage: usize,
	pub svc_weight: usize,
	pub allow_svc_change: bool,
}

#[derive(SmoothingOperation)]
pub struct SmoothingOperationSetAltitudeForLayer {
	pub layer: usize,
	pub altitude: f32,
	pub allow_svc_change: bool,
}

#[derive(SmoothingOperation)]
pub struct SmoothingOperationApplySmoothToAltitudeGroup {
	pub altitude_group: usize,
	pub strength: f32,
	pub coverage: usize,
	pub svc_weight: usize,
	pub allow_svc_change: bool,
}

#[derive(SmoothingOperation)]
pub struct SmoothingOperationIncreaseAltitudeForAltitudeGroup {
	pub altitude_group: usize,
	pub percentage_of_altitude_step: f32,
	pub allow_svc_change: bool,
}

#[derive(SmoothingOperation)]
pub struct SmoothingOperationApplySmoothToMiddleLayers {
	pub strength: f32,
	pub coverage: usize,
	pub svc_weight: usize,
	pub allow_svc_change: bool,
}

#[derive(SmoothingOperation)]
pub struct SmoothingOperationApplySmoothToMountainTops {
	pub strength: f32,
	pub coverage: usize,
	pub svc_weight: usize,
	pub allow_svc_change: bool,
}

#[derive(SmoothingOperation)]
pub struct SmoothingOperationIncreaseAltitudeForMountainTops {
	pub percentage_of_altitude_step: f32,
	pub allow_svc_change: bool,
}

#[derive(SmoothingOperation)]
pub struct SmoothingOperationApplySmoothToAll {
	pub strength: f32,
	pub coverage: usize,
	pub svc_weight: usize,
	pub allow_svc_change: bool,
}

#[derive(SmoothingOperation)]
pub struct SmoothingOperationCorrectForAltitudeConstraintsToAllLayers {}
