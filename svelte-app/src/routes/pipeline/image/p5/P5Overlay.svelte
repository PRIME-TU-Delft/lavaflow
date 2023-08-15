<script lang="ts">
	import P5 from '$lib/components/p5/P5.svelte';
	import type p5 from 'p5';
	import SnapRegion from './SnapRegion';

	export let width: number = 100;
	export let height: number = 100;

	let p5Canvas: p5.Renderer;
	let snapRegion: SnapRegion;

	function sketch(p5: p5) {
		// Preload
		p5.preload = () => {};

		// Setup
		p5.setup = () => {
			p5Canvas = p5.createCanvas(width, height);
			p5Canvas.id('p5-overlay');
			p5.pixelDensity(p5.displayDensity());

			snapRegion = new SnapRegion(p5, width, height);
		};

		// Draw
		p5.draw = () => {
			p5.clear(0, 0, 0, 0);

			snapRegion.draw(p5);
		};

		p5.mousePressed = () => {
			snapRegion.mousePressed(p5);

			// Prevent default behaviour
			return false;
		};

		p5.mouseDragged = () => {
			snapRegion.mouseDragged(p5);

			// Prevent default behaviour.
			return false;
		};

		p5.mouseReleased = () => {
			snapRegion.mouseReleased(p5);

			// Prevent default behaviour.
			return false;
		};
	}
</script>

<P5 {sketch} />
