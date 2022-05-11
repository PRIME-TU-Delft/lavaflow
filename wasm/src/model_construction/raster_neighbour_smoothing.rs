use super::level_curves::{LevelCurve, LevelCurveSet};
use super::point::Point;
use super::raster::Raster;
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


    pub fn apply(model_constructor: &mut ModelConstructor, strength: f32, coverage: usize) -> Result<()> {

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
                    continue;
                }

                // 2. This cell is not svc. Select the specified amount of neighbors
                let mut neighbour_altitudes: Vec<f32> = Vec::new();

                // Loop over all the neighbours that fall within the rectangle of coverage x coverage
                for alt_row in -((coverage as isize) / 2)..((coverage as isize) / 2) {
                    for alt_col in -((coverage as isize) / 2)..((coverage as isize) / 2) {
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

            }
        }

        // We have now finished computing the new altitudes, set the altitudes correctly in the model_constructor's raster
        model_constructor.raster.altitudes = new_altitudes;

        Ok(())
    }


}