<script lang="ts">
	/**
	 * Page for dragging markers on document edges
	 */
	import Button from '$lib/components/Button.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import Page from '$lib/components/Page.svelte';
	import NavigationButton from '$lib/components/NavigationButton.svelte';

	import type Draggable from '$lib/data/draggable';
	import { rawImage, perspectiveImage } from '$lib/stores/imageStore';
	import { contourLines } from '$lib/stores/contourLineStore';
	import { goto } from '$app/navigation';
	import removePerspective from '$lib/opencv/removePerspective';
	import { getCurves } from '$lib/opencv/detectCurves';

	import cv from 'opencv-ts';
	import { onMount } from 'svelte';
	import P5Transform from '$lib/components/P5Transform.svelte';
	import { mdiInformation, mdiChevronRight } from '@mdi/js';

	let foregroundWidth: number;
	let foregroundHeight: number;
	let outputCanvas: HTMLCanvasElement;
	let points: Draggable[] = [];

	function gotoPreview() {
		let mat = cv.imread('foregroundImage');

		// Fetch the marker coordinates of the draggable buttons
		let markerCoords: number[] = [];
		for (let p of points) {
			markerCoords.push(p.x - p.offsetX);
			markerCoords.push(p.y - p.offsetY);
		}

		// Apply the perspective transformation using the selected marker coords
		const result = removePerspective(mat, markerCoords, foregroundWidth, foregroundHeight);

		// Set contour line store to the detected contour lines with hirarchy
		const { curves, hierarchy } = getCurves(result);

		if (curves.length == 0 || hierarchy.length == 0) {
			alert('No contours found');
			return;
		}

		const contourArray = curves.map((c) => Array.from(c));
		const contourTuples: [number, number][][] = contourArray.map((contour) => {
			let contourTuple: [number, number][] = [];

			for (let i = 0; i < contour.length - 1; i += 2) {
				contourTuple.push([contour[i], contour[i + 1]]);
			}

			return contourTuple;
		});

		contourLines.set({
			curves: contourTuples,
			hierarchy: hierarchy
		});

		cv.imshow('canvasOutput', result);

		perspectiveImage.set(outputCanvas.toDataURL());

		result.delete();
		mat.delete();

		goto('/scan/preview');
	}

	onMount(() => {
		// If no raw image in cache, go back to scan/mapscanning
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

	<canvas bind:this={outputCanvas} id="canvasOutput" />

	<div slot="footer">
		<Button secondary icon={mdiInformation}>
			Drag each marker to the correct corner on the map
		</Button>

		<Button on:click={gotoPreview}>
			<span>Preview transformation</span>
			<Icon path={mdiChevronRight} />
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
