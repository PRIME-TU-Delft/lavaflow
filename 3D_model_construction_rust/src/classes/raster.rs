//
// Class: Raster
//

#[derive(Debug)]
pub struct Raster {
	pub rows: usize,
	pub columns: usize,
	pub row_height: f64,
	pub column_width: f64,
	pub altitudes: Vec<Vec<Option<f64>>>
}


impl Raster {
	// Constructor
	pub fn new(width: f64, height: f64, rows: usize, columns: usize) -> Self {
		// Initialize the 2d array of altitudes
		let mut altitudes_init: Vec<Vec<f64>> = Vec::new();

		// Set all the altitude values to zero
		for _i in 0..rows {
			let mut current_col: Vec<f64> = Vec::new();
			for _j in 0..columns {
				current_col.push(None());
			}
			altitudes_init.push(current_col);
		}

		Self {
			rows,
			columns,
			row_height: height / (rows as f64),
			column_width: width / (columns as f64),
			altitudes: altitudes_init
		}
	}

	pub fn set(&mut self, row: usize, col: usize, val: f64) {
		self.altitudes[row][col] = Some(val);
	}

	pub fn get(&self, row: usize, col: usize) -> &Option<f64> {
		&self.altitudes[row][col]
	}
}
