/**
 * Svelte stores, store reactive global data for more info:
 * https://svelte.dev/docs#run-time-svelte-store
 *
 * This stores contour line data
 */

import { writable } from 'svelte/store';
import { targetLocations } from '$lib/stores/locationStore.ts';

export type CurveTree = {
	curves: [number, number][][];
	hierarchy: number[];
	size: { width: number; height: number };
};

function contourLineStore() {
	const { subscribe, set } = writable<CurveTree>();
	let setup = false;

	if (!setup && typeof window !== 'undefined') {
		const localContourLines = localStorage?.getItem('contourlines');
		if (localContourLines) {
			set(JSON.parse(localContourLines) as CurveTree);
			setup = true;
		}
	}

	return {
		subscribe,
		setup: (curveTree: CurveTree) => {
			localStorage.setItem('contourlines', JSON.stringify(curveTree));

			set(curveTree);
			targetLocations.clear();
			setup = true;
		},
		clear: () => {
			setup = false;

			localStorage.removeItem('contourlines');
		}
	};
}

export const contourLines = contourLineStore();
