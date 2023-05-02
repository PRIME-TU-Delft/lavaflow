mod lava_path_finder;
mod model_construction;
mod objects;
mod surface_subdivision;
mod utils;
use wasm_bindgen::prelude::*;
mod api;
mod api_helper_fns;
mod gltf_conversion;

#[wasm_bindgen]
extern "C" {
	fn alert(s: &str);
}
