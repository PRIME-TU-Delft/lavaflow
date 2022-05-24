// Imports for WASM
use wasm_bindgen::prelude::wasm_bindgen;
// use wasm_bindgen::JsValue;

// Imports for miette
use miette::Result;

// Internal imports
use crate::model_construction::smoother::Smoother;


// Create a trait that will be used for the procedural macro 'SmoothingOperation'
pub trait SmoothingOperation {
	fn apply(&self, smoother: &mut Smoother) -> Result<()>;
}

#[wasm_bindgen]
pub struct ModelConstructionApi {
    // API properties
    pub rows: usize,
    pub columns: usize,
    pub border_size: f32,
    pub svc_distance: f32,
    pub catmull_clark_iterations: usize,

    // Private properties
}

// Implement the 'Default' trait for ModelConstructionApi
impl Default for ModelConstructionApi {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl ModelConstructionApi {

    //
    // PUBLIC API
    //

    /// Constructor
    /// 
    /// Setup the properties for the ModelConstructionApi and give them a default value.
    /// If the user of the API wants, the parameters for the algorithm can be changed by calling other methods afterwards.
    pub fn new() -> ModelConstructionApi {
        // The presented values below are the default values for the different parameters
        Self {
            rows: 10,
            columns: 10,
            border_size: 0.0,
            svc_distance: 10.0,
            catmull_clark_iterations: 0
        }
    }

    /// Parameter Setup: basic 3D construction algorithm
    /// 
    /// The parameters that are of interest for this algorithm are:
    /// - The number of raster rows
    /// - The number of raster columns
    /// - The size of the border around the mountain
    pub fn set_basic_parameters(&mut self, rows: usize, columns: usize, border_size: f32) {
        self.rows = rows;
        self.columns = columns;
        self.border_size = border_size;
    }

    /// Parameter Setup: SVC algorithm
    /// 
    /// The parameters that are of interest for this algorithm are:
    /// - The svc distance, also referred to as contour-margin.
    pub fn set_svc_parameter(&mut self, svc_distance: f32) {
        self.svc_distance = svc_distance;
    }

    /// Parameter Setup: Catmull Clark surface subdivision
    /// 
    /// The parameters tht are of interest for this algorithm are:
    /// - The number of iterations that should be performed.
    pub fn set_catmull_clark_parameter(&mut self, catmull_clark_iterations: usize) {
        self.catmull_clark_iterations = catmull_clark_iterations;
    }

    /// INIT: Perform the basic 3D construction algorithm and store the result
    /// 
    /// Before calling this method, the user should have setup all the desired parameters already.
    pub fn init(&self) {



    }

    pub fn perform_catmull_clark() {

    }


    //
    // PRIVATE FUNCTIONS
    //




}