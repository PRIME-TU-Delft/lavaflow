
final int mouseDistanceThreshold = 10;

ArrayList<ArrayList<Point>> contourMap;
ArrayList<Point> currentContour;
PVector lastMouse;

public void setup() {

    fullScreen(P3D);
    pixelDensity(displayDensity());

    smooth(8);

    //
    // Initialization
    //  

    contourMap = new ArrayList<>();
    currentContour = new ArrayList<>();
    lastMouse = new PVector(mouseX, mouseY);

}


public void draw() {

    background(255);

    if (mousePressed) {

        // The user is now pressing the mouse
        // Add new points to the contour map
        if (mouseFarEnoughAway()) {
            // If the mouse has traveled far away (prevent adding millions of points)
            // Add a new point
            currentContour.add(new Point(mouseX, mouseY));

            // Update the last mouse location
            lastMouse.x = mouseX;
            lastMouse.y = mouseY;
        }

    } else {

        // The user is now not pressing the mouse
        // If the currentContour array is full, add this new contour to the contour map
        if (currentContour.size() > 0) {
            contourMap.add(currentContour);

            // And clear out the new drawing-pad
            currentContour = new ArrayList<>();
        }

    }

    // Draw the contour map
    drawContourMap();

    // Draw the current contour
    drawCurrentContour();

}

public boolean mouseFarEnoughAway() {
    PVector m = new PVector(mouseX, mouseY);

    if (m.dist(lastMouse) > mouseDistanceThreshold) {
        return true;
    }
    
    return false;
}

public void drawCurrentContour() {
    stroke(150);
    strokeWeight(2);
    noFill();

    beginShape();

    for (Point p : currentContour) {
        vertex(p.pos.x, p.pos.y);
    }

    endShape(CLOSE);
}

public void drawContourMap() {

    for (ArrayList<Point> level : contourMap) {
        stroke(0);
        strokeWeight(2);
        noFill();

        beginShape();

        for (Point p : level) {
            vertex(p.pos.x, p.pos.y);
        }

        endShape(CLOSE);

    }

}