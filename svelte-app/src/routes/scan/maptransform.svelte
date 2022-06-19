<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
	/**
	 * Page for dragging markers on document edges
	 */
	import Button from '$lib/components/Button.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import Page from '$lib/components/Page.svelte';
	import Modal from '$lib/components/Modal.svelte';
	import NavigationButton from '$lib/components/NavigationButton.svelte';
	import Instructions from '$lib/components/InstructionsTransformation.svelte';

	import type Draggable from '$lib/data/draggable';
	import { rawImage } from '$lib/stores/imageStore';
	import { contourLines } from '$lib/stores/contourLineStore';
	import { goto } from '$app/navigation';
	import removePerspective from '$lib/opencv/removePerspective';
	import { getCurves } from '$lib/opencv/detectCurves';

	import cv from 'opencv-ts';
	import { onMount } from 'svelte';
	import P5Transform from '$lib/components/p5/P5Transform.svelte';
	import { mdiChevronRight, mdiBookOpenVariant } from '@mdi/js';

	let outputCanvas: HTMLCanvasElement;
	let points: Draggable[] = [];
	let instructionVisible = false;

	const toggleInstruction = () => (instructionVisible = !instructionVisible);

	function gotoPreview(width: number, height: number) {
		const mat = cv.imread('foregroundImage');

		// Fetch the marker coordinates of the draggable buttons
		let markerCoords: number[] = [];
		for (let p of points) {
			markerCoords.push(p.x - p.offsetX);
			markerCoords.push(p.y - p.offsetY);
		}

		// Apply the perspective transformation using the selected marker coords
		const result = removePerspective(mat, markerCoords, width, height);

		try {
			// Set contour line store to the detected contour lines with hierarchy
			const { curves, hierarchy } = getCurves(result);

			// Convert the OpenCV Mat to a array of tuples for mountain model construction
			const contourTuples: [number, number][][] = curves.map((contour) => {
				let contourTuple: [number, number][] = [];

				for (let i = 0; i < contour.length - 1; i += 2) {
					contourTuple.push([contour[i], contour[i + 1]]);
				}

				return contourTuple;
			});

			contourLines.setup({
				curves: contourTuples,
				hierarchy: hierarchy,
				size: { width, height }
			});

			cv.imshow('canvasOutput', result);

			goto('/scan/preview');
		} catch (message) {
			alert(message);
		}

		result.delete();
		mat.delete();
	}

	onMount(() => {
		// If no raw image in cache, go back to scan/mapscanning
		if (!$rawImage) goto('/scan/mapscanning');
	});
</script>

<Modal title="transformation instructions" closeButtons="top" bind:visible={instructionVisible}>
	<Instructions />
</Modal>

<Page let:foregroundHeight let:foregroundWidth>
	<NavigationButton slot="headerButton" to="/scan/mapscanning" back>Rescan image</NavigationButton>

	<div slot="background">
		<img id="backgroundImage" src={$rawImage} alt="background" />
	</div>

	<div class="sketch">
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

	<svelte:fragment slot="footer">
		<Button secondary small icon={mdiBookOpenVariant} on:click={toggleInstruction}>
			Select markers | Instructions
		</Button>

		<Button on:click={() => gotoPreview(foregroundWidth, foregroundHeight)}>
			<span>Apply selection</span>
			<Icon path={mdiChevronRight} />
		</Button>
	</svelte:fragment>
</Page>

<style>
	.sketch {
		height: 100%;
		touch-action: none;
		user-select: none;
		-moz-user-select: none;
		-khtml-user-select: none;
		-webkit-user-select: none;
		-o-user-select: none;
	}

	#backgroundImage {
		height: 100%;
		width: 100%;
		object-fit: cover;
		touch-action: none;
		user-select: none;
		-moz-user-select: none;
		-khtml-user-select: none;
		-webkit-user-select: none;
		-o-user-select: none;
	}
</style>
