<script lang="ts">
	import Instructions from '$lib/components/Instructions.svelte';
	import Video from '$lib/components/Video.svelte';
	import Menubar from '$lib/components/Menubar.svelte';
	import Dropdown from '$lib/components/Dropdown.svelte';
	import ActionMenu from '$lib/components/ActionMenu.svelte';
	import CaptureMenu from './CaptureMenu.svelte';
	import { drawingInstructions, scanningInstructions } from './instructions';
	import { onMount } from 'svelte';
	import { PARAMS, calculateIntersectionPoints } from './line_intersections';
	import handleCapture from './handleCapture';
	import * as gm from 'gammacv';

	let deviceId: string; // camera id
	let width: number;
	let height: number;
	let loadingNextPage = false;

	let tickLoop: number;

	let outputCanvas: HTMLCanvasElement;
	let gmSession: gm.Session; // GammaCV session

	/**
	 * Change the camera id
	 */
	function setCameraId(label: any): any {
		throw new Error('Function not implemented.');
	}

	/**
	 * Handle capture if the user clicks on the capture button
	 * If the capture is successful, the user is redirected to the next page
	 * Otherwise, an error is displayed
	 * @param videoSource
	 */
	function capture(videoSource: HTMLVideoElement | undefined) {
		try {
			handleCapture(videoSource, outputCanvas, width, height, gmSession);
		} catch (error) {
			// TODO: proper error handling
			console.error(error);
		}
	}

	onMount(() => {
		if (!gmSession) gmSession = new gm.Session(); // Setup the GM session
	});
</script>

<svelte:window bind:innerHeight={height} bind:innerWidth={width} />

<Instructions
	title="Capture instructions"
	instructions={{
		'Drawing instructions': drawingInstructions,
		'Scanning instructions': scanningInstructions
	}}
>
	<Video bind:deviceId let:cameraOptions let:videoSource>
		<canvas style="position: absolute; top: 0;" {width} {height} bind:this={outputCanvas} />

		<Menubar back="./" title="Capture">
			<!-- If more than one camera are available, display a dropdown to allow the user to choose -->
			{#if cameraOptions.length > 1}
				<Dropdown items={cameraOptions} title={deviceId || 'Select other camera'} let:item={camera}>
					<li>
						<button on:click={() => setCameraId(camera.label)}>{camera.label}<button /></button>
					</li>
				</Dropdown>
			{/if}
		</Menubar>

		<ActionMenu>
			<CaptureMenu loading={loadingNextPage} on:click={() => capture(videoSource)} />
		</ActionMenu>
	</Video>
</Instructions>
