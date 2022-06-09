import type p5 from 'p5';

const STROKE_WIDTH = 4;

export default class Draggable {
	dragging: boolean = false;
	last_selected: boolean = true;
	drag_translated: boolean = false;
	x: number;
	y: number;
	old_x: number;
	old_y: number;
	too_close_to_crater: boolean = false;
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
		this.old_x = x;
		this.old_y = y;
		this.size = size;
	}

	/**
	 * Update the location of a marker if it's being dragged, and the mouse is inside the canvas
	 *
	 * @param p5 Instance of a p5 sketch
	 */
	update(p5: p5, avoid_points?: [number, number][], with_distance?: number) {
		if (!this.dragging) return;

		if (p5.mouseX <= 0 || p5.mouseY <= 0) return;
		if (p5.mouseX >= p5.width || p5.mouseY >= p5.height) return;

		this.x = p5.mouseX + this.offsetX;
		this.y = p5.mouseY + this.offsetY;

		// Translate above the user's finger
		this.translateForDragging();

		if (avoid_points != undefined && with_distance != undefined) {
			this.too_close_to_crater = false;

			for (let i = 0; i < avoid_points.length; i++) {
				let dx = avoid_points[i][0] - this.x;
				let dy = avoid_points[i][1] - this.y;
				let dist = Math.sqrt(dx*dx + dy*dy);
				if (dist <= with_distance) {
					this.too_close_to_crater = true;
					break;
				}
			}
		}
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

	translateForDragging() {
		if (this.dragging && !this.drag_translated) {
			this.y -= 50;
			this.drag_translated = true;
		}
	}

	translateBackAfterDragging() {
		if (this.drag_translated) {
			this.drag_translated = false;
		}
	}

	/**
	 * Visualize the marker, by drawing a rectangle at the marker's current position
	 *
	 * @param p5 Instance of a p5 sketch
	 */
	drawRect(p5: p5, markerSize: number) {

		this.translateForDragging();

		p5.stroke(0);
		p5.fill(255);
		p5.strokeWeight(STROKE_WIDTH);
		p5.rect(this.x - markerSize/2, this.y - markerSize/2, markerSize, markerSize);

		this.translateBackAfterDragging();
	}

	/**
	 * Visualize the marker by drawing a circle at the marker's current position
	 *
	 * @param p5 Instance of a p5 sketch
	 * @param markerSize The size in pixels of the marker to be drawn
	 */
	drawCircle(p5: p5, markerSize: number, index?: number) {

		this.translateForDragging();

		if (index != undefined) {
			p5.strokeWeight(0.5);
			p5.stroke(0);
			p5.fill(200);
			p5.rect(this.x, this.y - markerSize/2, 50, markerSize);

			p5.fill(0);
			p5.textAlign(p5.CENTER);
			p5.textSize(15);
			p5.text("#" + index, this.x + 30, this.y + 5);
		}

		// Set a white background
		p5.fill(255);

		// Draw a different color if this is the selected point
		if (this.last_selected) {
			p5.fill(0, 140, 220);
		}

		// Draw a red color if this point is too close to one of the avoid_points
		if (this.too_close_to_crater) {
			p5.noStroke();
			p5.fill(51);
			p5.rect(this.x - 80, this.y - 35, 160, 20);
			p5.strokeWeight(1);
			p5.fill(255);
			p5.textSize(15);
			p5.textAlign(p5.CENTER);
			p5.text("Too close to the crater", this.x, this.y - 20);

			p5.fill(255, 0, 0);
		}
				
		p5.stroke(0);
		p5.strokeWeight(STROKE_WIDTH);

		p5.ellipse(this.x, this.y, markerSize);

		this.translateBackAfterDragging();
	}

	/**
	 * Visualize the marker by drawing a triangle at the marker's current position
	 *
	 * @param p5 Instance of a p5 sketch
	 * @param markerSize The size in pixels of the marker to be drawn
	 */
	drawTriangle(p5: p5, markerSize: number) {

		this.translateForDragging();

		p5.stroke(0);
		p5.fill(255);
		p5.strokeWeight(STROKE_WIDTH);

		// corners in order: bottom left corner, bottom right corner, top corner in the center
		p5.triangle(
			this.x - markerSize/2,
			this.y - markerSize/2,
			this.x + markerSize/2,
			this.y - markerSize/2,
			this.x,
			this.y + markerSize/2
		);

		this.translateBackAfterDragging();
	}

	/**
	 * Visualize the marker by drawing a cross at the marker's current position
	 *
	 * @param p5 Instance of a p5 sketch
	 * @param markerSize The size in pixels of the marker to be drawn
	 */
	drawCross(p5: p5, markerSize: number) {

		this.translateForDragging();

		p5.stroke(0);
		p5.strokeWeight(STROKE_WIDTH * 2);
		// cross is drawn using two thick lines

		// draw line from top left to bottom right
		p5.line(
			this.x - markerSize/2,
			this.y - markerSize/2,
			this.x + markerSize/2,
			this.y + markerSize/2
		);

		// draw line from top right to bottom left
		p5.line(
			this.x + markerSize/2,
			this.y - markerSize/2,
			this.x - markerSize/2,
			this.y + markerSize/2
		);

		this.translateBackAfterDragging();
	}

	deselect() {
		this.last_selected = false;
	}

	/**
	 * Global mouse is pressed
	 * check if one of the boxes is being dragged
	 * @param p5 Instance of a p5 sketch
	 */
	pressed(p5: p5) {
		// Check if mouse is over this object when global mouse is pressed
		const xBounded = p5.mouseX > this.x - this.size/2 && p5.mouseX < this.x + this.size/2;
		const yBounded = p5.mouseY > this.y - this.size/2 && p5.mouseY < this.y + this.size/2;
		if (xBounded && yBounded) {
			// if so, set this object to be dragged
			this.dragging = true;
			this.offsetX = this.x - p5.mouseX;
			this.offsetY = this.y - p5.mouseY;
			this.old_x = this.x;
			this.old_y = this.y;
			this.last_selected = true;

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
		this.translateBackAfterDragging();

		if (this.too_close_to_crater) {
			this.x = this.old_x;
			this.y = this.old_y;
		}

	}
}
