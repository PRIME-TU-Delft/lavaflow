import type p5 from 'p5';

export default class Draggable {
	dragging: boolean = false;
	x: number;
	y: number;
	size: number;
	offsetX: number = 0;
	offsetY: number = 0;

	/**
	 * Initialize a draggable marker at a given position
	 *
	 * The offset is used to compensate for the p5 canvas being offset on a webpage.
	 * This means that to get the marker coordinates relative to the p5 canvas,
	 * you will need to use `point.x - point.offsetX` and `point.y - point.offsetY`.
	 *
	 * @param {number} x initial x coordinate of the marker
	 * @param {number} y initial y coordinate of the marker
	 * @param {number} size height/width of the draggable surface in pixels
	 */
	constructor(x: number, y: number, size: number) {
		this.x = x;
		this.y = y;
		this.size = size;
	}

	/**
	 * Update the location of a marker if it's being dragged, and the mouse is inside the canvas
	 *
	 * @param p5 Instance of a p5 sketch
	 */
	update(p5: p5) {
		if (!this.dragging) return;

		if (p5.mouseX <= 0 || p5.mouseY <= 0) return;
		if (p5.mouseX >= p5.width || p5.mouseY >= p5.height) return;

		this.x = p5.mouseX + this.offsetX;
		this.y = p5.mouseY + this.offsetY;
	}

	/**
	 * Moves the point to the input x and y coordinates
	 *
	 * @param x  The x position the point should be moved to
	 * @param y  The y position the point should be moved to
	 */
	setPosition(x: number, y: number) {
		this.x = x;
		this.y = y;
	}

	/**
	 * Visualize the marker, by drawing a rectangle at the marker's current position
	 *
	 * @param p5 Instance of a p5 sketch
	 */
	drawRect(p5: p5, markerSize: number) {
		p5.stroke(0, 127);
		p5.fill(50, 127);
		p5.strokeWeight(1);
		p5.rectMode(p5.CENTER);

		p5.rect(this.x + this.size / 2, this.y + this.size / 2, markerSize, markerSize);
	}

	/**
	 * Visualize the marker by drawing a circle at the marker's current position
	 *
	 * @param p5 Instance of a p5 sketch
	 * @param markerSize The size in pixels of the marker to be drawn
	 */
	drawCircle(p5: p5, markerSize: number, index?: number) {
		p5.stroke(0, 12);
		p5.fill(50, 127);
		p5.strokeWeight(1);

		if (index != undefined) {
			p5.text('#' + index, this.x + this.size / 2, this.y + this.size / 2);
		}

		p5.circle(this.x + this.size / 2, this.y + this.size / 2, markerSize);
	}

	/**
	 * Visualize the marker by drawing a triangle at the marker's current position
	 *
	 * @param p5 Instance of a p5 sketch
	 * @param markerSize The size in pixels of the marker to be drawn
	 */
	drawTriangle(p5: p5, markerSize: number) {
		p5.stroke(0, 127);
		p5.fill(50, 127);
		p5.strokeWeight(1);

		// corners in order: bottom left corner, bottom right corner, top corner in the center
		p5.triangle(
			this.x + (this.size - markerSize) / 2,
			this.y + (this.size + markerSize) / 2,
			this.x + (this.size + markerSize) / 2,
			this.y + (this.size + markerSize) / 2,
			this.x + this.size / 2,
			this.y + (this.size - markerSize) / 2
		);
	}

	/**
	 * Visualize the marker by drawing a cross at the marker's current position
	 *
	 * @param p5 Instance of a p5 sketch
	 * @param markerSize The size in pixels of the marker to be drawn
	 */
	drawCross(p5: p5, markerSize: number) {
		p5.stroke(0, 127);
		p5.strokeWeight(7);
		// cross is drawn using two thick lines

		// draw line from top left to bottom right
		p5.line(
			this.x + (this.size - markerSize) / 2,
			this.y + (this.size + markerSize) / 2,
			this.x + (this.size + markerSize) / 2,
			this.y + (this.size - markerSize) / 2
		);

		// draw line from top right to bottom left
		p5.line(
			this.x + (this.size + markerSize) / 2,
			this.y + (this.size + markerSize) / 2,
			this.x + (this.size - markerSize) / 2,
			this.y + (this.size - markerSize) / 2
		);
	}

	/**
	 * Global mouse is pressed
	 * check if one of the boxes is being dragged
	 * @param p5 Instance of a p5 sketch
	 */
	pressed(p5: p5) {
		// Check if mouse is over this object when global mouse is pressed
		const xBounded = p5.mouseX > this.x && p5.mouseX < this.x + this.size;
		const yBounded = p5.mouseY > this.y && p5.mouseY < this.y + this.size;
		if (xBounded && yBounded) {
			// if so, set this object to be dragged
			this.dragging = true;
			this.offsetX = this.x - p5.mouseX;
			this.offsetY = this.y - p5.mouseY;

			return true;
		}

		return false;
	}

	/**
	 * Global mouse is released
	 * all boxes should stop being dragged
	 */
	released() {
		this.dragging = false;
	}
}
