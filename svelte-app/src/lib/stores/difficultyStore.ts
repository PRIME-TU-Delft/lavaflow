/**
 * Svelte stores, store reactive global data for more info:
 * https://svelte.dev/docs#run-time-svelte-store
 *
 * This stores gltf data
 */

import { writable } from 'svelte/store';
import { type DifficultyLevel, difficulty_modes } from '$lib/data/difficultyModes';

// Lava distance functions for every difficulty level

/**
 *  Factory for creating a difficultyStore
 * @returns target store with method subscribe, add and remove
 */
function createDifficultyStore() {
	const { subscribe, set } = writable<DifficultyLevel>();

	if (typeof window !== 'undefined') {
		const localDifficultyLevels = localStorage?.getItem('level');

		if (localDifficultyLevels) {
			try {
				set(JSON.parse(localDifficultyLevels) as DifficultyLevel);
			} catch (_) {
				set(difficulty_modes[0]);
			}
		} else {
			set(difficulty_modes[0]);
		}
	}

	return {
		subscribe,
		set: (level: DifficultyLevel) => {
			cacheDifficultyStore(level);
			set(level);
		},
		clear: () => {
			set(difficulty_modes[0]);
		}
	};
}
export const difficultyStore = createDifficultyStore();
export function cacheDifficultyStore(level: DifficultyLevel) {
	if (typeof window !== 'undefined') {
		localStorage.setItem('level', JSON.stringify(level));
	}
}
