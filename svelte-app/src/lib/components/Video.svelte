<script lang="ts">
	export let stream: MediaStream;
	export let style: string = '';
	export let loading: boolean = true;

	export let videoSource: HTMLVideoElement | undefined = undefined;

	$: {
		if (stream && !loading && videoSource) {
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
		height: inherit;
		width: 100%;
		object-fit: cover;
	}
</style>
