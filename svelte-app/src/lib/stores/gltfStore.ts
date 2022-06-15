/**
 * Svelte stores, store reactive global data for more info:
 * https://svelte.dev/docs#run-time-svelte-store
 *
 * This stores gltf data
 */

import { writable } from 'svelte/store';
import type { CurveTree } from '$lib/stores/contourLineStore';
import Draggable from '$lib/data/draggable';
import ApiSettings from '$lib/data/apiSettings';

import init, * as wasm from 'wasm';

type Vec2 = [number, number];
type Vec3 = [number, number, number];
export interface Model {
	gltf: string;
	gltf_url: string;
	lava_paths: Vec3[][];
	craters: Vec2[];
}

export interface AltitudeGradientPair {
	x: number;
	y: number;
	altitude: number;
	gradient: Vec3;
}

/**
 * Factory for creating a target of crater location store
 * @returns location store with method subscribe, add and remove
 */
// Get cache from local storage
function convertToTargets(cachedTargets: Draggable[]): Draggable[] {
	return cachedTargets.map((target) => {
		const dragItem = new Draggable(target.x, target.y, target.size, target.offsetX, target.offsetY);

		dragItem.enableSelection();
		dragItem.deselect();

		return dragItem;
	});
}

function createLocationStore<T>(storageIndex: string, convertToLocation: (locations: T[]) => T[]) {
	function getCache() {
		// Get initial locations from the local storage
		if (typeof window !== 'undefined') {
			const cachedLocations = localStorage?.getItem(storageIndex);

			try {
				if (cachedLocations) {
					const parsedLocations = JSON.parse(cachedLocations);
					return convertToLocation(parsedLocations);
				}
			} catch (_) {
				return [];
			}
		}
		return [];
	}

	// Set cache to local storage
	function setCache(locations: T[]) {
		if (typeof window !== 'undefined') {
			localStorage.setItem(storageIndex, JSON.stringify(locations));
		}
	}

	const { subscribe, update, set } = writable<T[]>(getCache());

	return {
		subscribe,
		clear: () => set([]),
		set: (locations: T[]) => {
			set(locations);
			setCache(locations);
		},
		add: (location: T) =>
			update((oldLocations) => {
				// append new location to the end of the array
				const newLocations = [...oldLocations, location];
				setCache(newLocations);
				return newLocations;
			}),
		remove: (index: number) =>
			update((oldLocations) => {
				// remove location at index
				const newLocations = oldLocations.filter((_, i) => i !== index);
				setCache(newLocations);
				return newLocations;
			})
	};
}

export const targetLocations = createLocationStore<Draggable>('targets', convertToTargets);
export const craterLocations = createLocationStore<Vec2>('craters', (ls) => ls);

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

			const api_settings = new ApiSettings(
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
		build: (curveTree: CurveTree) => {
			// Call the wasm api to build the model
			model = api.build().to_js() as Model;
			model.gltf_url = gltfStringToUrl(model.gltf);

			// (re-)set the crater locations
			craterLocations.set(model.craters.map(c => [
				(c[0] * curveTree.size.width) / 100,
				(c[1] * curveTree.size.width) / 100
			]));

			// set the gltf store to the gltf string
			set(model);
		},
		getAlitituteAndGradient: (marker: Draggable, noAdjustAxis = false): AltitudeGradientPair => {
			if (!api) return { x: 0, y: 0, altitude: 0, gradient: [0, 0, 0] };

			let [adjustedX, adjustedY] = [marker.x, marker.y];

			if (!noAdjustAxis) {
				// Rust creates a 100*100 grid, so we need to convert the marker coordinates to this grid
				adjustedX = (marker.x / paperSize.width) * 100;
				adjustedY = (marker.y / paperSize.height) * 100;
			}

			// ask api to get altitude and gradient for a certain point
			const altitudeGradientPair = api
				.get_altitude_and_gradient_for_point(adjustedX, adjustedY)
				.to_js() as AltitudeGradientPair;

			// Get radians from rust however Aframe expects degrees
			altitudeGradientPair.gradient.map((rad) => radToDeg(rad));

			// Apply modifier to altitude
			altitudeGradientPair.altitude = adjustAlititude(altitudeGradientPair);

			return altitudeGradientPair;
		}
	};
}
export const gltfStore = createGltfStore();
