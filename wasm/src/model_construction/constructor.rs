use super::point::Point;
use super::{level_curves::LevelCurveSet, raster::Raster};

#[derive(Debug)]
pub struct ModelConstructor<'a> {
	contour_margin: f32,
	level_curve_map: &'a LevelCurveSet,
	is_svc: Vec<Vec<bool>>,
	raster: &'a mut Raster,
}

// TODO: implement once model is working
fn local_tin(_p: Vec<f32>) -> f32 {
	todo!()
}

impl<'a> ModelConstructor<'a> {
	pub fn new(raster: &'a mut Raster, contour_margin: f32, level_curve_map: &'a LevelCurveSet) -> ModelConstructor<'a> {
		let x = raster.columns;
		let y = raster.rows;
		let is_svc = vec![vec![false; x]; y];
		ModelConstructor {
			contour_margin,
			level_curve_map,
			is_svc,
			raster,
		}
	}
	/// Using a given set of level curves, determines the heights for each cell in a given raster. *See README for further implementation details*.
	///
	/// # Arguments
	///
	/// * `raster` - empty raster for which heights are to be calculated
	/// * `contour_margin` - margin that defines when a point is considered 'on' a contour line NOTE: must be above max(raster height, column width) so long as `local_tin`() is not implemented
	/// * `level_curve_map` - set of level curves used to determine heights of each point
	///
	///
	pub fn construct(&mut self) {
		let x = self.raster.columns;
		let y = self.raster.rows;

		// Set the edges of the raster to zero
		self.set_raster_edges_to_zero();

		for i in 0..x {
			for j in 0..y {
				if self.check_svc(i, j) {
					// if a point is an svc but height is not yet known it has to be interpolated using local triangulated irregular network
					if self.raster.altitudes[i][j].is_none() {
						// local_tin(cellCentre)
					}
				}
			}
		}
		// calculate heights of all no value cells
		self.calc_heights_nvc()
	}

	// Function: initialize
	fn set_raster_edges_to_zero(&mut self) {
		// Initialize all the outer-most raster values to zero
		// (According to requirement that the edges of the mountain must have level 0)
		for i in 0..self.raster.rows {
			for j in 0..self.raster.columns {
				// If this box is on the outer edge
				if i == 0 || j == 0 || i == self.raster.rows - 1 || j == self.raster.columns - 1 {
					// Set its value to zero
					self.raster.set(i, j, 0.0);

					// Set this box to an svc-box
					self.is_svc[i][j] = true;
				}
			}
		}
	}

	// Function: Calculate Height of NVCs
	fn calc_heights_nvc(&mut self) {
		for row in 0..self.raster.rows {
			for col in 0..self.raster.columns {
				if self.raster.get(row, col).is_none() {
					let neighbours: Vec<(f32, f32)> = vec![
						self.find_svc_north(row, col),
						self.find_svc_north_east(row, col),
						self.find_svc_north_west(row, col),
						self.find_svc_south(row, col),
						self.find_svc_south_east(row, col),
						self.find_svc_south_west(row, col),
						self.find_svc_east(row, col),
						self.find_svc_west(row, col),
					];

					self.raster.set(row, col, calc_inverse_weighted_average(&neighbours));
				}
			}
		}
	}

	fn check_svc(&mut self, row: usize, col: usize) -> bool {
		// TODO z point is now "0" but doesnt really exist

		// define which points are corner and center of current cell
		let corner: Point = Point {
			x: (row as f32) * self.raster.row_height,
			y: (col as f32) * self.raster.column_width,
			z: 0.0,
		};
		let center: Point = Point {
			x: ((row as f32) + 0.5) * self.raster.row_height,
			y: ((col as f32) + 0.5) * self.raster.column_width,
			z: 0.0,
		};

		// find point on a contour line closest to center of cell
		let optional: Option<&Point> = self.level_curve_map.find_closest_point_on_level_curve(&center);

		match optional {
			Some(p) =>
			// check closest point is outside of cell
			{
				if p.x < corner.x || p.x > corner.x + self.raster.row_height || p.y < corner.y || p.y > corner.y + self.raster.column_width {
					false
				}
				// check if center of cell is within distance [contour margin] of closest contour point, if it is we consider it 'exactly' on the contour line
				else if (center.x - p.x).abs() < self.contour_margin && (center.y - p.y).abs() < self.contour_margin {
					self.is_svc[row][col] = true;
					self.raster.altitudes[row][col] = Some(p.z);
					true
				}
				// if center of cell is not in distance [contour margin], its height must be interpolated
				else {
					self.is_svc[row][col] = true;
					true
				}
			}

			None => false,
		}
	}

	//
	// A set of functions for finding the nearest SVC box in directions north (west, east) and south (west, east)
	//

	// TODO: I *really* wanna refactor these 8 functions

	// Function: find SVC north
	fn find_svc_north(&self, i: usize, j: usize) -> (f32, f32) {
		let mut row = i;
		let col = j;

		while row < self.raster.rows && col < self.raster.columns {
			// If this box is svc, return its position
			if self.is_svc[row][col] {
				return (self.raster.get(row, col).expect("SVC Without Value was Found"), calc_distance_between_cells(i, j, row, col));
			}

			// Walk one step further
			row -= 1;
		}

		// If no value can be found, return (0, 0)
		(0.0, 0.0)
	}

	// Function: find SVC south
	fn find_svc_south(&self, i: usize, j: usize) -> (f32, f32) {
		let mut row = i;
		let col = j;

		while row < self.raster.rows && col < self.raster.columns {
			// If this box is svc, return its position
			if self.is_svc[row][col] {
				return (self.raster.get(row, col).expect("SVC Without Value was Found"), calc_distance_between_cells(i, j, row, col));
			}

			// Walk one step further
			row += 1;
		}

		// If no value can be found, return (0, 0)
		(0.0, 0.0)
	}

	// Function: find SVC west
	fn find_svc_west(&self, i: usize, j: usize) -> (f32, f32) {
		let row = i;
		let mut col = j;

		while row < self.raster.rows && col < self.raster.columns {
			// If this box is svc, return its position
			if self.is_svc[row][col] {
				return (self.raster.get(row, col).expect("SVC Without Value was Found"), calc_distance_between_cells(i, j, row, col));
			}

			// Walk one step further
			col -= 1;
		}

		// If no value can be found, return (0, 0)
		(0.0, 0.0)
	}

	// Function: find SVC east
	fn find_svc_east(&self, i: usize, j: usize) -> (f32, f32) {
		let row = i;
		let mut col = j;

		while row < self.raster.rows && col < self.raster.columns {
			// If this box is svc, return its position
			if self.is_svc[row][col] {
				return (self.raster.get(row, col).expect("SVC Without Value was Found"), calc_distance_between_cells(i, j, row, col));
			}

			// Walk one step further
			col += 1;
		}

		// If no value can be found, return (0, 0)
		(0.0, 0.0)
	}

	// Function: find SVC north west
	fn find_svc_north_west(&self, i: usize, j: usize) -> (f32, f32) {
		let mut row = i;
		let mut col = j;

		while row < self.raster.rows && col < self.raster.columns {
			// If this box is svc, return its position
			if self.is_svc[row][col] {
				return (self.raster.get(row, col).expect("SVC Without Value was Found"), calc_distance_between_cells(i, j, row, col));
			}

			// Walk one step further
			row -= 1;
			col -= 1;
		}

		// If no value can be found, return (0, 0)
		(0.0, 0.0)
	}

	// Function: find SVC north east
	fn find_svc_north_east(&self, i: usize, j: usize) -> (f32, f32) {
		let mut row = i;
		let mut col = j;

		while row < self.raster.rows && col < self.raster.columns {
			// If this box is svc, return its position
			if self.is_svc[row][col] {
				return (self.raster.get(row, col).expect("SVC Without Value was Found"), calc_distance_between_cells(i, j, row, col));
			}

			// Walk one step further
			row -= 1;
			col += 1;
		}

		// If no value can be found, return (0, 0)
		(0.0, 0.0)
	}

	// Function: find SVC south east
	fn find_svc_south_east(&self, i: usize, j: usize) -> (f32, f32) {
		let mut row = i;
		let mut col = j;

		while row < self.raster.rows && col < self.raster.columns {
			// If this box is svc, return its position
			if self.is_svc[row][col] {
				return (self.raster.get(row, col).expect("SVC Without Value was Found"), calc_distance_between_cells(i, j, row, col));
			}

			// Walk one step further
			row += 1;
			col += 1;
		}

		// If no value can be found, return (0, 0)
		(0.0, 0.0)
	}

	// Function: find SVC south west
	fn find_svc_south_west(&self, i: usize, j: usize) -> (f32, f32) {
		let mut row = i;
		let mut col = j;

		while row < self.raster.rows && col < self.raster.columns {
			// If this box is svc, return its position
			if self.is_svc[row][col] {
				return (self.raster.get(row, col).expect("SVC Without Value was Found"), calc_distance_between_cells(i, j, row, col));
			}

			// Walk one step further
			row += 1;
			col -= 1;
		}

		// If no value can be found, return (0, 0)
		(0.0, 0.0)
	}
}

//
// Additional functions
//

fn calc_inverse_weighted_average(weighted_values: &[(f32, f32)]) -> f32 {
	let mut res: f32 = 0.0;
	let mut sum_weight: f32 = 0.0;

	for weighted_value in weighted_values {
		res += weighted_value.0 * weighted_value.1;
		sum_weight += weighted_value.1;
	}

	res / sum_weight
}

fn calc_distance_between_cells(row0: usize, col0: usize, row1: usize, col1: usize) -> f32 {
	f32::sqrt((row1 as f32 - row0 as f32).powi(2) + (col1 as f32 - col0 as f32).powi(2))
}
