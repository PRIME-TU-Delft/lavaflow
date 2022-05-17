/// # Struct: Raster
/// This struct is used for keeping track of the altitude levels of rows and columns.
/// It is required as input for many other structs, because all construction and smoothing algorithms depend on it.
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

	/// ## Instance Method
	/// Set the altitude of a certain row and column
	pub fn set(&mut self, row: usize, col: usize, val: f32) {
		self.altitudes[row][col] = Some(val);
	}

	/// ## Instance Method
	/// Get the altitude of a certain row and column
	pub fn get(&self, row: usize, col: usize) -> &Option<f32> {
		&self.altitudes[row][col]
	}

	/// # Instance Method
	/// Get the pixels (x, y) for a given row and column.
	/// 
	/// What is meant by this: the raster has a certain pixel-width and pixel-height, as well as a number of
	/// rows and columns. This function maps the row-column to pixel-positions.
	pub fn get_pixel(&self, row: usize, col: usize) -> (f32, f32) {
		(
			(col as f32) * self.column_width,
			(row as f32) * self.row_height
		)
	}
}
