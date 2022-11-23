<script context="module" lang="ts">
	export const prerender = true;
</script>

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
	import { difficultyStore } from '$lib/stores/difficultyStore';
	import { debugMode } from '$lib/stores/debugStore';

	let foregroundWidth: number;
	let foregroundHeight: number;

	let gltfLoaded = false;
	let webXRSupport = false;

	$: isIOS = /iPhone|iPod|iPad/.test(navigator?.userAgent || navigator?.platform || 'unknown');

	onMount(async () => {
		if (!$contourLines.curves || !$contourLines.hierarchy) {
			return goto('/scan/mapscanning');
		}
		await gltfStore.setup($contourLines, $difficultyStore.lava_forking);
		gltfStore.build($contourLines);

		webXRSupport = (await navigator?.xr?.isSessionSupported('immersive-ar')) ?? false;

		gltfLoaded = true;
	});
</script>

<Page>
	<NavigationButton slot="headerButton" to="/scan/maptransform" back>
		Back to selection
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

	<svelte:fragment slot="footer">
		<Button secondary loading={!gltfLoaded} on:click={() => goto('/visualise/model')}>
			<span>Visualise as 3D model</span>
			<Icon path={mdiChevronRight} />
		</Button>

		{#if webXRSupport}
			<Button secondary loading={!gltfLoaded} on:click={() => goto('/visualise/ar')}>
				<span>Visualise in AR {isIOS ? '! Experimental !' : ''}</span>
				<Icon path={mdiChevronRight} />
			</Button>
		{:else}
			<Button
				secondary
				loading={!gltfLoaded}
				on:click={() => goto('https://apps.apple.com/nl/app/webxr-viewer/id1295998056')}
			>
				<span>WebXR not supported: Download here</span>
				<Icon path={mdiChevronRight} />
			</Button>
		{/if}
	</svelte:fragment>
</Page>

<style>
	.sketch {
		height: 100%;
		width: 100%;
		position: absolute;
		top: 0;
		touch-action: none;
		border-radius: 0.5rem;
		overflow: hidden;
	}
</style>
