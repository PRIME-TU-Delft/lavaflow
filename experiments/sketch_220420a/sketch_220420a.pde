
PVector lastMouse;

ArrayList<ArrayList<LineSegment>> positions;
//ArrayList<LineSegment> positions;
int currentList;

boolean currentlyDrawing;

void setup() {
  size(800, 600, P2D);
  pixelDensity(displayDensity());
  
  lastMouse = new PVector(mouseX, mouseY);
  
  positions = new ArrayList<>();
  positions.add(new ArrayList<LineSegment>());
  
  currentList = 0;
  
  currentlyDrawing = false;
  
}

void draw() {
  
  background(255);
 
  if (mouseMovedFarEnough() && currentlyDrawing) {
    positions.get(currentList).add(new LineSegment(lastMouse.x, lastMouse.y, mouseX, mouseY));
    
    lastMouse.x = mouseX;
    lastMouse.y = mouseY;
  }
  
  drawAllLineSegments();
  
}


//void addNewLineSection() {
  
//  positions.add(new LineSegment(lastMouse.x, lastMouse.y, mouseX, mouseY));
  
//}

void mousePressed() {
  
  if (currentlyDrawing) {
    currentlyDrawing = false;
    
    positions.get(currentList).add(new LineSegment(
      positions.get(currentList).get(0).from.x,
      positions.get(currentList).get(0).from.y,
      positions.get(currentList).get(positions.get(currentList).size()-1).to.x,
      positions.get(currentList).get(positions.get(currentList).size()-1).to.y
      ));
    
    positions.add(new ArrayList<LineSegment>());
    currentList++;
  } else {
    lastMouse.x = mouseX;
    lastMouse.y = mouseY;
    currentlyDrawing = true;
  }
  
}

void drawAllLineSegments() {
  for (ArrayList<LineSegment> l : positions) {
    for (LineSegment s : l) {
      s.draw();
    }
  }
}
  
  
boolean mouseMovedFarEnough() {
  PVector m = new PVector(mouseX, mouseY);
  
  if (m.dist(lastMouse) > 10) {
    return true;
  }
  
  return false;
}
