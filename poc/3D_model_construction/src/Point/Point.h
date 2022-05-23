
class Point {

    public:
        // Create public members x,y and z for storing the position of the point
        float x;
        float y;
        float z;

        // Constructor
        Point(float x1=0, float y1=0, float z1=0);

        // Methods
        static float distSqr(Point a, Point b);
        static float dist(Point a, Point b);

};