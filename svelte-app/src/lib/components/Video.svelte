<script lang="ts">
	/**
	 * This component takes in a stream a displays it in a video element
	 */

	export let stream: MediaStream;
	export let style: string = '';
	export let loading: boolean = true;

	export let videoSource: HTMLVideoElement | undefined = undefined;

	$: {
		if (stream && !loading && videoSource && videoSource.paused) {
			videoSource.srcObject = stream;
			videoSource.play();
		}
	}
</script>

{#if loading}
	<slot />
{:else}
	<!-- svelte-ignore a11y-media-has-caption -->
	<video {style} bind:this={videoSource} />
{/if}

<style>
	video {
		height: 100%;
		width: 100%;
		object-fit: cover;
		border-radius: var(--corner-radius, 0);
	}
</style>
