<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import Dropdown from '$lib/components/input/Dropdown.svelte';
	import Instructions from '$lib/components/Instructions.svelte';
	import Page from '$lib/components/Page.svelte';
	import VideoStream from '$lib/components/VideoStream.svelte';
	import Video from '$lib/components/Video.svelte';

	import { mdiCamera, mdiBookOpenVariant } from '@mdi/js';
	import { goto } from '$app/navigation';
	import { rawImage } from '$lib/stores/imageStore';
	import { onMount } from 'svelte';

	let instructionVisible = false;
	let videoSource: HTMLVideoElement;
	let cameraSelected: MediaDeviceInfo;
	let cameraOptions: MediaDeviceInfo[] = [];
	let canvas: HTMLCanvasElement;
	let loadingNextPage: boolean = false;
	$: title = instructionVisible ? 'Instructions' : 'Capture the image';

	const toggleInstuction = () => (instructionVisible = !instructionVisible);

	/**
	 * Goto scan/maptransform
	 */
	async function gotoTransform() {
		loadingNextPage = true;

		// Create an atificial canvas element
		const context = canvas.getContext('2d');
		canvas.height = videoSource.videoHeight;
		canvas.width = videoSource.videoWidth;

		// Take screenshot of video
		if (context) context.drawImage(videoSource, 0, 0, canvas.width, canvas.height);
		const image = canvas.toDataURL('image/png');

		// Set image in (raw)image store
		rawImage.set(image);

		goto('/scan/maptransform');
	}

	export async function getStream() {
		const deviceId = cameraSelected
			? { deviceId: cameraSelected.deviceId }
			: { facingMode: 'environment' };

		const constraints = {
			video: deviceId
		};

		console.log(JSON.stringify(constraints));

		try {
			const stream = await navigator.mediaDevices.getUserMedia(constraints);
			videoSource.srcObject = stream;
			videoSource.play();
		} catch (error) {
			console.error('Error:', error);
		}
	}

	onMount(async () => {
		await getStream();
		const devices = await navigator.mediaDevices.enumerateDevices();
		cameraOptions = devices.filter((device) => device.kind == 'videoinput');

		if (cameraOptions?.length) {
			cameraSelected = cameraOptions[0];
		}
	});
</script>

<VideoStream let:loading let:stream let:error>
	<Page fullscreen={!loading} {title} closeButton={instructionVisible} on:close={toggleInstuction}>
		<div slot="background">
			<Video {loading} {stream} />
		</div>

		<div slot="options">
			{#if !instructionVisible && cameraOptions?.length > 1}
				<Dropdown value={cameraSelected} options={cameraOptions} let:option let:index>
					{option.label || 'Camera ' + (index + 1)}
				</Dropdown>
			{/if}
		</div>

		{#if instructionVisible}
			<Instructions />
		{:else}
			<Video bind:videoSource --corner-radius="1rem" {error} {loading} {stream} />
		{/if}

		<div slot="footer">
			{#if !instructionVisible}
				<Button secondary icon={mdiBookOpenVariant} on:click={toggleInstuction}>
					Read scan instructions
				</Button>
				<Button loading={loadingNextPage} icon={mdiCamera} on:click={gotoTransform}>
					Capture photo of map
				</Button>
			{/if}
		</div>
	</Page>
</VideoStream>

<canvas id="captureCanvas" bind:this={canvas} style="display: none; position: absolute" />
