use super::point::Point;

// The Triangle struct is used for computations that are related to the triangulation step
#[derive(Debug)]
pub struct Triangle<'a> {
	pub a: &'a Point,
	pub b: &'a Point,
	pub c: &'a Point,
}

impl<'a> Triangle<'a> {
	/// Constructor
	pub fn new(a: &'a Point, b: &'a Point, c: &'a Point) -> Self {
		Self { a, b, c }
	}

	/// Compute the interior angle at point a
	fn angle_at_a(&self) -> f32 {
		// Compute the three sides (squared): a, b and c
        let a2 = Point::xy_dist_sqr(self.c, self.b);
        let b2 = Point::xy_dist_sqr(self.a, self.c);
        let c2 = Point::xy_dist_sqr(self.a, self.b);

        // Compute the cosin of angle A (angle at self.a)
        let cosA = (b2 + c2 - a2) / (2.0 * f32::sqrt(b2) * f32::sqrt(c2));

        // Comself.cte and return the angle
        f32::acos(cosA)
	}

    /// Method: compute angle at pv
    /// 
    /// According to the cosin-rule
    /// 
    /// # Arguments:
    /// * pu: The previous point in the triangulation
    /// * pv: The current point in the triangulation
    /// * pw: The next point in teh triangulation
    pub fn angle_at_pv(pu: &Point, pv: &Point, pw: &Point) -> f32 {

        // Compute the three sides (squared): a, b and c
        let a2 = Point::xy_dist_sqr(pu, pw);
        let b2 = Point::xy_dist_sqr(pv, pu);
        let c2 = Point::xy_dist_sqr(pv, pw);

        // Compute the cosin of angle A (angle at pv)
        let cosA = (b2 + c2 - a2) / (2.0 * f32::sqrt(b2) * f32::sqrt(c2));

        // Compute and return the angle
        f32::acos(cosA)

    }

	/// Comself.cte the minimal internal angle of this triangle
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

	/// Method: Does this tringle contain this point?
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
}