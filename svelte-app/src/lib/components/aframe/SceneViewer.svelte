<script lang="ts">
	import AframeModels from '$lib/components/aframe/AframeModels.svelte';
	import NavigationButton from '$lib/components/NavigationButton.svelte';

	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	import { gltfStore } from '$lib/stores/gltfStore';
	import { contourLines } from '$lib/stores/contourLineStore';
	import { hc_curves, hc_hierarchy } from '$lib/data/hardCoded';
	import { debugMode } from '$lib/stores/debugStore';

	export let arMode = false;

	onMount(async () => {
		if ($gltfStore) return; // When gltf store is loaded -> don't (re)load again

		// If debug mode is disabled and there are no contourlines -> goto scanning of paper
		if (!$debugMode && !$contourLines) {
			goto('/scan/mapscanning');
			return;
		}

		// If debug mode is enabled -> load hardcoded data
		console.warn('gltf is loaded from hardcoded data');

		if (!$contourLines) {
			contourLines.set({
				curves: hc_curves,
				hierarchy: hc_hierarchy,
				size: { width: 850, height: 950 }
			});
		}

		await gltfStore.setup($contourLines);
		gltfStore.build();
	});
</script>

<a-scene class:arMode renderer="logarithmicDepthBuffer: true;" embedded vr-mode-ui="enabled: false">
	<slot name="overlay">
		<div class="button backButton">
			<NavigationButton back to="/scan/preview">Back to preview</NavigationButton>
		</div>

		<div class="button placeTargets">
			<NavigationButton to="/targetplacement">Place targets</NavigationButton>
		</div>
	</slot>

	<a-entity light="type: ambient; color: #fff" />

	<!-- If AR is enabled -> wrap model in  -->
	{#if arMode}
		<a-marker preset="hiro">
			<a-box position="0 0 -1" rotation="0 0 0" color="red" />

			<AframeModels />
		</a-marker>
	{:else}
		<AframeModels />
	{/if}

	<a-camera />
</a-scene>

<style>
	.arMode {
		position: inherit !important;
	}

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
