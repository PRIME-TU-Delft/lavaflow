use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Point {
	x: f64,
	y: f64,
	z: f64
}

#[wasm_bindgen]
impl Point {
	#[wasm_bindgen(constructor)]
	pub fn new(x: f64, y: f64, z: f64) -> Self {
		Self {x, y, z}
	}

	pub fn length(&self) -> f64 {
		f64::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
	}

	pub fn distance(&self, b: &Point) -> f64 {
		let diff = Point {
			x: self.x - b.x,
			y: self.y - b.y,
			z: self.z - b.z
		};
		diff.length()
	}
}
