#include "Point.h"
#include <math.h>

Point::Point(float x1, float y1, float z1) {
    x = x1;
    y = y1;
    z = z1;
}

float Point::distSqr(Point a, Point b) {
    return pow(b.x-a.x, 2) + pow(b.y-a.y, 2) + pow(b.z-a.z, 2);
}

float Point::dist(Point a, Point b) {
    return sqrt(Point::distSqr(a, b));
}