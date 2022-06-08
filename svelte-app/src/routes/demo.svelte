<script lang="ts">
	import NavigationButton from '$lib/components/NavigationButton.svelte';

	import { contourLines } from '$lib/stores/contourLineStore';

	import { gltfStore } from '$lib/stores/gltfStore';
	import { hc_curves, hc_hierarchy } from '$lib/data/hardCoded';
	import { onMount } from 'svelte';

	let mounted: boolean;
	let aframeLoaded: boolean;
	$: ready = (aframeLoaded || window.AFRAME) && mounted;

	function loadAframe() {
		aframeLoaded = true;
		console.warn('aframe loaded for first time');
	}

	onMount(async () => {
		if (aframeLoaded) console.warn('aframe already loaded');

		if (!$gltfStore) {
			contourLines.set({
				curves: hc_curves,
				hierarchy: hc_hierarchy,
				size: { width: 850, height: 950 }
			});

			await gltfStore.setup($contourLines);
			gltfStore.build();
			console.warn('gltf is loaded from hardcoded data', $gltfStore);
		}

		mounted = true;
	});
</script>

<svelte:head>
	{#if mounted && !window.AFRAME}
		<script src="https://aframe.io/releases/1.0.0/aframe.min.js" on:load={loadAframe}></script>
	{/if}
</svelte:head>

{#if ready}
	<a-scene embedded background="color:#445;">
		<div class="button backButton">
			<NavigationButton back to="/scan/mapscanning">Rescan image</NavigationButton>
		</div>

		<div class="button placeTargets">
			<NavigationButton to="/targetplacement">Place targets</NavigationButton>
		</div>

		<a-entity light="color: #AFA; intensity: 1.5; decay:1000" position="-1 1 0" />
		<a-entity light="color: #AFA; intensity: 1.5; decay:1000" position="3 1 -4" />

		<a-entity light="type: ambient; color: #fff; intensity: 5;"></a-entity>

		{#if $gltfStore}
			<a-entity position="5 -2 -5" scale="0.05 0.025 0.05" rotation="0 -90 0">
				<a-entity gltf-model="url({$gltfStore})" />
			</a-entity>
		{:else}
			<a-entity
				gltf-model="url(output20.gltf)"
				scale="0.0001 0.04 0.0001"
				position="1 1 -3"
				rotation="0 -90 0"
			/>
		{/if}

		<a-camera look-controls />
	</a-scene>
{/if}

<style>
	.button {
		position: absolute;
		width: 15rem;
		max-width: calc(50vw - 2rem);
		z-index: 1;
	}

	.backButton {
		top: 1rem;
		left: 1rem;
	}

	.placeTargets {
		top: 1rem;
		right: 1rem;
	}
</style>
