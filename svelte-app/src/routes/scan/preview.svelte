<script lang="ts">
	/**
	 * This is where the user inspects the level curves that are drawn, to check if the user wants to rescan the image or not
	 */
	import Button from '$lib/components/Button.svelte';
	import Page from '$lib/components/Page.svelte';
	import Image from '$lib/components/Image.svelte';
	import NavigationButton from '$lib/components/NavigationButton.svelte';

	import { perspectiveImage } from '$lib/stores/imageStore';
	import { contourLines } from '$lib/stores/contourLineStore';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { gltfStore } from '$lib/stores/gltfStore';
	import { debugMode } from '$lib/stores/debugStore';
	import P5CurvesDebugView from '$lib/components/p5/P5CurvesDebugView.svelte';

	let foregroundWidth: number;
	let foregroundHeight: number;

	let gltfLoaded = false;

	onMount(async () => {
		if (!$perspectiveImage || !$contourLines.curves || !$contourLines.hierarchy) {
			return goto('/scan/mapscanning');
		}
		await gltfStore.setup($contourLines);
		gltfStore.build();

		gltfLoaded = true;
	});
</script>

<Page title="image transformation">
	<NavigationButton slot="headerButton" to="/scan/maptransform" back>
		Redraw borders
	</NavigationButton>

	<div slot="background">
		<Image src={$perspectiveImage} alt="background" />
	</div>

	<Image src={$perspectiveImage} alt="foreground" />

	{#if $contourLines?.curves && $contourLines?.hierarchy}
		<div class="sketch" bind:clientWidth={foregroundWidth} bind:clientHeight={foregroundHeight}>
			<P5CurvesDebugView
				debugMode={$debugMode}
				curves={$contourLines.curves}
				hierarchy={$contourLines.hierarchy}
				{foregroundHeight}
				{foregroundWidth}
			/>
		</div>
	{/if}

	<div slot="footer">
		<!-- TODO: add icons to buttons -->
		<Button secondary loading={!gltfLoaded} on:click={() => goto('/visualise/model')}>
			<span>Visualise as model</span>
		</Button>
		<Button loading={!gltfLoaded} on:click={() => goto('/visualise/ar')}>
			<span>Visualise in AR</span>
		</Button>
	</div>
</Page>

<style>
	.sketch {
		height: 100%;
		width: 100%;
		position: absolute;
		top: 0;
		touch-action: none;
	}
</style>
