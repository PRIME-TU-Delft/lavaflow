//
// Class: Raster
//

#[derive(Debug)]
pub struct Raster {
	pub rows: usize,
	pub columns: usize,
	pub row_height: f32,
	pub column_width: f32,
	pub altitudes: Vec<Vec<Option<f32>>>,
}


impl Raster {
	
	/// Construtor for a Raster
	///
	/// # Arguments
	///
	/// * `width` - total width of raster
	/// * `height` - total height of raster
	/// * `rows` - desired number of rows
	/// * `columns` - desired number of columns
	pub fn new(width: f32, height: f32, rows: usize, columns: usize) -> Self {
		// Initialize the 2d array of altitudes
		let mut altitudes_init: Vec<Vec<Option<f32>>> = Vec::new();

		// Set all the altitude values to zero
		for _ in 0..rows {
			let mut current_col: Vec<Option<f32>> = Vec::new();
			for _ in 0..columns {
				current_col.push(None);
			}
			altitudes_init.push(current_col);
		}

		Self {
			rows,
			columns,
			row_height: height / (rows as f32),
			column_width: width / (columns as f32),
			altitudes: altitudes_init,
		}
	}

	pub fn set(&mut self, row: usize, col: usize, val: f32) {
		self.altitudes[row][col] = Some(val);
	}

	pub fn get(&self, row: usize, col: usize) -> &Option<f32> {
		&self.altitudes[row][col]
	}
}
