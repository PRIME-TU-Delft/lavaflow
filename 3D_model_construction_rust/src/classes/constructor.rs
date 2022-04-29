#[derive(Debug)]



pub struct Constructor {
    rasterSize : i64,
    contourMargin: i64,

    xCells: i64,
    yCells: i64 ,

    xDimension : i64,
    yDimenstion : i64,


    isSVC : Vec<Vec<bool>>,
    //TODO: should be raster class
    heights : Vec<Vec<f64>>
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




