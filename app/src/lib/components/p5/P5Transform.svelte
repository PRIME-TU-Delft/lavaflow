<script lang="ts">
	/**
	 * Dragging marker over an image to transform it.
	 */
	import Draggable from '$lib/data/draggable';
	import LavaError from '$lib/data/LavaError';
	import imageStore from '$lib/stores/imageStore';
	import sizeStore from '$lib/stores/sizeStore';
	import { createEventDispatcher } from 'svelte';
	import type p5 from 'p5';
	import P5 from './P5.svelte';
	import { get } from 'svelte/store';

	export let points: [Draggable, Draggable, Draggable, Draggable] | [] = [];

	export let detectSize: number = 30;
	export let markerSize: number = 10;

	const dispatch = createEventDispatcher();

	function dispatchError(title: string, message: string) {
		const error = new LavaError(title, message);
		dispatch('error', { error });
	}

	let image: p5.Image;
	let perspectiveImage: p5.Image;

	const sketch = (p5: p5) => {
		p5.preload = () => {
			if (!imageStore || !$imageStore) {
				return dispatchError('No template image found', 'Please try re-scanning the template');
			}

			image = p5.loadImage($imageStore);
		};

		p5.setup = () => {
			if (!sizeStore || !$sizeStore) {
				return dispatchError('No size found', 'Please try re-scanning the template');
			}

			const [width, height] = [p5.windowWidth, p5.windowHeight];

			p5.pixelDensity(p5.displayDensity())
			const cvs = p5.createCanvas(p5.windowWidth, p5.windowHeight);
			cvs.id('p5-transform');

			points = [
				new Draggable(width * 0.25, height * 0.25, detectSize), // [] Square shape
				new Draggable(width * 0.75, height * 0.25, detectSize), // >< Cross shape
				new Draggable(width * 0.75, height * 0.75, detectSize), // /\ Triangle shape
				new Draggable(width * 0.25, height * 0.75, detectSize) // () Circle shape
			];

			// Display an instruction at the rectangle
			points[0].setInstruction('Drag me to the circle\non the paper.', 190, 40, true);
			points[1].setInstruction('Drag me to the rectangle\non the paper.', 190, 40);
			points[2].setInstruction('Drag me to the cross\non the paper.', 190, 40);
			points[3].setInstruction('Drag me to the triangle\non the paper.', 190, 40);
		};

		p5.windowResized = () => {
			p5.resizeCanvas(p5.windowWidth, p5.windowHeight)

			points[0]?.setPosition(p5.windowWidth * 0.25, p5.windowHeight * 0.25)
			points[1]?.setPosition(p5.windowWidth * 0.75, p5.windowHeight * 0.25)
			points[2]?.setPosition(p5.windowWidth * 0.75, p5.windowHeight * 0.75)
			points[3]?.setPosition(p5.windowWidth * 0.25, p5.windowHeight * 0.75)
		}

		/**
		 * Draw line with p5 from point1 (p2) to point2 (p2)
		 * @param p1 - point1 with x and y coordinates and width and height
		 * @param p2 - point2 with x and y coordinates and width and height
		 */
		function drawLine(p1: Draggable, p2: Draggable) {
			p5.strokeWeight(markerSize);
			p5.line(p1.x, p1.y, p2.x, p2.y);
		}

		p5.draw = () => {

			p5.background(0, 0, 0)

			const [width, height] = [p5.windowWidth, p5.windowHeight];

			if (image) {
				p5.image(image, 0, 0, width, height);
			}

			// Render all Draggable points
			for (let i = 0; i < points.length; i++) {
				points[i].update(p5); // update position
				const $sizeStore = get(sizeStore);
				points[i].updateMappedCoordinates(p5, $sizeStore.width, $sizeStore.height)

				p5.stroke(0, 255, 0);
				drawLine(points[i], points[(i + 1) % points.length]); // draw line between points
			}

			if (points.length !== 4) return;

			points[0].drawCircle(p5, markerSize * 1.5);
			points[1].drawRect(p5, markerSize * 1.5);
			points[2].drawCross(p5, markerSize * 1.5);
			points[3].drawTriangle(p5, markerSize * 1.5);
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

<div class="p5Transform pointer-events-none relative h-full select-none overflow-hidden">
	<P5 {sketch} />
	<slot />
</div>

<style>
	:global(html:has(.p5Transform), body:has(.p5Transform)) {
		overflow: hidden;
	}

	.p5Transform {
		touch-action: none;
	}
</style>
