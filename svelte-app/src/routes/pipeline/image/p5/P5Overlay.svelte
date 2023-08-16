<script lang="ts">
	import P5 from '$lib/components/p5/P5.svelte';
	import type p5 from 'p5';
	import SnapRegion from './SnapRegion';
	import type { Corners } from '../../../capture/suggestedCorners';

	export let width: number = 100;
	export let height: number = 100;
	export let suggestedCorners: Corners;

	let oldWidth: number = 100;
	let oldHeight: number = 100;

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

			snapRegion = new SnapRegion(p5, width, height, suggestedCorners);

			oldWidth = width;
			oldHeight = height;
		};

		// Draw
		p5.draw = () => {
			p5.clear(0, 0, 0, 0);

			snapRegion.draw(p5);
		};

		p5.mousePressed = () => {
			snapRegion.mousePressed(p5);

			// Prevent default behaviour
			return true;
		};

		p5.mouseDragged = () => {
			snapRegion.mouseDragged(p5);

			// Prevent default behaviour.
			return true;
		};

		p5.mouseReleased = () => {
			snapRegion.mouseReleased(p5);

			// Prevent default behaviour.
			return true;
		};

		p5.windowResized = () => {
			p5.resizeCanvas(width, height);
			p5.pixelDensity(p5.displayDensity());

			snapRegion.resize(p5, oldWidth, oldHeight);

			oldWidth = width;
			oldHeight = height;
		};
	}
</script>

{#key suggestedCorners}
	<P5 {sketch} />
{/key}
