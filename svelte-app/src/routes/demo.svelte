<script lang="ts">
	import NavigationButton from '$lib/components/NavigationButton.svelte';

	import { onMount, onDestroy } from 'svelte';

	let mounted: boolean;
	let aframe: boolean;
	let gltfUrl: string = '/output20.gltf';
	$: ready = aframe && mounted; // TODO: mounted is always true when aframe is true

	onMount(async () => {
		mounted = true;
		console.log('mounted')
	});
</script>

<svelte:head>
	{#if mounted}
		<script src="https://aframe.io/releases/1.0.0/aframe.min.js" on:load={() => (aframe = true)}></script>
	{/if}
</svelte:head>

{#if ready}
	<a-scene embedded>
		<div class="backButton">
			<NavigationButton back to="/scan/mapscanning">Rescan image</NavigationButton>
		</div>

		<a-box position="0 1 0" material="opacity: 0.5;" color="red" />

		<a-entity
			gltf-model="url({gltfUrl})"
			position="3 0 -5"
			scale="0.00038 0.1 0.00038"
			rotation="0 -90 0"
			id="model"
		/>

		<a-camera look-controls />
	</a-scene>
{/if}

<style>
	.backButton {
		position: absolute;
		top: 1rem;
		left: 1rem;
		z-index: 1;
		width: 15rem;
		max-width: calc(100vw - 2rem);
	}
</style>
