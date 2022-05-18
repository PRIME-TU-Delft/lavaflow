<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import Page from '$lib/components/Page.svelte';
	import VideoStream from '$lib/components/VideoStream.svelte';
	import Video from '$lib/components/Video.svelte';
	import Instructions from '$lib/components/Instructions.svelte';

	import { mdiAlertCircleOutline, mdiBookOpenVariant } from '@mdi/js';
	import { imageUrl } from '$lib/stores/imageStore';
	import { goto } from '$app/navigation';

	let instructionVisible = false;
	let videoSource: HTMLVideoElement;
	$: title = instructionVisible ? 'Instructions' : 'Map Scanning';

	const toggleInstuction = () => (instructionVisible = !instructionVisible);

	/**
	 * Goto scan/maptransform
	 */
	function gotoTransform() {
		imageUrl.setImage(videoSource);

		console.log($imageUrl);
		return;
		goto('/scan/maptransform');
	}
</script>

<VideoStream let:loading let:stream>
	<Page {title} closeButton={instructionVisible} on:close={toggleInstuction}>
		<div style="height: var(--vh)" slot="background">
			<Video {loading} {stream} />
		</div>

		{#if instructionVisible}
			<Instructions />
		{:else}
			<Video bind:videoSource style="border-radius: 1rem;" {loading} {stream}>
				<h1>loading...</h1>
			</Video>
		{/if}

		<div slot="footer">
			{#if !instructionVisible}
				<Button secondary icon={mdiBookOpenVariant} on:click={toggleInstuction}>
					Read scan instructions
				</Button>
				<Button icon={mdiAlertCircleOutline} on:click={gotoTransform}>
					<span>No keystones found</span>
					<span slot="content">
						To recognize the level curves, we need to have 3 markers visible
					</span>
				</Button>
			{/if}
		</div>
	</Page>
</VideoStream>
