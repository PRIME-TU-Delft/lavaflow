use super::point::Point;

/// # Struct: Triangle
/// 
/// The Triangle struct is used for computations that are related to the triangulation of level-curves.
/// 
/// ## Properties:
/// - *public* `a: Point`
/// - *public* `b: Point`
/// - *public* `c: Point`
/// 
/// ## Private methods
/// - `angle_at_a`: Compute the interior angle at point a in this triangle.
/// 
/// ## Public Methods
/// - `new`: Constructor.
/// - `minimal_internal_angle`: Compute the minimal internal angle of this triangle.
/// - `contains_point`: Determine whether a certain point lies within the are of this triangle.
/// - *static* `angle_at_pv`: Compute the interior angle at point pv in a triple of points (pu, pv, pw).
#[derive(Debug)]
pub struct Triangle<'a> {
	pub a: &'a Point,
	pub b: &'a Point,
	pub c: &'a Point,
}

impl<'a> Triangle<'a> {
	/// ## Constructor
	/// Create a new instance of `Triangle`.
	/// 
	/// ### Parameters
	/// - `a: Point` as reference with lifetime specifier
	/// - `b: Point` as reference with lifetime specifier
	/// - `c: Point` as reference with lifetime specifier
	/// 
	/// ### Returns
	/// A new instance of `Triangle`, from points `a`, `b` and `c`.
	pub fn new(a: &'a Point, b: &'a Point, c: &'a Point) -> Self {
		Self { a, b, c }
	}

	/// ## Instance Method
	/// Compute the internal angle at point `a` of this triangle.
	/// 
	/// ### Parameters
	/// - `self`: The referenced instance to apply this method to.
	/// 
	/// ### Returns
	/// `f32`: The internal angle at point `a`.
	fn angle_at_a(&self) -> f32 {
		// Compute the three sides (squared): a, b and c
        let a2 = Point::xy_dist_sqr(self.c, self.b);
        let b2 = Point::xy_dist_sqr(self.a, self.c);
        let c2 = Point::xy_dist_sqr(self.a, self.b);

        // Compute the cosin of angle A (angle at self.a)
        let cos_a = (b2 + c2 - a2) / (2.0 * f32::sqrt(b2) * f32::sqrt(c2));

        // Comself.cte and return the angle
        f32::acos(cos_a)
	}

	/// ## Instance method
	/// Compute the minimal internal angle of this triangle.
	/// 
	/// ### Paremeters
	/// - `self`: The referenced instance to apply this method to.
	/// 
	/// ### Returns
	/// `f32`: The minimal internal angle of this triangle.
	pub fn minimal_internal_angle(&self) -> f32 {
		// Compute angle at a
		let a_angle = self.angle_at_a();

		// Compute angle at b
		let b_angle = Triangle::new(self.b, self.a, self.c).angle_at_a();

		// Compute angle at c
		let c_angle = Triangle::new(self.c, self.a, self.b).angle_at_a();

		// Return the minimal angle
		f32::min(f32::min(a_angle, b_angle), c_angle)
	}

	/// ## Instance Method
	/// Check whether the area of this triangle contains a certain point.
	/// 
	/// ### Parameters
	/// - `self`: The referenced instance to apply this method to.
	/// - `p: Point`: The point to test this property against.
	/// 
	/// ### Returns
	/// Return true iff this point lies within the area of this triangle. False otherwise.
	pub fn contains_point(&self, p: &Point) -> bool {
		// To make this function more readable, we temporarily store self.a, self.b and self.c into a, b and c
		let a = self.a;
		let b = self.b;
		let c = self.c;

		// Determine whether p lies in this triangle
		let w1_upper = p.x * (c.y - a.y) - p.y * (c.x - a.x) - a.x * (c.y - a.y) + a.y * (c.x - a.x);
		let w1_lower = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);

		let w1 = w1_upper / w1_lower;

		let w2_upper = p.y - a.y - w1 * (b.y - a.y);
		let w2_lower = c.y - a.y;

		let w2 = w2_upper / w2_lower;

		// If w1 and w2 are both >= 0 and w1+w2 <= 1, then this point lies within the triangle. Otherwise not.
		w1 >= 0.0 && w2 >= 0.0 && w1 + w2 <= 1.0
	}

    /// ## Static Method
    /// Compute the internal angle at point 'pv'.
	/// 
	/// ### Parameters
	/// - `pu: Point`: The first point of this triangle.
	/// - `pv: Point`: The second (and 'main') point of this triangle.
	/// - `pw: Point`: The third and last point of this triangle.
	/// 
	/// ### Returns
	/// `f32`: The internal angle at point `pv`.
    pub fn angle_at_pv(pu: &Point, pv: &Point, pw: &Point) -> f32 {

        // Compute the three sides (squared): a, b and c
        let a2 = Point::xy_dist_sqr(pu, pw);
        let b2 = Point::xy_dist_sqr(pv, pu);
        let c2 = Point::xy_dist_sqr(pv, pw);

        // Compute the cosin of angle A (angle at pv)
        let cos_a = (b2 + c2 - a2) / (2.0 * f32::sqrt(b2) * f32::sqrt(c2));

        // Compute and return the angle
        f32::acos(cos_a)

    }
}