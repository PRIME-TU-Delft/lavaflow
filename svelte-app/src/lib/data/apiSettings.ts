import type * as wasm from 'wasm';

type SmoothingOperation =
	| wasm.SmoothingOperationApplySmoothToLayer
	| wasm.SmoothingOperationSetAltitudeForLayer
	| wasm.SmoothingOperationApplySmoothToAltitudeGroup
	| wasm.SmoothingOperationIncreaseAltitudeForAltitudeGroup
	| wasm.SmoothingOperationApplySmoothToMiddleLayers
	| wasm.SmoothingOperationApplySmoothToMountainTops
	| wasm.SmoothingOperationIncreaseAltitudeForMountainTops
	| wasm.SmoothingOperationApplySmoothToAll
	| wasm.SmoothingOperationCorrectForAltitudeConstraintsToAllLayers;

export default class ApiSettings {
	tree: wasm.OpenCVTree;
	rows: number;
	columns: number;
	width: number;
	height: number;
	curve_point_separation: number;
	svc_distance: number;
	catmull_clark_iterations: number;
	lava_path_length: number;
	lava_path_fork_val: number;
	smoothing_operations: SmoothingOperation[];

	constructor(
		tree: wasm.OpenCVTree,
		rows: number,
		columns: number,
		width: number,
		height: number,
		curve_point_separation: number,
		svc_distance: number,
		catmull_clark_iterations: number,
		lava_path_length: number,
		lava_path_fork_val: number,
		smoothing_operations: SmoothingOperation[]
	) {
		this.tree = tree;
		this.rows = rows;
		this.columns = columns;
		this.width = width;
		this.height = height;
		this.curve_point_separation = curve_point_separation;
		this.svc_distance = svc_distance;
		this.catmull_clark_iterations = catmull_clark_iterations;
		this.lava_path_length = lava_path_length;
		this.lava_path_fork_val = lava_path_fork_val;
		this.smoothing_operations = smoothing_operations;
	}

	apply_to_api(api: wasm.ModelConstructionApi) {
		api.base(this.tree, this.curve_point_separation);
		api.set_basic_parameters(this.rows, this.columns, this.width, this.height);
		api.set_svc_parameters(this.svc_distance);
		api.set_catmull_clark_parameters(this.catmull_clark_iterations);
		api.set_lava_path_parameters(this.lava_path_length, this.lava_path_fork_val);

		for (const op of this.smoothing_operations) {
			op.apply_to_api(api);
		}
	}
}
