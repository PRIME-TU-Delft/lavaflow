use std::thread::current;

use crate::utils::log;

use super::constructor::ModelConstructor;
use super::raster::Raster;
use super::level_curves::LevelCurve;
use super::point::Point;
use super::triangle::Triangle;
use miette::{Result, miette};


pub struct Smoother<'a> {
    raster: &'a mut Raster,
    is_svc: &'a Vec<Vec<bool>>,
    point_indices_per_layer: Vec<Vec<(usize, usize)>>,
    layer_is_top: Vec<bool>,
    altitude_of_top: Vec<f32>
}


impl<'a> Smoother<'a> {

    pub fn new(model_constructor: &'a mut ModelConstructor) -> Result<Self> {

        // Construct the list of point-indices per layer
        let mut point_indices_per_layer: Vec<Vec<(usize, usize)>> = Vec::new();
        let mut layer_is_top: Vec<bool> = Vec::new();
        let mut altitude_of_top: Vec<f32> = Vec::new();

        // Initialize the list, by pushing empty lists for every level curve
        for _lc in 0..model_constructor.level_curve_map.level_curves.len()+1 {
            point_indices_per_layer.push(Vec::new());
            layer_is_top.push(false);
            altitude_of_top.push(0.0);
        }

        // Create a vector of all pixels that have not yet been assigned a place
        let mut unassigned_points: Vec<(usize, usize)> = Vec::new();
        for row in 0..model_constructor.raster.rows {
            for col in 0..model_constructor.raster.columns {
                unassigned_points.push((row, col));
            }
        }

        // 1. Triangulate all the level-curves.
        let mut triangles_per_level_curve: Vec<(usize, Vec<(&Point, &Point, &Point)>, bool, f32)> = Vec::new();

        // for lc in &model_constructor.level_curve_map.level_curves {
        //     triangles_per_level_curve.push(Smoother::triangulate_level_curve(lc)?);
        // }

        let mut current_altitude: f32 = -10.0;
        let mut current_lc: usize = 0;

        for (i, lc) in model_constructor.level_curve_map.level_curves.iter().enumerate() {
            if lc.altitude > current_altitude {
                current_altitude = lc.altitude;
                current_lc = i;
            }
        }

        let mut drawn_lcs: Vec<usize> = Vec::new();

        loop {
            triangles_per_level_curve.push((
                current_lc,
                Smoother::triangulate_level_curve(&model_constructor.level_curve_map.level_curves[current_lc])?,
                model_constructor.level_curve_map.level_curves[current_lc].is_mountain_top,
                model_constructor.level_curve_map.level_curves[current_lc].altitude
            ));


            drawn_lcs.push(current_lc);

            current_altitude = -10.0;

            let mut found_new = false;

            for (i, lc) in model_constructor.level_curve_map.level_curves.iter().enumerate() {
                if lc.altitude > current_altitude && !drawn_lcs.contains(&i) {
                    current_altitude = lc.altitude;
                    current_lc = i;
                    found_new = true;
                }
            }

            if !found_new {
                break;
            }
        }

        // 2. For every point in the raster, assign it to the right layer, according to this triangulation.
        // Loop over all the level curves
        for (i, triangle_set) in triangles_per_level_curve.iter().enumerate() {
            // Loop over all unassigned points
            for j in (0..unassigned_points.len()).rev() {
                let row = unassigned_points[j].0;
                let col = unassigned_points[j].1;

                let raster_pixel = model_constructor.raster.get_pixel(row, col);

                let p = Point{
                    x: raster_pixel.0,
                    y: raster_pixel.1,
                    z: 0.0
                };

                // If this point is in this level-curve, assign this point to the level-curve
                if Smoother::point_in_triangle_set(&triangle_set.1, &p) {
                    point_indices_per_layer[triangle_set.0+1].push((row, col));
                    layer_is_top[triangle_set.0+1] = triangle_set.2;
                    altitude_of_top[triangle_set.0+1] = triangle_set.3;
                    unassigned_points.remove(j);
                }
            }
        }

        // 4. Points that have not yet been assigned fall outside all level-curves, meaning they are in the lowest
        //    level of the mountain (the bottom of the mountain). This should be layer 0.
        for (row, col) in unassigned_points {
            point_indices_per_layer[0].push((row, col));
        }

        Ok(Self {
            raster: model_constructor.raster,
            is_svc: &model_constructor.is_svc,
            point_indices_per_layer: point_indices_per_layer,
            layer_is_top,
            altitude_of_top
        })

    }

    pub fn layers(&self) -> usize {
        self.point_indices_per_layer.len()
    }

    /// Method: get all (row, col) indices of points in a certain layer
    pub fn get_point_indices_in_level(&self, level: usize) -> Result<&Vec<(usize, usize)>> {
        if level < self.point_indices_per_layer.len() {
            return Ok(&self.point_indices_per_layer[level]);
        }

        Err(miette!("Requested points of non-existing layer."))
    }


    /// Method: triangulate all points in a single level-curve.
    /// 
    /// # Arguments:
    /// * level_curve: The level curve to apply triangulation on
    /// 
    /// # Returns:
    /// * A triple of Points, representing a triangle
    pub fn triangulate_level_curve(level_curve: &LevelCurve) -> Result<Vec<(&Point, &Point, &Point)>> {

        // Prebase: If this level-curve contains less than three points, throw an exception
        if level_curve.points.len() < 3 {
            return Err(miette!("Level Curve contains less than three points."));
        }

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


    /// Method: check if a point falls within a set of triangles
    fn point_in_triangle_set(triangle_set: &Vec<(&Point, &Point, &Point)>, p: &Point) -> bool {

        // Loop over all the triangles
        for i in 0..triangle_set.len() {

            let a = triangle_set[i].0;
            let b = triangle_set[i].1;
            let c = triangle_set[i].2;

            // Create a triangle from these three points
            let tri = Triangle::new(a, b, c);

            // If this triangle contains the point, set 'result' to true and break out of the loop
            if tri.contains_point(p) {
                return true;
            }

        }

        false

    }



    //
    // SMOOTHING ALGORITHMS
    //

    fn get_neighbour_altitude(&self, row: usize, col: usize, alt_row: isize, alt_col: isize) -> Result<f32> {

        let row_n = row as isize + alt_row;
        let col_n = col as isize + alt_col;

        // If the considered neighbour falls outside of the raster, return altitude 0.0
        if row_n < 0 || col_n < 0 || row_n as usize >= self.raster.rows || col_n as usize >= self.raster.columns {
            return Ok(0.0);
        }

        // Else, find the neighbour and return its altitude value
        else {
            return Ok(self.raster.altitudes[row_n as usize][col_n as usize].ok_or_else(|| miette!("Altitude not present."))?);
        }

    }

    fn neighbour_is_svc(&self, row: usize, col: usize, alt_row: isize, alt_col: isize) -> bool {

        let row_n = row as isize + alt_row;
        let col_n = col as isize + alt_col;

        // If the considered neighbour falls outside of the raster, return altitude 0.0
        if row_n < 0 || col_n < 0 || row_n as usize >= self.raster.rows || col_n as usize >= self.raster.columns {
            return false;
        }

        // Else, find the neighbour and return its altitude value
        else {
            return self.is_svc[row_n as usize][col_n as usize];
        }

    }

    fn neighbour_belongs_to_layer(&self, layer: usize, row: usize, col: usize, alt_row: isize, alt_col: isize) -> bool {

        // If the specified layer doesn't exist, return error
        if layer >= self.point_indices_per_layer.len() {
            return false;
        }

        let row_n = row as isize + alt_row;
        let col_n = col as isize + alt_col;

        // If the considered neighbour falls outside of the raster, return altitude 0.0
        if row_n < 0 || col_n < 0 || row_n as usize >= self.raster.rows || col_n as usize >= self.raster.columns {
            return false;
        }

        // Else, determine whether this neighbour is part of the specified layer.
        else {
            return self.point_indices_per_layer[layer].contains(&(row_n as usize, col_n as usize));
        }

    }

    pub fn apply_smooth_to_layer(&mut self, layer: usize, strength: f32, coverage: usize, svc_weight: usize, allow_svc_change: bool) -> Result<()> {

        // If the specified layer doesn't exist, return error
        if layer >= self.point_indices_per_layer.len() {
            return Err(miette!("Specified layer does not exist."));
        }

        // Iterate over all (row, col) pairs
        for row in 0..self.raster.rows {
            for col in 0..self.raster.columns {

                // Skip this pair if it isn't part of this layer
                if !self.point_indices_per_layer[layer].contains(&(row, col)) {
                    continue;
                }

                // This point is part of the layer

                if self.is_svc[row][col] {
                    if !allow_svc_change {
                        continue;
                    }
                }
                
                // 1. Discover the neighbours
                let mut neighbour_altitudes: Vec<f32> = Vec::new();

                for alt_row in -(coverage as isize)..(coverage as isize) {
                    for alt_col in -(coverage as isize)..(coverage as isize) {

                        // Skip if this neighbour is of another layer
                        if !self.neighbour_belongs_to_layer(layer, row, col, alt_row, alt_col) {
                            continue;
                        }

                        // Compute the altitude of this particular neighbour and add it to the vector
                        neighbour_altitudes.push(self.get_neighbour_altitude(row, col, alt_row, alt_col)?);

                        if self.neighbour_is_svc(row, col, alt_row, alt_col) {
                            for i in 0..svc_weight-1 {
                                neighbour_altitudes.push(self.get_neighbour_altitude(row, col, alt_row, alt_col)?);
                            }
                        }

                    }
                }

                // 2. Compute the average of the altitudes of the neighbours.
                let mut average_neighbour_altitude: f32 = 0.0;
                for neighbour in &neighbour_altitudes {
                    average_neighbour_altitude += neighbour;
                }
                average_neighbour_altitude /= neighbour_altitudes.len() as f32;

                // 3. Compute the difference between this average altitude and the current altitude for this point
                let mut deviation = average_neighbour_altitude - self.raster.altitudes[row][col].ok_or_else(|| miette!("Altitude not present."))?;

                // 4. Apply the provided strength factor to this deviation and add the deviation once to the new altitude
                deviation *= strength;

                self.raster.altitudes[row][col] = Some(self.raster.altitudes[row][col].ok_or_else(|| miette!("Altitude not present."))? + deviation);
                
            }
        }

        Ok(())

    }

    pub fn apply_smooth_to_mountain_tops(&mut self, strength: f32, coverage: usize, svc_weight: usize, allow_svc_change: bool) -> Result<()> {

        for i in 0..self.layer_is_top.len() {
            if self.layer_is_top[i] {
                self.apply_smooth_to_layer(i, strength, coverage, svc_weight, allow_svc_change)?;
            }
        }

        Ok(())

    }

    pub fn increase_altitude_for_mountain_tops(&mut self, amount: f32) -> Result<()> {

        for i in 0..self.layer_is_top.len() {
            if self.layer_is_top[i] {
                self.set_altitude_for_layer(i, self.altitude_of_top[i] + amount)?;
            }
        }

        Ok(())

    }


    pub fn set_altitude_for_layer(&mut self, layer: usize, altitude: f32) -> Result<()> {

        // If the specified layer doesn't exist, return error
        if layer >= self.point_indices_per_layer.len() {
            return Err(miette!("Specified layer does not exist."));
        }

        // Iterate over all (row, col) pairs
        for row in 0..self.raster.rows {
            for col in 0..self.raster.columns {

                // Skip this pair if it isn't part of this layer
                if !self.point_indices_per_layer[layer].contains(&(row, col)) {
                    continue;
                }

                self.raster.altitudes[row][col] = Some(altitude);

            }
        }

        Ok(())

    }

}