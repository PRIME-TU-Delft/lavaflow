use contour_isobands::{ContourBuilder, Band};
use geojson::{GeoJson, FeatureCollection};
use miette::{miette, Result};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};


/// Converts a vector of bands to a GeoJSON string
fn to_json(bands: &[Band]) -> String {
	let features: Vec<geojson::Feature> = bands.iter()
		.map(|band| band.to_geojson())
		.collect();

	GeoJson::from(
		FeatureCollection {
			bbox: None,
			features,
			foreign_members: None
		}
	).to_string()
}

/// Takes a 2D array of values and returns a GeoJSON of detected contours
#[wasm_bindgen]
pub fn contours(data: JsValue) -> Result<String, JsValue> {
	let data: Vec<Vec<f64>> = serde_wasm_bindgen::from_value(data).unwrap();

	// Transpose data vec
	let data_transpose = data.clone().into_iter()
		.enumerate()
		.map(|(i, _)| data.iter().map(|row| row[i]).collect())
		.collect::<Vec<Vec<f64>>>();

	// Flatten data vec
	let data_flat: Vec<f64> = data_transpose.clone().into_iter().flatten().collect();

	let len_x = data_transpose.len();
	let len_y = data_transpose.get(0).
		ok_or(miette!("Image has no width lol"))
		.map_err(|e| e.to_string())?
		.len();

	let contours = ContourBuilder::new(len_x, len_y)
		.use_quad_tree(true)
		.contours(&data_flat, &[0., 0.3, 1.])
		.map_err(|e| {
			miette!("Error in contour detection")
				.context(e.to_string())
				.to_string()
		})?;

	Ok(to_json(&contours))
}
