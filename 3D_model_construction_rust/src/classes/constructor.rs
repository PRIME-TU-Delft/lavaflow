use super::{raster::Raster, level_curves::{LevelCurveMap, LevelCurve}};

#[derive(Debug)]
pub struct ModelConstructor {
	contour_margin: u64,
    level_curve_map: LevelCurveMap,
	is_svc: Vec<Vec<bool>>,
	raster: Raster
}

// @Pauline are you planning to implement this trait for other structs as well?
// Otherwise using a trait is unnecessary, you can just implement functions directly for a struct without using traits.
// (Traits are only really meant for defining shared behavior. If you want a "contract" for a function that still has to be implemented, you can use the todo macro: https://doc.rust-lang.org/std/macro.todo.html)
// - Julia
trait HelperFunctions {
	fn initialize();
	fn check_svc(p: Vec<f64>, ix: i64, iy: i64) -> bool;
	// fn getClostestCOntourPoint(p, Vec<f64>, );
	fn local_tin(p: Vec<f64>);
	fn calc_heights_nvcs();
}


// Implementation of the ModelConstructor
impl ModelConstructor {

    // Function: Initialize the SVC map with 'false' for every element
	fn construct_map(&mut self, raster: &Raster, _contour_margin: i64) {
		
	}

    // Function: initialize
    fn setRasterEdgesToZero(&mut self) {

        // Initialize all the outer-most raster values to zero
        // (According to requirement that the edges of the mountain must have level 0)
        for i in 0..self.raster.rows {
            for j in 0..self.raster.columns {
                // If this box is on the outer edge
                if i == 0 || j == 0 || i == self.raster.rows-1 || j == self.raster.columns-1 {
                    // Set its value to zero
                    self.raster.set(i, j, 0.0);
                    
                    // Set this box to an svc-box
                    self.is_svc[i][j] = true;
                }
            }
        }

    }

}

