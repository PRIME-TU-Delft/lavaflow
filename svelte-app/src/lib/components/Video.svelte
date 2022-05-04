<script lang="ts">
	import { onMount } from 'svelte';

	export let stream: MediaStream;
	export let style: string = '';
	export let loading: boolean = true;

	let videoSource: HTMLVideoElement;

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
