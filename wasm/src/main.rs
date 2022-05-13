mod model_construction;
mod utils;

use model_construction::catmull_clark::catmull_clark_super;
use model_construction::catmull_clark::Face;
use model_construction::catmull_clark::Vertex;
use model_construction::raster::Raster;

use std::fs::File;
use std::io::Write;

pub fn main() {
	// OBJ GENERATION
	//VERTEX LINE:  v x y z
	//FACE TRIANGLE: f [v1] [v2] [v3]

	let mut raster = Raster::new(0.0, 0.0, 0, 0);
	let mut is_sharp: Vec<Vec<bool>> = Vec::new();

	let (vs, fs) = catmull_clark_super(5, &is_sharp, &mut raster).expect("catumull broke");
}
