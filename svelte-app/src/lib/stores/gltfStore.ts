/**
 * Svelte stores, store reactive global data for more info:
 * https://svelte.dev/docs#run-time-svelte-store
 *
 * This stores gltf data
 */

import { writable } from 'svelte/store';
import type Draggable from '$lib/data/draggable';
import type { CurveTree } from '$lib/stores/contourLineStore';

import init, * as wasm from 'wasm';
import { hc_curves, hc_hierarchy } from '$lib/data/hardCoded';

/**
 *  Factory for creating a target store
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

export interface Model {
	gltf: string;
	lavapath: [number, number, number][][];
}

/**
 *  Factory for creating a gltf store
 * @returns target store with method subscribe, add and remove
 */
function createGltfStore() {
	const { subscribe, set } = writable<string>('/output20.gltf');
	let api: wasm.ModelConstructionApi;
	let isSetup = false;

	return {
		subscribe,
		setup: async (curveTree: CurveTree) => {
			if (!isSetup) {
				await init();
				isSetup = true;
			}

			// TODO: Remove hardcoded data
			const tree = new wasm.OpenCVTree({
				pixels_per_curve: hc_curves,
				parent_relations: hc_hierarchy
			});

			api = new wasm.ModelConstructionApi();
			api.base(tree);
		},
		build: () => {
			const model = api.build().to_json() as Model;
			console.log(model);
			set(model.gltf);
		}
	};
}
export const gltfStore = createGltfStore();
