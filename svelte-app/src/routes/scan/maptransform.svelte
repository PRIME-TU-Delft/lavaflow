<script lang="ts">
	/**
	 * Page for dragging markers on document edges
	 */
	import Button from '$lib/components/Button.svelte';
	import Page from '$lib/components/Page.svelte';
	import NavigationButton from '$lib/components/NavigationButton.svelte';

	import type Draggable from '$lib/data/draggable';
	import { rawImage, perspectiveImage } from '$lib/stores/imageStore';
	import { goto } from '$app/navigation';
	import removePerspective from '$lib/opencv/removePerspective';

	import cv from 'opencv-ts';
	import { onMount } from 'svelte';
	import P5Transform from '$lib/components/P5Transform.svelte';

	let foregroundWidth: number;
	let foregroundHeight: number;
	let points: Draggable[] = [];

	function gotoPreview() {
		// TODO: move this to seperate file
		let mat = cv.imread('foregroundImage');

		console.log(points);

		// Fetch the marker coordinates of the draggable buttons
		let markerCoords: number[] = [];
		for (let p of points) {
			markerCoords.push(p.x - p.offsetX);
			markerCoords.push(p.y - p.offsetY);
		}

		// Apply the perspective transformation using the selected marker coords
		let result = removePerspective(mat, markerCoords, foregroundWidth, foregroundHeight);
		cv.imshow('canvasOutput', result);

		// TODO implement svelte magic
		const canvasOutput = document.getElementById('canvasOutput') as HTMLCanvasElement;
		perspectiveImage.set(canvasOutput.toDataURL());

		result.delete();
		mat.delete();

		goto('/scan/preview');
	}

	onMount(() => {
		if (!$rawImage) goto('/scan/mapscanning');
	});
</script>

<Page title="Image transformation">
	<NavigationButton slot="headerButton" to="/scan/mapscanning" back>Rescan image</NavigationButton>

	<div slot="background">
		<img id="backgroundImage" src={$rawImage} alt="background" />
	</div>

	<div class="sketch" bind:clientWidth={foregroundWidth} bind:clientHeight={foregroundHeight}>
		<P5Transform bind:points {foregroundHeight} {foregroundWidth} />
	</div>

	<img
		style="display:none"
		height={foregroundHeight}
		width={foregroundWidth}
		id="foregroundImage"
		src={$rawImage}
		alt="background"
	/>

	<canvas id="canvasOutput" />

	<div slot="footer">
		<Button on:click={gotoPreview}>
			<span>Proceed</span>
		</Button>
	</div>
</Page>

<style>
	.sketch {
		height: 100%;
		touch-action: none;
	}

	#backgroundImage {
		height: 100%;
		width: 100%;
		object-fit: cover;
		touch-action: none;
	}
</style>
