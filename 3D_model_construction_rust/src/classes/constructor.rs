use super::raster::Raster;

#[derive(Debug)]
pub struct ModelConstructor {
	contour_margin: u64,
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

impl ModelConstructor {
	fn construct_map(&mut self, raster: &Raster, contour_margin: i64) {
		let x = raster.columns as usize;
		let y = raster.rows as usize;
		self.is_svc = vec![vec![false; x]; y];
	}
}
