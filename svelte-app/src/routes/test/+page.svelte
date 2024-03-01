<script lang="ts">
	import P5 from '$lib/components/p5/P5.svelte';
	import type p5 from 'p5';
	import image from './image.png?url';

	let width: number;
	let height: number;

	let p5Canvas: p5.Renderer;
	let img: p5.Image;

	const GRID_SIZE = 25;

	function drawGrid(p5: p5) {
		p5.stroke(255, 0, 0, 50);
		for (let i = 0; i < width; i += GRID_SIZE) {
			p5.line(i, 0, i, height);
		}

		for (let i = 0; i < height; i += GRID_SIZE) {
			p5.line(0, i, width, i);
		}
	}

	function drawBand(p5: p5, bands: [number, number][][]) {
		p5.stroke(255, 0, 0);
		p5.strokeWeight(2);
		p5.noFill();
		for (const countour of bands) {
			p5.beginShape();
			for (const [y, x] of countour) {
				console.log(x, y);
				p5.vertex(x * 25, y * 25);
			}
			p5.endShape();
		}
	}

	function drawDots(p5: p5) {
		const data: number[][] = [];

		for (let i = 0; i < width; i += GRID_SIZE) {
			let row: number[] = [];

			for (let j = 0; j < height; j += GRID_SIZE) {
				let avg_black = 0;

				for (let x = 0; x < GRID_SIZE; x += 5) {
					for (let y = 0; y < GRID_SIZE; y += 5) {
						const c = p5.get(i + x, j + y);
						if (c[0] == 0) avg_black++;
					}
				}

				avg_black /= 25;

				row.push(avg_black);

				p5.arc(i + GRID_SIZE / 2, j + GRID_SIZE / 2, 5, 5, 0, 2 * Math.PI);
				p5.fill(255 * avg_black, 255 - 255 * avg_black, 0);
			}

			data.push(row);
		}

		const lowerBound = 0.33;
		const upperBound = 1;

		const bandWidth = upperBound - lowerBound;
		const bands: [number, number][][] = MarchingSquaresJS.isoBands(data, lowerBound, bandWidth);

		console.log(data);
		console.log(bands);
		drawBand(p5, bands);
	}

	function sketch(p5: p5) {
		// Preload
		p5.preload = () => {
			img = p5.loadImage(image);
		};

		// Setup
		p5.setup = () => {
			p5Canvas = p5.createCanvas(width, height);
			p5Canvas.id('p5-overlay');
			p5.pixelDensity(p5.displayDensity());

			p5.noLoop();
			p5.image(img, 0, 0, width, height);

			drawGrid(p5);

			drawDots(p5);
		};
	}
</script>

<svelte:window bind:innerHeight={height} bind:innerWidth={width} />

{#if height && width}
	<P5 {sketch} />
{/if}
