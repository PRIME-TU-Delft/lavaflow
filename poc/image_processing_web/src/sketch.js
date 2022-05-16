import Draggable from './draggable.js';
import p5 from "p5";
import * as opencv from "@juliavdkris/opencv-wasm";


/** @type Draggable[] */
let points = [];


const sketch = (p) => {
	p.setup = () => {
		p.createCanvas(400, 400);

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
		p.line(p1.x + p1.w / 2, p1.y + p1.h / 2, p2.x + p2.w / 2, p2.y + p2.h / 2);
	}

	p.draw = () => {
		p.background(255);
		for (let i = 0; i < points.length; i++) {
			points[i].update(p); // update position
			points[i].draw(p); // redraw
			drawLine(p, points[i], points[(i + 1) % points.length]); // draw line between points
		}
	};

	p.mousePressed = () => {
		for (let i = 0; i < points.length; i++) points[i].pressed(p);
	};

	p.mouseReleased = () => {
		for (let i = 0; i < points.length; i++) points[i].released(p);
	};
}

new p5(sketch, document.getElementById("sketch"));



let cv = await opencv;
document.getElementById('status').innerHTML = 'OpenCV.js is ready.';

let imgElement = document.getElementById('imageSrc');
let inputElement = document.getElementById('fileInput');
inputElement.addEventListener('change', (e) => {
	imgElement.src = URL.createObjectURL(e.target.files[0]);
}, false);
imgElement.onload = function () {
	let mat = cv.imread(imgElement);
	// replace values in this array with correct pixel coordinates for the image
	result = removePerspective(mat, [2522, 2876, 1401, 724, 187, 1461, 783, 3669])
	cv.imshow('canvasOutput', result);
	mat.delete();
};



