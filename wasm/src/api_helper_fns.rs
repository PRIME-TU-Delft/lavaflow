use crate::api::ModelConstructionApi;

impl ModelConstructionApi {
	fn map(val: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
		to_min + ((val - from_min) / (from_max - from_min) * (to_max - to_min))
	}

	fn map_color(val: f32, min: f32, max: f32, color_1: (f32, f32, f32), color_2: (f32, f32, f32)) -> (f32, f32, f32) {
		(
			ModelConstructionApi::map(val, min, max, color_1.0, color_2.0),
			ModelConstructionApi::map(val, min, max, color_1.1, color_2.1),
			ModelConstructionApi::map(val, min, max, color_1.2, color_2.2),
		)
	}

	pub fn color_for_altitude(&self, min_alt: f32, max_alt: f32, alt: f32) -> [f32; 3] {
		// Initialisation: tuples for every color that we'd like to use
		let color_rock_dark = (
			ModelConstructionApi::map(30.0, 0.0, 255.0, 0.0, 1.0),
			ModelConstructionApi::map(15.0, 0.0, 255.0, 0.0, 1.0),
			ModelConstructionApi::map(25.0, 0.0, 255.0, 0.0, 1.0),
		);

		let color_rock_light = (
			ModelConstructionApi::map(45.0, 0.0, 255.0, 0.0, 1.0),
			ModelConstructionApi::map(28.0, 0.0, 255.0, 0.0, 1.0),
			ModelConstructionApi::map(34.0, 0.0, 255.0, 0.0, 1.0),
		);

		let color_healthy_grass = (
			ModelConstructionApi::map(42.0, 0.0, 255.0, 0.0, 1.0),
			ModelConstructionApi::map(48.0, 0.0, 255.0, 0.0, 1.0),
			ModelConstructionApi::map(10.0, 0.0, 255.0, 0.0, 1.0),
		);

		let color_dry_grass = (
			ModelConstructionApi::map(147.0, 0.0, 255.0, 0.0, 1.0),
			ModelConstructionApi::map(120.0, 0.0, 255.0, 0.0, 1.0),
			ModelConstructionApi::map(0.0, 0.0, 255.0, 0.0, 1.0),
		);

		// 1. Map the altitude so it becomes a value between [0, 1]
		let current_altitude = ModelConstructionApi::map(alt, min_alt, max_alt, 0.0, 1.0);

		// Make a variable that holds the resulting color
		let result: (f32, f32, f32) = ModelConstructionApi::map_color(current_altitude, 0.0, 1.0, color_rock_dark, color_dry_grass);

		[result.0, result.1, result.2]
	}
}
