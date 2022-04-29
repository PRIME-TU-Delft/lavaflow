//
// Class: Raster
//

#[derive(Debug)]
pub struct Raster {

    rows: u64,
    columns: u64,
    row_height: f64,
    column_width: f64,
    altitudes: Vec<Vec<f64>>

}


impl Raster {

    // Constructor
    pub fn new(width: f64, height: f64, rows: u64, columns: u64) -> Self {
        // Initialize the 2d array of altitudes
        let mut altitudes_init: Vec<Vec<f64>> = Vec::new();


        // Set all the altitude values to zero
        for i in 0..rows {
            let mut current_col: Vec<f64> = Vec::new();
            for j in 0..columns {
                current_col.push(0.0);
            }
            altitudes_init.push(current_col);
        }


        Self{ rows, columns, row_height: height/(rows as f64), column_width: width/(columns as f64), altitudes:altitudes_init }
    }

    pub fn set(&mut self, row: u64, col: u64, val: f64) {
        self.altitudes[(row as usize)][(col as usize)] = val;
    }

    pub fn get(&self, row: u64, col: u64) -> f64 {
        return self.altitudes[(row as usize)][(col as usize)];
    }

}