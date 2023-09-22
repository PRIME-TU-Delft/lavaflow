mod lava_path_finder;
mod model_construction;
mod objects;
mod surface_subdivision;
mod utils;
mod api;
mod api_helper_fns;
mod gltf_conversion;
mod contours;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
	fn alert(s: &str);
}
