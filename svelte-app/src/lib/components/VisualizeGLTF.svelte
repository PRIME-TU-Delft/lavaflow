<script lang="ts">
	import { onMount } from 'svelte';

	export let gltf: string;
	let model: HTMLElement;

	onMount(() => {
		if (!gltf) return;

		const gltfBlob = new Blob([gltf], { type: 'application/json' });
		const gltfUrl = URL.createObjectURL(gltfBlob);

		console.log(model, gltfUrl);
		model.setAttribute('gltf-model', `url(${gltfUrl})`);
	});
</script>

<a-scene>
	<!-- <a-marker type="barcode" value="0"> -->
	<a-entity bind:this={model} id="model" material="side: double" position="0 0 0" />
	<a-box position="-1 0.5 -3" rotation="0 45 0" color="#4CC3D9" />
	<a-sphere position="0 1.25 -5" radius="1.25" color="#EF2D5E" />
	<a-cylinder position="1 0.75 -3" radius="0.5" height="1.5" color="#FFC65D" />
	<a-plane position="0 0 -4" rotation="-90 0 0" width="4" height="4" color="#7BC8A4" />
	<!-- </a-marker> -->

	<!-- <a-entity camera /> -->
</a-scene>

<style>
	:global(.canvas canvas) {
		width: calc(95vmin - 3rem) !important;
		height: calc(95vmin - 3rem) !important;
		border-radius: 1.5rem;
		margin: 1rem auto;
	}
</style>
