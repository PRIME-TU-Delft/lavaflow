use crate::utils::log;

use super::constructor::ModelConstructor;
use super::raster::Raster;
use super::level_curves::LevelCurve;
use super::point::Point;
use super::triangle::Triangle;
use miette::{Result, miette};


pub struct Smoother<'a> {
    raster: &'a Raster,
    is_svc: &'a Vec<Vec<bool>>,
    point_indices_per_layer: Vec<Vec<(usize, usize)>>
}


impl<'a> Smoother<'a> {

    pub fn new(model_constructor: &'a mut ModelConstructor) -> Result<Self> {

        // Construct the list of point-indices per layer
        let mut point_indices_per_layer: Vec<Vec<(usize, usize)>> = Vec::new();

        // Initialize the list, by pushing empty lists for every level curve
        for _lc in &model_constructor.level_curve_map.level_curves {
            point_indices_per_layer.push(Vec::new());
        }

        // 1. Triangulate all the level-curves.
        let mut triangles_per_level_curve: Vec<Vec<(&Point, &Point, &Point)>> = Vec::new();

        for lc in &model_constructor.level_curve_map.level_curves {
            triangles_per_level_curve.push(Smoother::triangulate_level_curve(lc)?);
        }

        // 2. For every point in the raster, assign it to the right layer, according to this triangulation.
        

        // 3. Points that have not yet been assigned fall outside all level-curves, meaning they are in the lowest
        //    level of the mountain (the bottom of the mountain). This should be layer 0.

        Ok(Self {
            raster: model_constructor.raster,
            is_svc: &model_constructor.is_svc,
            point_indices_per_layer: point_indices_per_layer
        })

    }


    /// Method: triangulate all points in a single level-curve.
    /// 
    /// # Arguments:
    /// * level_curve: The level curve to apply triangulation on
    /// 
    /// # Returns:
    /// * A triple of Points, representing a triangle
    pub fn triangulate_level_curve(level_curve: &LevelCurve) -> Result<Vec<(&Point, &Point, &Point)>> {

        let mut result: Vec<(&Point, &Point, &Point)> = Vec::new();

        // Prebase: If this level-curve contains only three points, it is already triangulated.
        if level_curve.points.len() == 3 {
            result.push((
                &level_curve.points[0],
                &level_curve.points[1],
                &level_curve.points[2]
            ));

            return Ok(result);
        }

        // Construct a vertex of indices that are considered 'in the level curve' for this algorithm
        let mut indices_in_polygon: Vec<usize> = Vec::new();

        for i in 0..level_curve.points.len() {
            indices_in_polygon.push(i);
        }

        // 1. Loop over every point in the level curve
        for i in (0..indices_in_polygon.len()).rev() {

            // 1a.  Extract three points:
            //      pu: The point at index i-1
            //      pv: The point at index i
            //      pw: The point at index i+1

            let pu_index: usize;
            let pu: &Point;

            //  pv_index = i
            let pv: &Point = &level_curve.points[indices_in_polygon[i]];

            let pw_index: usize;
            let pw: &Point;

            // if pv is the first point in the list, pu must be the last point in the list
            if i == 0 {
                pu_index = indices_in_polygon.len()-1;
            } else {
                pu_index = i-1;
            }
            pu = &level_curve.points[indices_in_polygon[pu_index]];

            // if pv is the last point in the list, pw must be the first point in the list
            if i == indices_in_polygon.len() - 1 {
                pw_index = 0;
            } else {
                pw_index = i+1;
            }
            pw = &level_curve.points[indices_in_polygon[pw_index]];

            // 1b.  Determine the angle at pv
            let pv_angle = Triangle::angle_at_pv(pu, pv, pw);

            // 1c.  If the angle at pv is greater than PI, we can guarantee this triangle is not an ear
            if pv_angle > std::f32::consts::PI {
                continue;
            }

            // 1c---We can now guarantee that this triangle is a CANDIDATE for an ear
            // 1d.  Make sure no other point in the level curve is inside this triangle
            let tri: Triangle = Triangle::new(pv, pu, pw);
            let mut triangle_is_candidate = true;

            for j in 0..indices_in_polygon.len() {

                // Only determine points that are not part of this triangle
                if j == pu_index || j == i || j == pw_index {
                    continue;
                }

                // If this point is inside the triangle, this triangle is no longer a candidate
                if tri.contains_point(&level_curve.points[indices_in_polygon[j]]) {
                    triangle_is_candidate = false;
                    break;
                }

            }

            // 1d---If this triangle is no longer a candidate, skip this point
            if !triangle_is_candidate {
                continue;
            }

            // 1e   This triangle is an ear!! Add the triangle to the result vertex and remove pv from the level curve
            result.push((pu, pv, pw));
            indices_in_polygon.remove(i);

            // If there are only three points left in the list, break out of this loop
            if indices_in_polygon.len() == 3 {
                break;
            }

        }

        // After completing this triangulation, there are only three points left in the polygon.
        // Add these three points as the last index in the result
        if level_curve.points.len() == 3 {
            result.push((
                &level_curve.points[indices_in_polygon[0]],
                &level_curve.points[indices_in_polygon[1]],
                &level_curve.points[indices_in_polygon[2]]
            ));
        }


        // Return the resulting vector
        Ok(result)

    }

}