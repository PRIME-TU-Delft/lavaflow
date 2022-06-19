<script lang="ts">
	/**
	 * Dragging marker over an image to transform it.
	 */
	import P5 from 'p5-svelte';
	import type p5 from 'p5';

	export let debugMode = false;

	export let foregroundWidth: number; // Width of the foreground canvas
	export let foregroundHeight: number; // Height of the foreground canvas

	export let curves: [number, number][][];
	export let hierarchy: number[];

	const sketch = (p5: p5) => {
		p5.setup = () => {
			p5.createCanvas(foregroundWidth, foregroundHeight);

			p5.background(230);

			curves.forEach((layer, index) => {
				// Add text to the contour: "index (index of parent)"
				p5.strokeWeight(1);
				p5.stroke(30);
				p5.fill(30);
				if (debugMode)
					p5.text(`${index} (${hierarchy[index]})`, layer[0][0] - 10, layer[0][1] - 10);

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

{#key debugMode}
	<P5 {sketch} />
{/key}
