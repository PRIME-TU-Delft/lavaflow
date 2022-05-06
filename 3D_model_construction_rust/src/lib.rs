mod classes;
use classes::level_curves;
use classes::level_curves::*;
use classes::raster::*;
use classes::constructor::*;

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
pub fn generate_3D_model(  contour_margin: f64, plane_length: f64, plane_width: f64, columns : usize, rows: usize, altitude_step : f64, desired_dist: f64) -> Vec<Vec<f64>> {

        //TODO: add "tree : &'a mut LevelCurveTree<'a>" to param list once tree is merged to this branch

        //convert openCV tree to levelCurveMap (input for construction algorithm)
        //TODO: replace declaration of level curve map with call to transform_...
        //let level_curve_map = LevelCurveMap::transform_to_LevelCurveMap( tree, altitude_step, desired_dist, 0);
        let level_curve_map = LevelCurveMap::new(altitude_step);

        //create raster based on given params
        let mut raster = Raster::new(plane_width/columns as f64, plane_length/rows as f64 , rows, columns );

        //create new modelConstructor (module containing 3D-model construction algorithm)
        let mut model_constructor = ModelConstructor::new(&mut raster, contour_margin, &level_curve_map);
        //get heights in raster
        model_constructor.construct_map();
        let heights = raster.altitudes;

        //TODO: add conversion to GLTF
         return Vec::new();

}