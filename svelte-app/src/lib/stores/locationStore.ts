/**
 * Svelte stores, store reactive global data for more info:
 * https://svelte.dev/docs#run-time-svelte-store
 *
 * This stores location data 😈
 */

import { browser } from '$app/environment';
import { writable } from 'svelte/store';

export interface Turbine {
	x: number;
	y: number;
	index: string;
	hasConfirmOpen: boolean;
}

const CACHE_KEY_TURBINES = 'turbines';
const CACHE_KEY_CRATER = 'crater';

function createLocationStore() {
	const stored = browser ? localStorage?.getItem(CACHE_KEY_TURBINES) || '[]' : '[]';
	const turbines = JSON.parse(stored) as Turbine[];
	const hasTurbines = turbines?.length > 0;
	console.log('hasTurbines', hasTurbines, turbines);

	const { subscribe, set, update } = writable<Turbine[]>(hasTurbines ? turbines : []);

	return {
		subscribe,
		add: (turbine: Turbine) => {
			update((ts) => [...ts, turbine]);
		},
		remove: (index: string) => {
			update((ts) => ts.filter((t) => t.index !== index));
		},
		toggleOpen: (index: string) => {
			update((ts) =>
				ts.map((t) => {
					if (t.index === index) {
						t.hasConfirmOpen = !t.hasConfirmOpen;
					}
					return t;
				})
			);
		},
		closeAll: () => {
			update((ts) => {
				if (!ts.length) return ts;

				return ts.map((t) => {
					t.hasConfirmOpen = false;
					return t;
				});
			});
		},
		clear: () => {
			console.log('clear');
			set([]);
		}
	};
}

type Vec2 = [number, number];
function createCraterStore() {
	const stored = browser ? localStorage?.getItem(CACHE_KEY_CRATER) || '{}' : '{}';
	const crater = JSON.parse(stored) as Vec2;

	return writable<Vec2>(crater ?? [0, 0]);
}

export const turbineLocations = createLocationStore();
export const craterLocation = createCraterStore();

turbineLocations.subscribe((value) => {
	if (browser) localStorage?.setItem(CACHE_KEY_TURBINES, JSON.stringify(value));
});
craterLocation.subscribe((value) => {
	if (browser) localStorage?.setItem(CACHE_KEY_CRATER, JSON.stringify(value));
});
