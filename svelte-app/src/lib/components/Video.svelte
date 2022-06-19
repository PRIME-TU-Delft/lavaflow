<script lang="ts">
	/**
	 * This component takes in a stream a displays it in a video element
	 */
	export let stream: MediaStream;
	export let style: string = '';
	export let loading: boolean = true;
	export let error: string = '';

	export let videoSource: HTMLVideoElement | undefined = undefined;

	$: {
		if (stream && !loading && videoSource && videoSource.paused) {
			videoSource.srcObject = stream;
			videoSource.play();
		}
	}
</script>

{#if loading}
	<div class="loadingVideo">
		<div>
			<h2>Loading video...</h2>

			{#if error && error.includes('Permission denied')}
				The browser is unable to access the camera, try re-enabling the camera and reload
			{/if}
		</div>
	</div>

	<slot />
{:else}
	<!-- svelte-ignore a11y-media-has-caption -->
	<video
		{style}
		loop
		playsInline
		autoPlay={false}
		controls={false}
		preload="auto"
		type="video/mp4"
		bind:this={videoSource}
	/>
{/if}

<style lang="scss">
	video {
		height: 100%;
		width: 100%;
		object-fit: cover;
		border-radius: var(--corner-radius, 0);
		// display: none;
	}

	.loadingVideo {
		height: 100%;
		width: 100%;
		border-radius: var(--corner-radius, 0);
		display: grid;
		align-items: center;
		justify-content: center;
		text-align: center;

		div {
			max-width: 90%;
			margin: auto;

			h2 {
				width: max-content;
				margin: auto;
			}
		}
	}
</style>
