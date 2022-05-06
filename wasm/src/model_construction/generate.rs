/// Supermethod that takes in an openCV tree and outputs an GTLF model.
///
/// # Arguments
///
/// * `tree`- input from the image processing step, a representation of level curves. To be converted to 3D model
/// * `contour_margin` - Margin that defines when a point is considered 'on' a contour line, high value results in more staircase-like appearance, low value might lead to innacurate result.
///  NOTE: margin must be above max(raster height, column width) so long as local_tin() is not implemented
/// * `plane_length`- y-axis measuremnt of the model to be generated
/// * `plane_width` - x-axis measuremnt of the model to be generated
/// * `columns` - desired number columns used for raster
/// * `rows` - desired number rows used for raster
/// * `altitude_step` - fixed increase in height per level curve
///
///
///
pub fn generate_3d_model<'a>( tree : &'a mut LevelCurveTree<'a>,  contour_margin: f64, plane_length: f64, plane_width: f64, columns : usize, rows: usize, altitude_step : f64, desired_dist: f64) -> Vec<Vec<f64>> {

        //convert openCV tree to levelCurveMap (input for construction algorithm)
        let level_curve_map = LevelCurveSet::new(altitude_step).transform_to_LevelCurveMap( tree, altitude_step, desired_dist, 0);

        //create raster based on given params
        let mut raster = Raster::new(plane_width/columns as f64, plane_length/rows as f64 , rows, columns );

        //create new modelConstructor (module containing 3D-model construction algorithm)
        let mut model_constructor = ModelConstructor::new(&mut raster, contour_margin, &level_curve_map);

        //determine heights
        model_constructor.construct();

        //get heights from raster
        let heights = &raster.altitudes;

        //convert height raster to flat list of x,y,z points for GLTF format
        //every cell had 4 corners, becomes two triangles
        let mut     final_points: Vec<(f64, f64, f64)> = Vec::new();
        for x in 0..raster.rows {
            for y in 0 ..raster.columns {
                //triangle 1
                //(0,0)
                final_points.push((columns  as f64 * raster.column_width, (rows - y) as f64 * raster.row_height, heights[x][y].unwrap()));
                //(0, 1)
                final_points.push((columns as f64 * raster.column_width, ((rows - y) as f64 + 1.0) * raster.row_height, heights[x][y + 1].unwrap()));
                //(1, 1)
                final_points.push(((columns  as f64 + 1.0) * raster.column_width, ((rows - y) as f64 + 1.0) * raster.row_height, heights[x + 1][y + 1].unwrap()));

                //triangle 2
                //(0, 0)
                final_points.push((columns as f64 * raster.column_width, (rows - y) as f64 * raster.row_height, heights[x][y].unwrap()));
                //(1, 0)
                final_points.push(((columns as f64  + 1.0) * raster.column_width, (rows - y) as f64 * raster.row_height, heights[x + 1][y].unwrap()));
                //(1, 1)
                final_points.push(((columns as f64 + 1.0) * raster.column_width, ((rows - y) as f64 + 1.0) * raster.row_height, heights[x][y+ 1].unwrap()));
            }
        }

        //TODO: add conversion to GLTF
         return Vec::new();

}
