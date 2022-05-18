import { getCurves } from "./detectCurves.js";

import cv from "opencv-ts";
import p5 from "p5";

/** @type Draggable[] */
let points = [];

let opencvDummyImg = document.getElementById("imageSrc");
let inputElement = document.getElementById("fileInput");


const sketch = (p5) => {
	p5.setup = () => {
		p5.createCanvas(1000, 1000);
	};


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

		let [contours, hierarchy] = getCurves(mat);

		contours.forEach((contour, index) => {
			// Add text to the contour: "index (index of parent)"
			p5.text(`${index} (${hierarchy[index]})`, contour[0], contour[1]);

			// `contour` is a flattened array of coordinates, so group them by pairs of two when iterating over it
			for (let i = 0; i < contour.length; i += 2) {
				let x = contour[i];
				let y = contour[i+1];
				p5.point(x, y);
			}
		});
	});
};


new p5(sketch, document.getElementById("sketch"));
