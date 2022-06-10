<script lang="ts">
	/**
	 * This is where the user inspects the level curves that are drawn, to check if the user wants to rescan the image or not
	 */
	import P5CurvesDebugView from '$lib/components/p5/P5CurvesDebugView.svelte';
	import Button from '$lib/components/Button.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import Page from '$lib/components/Page.svelte';
	import NavigationButton from '$lib/components/NavigationButton.svelte';

	import { mdiChevronRight } from '@mdi/js';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	import { contourLines } from '$lib/stores/contourLineStore';
	import { gltfStore } from '$lib/stores/gltfStore';
	import { debugMode } from '$lib/stores/debugStore';

	let foregroundWidth: number;
	let foregroundHeight: number;

	let gltfLoaded = false;

	onMount(async () => {
		if (!$contourLines.curves || !$contourLines.hierarchy) {
			return goto('/scan/mapscanning');
		}
		await gltfStore.setup($contourLines);
		gltfStore.build();

		gltfLoaded = true;
	});
</script>

<Page title="Result of transformation">
	<NavigationButton slot="headerButton" to="/scan/maptransform" back>
		Redraw borders
	</NavigationButton>

	<div slot="background" style="background:#aaa;" />

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
		<Button secondary loading={!gltfLoaded} on:click={() => goto('/visualise/model')}>
			<span>Visualise as model</span>
			<Icon path={mdiChevronRight} />
		</Button>
		<Button loading={!gltfLoaded} on:click={() => goto('/visualise/ar')}>
			<span>Visualise in AR</span>
			<Icon path={mdiChevronRight} />
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
