use contour_isobands::{ContourBuilder, Band};
use geo::{Area, Contains, Polygon};
use miette::{miette, Result};


pub struct ContourHierarchy {
	contour: Polygon,
	children: Vec<ContourHierarchy>,
}


pub fn pipeline(data: Vec<Vec<f64>>) -> Result<ContourHierarchy> {
	let bands = contours(data)?;
	let band = bands.last()
		.ok_or(miette!("No contours detected in drawing"))?;

	let filtered = band.geometry.clone().into_iter()
		.filter(|polygon| polygon.exterior().coords().collect::<Vec<_>>().len() > 10)  // Filter out small polygons
		.collect::<Vec<_>>();

	let hierarchy = hierarchy(filtered)?;
	Ok(hierarchy)
}


/// Takes a 2D array of values and returns a GeoJSON of detected contours
fn contours(data: Vec<Vec<f64>>) -> Result<Vec<Band>> {
	// Transpose data vec
	// TODO: move this to frontend before calling this function
	let data_transpose = data.clone().into_iter()
		.enumerate()
		.map(|(i, _)| data.iter().map(|row| row[i]).collect())
		.collect::<Vec<Vec<f64>>>();

	let len_x = data_transpose.len();
	let len_y = data_transpose.get(0).
		ok_or(miette!("Image has no width lol"))?
		.len();

	// Flatten data vec
	let data_flat: Vec<f64> = data_transpose.into_iter().flatten().collect();

	ContourBuilder::new(len_x, len_y)
		.use_quad_tree(true)
		.contours(&data_flat, &[0., 0.3, 1.])
		.map_err(|e| miette!("Error building contours: {}", e))
}


/// Builds a topological hierarchy of polygons
fn hierarchy(polygons: Vec<Polygon>) -> Result<ContourHierarchy> {
	let mut polygons = polygons.to_vec();
	polygons.sort_by(|a, b| a.unsigned_area().total_cmp(&b.unsigned_area()));
	// TODO: ^ cache computed areas during sort?
	// Can't use sort_by_cached_key because f64 doesn't implement Ord :(

	let mut root = ContourHierarchy {
		contour: polygons.first().ok_or(miette!("No polygons in contour"))?.clone(),
		children: vec![],
	};

	for polygon in polygons.iter().skip(1) {
		assert!(root.contour.contains(polygon.exterior()), "Polygon is not contained in root contour");
		let mut parent = &mut root;

		// Recursively find the deepest polygon that contains the current polygon
		while !parent.children.is_empty() {
			let child = parent.children.iter_mut()
				.filter(|child| child.contour.contains(polygon.exterior()))
				.last()
				.ok_or(miette!("Polygon is not contained in any children"))?;
			parent = child;
		}

		// Found a leaf node, add the polygon as a child
		parent.children.push(ContourHierarchy {
			contour: polygon.clone(),
			children: vec![],
		});
	}

	Ok(root)
}
