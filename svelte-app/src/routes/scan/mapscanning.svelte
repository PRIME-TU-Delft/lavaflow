<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import Page from '$lib/components/Page.svelte';

	import { mdiAlertCircleOutline } from '@mdi/js';
	import { onDestroy, onMount } from 'svelte';

	let videoSource: HTMLVideoElement;
	let backgroundSource: HTMLVideoElement;
	let loading = false;
	const obtenerVideoCamara = async () => {
		try {
			loading = true;
			const stream = await navigator.mediaDevices.getUserMedia({
				video: {
					facingMode: 'environment'
				}
			});
			videoSource.srcObject = stream;
			backgroundSource.srcObject = stream;
			videoSource.play();
			backgroundSource.play();
			loading = false;
		} catch (error) {
			console.log(error);
		}
	};

	onMount(async () => {
		await obtenerVideoCamara();
	});

	onDestroy(() => {
		const videoStream = videoSource?.srcObject as MediaStream;
		const backgroundStream = backgroundSource?.srcObject as MediaStream;

		if (!videoStream || !backgroundStream) return;

		videoStream.getTracks().forEach((track) => track.stop());
		backgroundStream.getTracks().forEach((track) => track.stop());
	});
</script>

<Page title="Map scanning">
	<!-- svelte-ignore a11y-media-has-caption -->
	<video slot="background" bind:this={backgroundSource}>
		<track src="captions_en.vtt" kind="captions" srclang="en" label="english_captions" />
	</video>

	{#if loading}
		<!-- TODO: add better loading screen -->
		loading...
	{/if}
	<!-- svelte-ignore a11y-media-has-caption -->
	<video class="foreground" bind:this={videoSource} />

	<div slot="footer">
		<Button icon={mdiAlertCircleOutline}>
			<span>No keystones found</span>
			<span slot="content">
				To recognize the level captureEvents, we need to have 3 markers visible
			</span>
		</Button>
	</div>
</Page>

<style>
	video {
		height: inherit;
		width: 100%;
		object-fit: cover;
	}

	.foreground {
		border-radius: 1rem;
	}
</style>
