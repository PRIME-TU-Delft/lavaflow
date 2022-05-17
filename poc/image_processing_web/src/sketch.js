import Draggable from './draggable.js';
import { detectCurves, drawCurves } from './detectCurves.js';
import { removePerspective } from './removePerspective.js';

import p5 from "p5";
import cv from "opencv-ts";


/** @type Draggable[] */
let points = [];

let img;
let opencvDummyImg = document.getElementById('imageSrc');
let inputElement = document.getElementById('fileInput');


const sketch = (p5) => {
	p5.setup = () => {
		p5.createCanvas(400, 400);

		const size = 120; // size of draggble surface
		points.push(new Draggable(25, 25, size));
		points.push(new Draggable(25, 300, size));
		points.push(new Draggable(300, 300, size));
		points.push(new Draggable(300, 25, size));
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
		}

		// Render all Draggable points
		for (let i = 0; i < points.length; i++) {
			points[i].update(p5); // update position
			points[i].draw(p5); // redraw
			drawLine(p5, points[i], points[(i + 1) % points.length]); // draw line between points
		}
	};


	// If the user presses/releases their mouse, signal this to all Draggable points
	p5.mousePressed = () => {
		for (let i = 0; i < points.length; i++) points[i].pressed(p5);
	};

	p5.mouseReleased = () => {
		for (let i = 0; i < points.length; i++) points[i].released(p5);
	};


	// Trigger: a file has been selected
	inputElement.addEventListener('change', (e) => {
		// Create an invisible dummy image for OpenCV to load the file from
		let imgURL = URL.createObjectURL(e.target.files[0]);
		opencvDummyImg.src = imgURL;

		// Load the uploaded image into p5 and show it on the canvas
		img = p5.loadImage(imgURL, i => {
			p5.resizeCanvas(i.width, i.height);
		});
		document.getElementById("perspective-button").style.display = 'block';  // Unhide button
	}, false);


	// Trigger: user has set the perspective markers and clicked the "warp perspective!" button
	document.getElementById("perspective-button").addEventListener('click', () => {
		let mat = cv.imread(opencvDummyImg);

		// Fetch the marker coordinates of the draggable buttons
		let markerCoords = [];
		for (let p of points) {
			markerCoords.push(p.x - p.offsetX);
			markerCoords.push(p.y - p.offsetY);
		}

		// Apply the perspective transformation using the selected marker coords
		let result = removePerspective(mat, markerCoords)
		cv.imshow('canvasOutput', result);
		mat.delete();
	});

}

new p5(sketch, document.getElementById("sketch"));
