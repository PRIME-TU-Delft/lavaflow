#[derive(Debug)]
pub struct Point {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl Point {
	// Constructor is not necessary

	pub fn dist_sqr(a: &Point, b: &Point) -> f64 {
		(b.x - a.x).powi(2) + (b.y - a.y).powi(2) + (b.z - a.z).powi(2)
	}

	pub fn dist(a: &Point, b: &Point) -> f64 {
		f64::sqrt(Point::dist_sqr(a, b))
	}

	pub fn xy_dist_sqr(a: &Point, b: &Point) -> f64 {
		(b.x - a.x).powi(2) + (b.y - a.y).powi(2)
	}

	pub fn xy_dist(a: &Point, b: &Point) -> f64 {
		f64::sqrt(Point::xy_dist_sqr(a, b))
	}
}
