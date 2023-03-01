/**
 * Svelte stores, store reactive global data for more info:
 * https://svelte.dev/docs#run-time-svelte-store
 *
 * This stores the current location of the draggable turbine markers & the crater position
 */

import { writable } from 'svelte/store';
import type Draggable from '$lib/data/draggable';

const CACHE_KEY = 'targets';

const stored = localStorage.getItem(CACHE_KEY) || '[]';
const draggables = JSON.parse(stored) as Draggable[];

function createDifficultyStore() {
	const { subscribe, set, update } = writable<Draggable[]>(draggables?.map((d) => d.clone()) ?? []);

	return {
		subscribe,
		set: (locations: Draggable[]) => {
			set(locations);
		},
		add: (location: Draggable) => update((oldLocations) => [...oldLocations, location]),
		clear: () => set([]),
		remove: (index: number) => update((oldLocations) => oldLocations.filter((_, i) => i !== index))
	};
}

export const targetLocations = createDifficultyStore();

// Set cache to local storage
targetLocations.subscribe((value) => localStorage.setItem(CACHE_KEY, JSON.stringify(value)));
