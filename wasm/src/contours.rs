use contour_isobands::{ContourBuilder, Band};
use geo::Coord;
use miette::{miette, Result, IntoDiagnostic};


struct ContourHierarchy<'a> {
	contour: Vec<&'a Coord>,
	children: Vec<ContourHierarchy<'a>>,
}


pub fn pipeline(data: Vec<Vec<f64>>) -> Result<()> {
	let bands = contours(data)?;
	let band = bands.last()
		.ok_or(miette!("No contours detected in drawing"))?;

	let filtered = band.geometry.iter()
		.map(|polygon| polygon.exterior().coords().collect::<Vec<_>>())  // Extract exterior coords
		.filter(|polygon| polygon.len() > 10)  // Filter out small polygons
		.collect::<Vec<_>>();

	let hierarchy = hierarchy(&filtered)?;

	Ok(())
}


/// Takes a 2D array of values and returns a GeoJSON of detected contours
fn contours(data: Vec<Vec<f64>>) -> Result<Vec<Band>> {
	// Transpose data vec
	// TODO: move this to frontend before calling this function
	let data_transpose = data.clone().into_iter()
		.enumerate()
		.map(|(i, _)| data.iter().map(|row| row[i]).collect())
		.collect::<Vec<Vec<f64>>>();

	// Flatten data vec
	let data_flat: Vec<f64> = data_transpose.clone().into_iter().flatten().collect();

	let len_x = data_transpose.len();
	let len_y = data_transpose.get(0).
		ok_or(miette!("Image has no width lol"))?
		.len();

	ContourBuilder::new(len_x, len_y)
		.use_quad_tree(true)
		.contours(&data_flat, &[0., 0.3, 1.])
		.map_err(|e| miette!("Error building contours: {}", e))
}


fn hierarchy<'a>(polygons: &'a [Vec<&Coord>]) -> Result<ContourHierarchy<'a>> {
	let root = polygons.first()
		.ok_or(miette!("No polygons to build hierarchy from"))?;

	todo!("Actually build the hierarchy");
	let hierarchy = ContourHierarchy {
		contour: root.clone(),
		children: Vec::new(),
	};

	Ok(hierarchy)
}