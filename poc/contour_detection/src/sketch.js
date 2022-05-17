import { detectCurves, drawCurves } from './detectCurves.js';

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

	};



	p5.draw = () => {
		p5.background(255);
		if (img) {
			p5.image(img, 0, 0, img.width, img.height);
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

		// Apply the perspective transformation using the selected marker coords
		let result = drawCurves(mat)
		cv.imshow('canvasOutput', result);
		mat.delete();
	});

}

new p5(sketch, document.getElementById("sketch"));
