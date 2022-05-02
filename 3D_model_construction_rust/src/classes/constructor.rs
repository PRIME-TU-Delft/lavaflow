
use super::{raster::Raster, level_curves::{LevelCurveMap, LevelCurve}};
use super::point::Point;

#[derive(Debug)]
pub struct ModelConstructor<'a> {
	contour_margin: u64,
    level_curve_map: LevelCurveMap,
	is_svc: Vec<Vec<bool>>,
	raster: &'a mut Raster,
}

// @Pauline are you planning to implement this trait for other structs as well?
// Otherwise using a trait is unnecessary, you can just implement functions directly for a struct without using traits.
// (Traits are only really meant for defining shared behavior. If you want a "contract" for a function that still has to be implemented, you can use the todo macro: https://doc.rust-lang.org/std/macro.todo.html)
// - Julia


trait HelperFunctions {
	fn initialize();
	fn check_svc(p: Vec<f64>, ix: i64, iy: i64) -> bool;
	// fn getClostestCOntourPoint(p, levelcurvemap, );
	fn local_tin(p: Vec<f64>);
	fn calc_heights_nvcs();
}

impl<'a> ModelConstructor<'a> {
	//TODO add levelmap to function parameters
	fn construct_map(&mut self, raster: &'a mut Raster, _contour_margin: i64)  {
		let x = raster.columns;
		let y = raster.rows;
		self.raster = raster;
		self.is_svc = vec![vec![false; x]; y];
		
        // Set the edges of the raster to zero
        self.setRasterEdgesToZero();

		for i in 0 .. x {
			for j in 0 .. y{

				//TODO z point is now "0" but doesnt really exist
				let cellCentre : Point = Point{ x : (i as f64 + 0.5) * self.raster.row_height,
												y: (j as f64 +  0.5) * self.raster.column_width,
												z: 0.0 } ;
				//bool isSVC = checksvc(cellCentre , levelmap);
				//TODO remove line here once checkSVC is implemented
				let isSVC : bool = false;
				if isSVC {
					if self.raster.altitudes[i][j].is_none() {
						//local_tin(cellCentre)
					}
				}
			}		
		}
		//calc_heights_nvc()

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

    //
    // A set of functions for finding the nearest SVC box in directions north (west, east) and south (west, east)
    //

    // Function: find SVC north
    fn findSVCNorth(&self, i: usize, j: usize) -> (usize, usize) {

        let mut row = i;
        let     col = j;

        while row < self.raster.rows && col < self.raster.columns && row >= 0 && col >= 0 {

            // If this box is svc, return its position
            if self.is_svc[row][col] {
                return (row, col);
            }

            // Walk one step further
            row -= 1;

        }

        // If no value can be found, return (0, 0)
        (0, 0)

    }

    // Function: find SVC south
    fn findSVCSouth(&self, i: usize, j: usize) -> (usize, usize) {

        let mut row = i;
        let     col = j;

        while row < self.raster.rows && col < self.raster.columns && row >= 0 && col >= 0 {

            // If this box is svc, return its position
            if self.is_svc[row][col] {
                return (row, col);
            }

            // Walk one step further
            row += 1;

        }

        // If no value can be found, return (0, 0)
        (0, 0)

    }

    // Function: find SVC west
    fn findSVCWest(&self, i: usize, j: usize) -> (usize, usize) {

        let     row = i;
        let mut col = j;

        while row < self.raster.rows && col < self.raster.columns && row >= 0 && col >= 0 {

            // If this box is svc, return its position
            if self.is_svc[row][col] {
                return (row, col);
            }

            // Walk one step further
            col -= 1;

        }

        // If no value can be found, return (0, 0)
        (0, 0)

    }

    // Function: find SVC east
    fn findSVCEast(&self, i: usize, j: usize) -> (usize, usize) {

        let     row = i;
        let mut col = j;

        while row < self.raster.rows && col < self.raster.columns && row >= 0 && col >= 0 {

            // If this box is svc, return its position
            if self.is_svc[row][col] {
                return (row, col);
            }

            // Walk one step further
            col += 1;

        }

        // If no value can be found, return (0, 0)
        (0, 0)

    }

    // Function: find SVC north west
    fn findSVCNorthWest(&self, i: usize, j: usize) -> (usize, usize) {

        let mut row = i;
        let mut col = j;

        while row < self.raster.rows && col < self.raster.columns && row >= 0 && col >= 0 {

            // If this box is svc, return its position
            if self.is_svc[row][col] {
                return (row, col);
            }

            // Walk one step further
            row -= 1;
            col -= 1;

        }

        // If no value can be found, return (0, 0)
        (0, 0)

    }

    // Function: find SVC north east
    fn findSVCNorthEast(&self, i: usize, j: usize) -> (usize, usize) {

        let mut row = i;
        let mut col = j;

        while row < self.raster.rows && col < self.raster.columns && row >= 0 && col >= 0 {

            // If this box is svc, return its position
            if self.is_svc[row][col] {
                return (row, col);
            }

            // Walk one step further
            row -= 1;
            col += 1;

        }

        // If no value can be found, return (0, 0)
        (0, 0)

    }

    // Function: find SVC south east
    fn findSVCSouthEast(&self, i: usize, j: usize) -> (usize, usize) {

        let mut row = i;
        let mut col = j;

        while row < self.raster.rows && col < self.raster.columns && row >= 0 && col >= 0 {

            // If this box is svc, return its position
            if self.is_svc[row][col] {
                return (row, col);
            }

            // Walk one step further
            row += 1;
            col += 1;

        }

        // If no value can be found, return (0, 0)
        (0, 0)

    }

    // Function: find SVC south west
    fn findSVCSouthWest(&self, i: usize, j: usize) -> (usize, usize) {

        let mut row = i;
        let mut col = j;

        while row < self.raster.rows && col < self.raster.columns && row >= 0 && col >= 0 {

            // If this box is svc, return its position
            if self.is_svc[row][col] {
                return (row, col);
            }

            // Walk one step further
            row += 1;
            col -= 1;

        }

        // If no value can be found, return (0, 0)
        (0, 0)

    }

}

