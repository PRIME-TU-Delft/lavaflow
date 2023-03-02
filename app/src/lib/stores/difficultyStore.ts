/**
 * Svelte stores, store reactive global data for more info:
 * https://svelte.dev/docs#run-time-svelte-store
 *
 * This stores gltf data
 */

import { writable } from 'svelte/store';
import {
	DifficultyLevel,
	difficulty_modes,
	type DifficultyLevelString
} from '$lib/data/difficultyModes';
import { browser } from '$app/environment';

const CACHE_KEY = 'difficulty';

/**
 *  Factory for creating a difficultyStore
 * @returns target store with method subscribe, add and remove
 */
function createDifficultyStore() {
	const stored = browser ? localStorage?.getItem(CACHE_KEY) || '{}' : '{}';
	const level = JSON.parse(stored) as DifficultyLevelString;
	const difficultyLevel = DifficultyLevel.new(level);

	const { subscribe, set, update } = writable(difficultyLevel ?? difficulty_modes[0]);

	return {
		subscribe,
		set: (level: DifficultyLevel) => {
			set(level);
		},
		update: () => update((l) => l),
		clear: () => {
			set(difficulty_modes[0]);
		}
	};
}
export const difficultyStore = createDifficultyStore();

difficultyStore.subscribe((value) => {
	if (browser) localStorage?.setItem(CACHE_KEY, JSON.stringify(value));
});
