<script lang="ts">
	import { onDestroy, onMount } from 'svelte';

	export let deviceId: string = '';

	let stream: MediaStream;
	let loading = true;
	let error: string;
	let cameraOptions: MediaDeviceInfo[] = [];
	export let videoSource: HTMLVideoElement | undefined = undefined;

	/**
	 * Obtain all video cameras on device and set the stream
	 */
	async function obtainVideoCamara() {
		loading = true;

		// If there are still active streams -> stop them
		if (stream) stream.getTracks().forEach((track) => track.stop());

		// If there is a deviceId -> get the stream | else -> get the default camera
		const constraints = deviceId
			? { deviceId: { exact: deviceId } }
			: { facingMode: 'environment' };

		try {
			stream = await navigator.mediaDevices.getUserMedia({ video: constraints });

			// TODO: set stream id to deviceId

			loading = false;
			error = '';
		} catch (err) {
			error = ('' + err) as string;
		}
	}

	$: {
		if (stream && videoSource && videoSource.paused) {
			videoSource.srcObject = stream;

			videoSource.play();
		}
	}

	// When devideId updated -> optain a new video stream
	$: deviceId && obtainVideoCamara();

	onMount(async () => {
		await obtainVideoCamara();

		// List all cameras
		const devices = await navigator.mediaDevices.enumerateDevices();
		cameraOptions = devices.filter((device) => device.kind == 'videoinput');
	});

	// When component is destoryed -> close all video streams
	onDestroy(() => {
		if (!stream) return;

		stream.getTracks().forEach((track) => track.stop());
	});
</script>

{#if loading}
	<div class="grid h-full w-full items-center text-center">
		<div class="prose lg:prose-xl mx-auto">
			<h2>Loading video camera...</h2>

			{#if error && error.includes('Permission denied')}
				The browser is unable to access the camera, try re-enabling the camera and reload
			{:else if error}
				{error}
			{/if}
		</div>
	</div>

	<slot name="error" />
{:else}
	<!-- svelte-ignore a11y-media-has-caption -->
	<video
		class="h-full w-full object-cover"
		loop
		playsInline
		autoPlay={false}
		controls={false}
		preload="auto"
		bind:this={videoSource}
	/>

	<slot {cameraOptions} {videoSource} {stream} />
{/if}
