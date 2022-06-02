<script lang="ts">
	import NavigationButton from '$lib/components/NavigationButton.svelte';

	import {gltfStore, targetLocations} from "$lib/stores/gltfStore"
	import { onMount } from 'svelte';

	let mounted: boolean;
	let aframe: boolean;
	$: ready = (aframe || window.AFRAME) && mounted;

	onMount(async () => {
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

		<a-entity
			gltf-model="url({$gltfStore})"
			scale="0.01 0.1 0.01"
			position="5 0 5"
			rotation="0 -90 0"
			id="model"
		/>

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
