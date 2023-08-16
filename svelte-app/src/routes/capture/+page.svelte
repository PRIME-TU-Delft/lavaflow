<script lang="ts">
	import ActionMenu from '$lib/components/ActionMenu.svelte';
	import Dropdown from '$lib/components/Dropdown.svelte';
	import Instructions from '$lib/components/Instructions.svelte';
	import Menubar from '$lib/components/Menubar.svelte';
	import Video from '$lib/components/Video.svelte';
	import * as gm from 'gammacv';
	import { onMount } from 'svelte';
	import CaptureMenu from './CaptureMenu.svelte';
	import { handleCapture, videoToTensor } from './handleCapture';
	import { drawingInstructions, scanningInstructions } from './instructions';
	import imageStore from '$lib/stores/imageStore';
	import { goto } from '$app/navigation';

	let deviceId: string; // camera id
	let deviceName: string; // camera name

	let width: number;
	let height: number;
	let loadingNextPage = false;

	let outputCanvas: HTMLCanvasElement;
	let gmSession: gm.Session; // GammaCV session

	/**
	 * Change the camera id
	 */
	function setCameraId(device: MediaDeviceInfo): void {
		deviceId = device.deviceId;
		deviceName = device.label;
	}

	/**
	 * Handle capture if the user clicks on the capture button
	 * If the capture is successful, the user is redirected to the next page
	 * Otherwise, an error is displayed
	 * @param videoSource
	 */
	async function capture(videoSource: HTMLVideoElement | undefined) {
		try {
			loadingNextPage = true;
			// 1. Get image url from video
			const { input, imageUrl } = await videoToTensor(videoSource, outputCanvas);

			// 2. Extract corners from image
			const corners = handleCapture(input, gmSession, outputCanvas);

			// 3. Set to image store
			imageStore.set({ imageUrl, corners, imagePerpotions: { width, height } });

			// 4. Redirect to next page
			goto('./crop');
			loadingNextPage = false;
		} catch (error) {
			// TODO: proper error handling
			console.error(error);
			loadingNextPage = false;
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
				<Dropdown
					items={cameraOptions}
					title={deviceName || 'Select other camera'}
					let:item={camera}
				>
					{#if camera.deviceId != deviceId}
						<li>
							<button on:click={() => setCameraId(camera)}>{camera.label}</button>
						</li>
					{/if}
				</Dropdown>
			{/if}
		</Menubar>

		<ActionMenu>
			<CaptureMenu loading={loadingNextPage} on:click={() => capture(videoSource)} />
		</ActionMenu>
	</Video>
</Instructions>
