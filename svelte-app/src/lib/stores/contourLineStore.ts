/**
 * Svelte stores, store reactive global data for more info:
 * https://svelte.dev/docs#run-time-svelte-store
 *
 * This stores contour line data
 */

import { writable } from 'svelte/store';

export type CurveTree = {
	curves: [number, number][][];
	hierarchy: number[];
	size: { width: number; height: number };
};

function contourLineStore() {
	const { subscribe, set } = writable<CurveTree>();

	return {
		subscribe,
		set: (contours: CurveTree) => {
			console.log('set contours in store', contours);
			set(contours);
		}
	};
}

export const contourLines = contourLineStore();
