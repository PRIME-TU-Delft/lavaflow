<script lang="ts">
	import NavigationButton from '$lib/components/NavigationButton.svelte';

	import { contourLines } from '$lib/stores/contourLineStore';

	import {gltfStore, targetLocations, gltfStringToUrl} from "$lib/stores/gltfStore"
	import {hc_curves, hc_hierarchy} from "$lib/data/hardCoded"
	import { onMount } from 'svelte';

	let mounted: boolean;
	let aframe: boolean;
	$: ready = (aframe || window.AFRAME) && mounted;

	onMount(async () => {
		console.log($targetLocations)

		if (!$gltfStore) {
			contourLines.set({
				curves: hc_curves,
				hierarchy: hc_hierarchy
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
		<script src="https://aframe.io/releases/1.0.0/aframe.min.js" on:load={() => (aframe = true)}></script>
	{/if}
</svelte:head>

{#if ready}
	<a-scene embedded>
		<div class="button backButton">
			<NavigationButton back to="/scan/mapscanning">Rescan image</NavigationButton>
		</div>

		<div class="button placeTargets">
			<NavigationButton to="/targetplacement">Place targets</NavigationButton>
		</div>

		<a-box position="0 1 0" material="opacity: 0.5;" color="red" />
		<a-entity light="color: #AFA; intensity: 1.5" position="-1 1 0"></a-entity>

		<a-entity position="7 -1 -5" scale="0.005 0.05 0.005" rotation="0 -90 0">
			<a-entity gltf-model="url({$gltfStore})"/>

			{#each $targetLocations as target}
				{console.log(target, gltfStore.getAlitituteAndGradient(target.x, target.y) )}
				
				<a-box depth="18" width="18" height="100" position="{target.x} 75 {target.y}" color="yellow" /> 
			{/each}
		</a-entity>
		

		

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
