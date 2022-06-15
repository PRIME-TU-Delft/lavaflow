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

    let difficulty_level: DifficultyLevel;

	return {
		subscribe,
		set,
		setup: async () => {
            difficulty_level = difficulty_modes[0].copy();
		},
		build: () => {
			set(difficulty_level);
		},
        get_options() {
            return difficulty_modes.map((l) => l.name);
        },
        set_difficulty_to(level: string) {
            for (let m of difficulty_modes) {
                if (m.name == level) {
                    difficulty_level.name = m.name;
                    difficulty_level.description = m.description;
                    difficulty_level.max_lava_distance = m.max_lava_distance;
                    difficulty_level.lava_forking = m.lava_forking;
                    difficulty_level.min_steam_turbines = m.min_steam_turbines;
                    difficulty_level.min_steam_turbine_separation = m.min_steam_turbine_separation;
                    set(difficulty_level);
                }
            }
        }
	};
}
export const difficultyStore = createDifficultyStore();