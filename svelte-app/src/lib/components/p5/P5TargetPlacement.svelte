<script lang="ts">
	import P5 from 'p5-svelte';
	import type p5 from 'p5';

	import { gltfStore, targetLocations } from '$lib/stores/gltfStore';

	export let foregroundWidth: number; // Width of the foreground canvas
	export let foregroundHeight: number; // Height of the foreground canvas
	export let targetSelected: number;
	export let curves: [number, number][][];

	console.log({ foregroundWidth, foregroundHeight });

	let image: p5.Image;

	function sketch(p5: p5) {
		// Preload image to use in p5

		p5.setup = () => {
			p5.createCanvas(foregroundWidth, foregroundHeight);
		};

		p5.draw = () => {
			
			p5.background(230);
			p5.stroke(30);

			curves.forEach((layer, index) => {
				// Display the level-curves to the user
				p5.strokeWeight(3);
				p5.noFill();
				p5.beginShape();
				for (let curvesPair of layer) {
					let x = curvesPair[0];
					let y = curvesPair[1];
					p5.vertex(x, y);
				}
				p5.endShape(p5.CLOSE);
			});

			// Draw the craters
			if ($gltfStore.craters?.length) {
				for (let i = 0; i < $gltfStore.craters.length; i++) {
					p5.strokeWeight(1);
					p5.stroke("#f2682c");
					p5.fill("#f2682c");
					p5.ellipse($gltfStore.craters[i][0], $gltfStore.craters[i][1], 50, 50);

					p5.noFill();

					p5.strokeWeight(0.5);
					p5.ellipse($gltfStore.craters[i][0], $gltfStore.craters[i][1], 150, 150)
				}
			}

			// Draw the turbine-markers
			let markerSize = 20;

			if ($targetLocations?.length) {
				for (let i = 0; i < $targetLocations.length; i++) {
					let target = $targetLocations[i];
					target.update(p5, $gltfStore.craters, 75);
					target.drawCircle(p5, markerSize, i);
				}
			}

		};

		// If the user presses/releases their mouse, signal this to all Draggable points
		p5.mousePressed = () => {
			if (!$targetLocations?.length) return;

			let foundFirstMarker = false

			for (let i = 0; i < $targetLocations.length; i++) {
				let isPressed = false;
				$targetLocations[i].deselect();

				if (!foundFirstMarker) {
					isPressed = $targetLocations[i].pressed(p5);
					foundFirstMarker = isPressed;
				}

				if (isPressed) {
					targetSelected = i;
				}
			}
		};

		p5.mouseReleased = () => {
			if (!$targetLocations?.length) return;

			for (let target of $targetLocations) target.released();
		};
	}
</script>

<P5 {sketch} />
