<script lang="ts">
	import { onDestroy, onMount } from 'svelte';

	import Input from '$lib/components/Input.svelte';

	export let gltf: string;
	let model: HTMLElement;

	let modelPos = -2;

	onMount(() => {
		if (!gltf) return;

		const gltfBlob = new Blob([gltf], { type: 'application/json' });
		const gltfUrl = URL.createObjectURL(gltfBlob);

		console.log(model, gltfUrl);
		model.setAttribute('gltf-model', `url(${gltfUrl})`);
	});

	AFRAME.registerComponent('model-relative-opacity', {
		schema: { opacityFactor: { default: 0.5 } },
		init: function () {
			this.traverseMesh.bind(this);

			this.el.addEventListener('model-loaded', () => {
				this.traverseMesh();
			});
		},
		traverseMesh: function () {
			const mesh = this.el.getObject3D('mesh');
			console.log('mesh', mesh);

			if (!mesh) return;

			mesh.traverse((node) => {
				if (node.isMesh) node.material.side = THREE.DoubleSide;
			});
		}
	});

	onDestroy(() => {
		delete AFRAME.components['model-relative-opacity'];
	});
</script>

<Input label="move y" bind:value={modelPos} />

<a-scene embedded renderer="colorManagement: true">
	<a-light position="0 2 -1.9" intensity="2" type="point" />

	<a-marker preset="hiro">
		<a-entity
			model-relative-opacity
			position="0 {modelPos} 0"
			scale="0.01 0.01 0.01"
			bind:this={model}
			id="model"
		/>
		<a-sphere material="opacity: 0.5" position="0 1.25 -5" radius="1.25" color="#EF2D5E" />
		<a-entity id="entity" position="0 2 -2" />
	</a-marker>
</a-scene>

<style>
	:global(.canvas canvas) {
		width: calc(95vmin - 3rem) !important;
		height: calc(95vmin - 3rem) !important;
		border-radius: 1.5rem;
		margin: 1rem auto;
	}
</style>
