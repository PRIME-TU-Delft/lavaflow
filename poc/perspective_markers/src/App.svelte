<script>
  import P5 from 'p5-svelte';
  import Draggable from './draggable.js';

  /** @type Draggable[] */
  let points = [];

  /**
   * Draw line with p5 from point1 (p2) to point2 (p2)
   * @param p5 - p5 instance
   * @param p1 - point1 with x and y coordinates and width and height
   * @param p2 - point2 with x and y coordinates and width and height
   */
  function drawLine(p5, p1, p2) {
    p5.line(p1.x + p1.w / 2, p1.y + p1.h / 2, p2.x + p2.w / 2, p2.y + p2.h / 2);
  }

  /**
   * Setup p5 instance
   * @param p5 - p5 instance
   */
  const sketch = (p5) => {
    // Run on startup
    p5.setup = function () {
      p5.createCanvas(400, 400);

      const size = 120; // size of draggble surface
      points.push(new Draggable(75, 50, size));
      points.push(new Draggable(50, 300, size));
      points.push(new Draggable(300, 300, size));
      points.push(new Draggable(225, 50, size));
    };

    // Run on every frame
    p5.draw = function () {
      p5.background(255);
      for (i = 0; i < points.length; i++) {
        points[i].update(p5); // update position
        points[i].draw(p5); // redraw
        drawLine(p5, points[i], points[(i + 1) % points.length]); // draw line between points
      }
    };

    // When global mouse is pressed
    p5.mousePressed = function () {
      for (i = 0; i < points.length; i++) points[i].pressed(p5);
    };

    // When global mouse is released
    p5.mouseReleased = function () {
      for (i = 0; i < points.length; i++) points[i].released(p5);
    };
  };
</script>

<P5 {sketch} />

<style>
  :global(canvas) {
    border: 1px solid red;
  }
</style>
