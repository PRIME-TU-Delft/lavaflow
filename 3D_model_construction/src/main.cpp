#include <iostream>
#include "Point/Point.h"

int main(int, char**) {
    
    Point a = Point(1, 1, 1);
    Point b = Point(1, 1, 1);

    std::cout << "Distance between a and b: " << Point::dist(a, b) << std::endl;

}
