<script lang="ts">
	/**
	 * Dragging marker over an image to transform it.
	 */
	import P5 from 'p5-svelte';
	import type p5 from 'p5';

	import Draggable from '$lib/data/draggable';
	import { rawImage } from '$lib/stores/imageStore';

	export let foregroundWidth: number; // Width of the foreground canvas
	export let foregroundHeight: number; // Height of the foreground canvas

	export let points: Draggable[] = [];
	let image: p5.Image;

	const sketch = (p5: p5) => {
		// Preload image to use in p5
		p5.preload = () => {
			if ($rawImage) image = p5.loadImage($rawImage);
		};

		p5.setup = () => {
			p5.createCanvas(foregroundWidth, foregroundHeight);

			const size = 100; // size of draggble surface

			points.push(new Draggable(foregroundWidth * 0.1, foregroundHeight * 0.1, size)); // []
			points.push(new Draggable(foregroundWidth * 0.9, foregroundHeight * 0.1, size)); // ><
			points.push(new Draggable(foregroundWidth * 0.9, foregroundHeight * 0.9, size)); // /\
			points.push(new Draggable(foregroundWidth * 0.1, foregroundHeight * 0.9, size)); // ()
		};

		/**
		 * Draw line with p5 from point1 (p2) to point2 (p2)
		 * @param p1 - point1 with x and y coordinates and width and height
		 * @param p2 - point2 with x and y coordinates and width and height
		 */
		function drawLine(p1: Draggable, p2: Draggable) {
			p5.strokeWeight(10);
			p5.line(p1.x, p1.y, p2.x, p2.y);
		}

		p5.draw = () => {
			if (image) {
				p5.image(image, 0, 0, foregroundWidth, foregroundHeight);
			}

			// Render all Draggable points
			for (let i = 0; i < points.length; i++) {
				points[i].update(p5); // update position

				drawLine(points[i], points[(i + 1) % points.length]); // draw line between points
			}

			let markerSize = 20;
			points[0].drawRect(p5, markerSize);
			points[1].drawCross(p5, markerSize);
			points[2].drawTriangle(p5, markerSize);
			points[3].drawCircle(p5, markerSize);
		};

		// If the user presses/releases their mouse, signal this to all Draggable points
		p5.mousePressed = () => {
			for (let i = 0; i < points.length; i++) points[i].pressed(p5);
		};

		p5.mouseReleased = () => {
			for (let i = 0; i < points.length; i++) points[i].released();
		};
	};
</script>

<P5 {sketch} />
