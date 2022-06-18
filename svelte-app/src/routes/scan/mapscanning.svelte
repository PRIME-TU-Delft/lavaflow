<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import NavigationButton from '$lib/components/NavigationButton.svelte';
	import Dropdown from '$lib/components/input/Dropdown.svelte';
	import Instructions from '$lib/components/Instructions.svelte';
	import Page from '$lib/components/Page.svelte';
	import Modal from '$lib/components/Modal.svelte';
	import VideoStream from '$lib/components/VideoStream.svelte';
	import Video from '$lib/components/Video.svelte';

	import { mdiCamera, mdiHelpCircle } from '@mdi/js';
	import { goto } from '$app/navigation';
	import { rawImage } from '$lib/stores/imageStore';

	let instructionVisible: boolean;
	let videoSource: HTMLVideoElement;
	let cameraSelected: MediaDeviceInfo;
	let canvas: HTMLCanvasElement;
	let loadingNextPage: boolean = false;
	let deviceId: string;

	const toggleInstruction = () => (instructionVisible = !instructionVisible);

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

	function updateCamera() {
		deviceId = cameraSelected.deviceId;
	}
</script>

<Modal title="scan instructions" bind:visible={instructionVisible}>
	<Instructions />
</Modal>

<VideoStream {deviceId} let:loading let:stream let:error let:cameraOptions>
	<Page fullscreen={!loading}>
		<NavigationButton slot="headerButton" back to="/">Back home</NavigationButton>

		<div slot="background">
			<Video {loading} {stream} />
		</div>

		<div slot="options">
			{#if !instructionVisible && cameraOptions?.length > 1}
				<Dropdown
					bind:value={cameraSelected}
					options={cameraOptions}
					let:option
					let:index
					on:change={updateCamera}
				>
					{option.label || 'Camera ' + (index + 1)}
				</Dropdown>
			{/if}
		</div>

		<Video bind:videoSource --corner-radius="1rem" {error} {loading} {stream} />

		<svelte:fragment slot="footer">
			{#if !instructionVisible}
				<Button secondary icon={mdiHelpCircle} on:click={toggleInstruction}>
					Image capture | Instructions
				</Button>
			{/if}
			<Button loading={loadingNextPage} icon={mdiCamera} on:click={gotoTransform}>
				Capture photo of map
			</Button>
		</svelte:fragment>
	</Page>
</VideoStream>

<canvas id="captureCanvas" bind:this={canvas} style="display: none; position: absolute" />
