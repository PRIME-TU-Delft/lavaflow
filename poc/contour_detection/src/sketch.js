import { detectCurves, drawCurves } from './detectCurves.js';

import cv from "opencv-ts";


/** @type Draggable[] */
let points = [];

let img;
let opencvDummyImg = document.getElementById('imageSrc');
let inputElement = document.getElementById('fileInput');


// Trigger: a file has been selected
inputElement.addEventListener('change', (e) => {
	// Create an invisible dummy image for OpenCV to load the file from
	let imgURL = URL.createObjectURL(e.target.files[0]);
	opencvDummyImg.src = imgURL;

	// Unhide button
	document.getElementById("opencv-button").style.display = 'block';
}, false);


// Trigger: user has set the perspective markers and clicked the "warp perspective!" button
document.getElementById("opencv-button").addEventListener('click', () => {
	let mat = cv.imread(opencvDummyImg);

	// Apply the perspective transformation using the selected marker coords
	let result = drawCurves(mat)
	cv.imshow('canvasOutput', result);
	mat.delete();
});
