#[derive(Debug)]
pub struct Point {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Point {
	// Constructor is not necessary

	pub fn dist_sqr(a: &Point, b: &Point) -> f32 {
		(b.x - a.x).powi(2) + (b.y - a.y).powi(2) + (b.z - a.z).powi(2)
	}

	pub fn dist(a: &Point, b: &Point) -> f32 {
		f32::sqrt(Point::dist_sqr(a, b))
	}

	pub fn xy_dist_sqr(a: &Point, b: &Point) -> f32 {
		(b.x - a.x).powi(2) + (b.y - a.y).powi(2)
	}

	pub fn xy_dist(a: &Point, b: &Point) -> f32 {
		f32::sqrt(Point::xy_dist_sqr(a, b))
	}
}
