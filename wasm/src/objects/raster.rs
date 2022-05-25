use miette::{Result, miette};

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

	/// ## Instance Method
	/// Get the pixels (x, y) for a given row and column.
	///
	/// What is meant by this: the raster has a certain pixel-width and pixel-height, as well as a number of
	/// rows and columns. This function maps the row-column to pixel-positions.
	pub fn get_pixel(&self, row: usize, col: usize) -> (f32, f32) {
		((col as f32) * self.column_width, (row as f32) * self.row_height)
	}

	/// ## Instance Method
	/// Normalise the dimensions of this mountain, while maintaining all altitude-information.
	/// 
	/// ### Concrete
	/// - The number of rows and columns remains the same.
	/// - The ```width``` will become 100.0
	/// - The ```height``` will become 100.0
	/// - The altitudes will be mapped to values between 0.0 and 100.0, where the minimum altitude will always be 0.0 and the maximal altitude 100.0
	/// After having performed all these operations, the volcano will fit exactly in a cube of 100.0 x 100.0 x 100.0.
	/// 
	/// This method changes the state of the raster and returns void.
	pub fn normalise(&mut self) -> Result<()> {

		// Set the width and height to 100.0 by 100.0
		self.row_height = 100.0 / (self.rows as f32);
		self.column_width = 100.0 / (self.columns as f32);

		// If this raster does not contain any information (aka self.altitudes is empty or not defined), throw an error
		if self.altitudes.is_empty() || self.altitudes[0].is_empty() {
			return Err(miette!("Cannot normalise empty raster"));
		}

		// Compute the min and max altitude
		let mut min_max: (f32, f32) = (self.altitudes[0][0].ok_or_else(|| miette!("Cannot normalise empty raster"))?, self.altitudes[0][0].ok_or_else(|| miette!("Cannot normalise empty raster"))?);

		for row in 0..self.altitudes.len() {
			for col in 0..self.altitudes[row].len() {
				if self.altitudes[row][col].ok_or_else(|| miette!("Cannot normalise empty raster"))? < min_max.0 {
					min_max.0 = self.altitudes[row][col].ok_or_else(|| miette!("Cannot normalise empty raster"))?;
				}
				if self.altitudes[row][col].ok_or_else(|| miette!("Cannot normalise empty raster"))? > min_max.1 {
					min_max.1 = self.altitudes[row][col].ok_or_else(|| miette!("Cannot normalise empty raster"))?;
				}
			}
		}

		// Map every altitude between these values to between 0.0 and 100.0
		for row in 0..self.altitudes.len() {
			for col in 0..self.altitudes[row].len() {
				self.altitudes[row][col] = Raster::map(self.altitudes[row][col], min_max.0, min_max.1, 0.0, 100.0);
			}
		}

		Ok(())

	}




	/// ## Private Static Method
	/// Map an Option<f32> value to a new range, return None if the input is None.
	fn map(val: Option<f32>, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> Option<f32> {
		// If this value is None, return None
		if val.is_none() {
			None
		} else {
			let unpacked: f32 = val?;

			Some(to_min + ((unpacked - from_min) / (from_max - from_min) * (to_max - to_min)))
		}
	}

}


//
// UNIT TESTS
//

#[cfg(test)]
mod tests {
    use super::Raster;

	/// ## Unit Test Provider
	fn assert_float_eq(actual: Option<f32>, desired: Option<f32>) {
		if desired.is_none() {
			assert!(actual.is_none());
		} else if actual.is_none() {
			assert!(desired.is_none());
		} else {
			assert!(f32::abs(desired.unwrap() - actual.unwrap()) <= 0.0001);
		}
	}

	/// ## Unit Test
	/// ### Input
	/// | Parameter | Value |
	/// | --------- | ----- |
	/// | val       | 0 |
	/// | from_min  | 0 |
	/// | from_max  | 10 |
	/// | to_min    | 0 |
	/// | to_max    | 100 |
	/// 
	/// ### Expected output
	/// **Return:** 0
	#[test]
	fn map_test_1() {
		assert_float_eq(
			Raster::map(Some(0.0), 0.0, 10.0, 0.0, 100.0),
			Some(0.0)
		);
	}

	/// ## Unit Test
	/// ### Input
	/// | Parameter | Value |
	/// | --------- | ----- |
	/// | val       | 1 |
	/// | from_min  | 0 |
	/// | from_max  | 10 |
	/// | to_min    | 0 |
	/// | to_max    | 100 |
	/// 
	/// ### Expected output
	/// **Return:** 10
	#[test]
	fn map_test_2() {
		assert_float_eq(
			Raster::map(Some(1.0), 0.0, 10.0, 0.0, 100.0),
			Some(10.0)
		);
	}

	/// ## Unit Test
	/// ### Input
	/// | Parameter | Value |
	/// | --------- | ----- |
	/// | val       | 0 |
	/// | from_min  | 5 |
	/// | from_max  | 10 |
	/// | to_min    | 0 |
	/// | to_max    | 100 |
	/// 
	/// ### Expected output
	/// **Return:** 50
	#[test]
	fn map_test_3() {
		assert_float_eq(
			Raster::map(Some(5.0), 0.0, 10.0, 0.0, 100.0),
			Some(50.0)
		);
	}

	/// ## Unit Test
	/// ### Input
	/// | Parameter | Value |
	/// | --------- | ----- |
	/// | val       | 11 |
	/// | from_min  | 0 |
	/// | from_max  | 10 |
	/// | to_min    | 0 |
	/// | to_max    | 100 |
	/// 
	/// ### Expected output
	/// **Return:** 110
	#[test]
	fn map_test_4() {
		assert_float_eq(
			Raster::map(Some(11.0), 0.0, 10.0, 0.0, 100.0),
			Some(110.0)
		);
	}

	/// ## Unit Test
	/// ### Input
	/// | Parameter | Value |
	/// | --------- | ----- |
	/// | val       | -1 |
	/// | from_min  | 0 |
	/// | from_max  | 10 |
	/// | to_min    | 0 |
	/// | to_max    | 100 |
	/// 
	/// ### Expected output
	/// **Return:** -10
	#[test]
	fn map_test_5() {
		assert_float_eq(
			Raster::map(Some(-1.0), 0.0, 10.0, 0.0, 100.0),
			Some(-10.0)
		);
	}

	/// ## Unit Test
	/// ### Input
	/// | Parameter | Value |
	/// | --------- | ----- |
	/// | val       | 0 |
	/// | from_min  | -10 |
	/// | from_max  | 10 |
	/// | to_min    | 0 |
	/// | to_max    | 100 |
	/// 
	/// ### Expected output
	/// **Return:** 50
	#[test]
	fn map_test_6() {
		assert_float_eq(
			Raster::map(Some(0.0), -10.0, 10.0, 0.0, 100.0),
			Some(50.0)
		);
	}
	
	/// ## Unit Test
	/// ### Input
	/// | Parameter | Value |
	/// | --------- | ----- |
	/// | val       | None |
	/// | from_min  | 0 |
	/// | from_max  | 10 |
	/// | to_min    | 0 |
	/// | to_max    | 100 |
	/// 
	/// ### Expected output
	/// **Return:** None
	#[test]
	fn map_test_7() {
		assert_float_eq(
			Raster::map(None, 0.0, 10.0, 0.0, 100.0),
			None
		);
	}

	/// ## Unit Test
	/// Test the behaviour of the normalise function in a Raster
	#[test]
	fn normalise_test_1() {
		// Create a raster
		let mut raster = Raster::new(200.0, 150.0, 2, 2);

		// Initialize altitude values for all four raster-boxes
		raster.set(0, 0, 0.8);
		raster.set(0, 1, 1.0);
		raster.set(1, 0, 0.5);
		raster.set(1, 1, 0.0);

		// Perform normalisation
		assert!(raster.normalise().is_ok());

		// Assertions
		assert_float_eq(Some(raster.row_height), Some(50.0));
		assert_float_eq(Some(raster.column_width), Some(50.0));
		assert_float_eq(*raster.get(0, 0), Some(80.0));
		assert_float_eq(*raster.get(0, 1), Some(100.0));
		assert_float_eq(*raster.get(1, 0), Some(50.0));
		assert_float_eq(*raster.get(1, 1), Some(0.0));
	}

}