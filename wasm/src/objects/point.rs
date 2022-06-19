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

/// # Struct: Vector
/// A vector is very similar to a point, but it provides different functionality
/// that makes it much more easy to work with
pub struct Vector {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vector {
	/// Static Method: Vector from point A to point B
	pub fn from_point_to_point(a: &Point, b: &Point) -> Self {
		Self {
			x: b.x - a.x,
			y: b.y - a.y,
			z: b.z - a.z,
		}
	}

	/// Static Method: Cross product of vectors a and b: a x b
	pub fn cross_product(a: &Vector, b: &Vector) -> Self {
		Self {
			x: a.y * b.z - a.z * b.y,
			y: a.z * b.x - a.x * b.z,
			z: a.x * b.y - a.y * b.x,
		}
	}

	/// Instance Method: length
	pub fn len(&self) -> f32 {
		f32::sqrt(f32::powi(self.x, 2) + f32::powi(self.y, 2) + f32::powi(self.z, 2))
	}
}
