use super::raster::Raster;

#[derive(Debug)]
pub struct ModelConstructor {
    contourMargin: u64,

    isSVC : Vec<Vec<bool>>,

    raster : Raster
}

trait helperFunctions{
    fn initialize();
    fn checkSVC(p:  Vec<f64> , ix: i64, iy: i64 ) -> bool; 
    // fn getClostestCOntourPoint(p, Vec<f64>, );
    fn localTIN(p: Vec<f64>) ;
    fn calcHeightsNVCs() ;
} 
pub trait ConstructorFunct{
    //TODO; add levelCurveMap to parameter list
    fn constructMap(&self, raster: Raster, contourMargin: i64) -> &Raster;
} 

impl  ModelConstructor {

    fn constructMap(&self, raster: &Raster, contourMargin: i64) -> &Raster{ 

        let x = raster.columns;
        let y = raster.rows;
        self.isSVC = [[false; (x as *const u64) ] ; y].to_vec();
        return raster;
    }


}




