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

	function drawDots(p5: p5) {
		for (let i = 0; i < width; i += GRID_SIZE) {
			for (let j = 0; j < height; j += GRID_SIZE) {
				for (let x = 0; x < GRID_SIZE; x += 5) {
					for (let y = 0; y < GRID_SIZE; y += 5) {
						const c = p5.get(i + x, j + y);
						if (c[0] == 0) {
							p5.fill(255, 0, 0);
							break;
						}
					}
				}

				p5.arc(i + GRID_SIZE / 2, j + GRID_SIZE / 2, 5, 5, 0, 2 * Math.PI);

				p5.fill(255);
			}
		}
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
