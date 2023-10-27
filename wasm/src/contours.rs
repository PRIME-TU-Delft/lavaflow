use contour_isobands::{ContourBuilder, Band};
use geo::{Area, Contains, Polygon};
use miette::{miette, Result};


#[derive(PartialEq)]
pub struct ContourHierarchy {
	contour: Option<Polygon>,
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
/// Algorithm outline:
/// Sort polygons by area, ascending
/// For each polygon:
///     Find the first (smallest) polygon that contains it, this is the parent
///     Add node as child of parent
///     OR: if no node found, add it as a root node to the hierarchy struct
fn hierarchy(polygons: Vec<Polygon>) -> Result<ContourHierarchy> {
	let mut polygons = polygons.to_vec();
	polygons.sort_by(|a, b| a.unsigned_area().total_cmp(&b.unsigned_area()));
	// TODO: ^ cache computed areas during sort?
	// Can't use sort_by_cached_key because f64 doesn't implement Ord :(

	let mut root = ContourHierarchy {
		contour: None,
		children: Vec::new(),
	};

	let mut nodes: Vec<_> = polygons.into_iter().map(|polygon| ContourHierarchy {
		contour: Some(polygon),
		children: Vec::new()
	}).collect();

	while let Some(node) = nodes.pop() {
		let parent = nodes.iter_mut().find(|other| {
			let node_contour = node.contour.as_ref().unwrap();
			let other_contour = other.contour.as_ref().unwrap();
			*other != &node && other_contour.contains(node_contour)
		});
		match parent {
			Some(parent) => parent.children.push(node),
			None => root.children.push(node),
		}
	}

	Ok(root)
}
