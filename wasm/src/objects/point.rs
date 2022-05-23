/// # Struct: Point
/// This struct is used in many places where coordinates (x, y, z) should be connected.
/// It provides functionality for computing the distance between two points, also isolated on (x,y) dimensions.
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Point {
	// Constructor is not necessary

	/// ## Static Method
	/// Compute the squared distance between two points (more efficient than non-squared)
	pub fn dist_sqr(a: &Point, b: &Point) -> f32 {
		(b.x - a.x).powi(2) + (b.y - a.y).powi(2) + (b.z - a.z).powi(2)
	}

	/// ## Static Method
	/// Compute the distance between two points (less efficient than squared)
	pub fn dist(a: &Point, b: &Point) -> f32 {
		f32::sqrt(Point::dist_sqr(a, b))
	}

	/// ## Static Method
	/// Compute the squared distance between two points (more efficient than non-squared), solely based on (x,y) dimensions.
	pub fn xy_dist_sqr(a: &Point, b: &Point) -> f32 {
		(b.x - a.x).powi(2) + (b.y - a.y).powi(2)
	}

	/// ## Static Method
	/// Compute the distance between two points (less efficient than squared), solely based on (x,y) dimensions.
	pub fn xy_dist(a: &Point, b: &Point) -> f32 {
		f32::sqrt(Point::xy_dist_sqr(a, b))
	}
}
