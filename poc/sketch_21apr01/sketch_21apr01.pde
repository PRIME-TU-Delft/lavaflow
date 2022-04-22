
final int mouseDistanceThreshold = 10;

ArrayList<ArrayList<Point>> contourMap;
ArrayList<Point> currentContour;
PVector lastMouse;

Engine3D engine;
PVector canvasRotation;
final float rotationFactor = 0.05;
boolean userInputFinished;

public void setup() {

    fullScreen(P3D);
    //size(1000, 800, P3D);
    pixelDensity(displayDensity());

    //smooth(8);

    //
    // Initialization
    //  

    contourMap = new ArrayList<>();
    currentContour = new ArrayList<>();
    lastMouse = new PVector(mouseX, mouseY);

    canvasRotation = new PVector(0, 0, 0);
    userInputFinished = false;

}


public void draw() {

    background(255);

    // As long as the user is inputting information, keep drawing
    if (!userInputFinished) {
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

    } else {

        // The user is now done drawing, show the 3D model and allow the user to rotate the canvas

        pushMatrix();

        if (keyPressed) {
            if (keyCode == UP) {
                canvasRotation.x += rotationFactor;
            } else if (keyCode == DOWN) {
                canvasRotation.x -= rotationFactor;
            } else if (keyCode == LEFT) {
                canvasRotation.z += rotationFactor;
            } else if (keyCode == RIGHT) {
                canvasRotation.z -= rotationFactor;
            }
        }

        // Translate to the middle of the canvas
        translate(width/2, height/2);

        rotateX(canvasRotation.x);
        rotateY(canvasRotation.y);
        rotateZ(canvasRotation.z);
        engine.draw();


        popMatrix();

    }

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

public void keyPressed() {


    if (key == ENTER) {
        // The user is now done drawing

        // Initialize the 3D engine
        //engine = new LowBuildup3D(contourMap, 50);
        engine = new PixelMap3D(contourMap, 50);

        engine.render();

        // Disable drawing
        userInputFinished = true;
    }


}