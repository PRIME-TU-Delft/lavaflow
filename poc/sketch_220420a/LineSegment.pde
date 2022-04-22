class LineSegment {
  
  public PVector from;
  public PVector to;
  
  public LineSegment(float fromx, float fromy, float tox, float toy) {
    this.from = new PVector(fromx, fromy);
    this.to = new PVector(tox, toy);
  }
  
  public void draw() {
    stroke(0);
    strokeWeight(2);
    line(this.from.x, this.from.y, this.to.x, this.to.y);
    
    pushMatrix();
    
    translate(this.from.x, this.from.y);
    rotate(atan((this.to.y - this.from.y) / (this.to.x - this.from.x)));
    
    stroke(100);
    
    //line(0, 0, 0, 50);
    
    popMatrix();
  }
  
}
