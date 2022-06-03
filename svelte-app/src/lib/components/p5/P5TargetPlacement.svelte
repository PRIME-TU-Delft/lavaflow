<script lang="ts">
	import P5 from 'p5-svelte';
	import type p5 from 'p5';

	import { perspectiveImage } from '$lib/stores/imageStore';
	import { targetLocations } from '$lib/stores/gltfStore';

	export let foregroundWidth: number; // Width of the foreground canvas
	export let foregroundHeight: number; // Height of the foreground canvas
	export let targetSelected: number;

	console.log({ foregroundWidth, foregroundHeight });

	let image: p5.Image;

	function sketch(p5: p5) {
		// Preload image to use in p5
		p5.preload = () => {
			if ($perspectiveImage) image = p5.loadImage($perspectiveImage);
		};

		p5.setup = () => {
			p5.createCanvas(foregroundWidth, foregroundHeight);
		};

		p5.draw = () => {
			if (!image) return;

			p5.image(image, 0, 0, foregroundWidth, foregroundHeight);

			let markerSize = 20;

			if ($targetLocations?.length) {
				for (let i = 0; i < $targetLocations.length; i++) {
					let target = $targetLocations[i];
					target.update(p5);
					target.drawCircle(p5, markerSize, i);
				}
			}
		};

		// If the user presses/releases their mouse, signal this to all Draggable points
		p5.mousePressed = () => {
			if (!$targetLocations?.length) return;

			for (let i = 0; i < $targetLocations.length; i++) {
				const isPressed = $targetLocations[i].pressed(p5);

				if (isPressed) {
					targetSelected = i;
					break;
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
