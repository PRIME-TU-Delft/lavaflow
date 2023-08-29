<script lang="ts">
	/**
	 * Dragging marker over an image to transform it.
	 */
	import sizeStore from '$lib/stores/sizeStore';
	import type p5 from 'p5';
	import P5 from './P5.svelte';

	export let curves: [number, number][][];

	const sketch = (p5: p5) => {
		p5.setup = () => {
			const [width, height] = [$sizeStore?.width, $sizeStore?.height];

			if (!curves || curves.length === 0) {
				console.error('Curve-data cannot be found');
				return;
			}

			if (!width || !height) {
				console.error('Curve-dimensions cannot be found');
				return;
			}

			p5.createCanvas(width, height);
			p5.background(255);

			curves.forEach((layer) => {
				// Add text to the contour: "index (index of parent)"
				p5.strokeWeight(1);
				p5.stroke(30);
				p5.fill(30);

				// Display the level-curves to the user
				p5.strokeWeight(4);
				p5.noFill();
				p5.beginShape();
				for (let curvesPair of layer) {
					let x = curvesPair[0];
					let y = curvesPair[1];
					p5.vertex(x, y);
				}
				p5.endShape(p5.CLOSE);
			});
		};
	};
</script>

{#key $sizeStore}
	<P5 {sketch} />
{/key}
