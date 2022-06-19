use crate::{api::ModelConstructionApi, objects::point::Point};

pub fn map(val: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
	to_min + ((val - from_min) / (from_max - from_min) * (to_max - to_min))
}

pub fn map_color(val: f32, min: f32, max: f32, color_1: (f32, f32, f32), color_2: (f32, f32, f32)) -> (f32, f32, f32) {
	(
		map(val, min, max, color_1.0, color_2.0),
		map(val, min, max, color_1.1, color_2.1),
		map(val, min, max, color_1.2, color_2.2),
	)
}

impl ModelConstructionApi {
	pub fn color_for_altitude(&self, alt_range: (f32, f32), alt: f32, p: &Point, lava_craters: &[(f32, f32)], color_crater_center: (f32, f32, f32), color_crater_outside: (f32, f32, f32)) -> [f32; 3] {
		let (min_alt, max_alt) = alt_range;

		// Initialisation: tuples for every color that we'd like to use
		let color_rock_dark = (map(30.0, 0.0, 255.0, 0.0, 1.0), map(15.0, 0.0, 255.0, 0.0, 1.0), map(25.0, 0.0, 255.0, 0.0, 1.0));

		let _color_rock_light = (map(45.0, 0.0, 255.0, 0.0, 1.0), map(28.0, 0.0, 255.0, 0.0, 1.0), map(34.0, 0.0, 255.0, 0.0, 1.0));

		let _color_healthy_grass = (map(42.0, 0.0, 255.0, 0.0, 1.0), map(48.0, 0.0, 255.0, 0.0, 1.0), map(10.0, 0.0, 255.0, 0.0, 1.0));

		let color_dry_grass = (map(147.0, 0.0, 255.0, 0.0, 1.0), map(120.0, 0.0, 255.0, 0.0, 1.0), map(0.0, 0.0, 255.0, 0.0, 1.0));

		// Compute the closest distance between the point p and one of the craters
		let mut closest_dist_sqr = f32::MAX;
		for (cx, cy) in lava_craters.iter() {
			let dx = p.x - cx;
			let dy = p.y - cy;
			let dist_sqr = dx * dx + dy * dy;
			if dist_sqr < closest_dist_sqr {
				closest_dist_sqr = dist_sqr;
			}
		}

		// Compute the closest distance to a lava path
		// let mut closest_dist_sqr_lava_path = f32::MAX;
		// for lava_path in lava_path_triples {
		// 	for (lpx, lpy, lpz) in lava_path {
		// 		let dx = p.x - lpx;
		// 		let dy = p.y - lpy;
		// 		let dz = alt - lpz;
		// 		let dist_sqr = dx * dx + dy * dy + dz * dz;
		// 		if dist_sqr < closest_dist_sqr_lava_path {
		// 			closest_dist_sqr_lava_path = dist_sqr;
		// 		}
		// 	}
		// }

		// If this distance is smaller than the threshold, make this color be a lava-crater
		let result: (f32, f32, f32) = if closest_dist_sqr <= 1.8
		/*|| closest_dist_sqr_lava_path <= 0.1 */
		{
			map_color(closest_dist_sqr, 0.0, 1.8, color_crater_center, color_crater_outside)
		} else {
			// 1. Map the altitude so it becomes a value between [0, 1]
			let current_altitude = map(alt, min_alt, max_alt, 0.0, 1.0);

			// Make a variable that holds the resulting color
			map_color(current_altitude, 0.0, 1.0, color_rock_dark, color_dry_grass)
		};

		[result.0, result.1, result.2]
	}

	pub fn points_for_turbine(turbine: (f32, f32), lava_path_triples: &[Vec<(f32, f32, f32)>], max_lava_distance: f32, max_points_per_turbine: usize) -> usize {
		// Compute distance to closest lava-path segment
		let mut lava_distance = (f32::MAX, f32::MAX);
		for lava_path in lava_path_triples {
			let mut current_lava_distance = f32::MAX;

			for (lpx, lpy, _lpz) in lava_path {
				let dx = turbine.0 - lpx;
				let dy = turbine.1 - lpy;
				let dist = f32::sqrt(dx * dx + dy * dy);
				if dist < current_lava_distance {
					current_lava_distance = dist;
				}
			}

			if current_lava_distance < lava_distance.0 {
				// Shift both over, remove the second one
				lava_distance.1 = lava_distance.0;
				lava_distance.0 = current_lava_distance;
			} else if current_lava_distance < lava_distance.1 {
				// Replace the second one
				lava_distance.1 = current_lava_distance;
			}
		}

		if lava_distance.0 <= max_lava_distance && lava_distance.1 <= max_lava_distance {
			let aggregate_distance = (lava_distance.0 + lava_distance.1) / 4.0;
			map(aggregate_distance, 0.0, max_lava_distance, max_points_per_turbine as f32, 0.0) as usize
		} else if lava_distance.0 <= max_lava_distance {
			map(lava_distance.0, 0.0, max_lava_distance, max_points_per_turbine as f32, 0.0) as usize
		} else if lava_distance.1 <= max_lava_distance {
			map(lava_distance.1, 0.0, max_lava_distance, max_points_per_turbine as f32, 0.0) as usize
		} else {
			0
		}
	}
}
