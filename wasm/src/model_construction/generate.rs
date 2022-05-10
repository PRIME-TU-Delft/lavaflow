use wasm_bindgen::prelude::wasm_bindgen;

use super::constructor::ModelConstructor;
use super::gltf_conversion::generate_gltf;
use super::level_curve_tree::LevelCurveTree;
use super::level_curves::LevelCurveSet;
use super::raster::Raster;

/// Struct representing a tree coming from OpenCV, that has not yet been converted to our internal tree structure
#[wasm_bindgen]
pub struct OpenCVTree {
	pixels_per_curve: Vec<Vec<(u64, u64)>>,
	parent_relations: Vec<Option<usize>>,
}

/// Struct used to nicely package settings for the `generate_3d_model` function.
/// - `contour_margin` - Margin that defines when a point is considered 'on' a contour line, high value results in more staircase-like appearance, low value might lead to innacurate result.
/// NOTE: margin must be above max(raster height, column width) so long as local_tin() is not implemented
/// - `columns` - desired number columns used for raster
/// - `rows` - desired number rows used for raster
/// - `altitude_step` - fixed increase in height per level curve
#[wasm_bindgen]
pub struct ModelGenerationSettings {
	contour_margin: f32,
	columns: usize,
	rows: usize,
	altitude_step: f32,
	desired_dist: f32,
}

/// Supermethod that takes in an openCV tree and outputs an GTLF model.
/// - `tree`- input from the image processing step, a representation of level curves. To be converted to 3D model
#[wasm_bindgen]
pub fn generate_3d_model(open_cv_tree: &OpenCVTree, settings: &ModelGenerationSettings) -> String {
	// Unpack function argument structs & build OpenCV tree struct
	let mut tree = LevelCurveTree::from_open_cv(&open_cv_tree.pixels_per_curve, &open_cv_tree.parent_relations);
	let ModelGenerationSettings {
		contour_margin,
		columns,
		rows,
		altitude_step,
		desired_dist,
	} = *settings;

	// convert openCV tree to levelCurveMap (input for construction algorithm)
	let mut level_curve_map = LevelCurveSet::new(altitude_step).transform_to_LevelCurveMap(&mut tree, altitude_step, desired_dist, 0);

	//find maximum and minimum cooridinates in level curve model
	let (min, max) = level_curve_map.get_bounding_points();

	//to keep border of 10% of each axis around model
	let border_x = 0.1 * (max.x - min.x);
	let border_y = 0.1 * (max.y - min.y);

	//ensure none of the level curve points have negative coordinates
	level_curve_map.align_with_origin(&min, border_x, border_y);

	//create raster based on level curve model and desired rows and columns
	let mut raster = Raster::new((max.x - min.x) + border_x, (max.y - min.y) + border_y, rows, columns);

	// create new modelConstructor (module containing 3D-model construction algorithm)
	let mut model_constructor = ModelConstructor::new(&mut raster, contour_margin, &level_curve_map);

	// determine heights
	model_constructor.construct();

	// get heights from raster
	let heights = &raster.altitudes;

	// convert height raster to flat list of x,y,z points for GLTF format
	// every cell had 4 corners, becomes two triangles
	let mut final_points: Vec<[f32; 3]> = Vec::new();
	for x in 0..raster.rows {
		for y in 0..raster.columns {
			// triangle 1
			//(0,0)
			final_points.push([columns as f32 * raster.column_width, (rows - y) as f32 * raster.row_height, heights[x][y].unwrap()]);
			//(0, 1)
			final_points.push([columns as f32 * raster.column_width, ((rows - y) as f32 + 1.0) * raster.row_height, heights[x][y + 1].unwrap()]);
			//(1, 1)
			final_points.push([
				(columns as f32 + 1.0) * raster.column_width,
				((rows - y) as f32 + 1.0) * raster.row_height,
				heights[x + 1][y + 1].unwrap(),
			]);

			// triangle 2
			//(0, 0)
			final_points.push([columns as f32 * raster.column_width, (rows - y) as f32 * raster.row_height, heights[x][y].unwrap()]);
			//(1, 0)
			final_points.push([(columns as f32 + 1.0) * raster.column_width, (rows - y) as f32 * raster.row_height, heights[x + 1][y].unwrap()]);
			//(1, 1)
			final_points.push([(columns as f32 + 1.0) * raster.column_width, ((rows - y) as f32 + 1.0) * raster.row_height, heights[x][y + 1].unwrap()]);
		}
	}

	generate_gltf(final_points)
}
