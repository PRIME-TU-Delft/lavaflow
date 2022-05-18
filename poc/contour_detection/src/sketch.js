import { detectCurves, drawCurves, getCurves } from "./detectCurves.js";

import cv from "opencv-ts";

/** @type Draggable[] */
let points = [];

let img;
let opencvDummyImg = document.getElementById("imageSrc");
let inputElement = document.getElementById("fileInput");

// Trigger: a file has been selected
inputElement.addEventListener(
	"change",
	(e) => {
		// Create an invisible dummy image for OpenCV to load the file from
		let imgURL = URL.createObjectURL(e.target.files[0]);
		opencvDummyImg.src = imgURL;

		// Unhide button
		document.getElementById("draw-contours-button").style.display = "block";
	},
	false
);

// Trigger: user clicked "draw contours" button
document.getElementById("draw-contours-button").addEventListener("click", () => {
	let mat = cv.imread(opencvDummyImg);

	// TODO: use p5 to show debug output
});
