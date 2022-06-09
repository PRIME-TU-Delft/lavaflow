/**
 * Svelte stores, store reactive global data for more info:
 * https://svelte.dev/docs#run-time-svelte-store
 *
 * This stores gltf data
 */

import { writable } from 'svelte/store';
import Draggable from '$lib/data/draggable';
import type { CurveTree } from '$lib/stores/contourLineStore';

import init, * as wasm from 'wasm';

function radToDeg(radians: number) {
  var pi = Math.PI;
  return radians * (180/pi);
}
/**
 * Factory for creating a target store
 * @returns target store with method subscribe, add and remove
 */
function createTargetLocations() {
	const { subscribe, update } = writable<Draggable[]>([new Draggable(500, 500, 0)]);

	return {
		subscribe,
		add: (newTarget: Draggable) => update((targets) => [...targets, newTarget]), // append new target to the end of the array
		remove: (index: number) => update((targets) => targets.filter((_, i) => i !== index)) // remove target at index
	};
}
export const targetLocations = createTargetLocations();

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

			// Create a wasm tree out of openCV contour tree
			const tree = new wasm.OpenCVTree({
				pixels_per_curve: curveTree.curves,
				parent_relations: curveTree.hierarchy
			});

			const svc_dist = curveTree.size.width / 25 / 3;

			// Set api and parameters
			api = new wasm.ModelConstructionApi();
			api.base(tree, 10);
			api.set_basic_parameters(25, 25, curveTree.size.width, curveTree.size.height);
			api.set_svc_parameters(svc_dist);
			api.correct_for_altitude_constraints_to_all_layers();
			api.apply_smooth_to_layer(0, 0.7, 4, 10, false);
			api.increase_altitude_for_mountain_tops(0.3, false);
			api.apply_smooth_to_mountain_tops(0.2, 2, 5, false);
			api.set_catmull_clark_parameters(0);
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
		getAlitituteAndGradient: (x: number, y: number): AltitudeGradientPair => {
			if (!api) return {x: 0, y: 0, altitude: 0, gradient: [0, 0, 0]}

			// ask api to get altitude and gradient for a certain point
			const altitudeGradientPair = api
				.get_altitude_and_gradient_for_point(x, y)
				.to_js() as AltitudeGradientPair;

			altitudeGradientPair.gradient[0] = radToDeg(altitudeGradientPair.gradient[0]);
			altitudeGradientPair.gradient[1] = radToDeg(altitudeGradientPair.gradient[1]);
			altitudeGradientPair.gradient[2] = radToDeg(altitudeGradientPair.gradient[2]);

			console.log(altitudeGradientPair);
			return altitudeGradientPair;
		},
		altitude_adjusted_to_gradient: (agp: AltitudeGradientPair): number => {
			return agp.altitude + Math.max(
				Math.abs(agp.gradient[0]),
				Math.abs(agp.gradient[1]),
				Math.abs(agp.gradient[2])
			)*0.1;
		}
	};
}
export const gltfStore = createGltfStore();
