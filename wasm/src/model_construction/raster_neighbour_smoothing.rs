use super::level_curves::LevelCurve;
use super::point::Point;
use super::constructor::ModelConstructor;
use miette::{Result, miette};


pub struct RasterNeighbourSmoothing {

}

impl RasterNeighbourSmoothing {

    fn get_neighbour_altitude(model_constructor: &mut ModelConstructor, row: usize, col: usize, alt_row: isize, alt_col: isize) -> Result<f32> {

        let row_n = row as isize + alt_row;
        let col_n = col as isize + alt_col;

        // If the considered neighbour falls outside of the raster, return altitude 0.0
        if row_n < 0 || col_n < 0 || row_n as usize >= model_constructor.raster.rows || col_n as usize >= model_constructor.raster.columns {
            return Ok(0.0);
        }

        // Else, find the neighbour and return its altitude value
        else {
            return Ok(model_constructor.raster.altitudes[row_n as usize][col_n as usize].ok_or_else(|| miette!("Altitude not present."))?);
        }

    }

    fn neighbour_is_svc(model_constructor: &mut ModelConstructor, row: usize, col: usize, alt_row: isize, alt_col: isize) -> bool {

        let row_n = row as isize + alt_row;
        let col_n = col as isize + alt_col;

        // If the considered neighbour falls outside of the raster, return altitude 0.0
        if row_n < 0 || col_n < 0 || row_n as usize >= model_constructor.raster.rows || col_n as usize >= model_constructor.raster.columns {
            return false;
        }

        // Else, find the neighbour and return its altitude value
        else {
            return model_constructor.is_svc[row_n as usize][col_n as usize];
        }

    }


    pub fn apply(model_constructor: &mut ModelConstructor, strength_positive: f32, strength_negative: f32, coverage: usize, svc_weight: usize, allow_svc_change: bool) -> Result<()> {

        let mut new_altitudes: Vec<Vec<Option<f32>>> = Vec::new();

        // Iterate over all the rows and columns in this raster
        for row in 0..model_constructor.raster.rows {

            // Push a new empty vector to 'new_altitudes'
            new_altitudes.push(Vec::new());

            for col in 0..model_constructor.raster.columns {

                // Push None for this entry in the new_altitudes, to make space
                new_altitudes[row].push(None);

                // For each of these cells in the raster, do the following:
                
                // 1. If this cell is an svc-cell, skip this cell.
                if model_constructor.is_svc[row][col] {
                    new_altitudes[row][col] = Some(model_constructor.raster.altitudes[row][col].ok_or_else(|| miette!("Altitude not present."))?);
                    
                    if !allow_svc_change {
                        continue;
                    }
                    
                }

                // 2. This cell is not svc. Select the specified amount of neighbors
                let mut neighbour_altitudes: Vec<f32> = Vec::new();

                // Loop over all the neighbours that fall within the rectangle of coverage x coverage
                for alt_row in -(coverage as isize)..(coverage as isize) {
                    for alt_col in -(coverage as isize)..(coverage as isize) {
                        // If both alt_row and alt_col are zero, don't include the altitude. (This is the current point we're considering)
                        if alt_row == 0 && alt_col == 0 {
                            continue;
                        }

                        // Compute the altitude of this particular neighbour and add it to the vector
                        neighbour_altitudes.push(RasterNeighbourSmoothing::get_neighbour_altitude(model_constructor, row, col, alt_row, alt_col)?);

                        if RasterNeighbourSmoothing::neighbour_is_svc(model_constructor, row, col, alt_row, alt_col) {
                            for _i in 0..svc_weight-1 {
                                neighbour_altitudes.push(RasterNeighbourSmoothing::get_neighbour_altitude(model_constructor, row, col, alt_row, alt_col)?);
                            }
                        }

                    }
                }

                // 3. Compute the average of the altitudes of the neighbours.
                let mut average_neighbour_altitude: f32 = 0.0;
                for neighbour in &neighbour_altitudes {
                    average_neighbour_altitude += neighbour;
                }
                average_neighbour_altitude /= neighbour_altitudes.len() as f32;

                // 4. Compute the difference between this average altitude and the current altitude for this point
                let mut deviation = average_neighbour_altitude - model_constructor.raster.altitudes[row][col].ok_or_else(|| miette!("Altitude not present."))?;

                // 5. Apply the provided strength factor to this deviation and add the deviation once to the new altitude
                if deviation >= 0.0 {
                    deviation *= strength_positive;
                } else {
                    deviation *= strength_negative;
                }

                new_altitudes[row][col] = Some(model_constructor.raster.altitudes[row][col].ok_or_else(|| miette!("Altitude not present."))? + deviation);

            }
        }

        // We have now finished computing the new altitudes, set the altitudes correctly in the model_constructor's raster
        model_constructor.raster.altitudes = new_altitudes;

        Ok(())
    }


    pub fn smooth_staircase_effect(model_constructor: &mut ModelConstructor, strength_positive: f32, strength_negative: f32, coverage: usize, svc_weight: usize, allow_svc_change: bool) -> Result<()> {

        let mut new_altitudes: Vec<Vec<Option<f32>>> = Vec::new();

        // Iterate over all the rows and columns in this raster
        for row in 0..model_constructor.raster.rows {

            // Push a new empty vector to 'new_altitudes'
            new_altitudes.push(Vec::new());

            for col in 0..model_constructor.raster.columns {

                // Push None for this entry in the new_altitudes, to make space
                new_altitudes[row].push(None);

                // For each of these cells in the raster, do the following:
                
                // 1. If this cell is an svc-cell, skip this cell.
                if model_constructor.is_svc[row][col] {
                    new_altitudes[row][col] = Some(model_constructor.raster.altitudes[row][col].ok_or_else(|| miette!("Altitude not present."))?);
                    
                    if !allow_svc_change {
                        continue;
                    }
                    
                }

                // 2. This cell is not svc. Select the specified amount of neighbors
                let mut neighbour_altitudes: Vec<f32> = Vec::new();

                // 2a. Compute the distance to the closest level-curve
                let mut closest_distance_to_level_curve = f32::MAX;

                let point_at_row_col = Point{x:(col as f32)*model_constructor.raster.column_width, y: (row as f32)*model_constructor.raster.row_height, z: 0.0};

                for current_lc in &model_constructor.level_curve_map.level_curves {
                    let current_dist = f32::powi(current_lc.find_closest_point_and_distance_on_level_curve(&point_at_row_col).1, 2);

                    if current_dist < closest_distance_to_level_curve {
                        closest_distance_to_level_curve = current_dist;
                    }
                }

                // Loop over all the neighbours that fall within the rectangle of coverage x coverage
                for alt_row in -(coverage as isize)..(coverage as isize) {
                    for alt_col in -(coverage as isize)..(coverage as isize) {
                        // If both alt_row and alt_col are zero, don't include the altitude. (This is the current point we're considering)
                        if alt_row == 0 && alt_col == 0 {
                            continue;
                        }

                        // If this point lies further away than 'closest_distance_to_level_curve', skip this neighbour
                        if Point::xy_dist_sqr(&point_at_row_col, &Point{x:(((col as isize) + alt_col) as f32)*model_constructor.raster.column_width, y: (((row as isize) + alt_row) as f32)*model_constructor.raster.row_height, z: 0.0}) > closest_distance_to_level_curve {
                            continue;
                        }

                        // Compute the altitude of this particular neighbour and add it to the vector
                        neighbour_altitudes.push(RasterNeighbourSmoothing::get_neighbour_altitude(model_constructor, row, col, alt_row, alt_col)?);

                        if RasterNeighbourSmoothing::neighbour_is_svc(model_constructor, row, col, alt_row, alt_col) {
                            for _i in 0..svc_weight-1 {
                                neighbour_altitudes.push(RasterNeighbourSmoothing::get_neighbour_altitude(model_constructor, row, col, alt_row, alt_col)?);
                            }
                        }

                    }
                }

                if !neighbour_altitudes.is_empty() {

                    // 3. Compute the average of the altitudes of the neighbours.
                    let mut average_neighbour_altitude: f32 = 0.0;
                    for neighbour in &neighbour_altitudes {
                        average_neighbour_altitude += neighbour;
                    }
                    average_neighbour_altitude /= neighbour_altitudes.len() as f32;

                    // 4. Compute the difference between this average altitude and the current altitude for this point
                    let mut deviation = average_neighbour_altitude - model_constructor.raster.altitudes[row][col].ok_or_else(|| miette!("Altitude not present."))?;

                    // 5. Apply the provided strength factor to this deviation and add the deviation once to the new altitude
                    if deviation >= 0.0 {
                        deviation *= strength_positive;
                    } else {
                        deviation *= strength_negative;
                    }

                    new_altitudes[row][col] = Some(model_constructor.raster.altitudes[row][col].ok_or_else(|| miette!("Altitude not present."))? + deviation);

                } else {

                    new_altitudes[row][col] = Some(model_constructor.raster.altitudes[row][col].ok_or_else(|| miette!("Altitude not present."))?);

                }

            }
        }

        // We have now finished computing the new altitudes, set the altitudes correctly in the model_constructor's raster
        model_constructor.raster.altitudes = new_altitudes;
        
        Ok(())

    }



    pub fn smooth_staircase_effect_without_border(model_constructor: &mut ModelConstructor, strength_positive: f32, strength_negative: f32, coverage: usize, svc_weight: usize, allow_svc_change: bool) -> Result<()> {

        let mut new_altitudes: Vec<Vec<Option<f32>>> = Vec::new();

        // Iterate over all the rows and columns in this raster
        for row in 0..model_constructor.raster.rows {

            // Push a new empty vector to 'new_altitudes'
            new_altitudes.push(Vec::new());

            for col in 0..model_constructor.raster.columns {

                // Push None for this entry in the new_altitudes, to make space
                new_altitudes[row].push(None);

                // For each of these cells in the raster, do the following:
                
                // 1. If this cell is an svc-cell, skip this cell.
                if model_constructor.is_svc[row][col] {
                    new_altitudes[row][col] = Some(model_constructor.raster.altitudes[row][col].ok_or_else(|| miette!("Altitude not present."))?);
                    
                    if !allow_svc_change {
                        continue;
                    }
                    
                }

                // 2. This cell is not svc. Select the specified amount of neighbors
                let mut neighbour_altitudes: Vec<f32> = Vec::new();

                // 2a. Compute the distance to the closest level-curve
                let point_at_row_col = Point{x:(col as f32)*model_constructor.raster.column_width, y: (row as f32)*model_constructor.raster.row_height, z: 0.0};

                let mut closest_two_levelcurves: (&LevelCurve, &LevelCurve) = (&model_constructor.level_curve_map.level_curves[0], &model_constructor.level_curve_map.level_curves[1]);
                let mut closest_two_distances: (f32, f32) = (model_constructor.level_curve_map.level_curves[0].dist_to_point(&point_at_row_col), model_constructor.level_curve_map.level_curves[1].dist_to_point(&point_at_row_col));

                // Go over every level curve in this set
                for i in 2..model_constructor.level_curve_map.level_curves.len() {
                    // Compute the distance from this level curve to this point
                    let current_dist = model_constructor.level_curve_map.level_curves[i].dist_to_point(&point_at_row_col);

                    // If this distance is smaller than both current level-curves, shift everything over
                    if current_dist < closest_two_distances.0 && current_dist < closest_two_distances.1 {
                        closest_two_levelcurves.1 = closest_two_levelcurves.0;
                        closest_two_levelcurves.0 = &model_constructor.level_curve_map.level_curves[i];

                        closest_two_distances.1 = closest_two_distances.0;
                        closest_two_distances.0 = current_dist;
                    }
                    // Else, if the distance is greater than the first level curve but smaller than the second, insert the new one
                    else if current_dist >= closest_two_distances.0 && current_dist < closest_two_distances.1 {
                        closest_two_levelcurves.1 = &model_constructor.level_curve_map.level_curves[i];

                        closest_two_distances.1 = current_dist;
                    }

                    // Else, the distance is larger than both current level curves, skip this one
                }

                // Go over every level curve in this set
                for i in 2..model_constructor.level_curve_map.level_curves.len() {
                    // Compute the distance from this level curve to this point
                    let current_dist = model_constructor.level_curve_map.level_curves[i].furthest_dist_to_point(&point_at_row_col);

                    // If this distance is smaller than both current level-curves, shift everything over
                    if current_dist < closest_two_distances.0 && current_dist < closest_two_distances.1 {
                        closest_two_levelcurves.1 = closest_two_levelcurves.0;
                        closest_two_levelcurves.0 = &model_constructor.level_curve_map.level_curves[i];

                        closest_two_distances.1 = closest_two_distances.0;
                        closest_two_distances.0 = current_dist;
                    }
                    // Else, if the distance is greater than the first level curve but smaller than the second, insert the new one
                    else if current_dist >= closest_two_distances.0 && current_dist < closest_two_distances.1 {
                        closest_two_levelcurves.1 = &model_constructor.level_curve_map.level_curves[i];

                        closest_two_distances.1 = current_dist;
                    }

                    // Else, the distance is larger than both current level curves, skip this one
                }

                // Compute the distance between these two level-curves
                let distance_between_level_curves = closest_two_distances.0 + closest_two_distances.1;

                // Loop over all the neighbours that fall within the rectangle of coverage x coverage
                for alt_row in -(coverage as isize)..(coverage as isize) {
                    for alt_col in -(coverage as isize)..(coverage as isize) {
                        // If both alt_row and alt_col are zero, don't include the altitude. (This is the current point we're considering)
                        if alt_row == 0 && alt_col == 0 {
                            continue;
                        }

                        let point_at_alt_row_col = Point{x:(((col as isize) + alt_col) as f32)*model_constructor.raster.column_width, y: (((row as isize) + alt_row) as f32)*model_constructor.raster.row_height, z: 0.0};

                        // If this point lies further away than 'closest_distance_to_level_curve', skip this neighbour
                        if closest_two_levelcurves.0.find_closest_point_and_distance_on_level_curve(&point_at_alt_row_col).1 > distance_between_level_curves
                            || closest_two_levelcurves.1.find_closest_point_and_distance_on_level_curve(&point_at_alt_row_col).1 > distance_between_level_curves {
                            continue;
                        }

                        // Compute the altitude of this particular neighbour and add it to the vector
                        neighbour_altitudes.push(RasterNeighbourSmoothing::get_neighbour_altitude(model_constructor, row, col, alt_row, alt_col)?);

                        if RasterNeighbourSmoothing::neighbour_is_svc(model_constructor, row, col, alt_row, alt_col) {
                            for _i in 0..svc_weight-1 {
                                neighbour_altitudes.push(RasterNeighbourSmoothing::get_neighbour_altitude(model_constructor, row, col, alt_row, alt_col)?);
                            }
                        }

                    }
                }

                if !neighbour_altitudes.is_empty() {

                    // 3. Compute the average of the altitudes of the neighbours.
                    let mut average_neighbour_altitude: f32 = 0.0;
                    for neighbour in &neighbour_altitudes {
                        average_neighbour_altitude += neighbour;
                    }
                    average_neighbour_altitude /= neighbour_altitudes.len() as f32;

                    // 4. Compute the difference between this average altitude and the current altitude for this point
                    let mut deviation = average_neighbour_altitude - model_constructor.raster.altitudes[row][col].ok_or_else(|| miette!("Altitude not present."))?;

                    // 5. Apply the provided strength factor to this deviation and add the deviation once to the new altitude
                    if deviation >= 0.0 {
                        deviation *= strength_positive;
                    } else {
                        deviation *= strength_negative;
                    }

                    new_altitudes[row][col] = Some(model_constructor.raster.altitudes[row][col].ok_or_else(|| miette!("Altitude not present."))? + deviation);

                } else {

                    new_altitudes[row][col] = Some(model_constructor.raster.altitudes[row][col].ok_or_else(|| miette!("Altitude not present."))?);

                }

            }
        }

        // We have now finished computing the new altitudes, set the altitudes correctly in the model_constructor's raster
        model_constructor.raster.altitudes = new_altitudes;
        
        Ok(())

    }


    pub fn smooth_near_svc(model_constructor: &mut ModelConstructor, strength: f32, coverage: usize) -> Result<()> {

        let mut new_altitudes: Vec<Vec<Option<f32>>> = Vec::new();

        // Iterate over all the rows and columns in this raster
        for row in 0..model_constructor.raster.rows {

            // Push a new empty vector to 'new_altitudes'
            new_altitudes.push(Vec::new());

            for col in 0..model_constructor.raster.columns {

                // Push None for this entry in the new_altitudes, to make space
                new_altitudes[row].push(None);

                // For each of these cells in the raster, do the following:
                
                // 1. If this cell is an svc-cell
                if model_constructor.is_svc[row][col] {
                    
                    // 2. This cell is not svc. Select the specified amount of neighbors
                    let mut neighbour_altitudes: Vec<f32> = Vec::new();

                    // Loop over all the neighbours that fall within the rectangle of coverage x coverage
                    for alt_row in -(coverage as isize)..(coverage as isize) {
                        for alt_col in -(coverage as isize)..(coverage as isize) {
                            // If both alt_row and alt_col are zero, don't include the altitude. (This is the current point we're considering)
                            if alt_row == 0 && alt_col == 0 {
                                continue;
                            }

                            // Compute the altitude of this particular neighbour and add it to the vector
                            neighbour_altitudes.push(RasterNeighbourSmoothing::get_neighbour_altitude(model_constructor, row, col, alt_row, alt_col)?);

                        }
                    }

                    // 3. Compute the average of the altitudes of the neighbours.
                    let mut average_neighbour_altitude: f32 = 0.0;
                    for neighbour in &neighbour_altitudes {
                        average_neighbour_altitude += neighbour;
                    }
                    average_neighbour_altitude /= neighbour_altitudes.len() as f32;

                    // 4. Compute the difference between this average altitude and the current altitude for this point
                    let mut deviation = average_neighbour_altitude - model_constructor.raster.altitudes[row][col].ok_or_else(|| miette!("Altitude not present."))?;

                    // 5. Apply the provided strength factor to this deviation and add the deviation once to the new altitude
                    deviation *= strength;

                    new_altitudes[row][col] = Some(model_constructor.raster.altitudes[row][col].ok_or_else(|| miette!("Altitude not present."))? + deviation);
                    
                } else {

                    new_altitudes[row][col] = Some(model_constructor.raster.altitudes[row][col].ok_or_else(|| miette!("Altitude not present."))?);

                }


            }


        }

        // We have now finished computing the new altitudes, set the altitudes correctly in the model_constructor's raster
        model_constructor.raster.altitudes = new_altitudes;

        Ok(())

    }


    pub fn elevate_mountain(model_constructor: &mut ModelConstructor, amount: f32, allow_svc_change: bool) -> Result<()> {

        // Iterate over all the rows and columns in this raster
        for row in 0..model_constructor.raster.rows {

            for col in 0..model_constructor.raster.columns {

                if !allow_svc_change && model_constructor.is_svc[row][col] {
                    continue;
                }

                model_constructor.raster.altitudes[row][col] = Some(model_constructor.raster.altitudes[row][col].ok_or_else(|| miette!("Altitude not present."))? + amount);

            }
        }

        Ok(())

    }


}