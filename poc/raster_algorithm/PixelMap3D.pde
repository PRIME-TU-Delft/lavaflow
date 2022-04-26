class PixelMap3D implements Engine3D {

    private ArrayList<ArrayList<Point>> contourMap;
    private ArrayList<PVector> flattenedContourMap;
    private float levelHeight;
    
    private boolean[][] isSVC;
    private float[][] knownHeights;
    final private int contourMargin = 700;
    private int cellNoX;
    private int cellNoY;

    private Range xDimensions;
    private Range yDimensions;

    private ArrayList<ArrayList<PVector>> pixels;

  //  final private int precision = 4;
    final private int rasterSize = 35;

    private boolean renderingCompleted;

    public PixelMap3D(ArrayList<ArrayList<Point>> contourMap, float levelHeight) {
        this.contourMap = contourMap;
        this.levelHeight = levelHeight;

        this.renderingCompleted = false;

        // Construct the flattened contour map
        this.constructFlattenedContourMap();
    }

    private void constructFlattenedContourMap() {

        this.flattenedContourMap = new ArrayList<>();
        this.xDimensions = new Range(0, 0);
        this.yDimensions = new Range(0, 0);
        // For every Point in the contourMap, create a PVector and include the correct height.
        for (int i = 0; i < this.contourMap.size(); ++i) {
            for (Point p : this.contourMap.get(i)) {
                this.flattenedContourMap.add(new PVector(p.pos.x - width/2, p.pos.y - height/2, (i+1) * this.levelHeight));
                if (p.pos.x - width/2 < this.xDimensions.min) {
                    this.xDimensions.min = p.pos.x - width/2;
                } else if (p.pos.x - width/2 > this.xDimensions.max) {
                    this.xDimensions.max = p.pos.x - width/2;
                }
                if (p.pos.y - height/2 < this.yDimensions.min) {
                    this.yDimensions.min = p.pos.y - height/2;
                } else if (p.pos.y - height/2 > this.yDimensions.max) {
                    this.yDimensions.max = p.pos.y - height/2;
                }
            }
        }

    }

    private float sqr(float x) {
        return x*x;
    }

    private float distSqr(PVector a, PVector b) {
        return this.sqr(b.x - a.x) + this.sqr(b.y - a.y);
    }

    private ArrayList<PVector> findClosestContourPoints(PVector pos, int amount) {

        ArrayList<PVector> result = new ArrayList<>();
        ArrayList<Float> distances = new ArrayList<>();

        // Populate the first points
        for (int i = 0; i < amount; ++i) {
            result.add(this.flattenedContourMap.get(i));
            distances.add(this.distSqr(pos, result.get(i)));
        }

        for (PVector p : this.flattenedContourMap) {
            float currentDist = this.distSqr(pos, p);
            for (int i = 0; i < amount; ++i) {

                if (currentDist < distances.get(i)) {
                    // Shift the point one over
                    result.add(i, p);
                    distances.add(i, currentDist);
                    break;
                }
                
            }
        }

        return result;

    }

    private float calcInvertedWeightedAverage(float[] values, float[] weights) {
        if (values.length != weights.length) {
            return 0;
        }
        
        for (int i = 0; i < weights.length; ++i) {
          if(weights[i] != 0){
            weights[i] = 1/weights[i];
          }
            
        }

        float result = 0;
        float sumOfWeights = 0;

        for (int i = 0; i < values.length; ++i) {
            sumOfWeights += weights[i];
            result += values[i] * weights[i];
        }
        if(sumOfWeights != 0){
          return result / sumOfWeights;
        } else {
           return 0.0;
        }
        
        
    }
    
    public void initialize(){
      
      
      
      //calculate total number of cells in x and y dir
      this.cellNoX = ceil((this.xDimensions.max  - this.xDimensions.min ) / rasterSize);
      this.cellNoY = ceil((this.yDimensions.max  - this.yDimensions.min ) / rasterSize);
      
      
      this.pixels = new ArrayList<ArrayList<PVector>>();
      
       this.isSVC = new boolean[cellNoX][cellNoY];
       this.knownHeights = new float[cellNoX][cellNoY];
       for (int i  = 0 ; i< cellNoX ; i++){
           for (int j  = 0 ; j< cellNoY ; j++){
                // edges of map should have height 0
                 if(i == 0 || j == 0 || i ==cellNoX - 1 || j == cellNoY -1){
                    this.isSVC[i][j] = true;
                    this.knownHeights[i][j] = 0;
                 }else {
                   this.knownHeights[i][j] = -1;
                   this.isSVC[i][j] = false;
                 }
                 
           }
        }
    }
    
    //given p is (0, 0) corner of cell
    public boolean checkSVC( PVector p, int ix, int iy ) {
      
       PVector centre = new PVector(p.x + (rasterSize/2), p.y + (rasterSize/2));
       ArrayList<PVector> closestP = findClosestContourPoints( centre , 1);
       
       //if dist point in within boundaries of box (if a<x<b , c<y<d) -> then at leastSVC, else ret false
       
       if(closestP.isEmpty()){
         return false;
       }
       
       PVector n = closestP.get(0);
       
       if(n.x < p.x || n.x > (p.x + this.rasterSize) || n.y < p.y || n.y > (p.y + this.rasterSize) ){
          return false;
       }
       
       //if its in the box check if goes through centre
       
       if(abs(centre.x - n.x  ) < contourMargin  &&  abs(centre.y - n.y  ) < contourMargin ){
         //assume point returned by findClosestPoint gives z cooridinate
           this.isSVC[ix][iy] = true;
           knownHeights[ix][iy] = n.z;
           return true;
       }
       
       this.isSVC[ix][iy] = true;
       return true;
    }
    
    
    public float localTIN(PVector p){

        return 10;
    }
    

    
    public void calcHeightsNVCs(){
        //run through NVCs and interpolate their heights
      for (int i  = 0 ; i< cellNoX ; i++){
           for (int j  = 0 ; j< cellNoY ; j++){
             
                   if( knownHeights[i][j] == -1){
                  
                       //search in 8 directions for SVC neighbor
                       //neighbor[a][0] = height
                       //neighbor[a][1] = weight (units till svc found)
                       float[][] neighbors = { findSvcN(i,j), findSvcNE(i,j), findSvcE(i,j), findSvcSE(i,j), findSvcS(i,j) , findSvcSW(i, j) , findSvcW(i, j), findSvcNW(i, j)} ;
                       
                       float[] weights = new float[8];
                       float[] values = new float[8];
                       
                       for (int m = 0 ; m < 8; m++){
                         weights[m] = neighbors[m][1];
                         values[m] = neighbors[m][0];
                         System.out.println("weight = " +neighbors[m][1]+ "val = " + neighbors[m][0]) ;
                       }
                        
                        knownHeights[i][j] = calcInvertedWeightedAverage(values, weights);
                        System.out.println(knownHeights[i][j]) ;
                        
                     
                }
                
            }
        }
    }
    
    public void render() {
        initialize();

        //run through + determine SVC's and their heights
          for (int i  = 0 ; i< cellNoX ; i++){
                System.out.println();
               for (int j  = 0 ; j< cellNoY ; j++){
                 
                 float x = this.xDimensions.min + i *rasterSize;
                 float y = this.yDimensions.min + j * rasterSize;
                 
                 PVector currentPoint = new PVector( x, y);
                
                
                if(checkSVC(currentPoint, i, j)){
                  
                    if(knownHeights[i][j] == -1){
                      
                       localTIN(currentPoint);
                       
                    }
                }
            }
        }
        calcHeightsNVCs();
        System.out.println("found all height");
        
        //all heights should now be above zero
        
        //add end results to pixels
         for (int i  = 0 ; i< cellNoX ; i++){
           this.pixels.add(new ArrayList<>());

            for (int j  = 0 ; j< cellNoY ; j++){
              
              //centre of cell should be added
                 float x = this.xDimensions.min + (i + 0.5) * rasterSize;
                 float y = this.yDimensions.min + (j + 0.5) * rasterSize;
  
                PVector newP = new PVector(x, y, knownHeights[i][j]);
                
               this.pixels.get(this.pixels.size()-1).add(newP);
              
            }
        }

        this.renderingCompleted = true;

    }


    private void drawVertexForVector(PVector p) {
        vertex(p.x, p.y, p.z);
    }

    public void draw() {

        if (!renderingCompleted) {
            return;
        }

        stroke(0);

        for (PVector p : this.flattenedContourMap) {
            translate(p.x, p.y, p.z);
            sphere(1);
            translate(-p.x, -p.y, -p.z);
        }

        stroke(0);
        strokeWeight(1);
        noFill();
        
        rect(this.xDimensions.min, this.yDimensions.min, this.xDimensions.length(), this.yDimensions.length());


        // Draw the gradient levels
        stroke(0);
        fill(150);
        
        for (int i = 1; i < this.pixels.size(); ++i) {
            for (int j = 1; j < this.pixels.get(i).size(); ++j) {
                //try{
                    beginShape();
                    this.drawVertexForVector(this.pixels.get(i).get(j));
                    this.drawVertexForVector(this.pixels.get(i).get(j-1));
                    this.drawVertexForVector(this.pixels.get(i-1).get(j-1));
                    endShape(CLOSE);
                    beginShape();
                    this.drawVertexForVector(this.pixels.get(i).get(j));
                    this.drawVertexForVector(this.pixels.get(i-1).get(j));
                    this.drawVertexForVector(this.pixels.get(i-1).get(j-1));
                    endShape(CLOSE);
                //}catch (Exception e) {}
            }
        }

    }
    //return arr of {height , stept to svc}
    public float[] findSvcN(int ix, int iy) {
          int x = ix;
            for (int y = iy; y < cellNoY; y ++) {

                if(this.isSVC[x][y] == true){
                  //TODO THROW EXCEPTION IF KNOWNHEIGHT -1
                  return new float[] { knownHeights[x][y] , abs(y - iy)};
                }
           }
           return new float[] {0.0, 0.0};
    }
    
    public float[] findSvcNE(int ix, int iy) {
          int x = ix;
          int y = iy;
          while(x < cellNoX && y < cellNoY){
              if(this.isSVC[x][y] == true){
                  //TODO THROW EXCEPTION IF KNOWNHEIGHT -1
                  return new float[] { knownHeights[x][y] , abs(y - iy)};
               
              }
              x++;
              y++;
          }
         return new float[] {0.0, 0.0};
    }
    
    public float[] findSvcE(int ix, int iy) {
          for (int x  = ix ; x< cellNoX ; x++){


                if(this.isSVC[x][iy] == true){
                  //TODO THROW EXCEPTION IF KNOWNHEIGHT -1
                  return new float[] { knownHeights[x][iy] , abs(x - ix)};
           }
       }
       return new float[] {0.0, 0.0};
    }
    public float[] findSvcSE(int ix, int iy) {
          int x = ix;
          int y = iy;
          while(x < cellNoX && y >= 0){
              if(this.isSVC[x][y] == true){
                  //TODO THROW EXCEPTION IF KNOWNHEIGHT -1
                  return new float[] { knownHeights[x][y] , abs(y - iy)};
               
              }
              x++;
              y--;
          }
         return new float[] {0.0, 0.0};
    }
    
    
    public float[] findSvcS(int ix, int iy) {
           
               for (int y  = iy ; y >= 0 ; y--){

                if(this.isSVC[ix][y] == true){
                  //TODO THROW EXCEPTION IF KNOWNHEIGHT -1
                  return new float[] { knownHeights[ix][y] , abs(y - iy)};
                }
       }
       return new float[] {0.0, 0.0};
    }
    
      public float[] findSvcSW(int ix, int iy) {
          int x = ix;
          int y = iy;
          while(x >= 0 && y >= 0){
              if(this.isSVC[x][y] == true){
                  //TODO THROW EXCEPTION IF KNOWNHEIGHT -1
                  return new float[] { knownHeights[x][y] , abs(y - iy)};
               
              }
              x--;
              y--;
          }
         return new float[] {0.0, 0.0};
    }
     public float[] findSvcW(int ix, int iy) {
               for (int x  = ix ; x > 0 ; x--){
                if(this.isSVC[x][iy] == true ) {
                  //TODO THROW EXCEPTION IF KNOWNHEIGHT -1
                    return new float[] { knownHeights[x][iy] , abs(x - ix)};
                }
       }
       return new float[] {0.0, 0.0};
    }
    public float[] findSvcNW(int ix, int iy) {
          int x = ix;
          int y = iy;
          while(x >= 0 && y < cellNoY){
              if(this.isSVC[x][y] == true){
                  //TODO THROW EXCEPTION IF KNOWNHEIGHT -1
                  return new float[] { knownHeights[x][y] , abs(y - iy)};
               
              }
              x--;
              y++;
          }
         return new float[] {0.0, 0.0};
    }

}
