/**
 * Svelte stores, store reactive global data for more info:
 * https://svelte.dev/docs#run-time-svelte-store
 *
 * This stores gltf data
 */

import { writable } from 'svelte/store';
import type { CurveTree } from '$lib/stores/contourLineStore';
import type Draggable from '$lib/data/draggable';

import init, * as wasm from 'wasm';

export interface Model {
	gltf: string;
	lavapath: [number, number, number][][];
}

export interface AltitudeGradientPair {
	x: number;
	y: number;
	altitude: number;
	gradient: [number, number, number];
}

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
	const { subscribe, set } = writable<string>('');
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
			api.base(tree, 5);
			api.set_basic_parameters(50, 50, curveTree.size.width, curveTree.size.height);
			api.set_svc_parameters(50);
			api.correct_for_altitude_constraints_to_all_layers();
			api.apply_smooth_to_layer(0, 0.7, 4, 10, false);
			api.apply_smooth_to_middle_layers(0.7, 4, 10, false);
			api.increase_altitude_for_mountain_tops(1, false);
			api.apply_smooth_to_mountain_tops(0.3, 5, 10, false);
			api.set_catmull_clark_parameters(1);
		},
		build: () => {
			// Call the wasm api to build the model
			console.log('before build');
			model = api.build().to_js() as Model;
			const gltfUrl = gltfStringToUrl(model.gltf);

			// set the gltf store to the gltf string
			set(gltfUrl);
			console.log(gltfUrl);
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
