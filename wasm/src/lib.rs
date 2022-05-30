#![allow(dead_code)] // TODO: remove this!
#![allow(unused_variables)] // TODO: remove this!
mod lava_path_finder;
mod model_construction;
mod objects;
mod surface_subdivision;
mod utils;
use wasm_bindgen::prelude::*;

mod generate;
mod gltf_conversion;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
	fn alert(s: &str);
}
