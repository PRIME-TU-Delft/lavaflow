class LowBuildup3D implements Engine3D {

    private ArrayList<ArrayList<Point>> contourMap;
    private float levelHeight;

    public LowBuildup3D(ArrayList<ArrayList<Point>> contourMap, float levelHeight) {
        this.contourMap = contourMap;
        this.levelHeight = levelHeight;
    }

    /**
     * Generate the height information for a given layer
     */
    private ArrayList<PVector> generateLayer(int layer) {
      
        // Retrieve the current layer information from the contour map
        ArrayList<Point> currentLayer = this.contourMap.get(layer);

        // Initialize result
        ArrayList<PVector> result = new ArrayList<>();

        // For every point in the current layer
        for (Point p : currentLayer) {
            // Create a vector that includes the correct height
            result.add(new PVector(p.pos.x, p.pos.y, (layer + 1) * this.levelHeight));
        }

        // Return the list
        return result;

    } 


    private ArrayList<PVector> translateLayerToMiddleOfCanvas(ArrayList<PVector> layer) {

        // Apply the transformation on x- and y-axes
        for (int i = 0; i < layer.size(); ++i) {
            layer.get(i).x -= width/2;
            layer.get(i).y -= height/2;
        }

        // Return result
        return layer;

    }

    public void render(){}

    /**
     * This function draws all the layers into a 3D model
     */
    public void draw() {

        boolean translateToMiddle = true;

        // For every layer
        for (int i = 0; i < this.contourMap.size(); ++i) {

            // Apply the 'generateLayer' function to retrieve 3D information for the current layer
            ArrayList<PVector> positions = this.generateLayer(i);

            // If necessary, apply translation
            if (translateToMiddle) {
                positions = this.translateLayerToMiddleOfCanvas(positions);
            }

            // Draw the shape
            stroke(0);
            strokeWeight(2);
            fill(150);


            // Draw the outside of the shape

            beginShape(TRIANGLE_STRIP);

            // Draw vertices for each point on level 0
            for (PVector p : positions) {
                vertex(p.x, p.y, 0);
                vertex(p.x, p.y, p.z);
            }

            // // Draw vertices for each point on the correct level
            // for (PVector p : positions) {
            //     vertex(p.x, p.y, p.z);
            // }

            endShape(CLOSE);


            // Draw the top of the shape

            beginShape();

            for (PVector p : positions) {
                vertex(p.x, p.y, p.z);
            }

            endShape(CLOSE);

        }

    }





}
