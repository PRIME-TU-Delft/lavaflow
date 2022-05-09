use super::level_curves::{LevelCurve, LevelCurveSet};
use super::point::Point;
use mathrs::trigonometry::arccos;
use std::cmp;

pub struct LevelCurveSet {
	altitude_step: f32,
	level_curves: Vec<LevelCurve>,
}



// The Triangle struct is used for computations that are related to the triangulation step
struct Triangle {
    a: &Point,
    b: &Point,
    c: &Point
}

impl Triangle {

    /// Constructor
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Self {a, b, c}
    }

    /// Compute the interior angle at point a
    fn angle_at_a(&self) -> f32 {

        let a_side = self.b.xy_dist(self.c);
        let b_side = self.a.xy_dist(self.c);
        let c_side = self.a.xy_dist(self.b);

        arccos(
            (f64::powi(b_side, 2) + f64::powi(c_side, 2) - f64::powi(a_side, 2)) / 2*b_side*c_side
        ) as f32
    }

    /// Compute the minimal internal angle of this triangle
    pub fn minimal_internal_angle(&self) -> f32 {

        // Compute angle at a
        let a_angle = self.angle_at_a;

        // Compute angle at b
        let b_angle = Triangle::new(self.b, self.a, self.c).angle_at_a();

        // Compute angle at c
        let c_angle = Triangle::new(self.c, self.a, self.b).angle_at_a();

        // Return the minimal angle
        cmp::min(cmp::min(a_angle, b_angle), c_angle)
    }

    /// Method: Does this tringle contain this point?
    pub fn contains_point(&self, p: &Point) -> bool {
        /// TODO: Determine whether this point lies in this triangle
        
        true
    }




}

/// Extension of the LevelCurveSet class
impl LevelCurveSet {

    /// Method: local_tin_interpolate
    /// 
    /// # Arguments
    /// * 'self': The LevelCurveSet instance to apply this method to
    /// * 'p':    The Point to interpolate
    pub fn local_tin_interpolate(&self, p: &Point) -> f32 {

        // If this set contains less than two level-curves, interpolation is not possible. Return 'p.z'
        if self.level_curves.is_empty() {
            return p.z;
        }

        //
        // 1. Find the two closest levelcurves to this point
        //
        let mut closest_two_levelcurves: (&LevelCurve, &LevelCurve) = (self.level_curves[0], self.level_curves[1]);
        let mut closest_two_distances: (f32, f32) = (self.level_curves[0].dist_to_point(p), self.level_curves[1].dist_to_point(p));

        // Go over every level curve in this set
        for i in 2..self.level_curves.len() {

            // Compute the distance from this level curve to this point
            let current_dist = self.level_curves[i].dist_to_point(p);

            // If this distance is smaller than both current level-curves, shift everything over
            if current_dist < closest_two_distances.0 && current_dist < closest_two_distances.1 {
                closest_two_levelcurves.1 = closest_two_levelcurves.0;
                closest_two_levelcurves.0 = self.level_curves[i];

                closest_two_distances.1 = closest_two_distances.0;
                closest_two_distances.0 = current_dist;
            }

            // Else, if the distance is greater than the first level curve but smaller than the second, insert the new one
            else if current_dist >= closest_two_distances.0 && current_dist < closest_two_distances.1 {
                closest_two_levelcurves.1 = self.level_curves[i];

                closest_two_distances.1 = current_dist;
            }

            // Else, the distance is larger than both current level curves, skip this one

        }

        //
        // 2. Compute the triangles between these two level-curves
        //

        let points_closest_0 = closest_two_levelcurves.0.get_points();
        let points_closest_1 = closest_two_levelcurves.1.get_points();
        
        // Additional checking constraint: if one of these level-curves has no points, return 'p.z'
        if points_closest_0.is_empty() || points_closest_1.is_empty() {
            return p.z;
        }

        // Create a vector of triangles (which are triples)
        let mut triangles: Vec<(&Point, &Point, &Point)> = Vec::new();

        // Select a random point from the first closest curve and find the closest point on the other curve
        let mut point_a = 0;
        let mut point_b = closest_two_levelcurves.1.find_closest_point_with_index_on_level_curve(
            closest_two_levelcurves.0.get_point(0).unwrap()
        ).0.unwrap();

        // Store both points for later reference
        let initial_points: (usize, usize) = (point_a, point_b);

        // We will now start computing the triangles
        loop {

            // We will consider the following two points for a new triangle
            let next_point_a = point_a + 1;
            let next_point_b = point_b + 1;

            // Compute minimal angle of triangle 1
            let min_angle_t1 = Triangle::new(
                closest_two_levelcurves.0.get_point(point_a).unwrap(),
                closest_two_levelcurves.1.get_point(point_b).unwrap(),
                closest_two_levelcurves.0.get_point(next_point_a).unwrap()
            ).minimal_internal_angle();

            // Compute minimal angle of triangle 2
            let min_angle_t2 = Triangle::new(
                closest_two_levelcurves.0.get_point(point_a).unwrap(),
                closest_two_levelcurves.1.get_point(point_b).unwrap(),
                closest_two_levelcurves.1.get_point(next_point_b).unwrap()
            ).minimal_internal_angle();

            // If the first triangle has a larger minimal interior angle, select that one
            if min_angle_t1 > min_angle_t2 {
                // Select the first triangle
                triangles.push((
                    closest_two_levelcurves.0.get_point(point_a).unwrap(),
                    closest_two_levelcurves.1.get_point(point_b).unwrap(),
                    closest_two_levelcurves.0.get_point(next_point_a).unwrap()
                ));

                // The point on the first level curve has now shifted one place further (A)
                point_a = next_point_a;

            }

            // Else, select the other one
            else {
                // Select the second triangle
                triangles.push((
                    closest_two_levelcurves.0.get_point(point_a).unwrap(),
                    closest_two_levelcurves.1.get_point(point_b).unwrap(),
                    closest_two_levelcurves.1.get_point(next_point_b).unwrap()
                ));

                // The point on the second level curve has now shifted one place further (B)
                point_b = next_point_b;

            }

            // The loop is complete if either point a or point b is equal to the initial point
            if point_a == initial_points.0 || point_b == initial_points.1 {
                break;
            }

        }

        //
        // 3. Determine the triangle that 'p' falls in
        //


        //
        // 4. Compute the interpolated value
        //

        0.0

    }



}