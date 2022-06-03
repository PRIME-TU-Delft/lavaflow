<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import Instructions from '$lib/components/Instructions.svelte';
	import Page from '$lib/components/Page.svelte';
	import VideoStream from '$lib/components/VideoStream.svelte';
	import Video from '$lib/components/Video.svelte';

	import { mdiCamera, mdiBookOpenVariant } from '@mdi/js';
	import { goto } from '$app/navigation';
	import { rawImage } from '$lib/stores/imageStore';

	let instructionVisible = false;
	let videoSource: HTMLVideoElement;
	let canvas: HTMLCanvasElement;
	let loadingNextPage: boolean = false;
	$: title = instructionVisible ? 'Instructions' : 'Map Scanning';

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
</script>

<VideoStream let:loading let:stream let:error>
	<Page {title} closeButton={instructionVisible} on:close={toggleInstuction}>
		<div slot="background">
			<Video {loading} {stream} />
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
