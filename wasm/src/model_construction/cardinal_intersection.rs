//provides functions to find intersection in each cardinal direction

use super::raster::Raster;

//  TODO; rest of directions
pub fn n(ix: usize, iy: usize, raster: &Raster, is_svc: &Vec<Vec<bool>>) -> (Option<f64>, usize) {
	for y in iy..raster.rows {
		if is_svc[ix][y] {
			return (raster.altitudes[ix][y], iy - y);
		}
	}
	(None, 0)
}
