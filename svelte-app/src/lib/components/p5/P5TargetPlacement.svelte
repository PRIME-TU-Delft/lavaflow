<script lang="ts">
	import P5 from 'p5-svelte';
	import type p5 from 'p5';

	import { craterLocations, gltfStore, targetLocations } from '$lib/stores/gltfStore';

	export let foregroundWidth: number; // Width of the foreground canvas
	export let foregroundHeight: number; // Height of the foreground canvas
	export let targetSelected: number;
	export let curves: [number, number][][];

	function sketch(p5: p5) {
		p5.setup = () => {
			p5.createCanvas(foregroundWidth, foregroundHeight);
		};

		p5.draw = () => {
			p5.background(230);
			p5.stroke(30);

			curves.forEach((layer) => {
				// Display the level-curves to the user
				p5.strokeWeight(4);
				p5.noFill();
				p5.beginShape();

				// Add points to the shape
				for (let [x, y] of layer) p5.vertex(x, y);

				// Close the shape
				p5.endShape(p5.CLOSE);
			});

			// Draw the craters | if the craters are stored in the gltf store -> pick those craters
			//                  | else -> pick the (cached) craters from the localstorage
			const craters = $gltfStore?.craters ?? $craterLocations ?? [];
			for (let crater of craters) {
				p5.strokeWeight(1);
				p5.stroke('#f2682c');

				// Inner ring
				p5.fill('#f2682c');
				p5.ellipse(crater[0], crater[1], 50, 50);
				p5.noFill();

				// Outer ring - for showing users the minumum distance from the crater
				p5.strokeWeight(0.5);
				p5.ellipse(crater[0], crater[1], 150, 150);
			}

			// Draw the turbine-markers
			let markerSize = 20;

			for (let i = 0; i < $targetLocations.length; i++) {
				let target = $targetLocations[i];
				target.update(p5, craters, 75, $targetLocations, 50, i);
				target.drawCircle(p5, markerSize, i);
			}
		};

		// If the user presses/releases their mouse, signal this to all Draggable points
		p5.mousePressed = () => {
			if (!$targetLocations?.length) return;

			let foundFirstMarker = false;

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

			// When mouse is released -> reset the cache
			targetLocations.set($targetLocations);
		};
	}
</script>

<P5 {sketch} />
