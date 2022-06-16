function map(
	val: number,
	from_min: number,
	from_max: number,
	to_min: number,
	to_max: number
): number {
	return to_min + ((val - from_min) / (from_max - from_min)) * (to_max - to_min);
}

export class DifficultyLevel {
	name: string;
	description: string;
	max_lava_distance: number;
	lava_forking: number;
	min_steam_turbines: number;
	max_steam_turbines: number = 10;
	min_steam_turbine_separation: number;
	min_crater_distance: number;

	constructor(
		name: string,
		description: string,
		max_lava_distance: number,
		lava_forking: number,
		min_steam_turbines: number,
		min_steam_turbine_separation: number,
		min_crater_distance: number
	) {
		this.name = name;
		this.description = description;
		this.max_lava_distance = max_lava_distance;
		this.lava_forking = lava_forking;
		this.min_steam_turbines = min_steam_turbines;
		this.min_steam_turbine_separation = min_steam_turbine_separation;
		this.min_crater_distance = min_crater_distance;
	}

	computePointsForLavaDistance(dist: number): number {
		return map(Math.max(0, -dist + this.max_lava_distance), 0, this.max_lava_distance, 0, 10);
	}
}

export const difficulty_modes = [
	new DifficultyLevel(
		'Tutorial',
		'This is the easiest level of difficulty, which is meant to get you familiarised with the game.',
		200,
		0.2,
		2,
		50,
		75
	),
	new DifficultyLevel('Easy', 'Take it easy.', 200, 0.2, 3, 50, 75),
	new DifficultyLevel(
		'Medium',
		'There is less lava and it must reach closer to the steam turbines. Steam turbines should be placed further apart from each other.',
		150,
		0.1,
		5,
		100,
		75
	),
	new DifficultyLevel(
		'Hard',
		"The lava must reach even closer to the steam turbines and you'll have to place them even further apart.",
		100,
		0.1,
		5,
		150,
		75
	),
	new DifficultyLevel('Impossible', 'Only Dennis can solve this level.', 50, 0.1, 10, 200, 200)
	// Custom difficulty: Add your own difficulty levels here, they'll automatically be imcorporated throughout the app.
];
