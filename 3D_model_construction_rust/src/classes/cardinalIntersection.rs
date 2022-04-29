
//provides functions to find intersection in each cardinal direction

use super::point::Point;
use super::raster::*;


 //  TODO; rest of directions
pub fn n(ix: u64, iy: u64, raster: Raster, isSVC: &Vec<Vec<bool>> ) -> (f64, u64){
        for y in iy..raster.rows {
            if isSVC[ix as usize][y as usize] == true {
                return (raster.altitudes[ix as usize][y as usize], iy - y );
            }
        }
        return (0.0, 0);
}

