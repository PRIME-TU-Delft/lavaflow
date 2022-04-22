class PixelMap3D implements Engine3D {

    private ArrayList<ArrayList<Point>> contourMap;
    private ArrayList<PVector> flattenedContourMap;
    private float levelHeight;

    private Range xDimensions;
    private Range yDimensions;

    private ArrayList<ArrayList<PVector>> pixels;

    final private int precision = 6;
    final private int rasterSize = 30;

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
            weights[i] = 1/weights[i];
        }

        float result = 0;
        float sumOfWeights = 0;

        for (int i = 0; i < values.length; ++i) {
            sumOfWeights += weights[i];
            result += values[i] * weights[i];
        }

        return result / sumOfWeights;
    }

    public void render() {

        this.pixels = new ArrayList<>();

        for (int x = floor(this.xDimensions.min); x <= ceil(this.xDimensions.max); x += this.rasterSize) {
            this.pixels.add(new ArrayList<>());
            for (int y = floor(this.yDimensions.min); y <= ceil(this.yDimensions.max); y += this.rasterSize) {

                PVector currentPoint = new PVector(x, y);

                ArrayList<PVector> closest = this.findClosestContourPoints(currentPoint, this.precision);

                float[] heightValues = new float[this.precision];
                float[] sqrDistances = new float[this.precision];

                for (int i = 0; i < this.precision; ++i) {
                    heightValues[i] = closest.get(i).z;
                    //sqrDistances[i] = this.distSqr(currentPoint, closest.get(i));
                    sqrDistances[i] = currentPoint.dist(closest.get(i));
                }

                float alt = this.calcInvertedWeightedAverage(heightValues, sqrDistances);

                this.pixels.get(this.pixels.size()-1).add(new PVector(currentPoint.x, currentPoint.y, alt));

            }
        }

        this.renderingCompleted = true;

    }

    // private boolean anyThreadIsRunning(PixelMap3DThread[] threadPool) {
    //     for (PixelMap3DThread t : threadPool) {
    //         if (t.isAlive()) {
    //             return true;
    //         }
    //     }

    //     return false;
    // }

    // public void render() {

    //     // Split the canvas into 8 sections

    //     int numThreads = 4;

    //     PixelMap3DThread[] threadPool = new PixelMap3DThread[numThreads];

    //     for (int t = 0; t < numThreads; ++t) {

    //         // Add a new thread to the pool and divide the work
    //         threadPool[t] = new PixelMap3DThread(this.flattenedContourMap, floor(this.xDimensions.min), ceil(this.xDimensions.max), floor(this.yDimensions.portion(numThreads, t)), floor(this.yDimensions.portion(numThreads, t+1)), this.rasterSize, this.precision);

    //         System.out.println("Starting thread: " + t);
    //         System.out.println("This thread will work from x=" + floor(this.xDimensions.min) + " to x=" + ceil(this.xDimensions.max));
    //         System.out.println("This thread will work from y=" + floor(this.yDimensions.portion(numThreads, t)) + " to y=" + floor(this.yDimensions.portion(numThreads, t+1)));

    //         // Start the thread
    //         threadPool[t].start();

    //     }

    //     // Wait for the threads to finish
    //     while(this.anyThreadIsRunning(threadPool)) {}

    //     try{Thread.sleep(10000);}catch(Exception e){}

    //     // Combine results
    //     this.pixels = new ArrayList<>();

    //     for (int t = 0; t < numThreads; ++t) {
    //         ArrayList<ArrayList<PVector>> result = threadPool[t].retrieveResult();

    //         System.out.println("Result: " + result);

    //         for (ArrayList<PVector> l : result) {
    //             this.pixels.add(l);
    //         }
    //     }

    //     this.renderingCompleted = true;


    // }
    
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
                    this.drawVertexForVector(this.pixels.get(i-1).get(j));
                    endShape(CLOSE);
                //}catch (Exception e) {}
            }
        }

    }

}