class PixelMap3DThread extends Thread {

    private ArrayList<PVector> flattenedContourMap;
    private PVector from;
    private PVector to;
    private int rasterSize;
    private int precision;

    private ArrayList<ArrayList<PVector>> pixels;

    public PixelMap3DThread(ArrayList<PVector> flattenedContourMap, int fx, int fy, int tx, int ty, int rasterSize, int precision) {
        this.flattenedContourMap = flattenedContourMap;
        this.from = new PVector(fx, fy);
        this.to = new PVector(tx, ty);
        this.rasterSize = rasterSize;
        this.precision = precision;
    }

    private float sqr(float x) {
        return x*x;
    }

    private float distSqr(PVector a, PVector b) {
        return this.sqr(b.x - a.x) + this.sqr(b.y - a.y);
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

    public void run() {

        this.pixels = new ArrayList<>();

        for (int x = floor(this.from.x); x <= ceil(this.to.x); x += this.rasterSize) {
            this.pixels.add(new ArrayList<>());
            for (int y = floor(this.from.y); y <= ceil(this.to.y); y += this.rasterSize) {

                PVector currentPoint = new PVector(x, y);

                ArrayList<PVector> closest = this.findClosestContourPoints(currentPoint, this.precision);

                float[] heightValues = new float[this.precision];
                float[] sqrDistances = new float[this.precision];

                for (int i = 0; i < this.precision; ++i) {
                    heightValues[i] = closest.get(i).z;
                    sqrDistances[i] = this.distSqr(currentPoint, closest.get(i));
                }

                float alt = this.calcInvertedWeightedAverage(heightValues, sqrDistances);

                this.pixels.get(this.pixels.size()-1).add(new PVector(currentPoint.x, currentPoint.y, alt));

            }
        }

        System.out.println("Thread is done computing, result: " + this.pixels);

    }

    public ArrayList<ArrayList<PVector>> retrieveResult() {
        return this.pixels;
    }

}