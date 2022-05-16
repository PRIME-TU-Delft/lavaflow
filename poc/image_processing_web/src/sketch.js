import Draggable from './draggable.js';
import { detectCurves, drawCurves } from './detectCurves.js';
import { removePerspective } from './removePerspective.js';

import p5 from "p5";
import cv from "opencv-ts";


/** @type Draggable[] */
let points = [];

let img;
let imgElement = document.getElementById('imageSrc');
let inputElement = document.getElementById('fileInput');


const sketch = (p5) => {
	p5.setup = () => {
		p5.createCanvas(400, 400);

		const size = 120; // size of draggble surface
		points.push(new Draggable(75, 50, size));
		points.push(new Draggable(50, 300, size));
		points.push(new Draggable(300, 300, size));
		points.push(new Draggable(225, 50, size));
	};

	/**
	 * Draw line with p5 from point1 (p2) to point2 (p2)
	 * @param p5 - p5 instance
	 * @param p1 - point1 with x and y coordinates and width and height
	 * @param p2 - point2 with x and y coordinates and width and height
	 */
	function drawLine(p, p1, p2) {
		p5.line(p1.x + p1.w / 2, p1.y + p1.h / 2, p2.x + p2.w / 2, p2.y + p2.h / 2);
	}

	p5.draw = () => {
		p5.background(255);
		if (img) {
			p5.image(img, 0, 0, img.width, img.height);
			// let img = p5.
		}

		for (let i = 0; i < points.length; i++) {
			points[i].update(p5); // update position
			points[i].draw(p5); // redraw
			drawLine(p5, points[i], points[(i + 1) % points.length]); // draw line between points
		}
	};

	p5.mousePressed = () => {
		for (let i = 0; i < points.length; i++) points[i].pressed(p5);
	};

	p5.mouseReleased = () => {
		for (let i = 0; i < points.length; i++) points[i].released(p5);
	};


	// A file has been selected
	inputElement.addEventListener('change', (e) => {
		let imgURL = URL.createObjectURL(e.target.files[0]);
		imgElement.src = imgURL;
		img = p5.loadImage(imgURL);
	}, false);
	imgElement.onload = function () {
		let mat = cv.imread(imgElement);
		// replace values in this array with correct pixel coordinates for the image
		result = removePerspective(mat, [2522, 2876, 1401, 724, 187, 1461, 783, 3669])
		cv.imshow('canvasOutput', result);
		mat.delete();
	};
}

new p5(sketch, document.getElementById("sketch"));
