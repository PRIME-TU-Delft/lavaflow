<script lang="ts">
	import SceneViewer from '$lib/components/aframe/SceneViewer.svelte';

	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	import { gltfStore } from '$lib/stores/gltfStore';

	// Todo remove these dependencies
	import { contourLines } from '$lib/stores/contourLineStore';
	import { hc_curves, hc_hierarchy } from '$lib/data/hardCoded';

	let ARLoaded = false;

	function loadAr() {
		ARLoaded = true;
	}

	onMount(async () => {
		// if no gltf is stored goto scanning of paper
		// if (!$gltfStore) goto('/scan/mapscanning');

		// TODO remove these in production
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
	});
</script>

<svelte:head>
	<script
		src="https://raw.githack.com/AR-js-org/AR.js/master/aframe/build/aframe-ar.js"
		on:load={loadAr}></script>
</svelte:head>

{#if ARLoaded}
	<SceneViewer arMode />
{:else}
	<h1>Loading</h1>
{/if}
