use super::raster::Raster;

#[derive(Debug)]
pub struct Constructor {
    rasterSize : u64,
    contourMargin: u64,

    isSVC : Vec<Vec<bool>>,

    raster : Raster
}

trait helperFunctions{
    fn initialize();
    fn checkSVC(p:  Vec<f64> , ix: i64, iy: i64 ) -> bool;
    fn localTIN(p: Vec<f64>) ;
    fn calcHeightsNVCs() ;
} 
pub trait ConstructorFunct{
    //TODO; add levelCurveMap to parameter list
    fn constructMap(rastersize: i64, contourMargin: i64);
} 




