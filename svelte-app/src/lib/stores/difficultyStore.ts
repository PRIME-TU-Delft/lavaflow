/**
 * Svelte stores, store reactive global data for more info:
 * https://svelte.dev/docs#run-time-svelte-store
 *
 * This stores gltf data
 */

import { writable } from 'svelte/store';
import { DifficultyLevel, difficulty_modes } from '$lib/data/difficultyModes';

// Lava distance functions for every difficulty level

/**
 *  Factory for creating a difficultyStore
 * @returns target store with method subscribe, add and remove
 */
function createDifficultyStore() {
	const { subscribe, set } = writable<DifficultyLevel>();

	return {
		subscribe,
		set,
		setup: () => set(difficulty_modes[0])
	};
}
export const difficultyStore = createDifficultyStore();
