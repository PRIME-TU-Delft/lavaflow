use super::raster::Raster;
use super::point::Point;

#[derive(Debug)]
pub struct ModelConstructor<'a> {
	contour_margin: u64,
	is_svc: Vec<Vec<bool>>,
	raster: &'a Raster,
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
	fn construct_map(&mut self, raster: &'a Raster, _contour_margin: i64)  {
		let x = raster.columns;
		let y = raster.rows;
		self.raster = &raster;
		self.is_svc = vec![vec![false; x]; y];
		//initialize();

		for i in 0 .. x {
			for j in 0 .. y{

				//TODO z point is now "0" but doesnt really exist
				let cellCentre : Point = Point{ x : (i as f64 + 0.5) * raster.row_height,
												y: (j as f64 +  0.5) * raster.column_width,
												z: 0.0 } ;
				//bool isSVC = checksvc(cellCentre , levelmap);
				//TODO remove line here once checkSVC is implemented
				let isSVC : bool = false;
				if(isSVC){
					if(raster.altitudes[i][j].is_none()){
						//local_tin(cellCentre)
					}
				}
			}		
		}
		//calc_heights_nvc()

	}

	
}
