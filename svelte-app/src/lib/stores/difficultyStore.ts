/**
 * Svelte stores, store reactive global data for more info:
 * https://svelte.dev/docs#run-time-svelte-store
 *
 * This stores gltf data
 */

import { writable } from 'svelte/store';

function map(val: number, from_min: number, from_max: number, to_min: number, to_max: number): number {
    return to_min + ((val - from_min) / (from_max - from_min) * (to_max - to_min));
}

export class DifficultyLevel {
    name: string;
    max_lava_distance: number;
    lava_forking: number;
    min_steam_turbines: number;
    max_steam_turbines: number = 10;
    min_steam_turbine_separation: number;

    options: string[] = ["Tutorial", "Easy", "Medium", "Hard", "Impossible"];

    constructor(
        name: string,
        max_lava_distance: number,
        lava_forking: number,
        min_steam_turbines: number,
        min_steam_turbine_separation: number,
    ) {
        this.name = name;
        this.max_lava_distance = max_lava_distance;
        this.lava_forking = lava_forking;
        this.min_steam_turbines = min_steam_turbines;
        this.min_steam_turbine_separation = min_steam_turbine_separation;
    }

    computePointsForLavaDistance(dist: number): number {
        return map(Math.max(0, -dist + this.max_lava_distance), 0, this.max_lava_distance, 0, 10);
    }
}

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
            difficulty_level = new DifficultyLevel(
                "Tutorial",
                200,
                0.8,
                2,
                50
            );
		},
		build: () => {
			set(difficulty_level);
		},
        set_difficulty_to(level: string) {
            switch (level) {
                case "Tutorial":
                    difficulty_level.max_lava_distance = 200;
                    difficulty_level.lava_forking = 0.8;
                    difficulty_level.min_steam_turbines = 2;
                    difficulty_level.min_steam_turbine_separation = 50;
                    break;
                case "Easy":
                    difficulty_level.max_lava_distance = 200;
                    difficulty_level.lava_forking = 0.8;
                    difficulty_level.min_steam_turbines = 3;
                    difficulty_level.min_steam_turbine_separation = 50;
                    break;
                case "Medium":
                    difficulty_level.max_lava_distance = 150;
                    difficulty_level.lava_forking = 0.6;
                    difficulty_level.min_steam_turbines = 5;
                    difficulty_level.min_steam_turbine_separation = 100;
                    break;
                case "Hard":
                    difficulty_level.max_lava_distance = 100;
                    difficulty_level.lava_forking = 0.6;
                    difficulty_level.min_steam_turbines = 5;
                    difficulty_level.min_steam_turbine_separation = 150;
                    break;
                case "Impossible":
                    difficulty_level.max_lava_distance = 50;
                    difficulty_level.lava_forking = 0.4;
                    difficulty_level.min_steam_turbines = 10;
                    difficulty_level.min_steam_turbine_separation = 200;
                    break;
            }

            difficulty_level.name = level;

            set(difficulty_level);
        }
	};
}
export const difficultyStore = createDifficultyStore();