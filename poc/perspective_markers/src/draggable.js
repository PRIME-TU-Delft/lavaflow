export default class Draggable {
  constructor(x, y, size) {
    this.dragging = false; // is this object being dragged
    this.rollover = false; // TODO: remove
    this.x = x;
    this.y = y;
    this.w = size;
    this.h = size;
    this.offsetX = 0;
    this.offsetY = 0;
  }

  update(p5) {
    if (!this.dragging) return;

    if (p5.mouseX <= 0 || p5.mouseY <= 0) return;
    if (p5.mouseX >= p5.width || p5.mouseY >= p5.height) return;

    this.x = p5.mouseX + this.offsetX;
    this.y = p5.mouseY + this.offsetY;
  }

  draw(p5) {
    // viualize this object
    p5.stroke(0);
    p5.fill(50);
    const indicatorSize = 20;

    p5.rect(
      this.x + (this.w - indicatorSize) / 2,
      this.y + (this.h - indicatorSize) / 2,
      indicatorSize,
      indicatorSize
    );
  }

  pressed(p5) {
    // Check if mouse is over this object when global mouse is pressed
    const xBounded = p5.mouseX > this.x && p5.mouseX < this.x + this.w;
    const yBounded = p5.mouseY > this.y && p5.mouseY < this.y + this.h;
    if (xBounded && yBounded) {
      // if so, set this object to be dragged
      this.dragging = true;
      this.offsetX = this.x - p5.mouseX;
      this.offsetY = this.y - p5.mouseY;
    }
  }

  released() {
    this.dragging = false;
  }
}
