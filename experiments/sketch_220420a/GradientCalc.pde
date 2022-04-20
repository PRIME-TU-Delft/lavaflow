class GradientCalc {
  
  public ArrayList<ArrayList<LineSegment>> positions;
  
  public GradientCalc(ArrayList<ArrayList<LineSegment>> positions) {
    this.positions = positions;
  }
  
  public void drawGradients() {
    
    
    for (int i = 1; i < this.positions.size(); ++i) {
      
      PVector b = this.calcCenterOfCircle(this.positions.get(i-1));
      
      for (int j = 0; j < this.positions.get(i).size(); ++j) {
        PVector a = this.positions.get(i).get(j).from;
        
        this.drawGradientLine(a, b);
        
      }
      
    }
      
    
    
  }
  
  private PVector calcCenterOfCircle(ArrayList<LineSegment> pos) {
    
    PVector result = new PVector(0, 0);
    
    for (LineSegment s : pos) {
      result.x += s.from.x / pos.size();
      result.y += s.from.y / pos.size();
    }
    
    return result;
    
  }
  
  private void drawGradientLine(PVector a, PVector b) {
    stroke(100);
    
    line(a.x, a.y, b.x, b.y);
  }
  
  
}
