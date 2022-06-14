/**
 * Svelte stores, store reactive global data for more info:
 * https://svelte.dev/docs#run-time-svelte-store
 *
 * This stores gltf data
 */

import { writable } from 'svelte/store';
import type { CurveTree } from '$lib/stores/contourLineStore';
import type Draggable from '$lib/data/draggable';
import ApiSettings from '$lib/data/apiSettings';

import init, * as wasm from 'wasm';

export interface Model {
	gltf: string;
	gltf_url: string;
	lava_paths: [number, number, number][][];
	craters: [number, number][];
}

export interface AltitudeGradientPair {
	x: number;
	y: number;
	altitude: number;
	gradient: [number, number, number];
}

export const lavapaths = writable<[number, number, number][][]>([])

/**
 * Factory for creating a target store
 * @returns target store with method subscribe, add and remove
 */
function createTargetLocations() {
	const { subscribe, update } = writable<Draggable[]>([]);

	return {
		subscribe,
		add: (newTarget: Draggable) => update((targets) => [...targets, newTarget]), // append new target to the end of the array
		remove: (index: number) => update((targets) => targets.filter((_, i) => i !== index)) // remove target at index
	};
}
export const targetLocations = createTargetLocations();

// GLTF STORE helper functions

/***
 * Converts a radian to degrees
 * @param radian - radian in range [-inf, inf]
 */
function radToDeg(radians: number) {
	return radians * (180 / Math.PI);
}

function adjustAlititude(altAndgrad: AltitudeGradientPair) {
	let altitude = altAndgrad.altitude;

	// Take a small modifier that will increase the altitude by a fraction of the largest absolute gradient
	altitude += 0.02 * altAndgrad.gradient.map((g) => Math.abs(g)).reduce((a, b) => Math.max(a, b));

	// Increment by 1 to prevent the altitude from being under the model
	return altitude + 1;
}

export function gltfStringToUrl(gltf: string): string {
	const gltfBlob = new Blob([gltf], { type: 'application/json' });
	const gltfUrl = URL.createObjectURL(gltfBlob);
	return gltfUrl;
}

/**
 *  Factory for creating a gltf store
 * @returns target store with method subscribe, add and remove
 */
function createGltfStore() {
	const { subscribe, set } = writable<Model>();
	let api: wasm.ModelConstructionApi;
	let paperSize: { width: number; height: number };

	let isSetup = false;
	let model: Model;

	return {
		subscribe,
		set,
		setup: async (curveTree: CurveTree) => {
			// if wasm is not yet setup, do so
			if (!isSetup) {
				await init();
				isSetup = true;
			}

			paperSize = curveTree.size;

			// Create a wasm tree out of openCV contour tree
			const tree = new wasm.OpenCVTree({
				pixels_per_curve: curveTree.curves,
				parent_relations: curveTree.hierarchy
			});

			// Set api and parameters
			api = new wasm.ModelConstructionApi();

			let api_settings = new ApiSettings(
/*				 OpenCV tree */ tree,
/*						Rows */ 45,
/*					 Columns */ 45,
/*					   Width */ curveTree.size.width,
/*					  Height */ curveTree.size.height,
/*	  Curve Point Separation */ 5,
/*		  		SVC Distance */ 50,
/*	Catmull Clark Iterations */ 1,
/*			Lava Path Length */ 20,
/*		   Lava Path Forking */ 0.2,
/*		Smoothing Operations */ [
					new wasm.SmoothingOperationApplySmoothToLayer(0, 0.9, 5, 1, false),
					new wasm.SmoothingOperationApplySmoothToMiddleLayers(0.7, 3, 5, false),
					new wasm.SmoothingOperationIncreaseAltitudeForMountainTops(2, false),
					new wasm.SmoothingOperationApplySmoothToMountainTops(1, 8, 1, false)
				]
			);

			api_settings.apply_to_api(api);

			
		},
		build: () => {
			// Call the wasm api to build the model
			model = api.build().to_js() as Model;
			model.gltf_url = gltfStringToUrl(model.gltf);

			//set lava path
			lavapaths.set(model.lava_paths);
			console.log(model.lava_paths);

			// set the gltf store to the gltf string
			set(model);
		},
		getAlitituteAndGradient: (marker: Draggable): AltitudeGradientPair => {
			if (!api) return { x: 0, y: 0, altitude: 0, gradient: [0, 0, 0] };

			//
			const adjustedX = (marker.x / paperSize.width) * 100;
			const adjustedY = (marker.y / paperSize.height) * 100;

			// ask api to get altitude and gradient for a certain point
			const altitudeGradientPair = api
				.get_altitude_and_gradient_for_point(adjustedX, adjustedY)
				.to_js() as AltitudeGradientPair;

			// Get radians from rust however Aframe expects degrees
			altitudeGradientPair.gradient[0] = radToDeg(altitudeGradientPair.gradient[0]);
			altitudeGradientPair.gradient[1] = radToDeg(altitudeGradientPair.gradient[1]);
			altitudeGradientPair.gradient[2] = radToDeg(altitudeGradientPair.gradient[2]);

			// Apply modifier to altitude
			altitudeGradientPair.altitude = adjustAlititude(altitudeGradientPair);

			return altitudeGradientPair;
		}
	};
}
export const gltfStore = createGltfStore();
