<script lang="ts">
	import { onDestroy, onMount } from 'svelte';

	import Input from '$lib/components/Input.svelte';

	export let gltf: string;
	let model: HTMLElement;

	let modelPos = 0;
	let modelScale = 1;

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
			this.nodeMap = {};
			this.prepareMap.bind(this);
			this.traverseMesh.bind(this);

			this.el.addEventListener('model-loaded', (e) => {
				this.prepareMap();
				this.update();
			});
		},
		prepareMap: function () {
			this.traverseMesh((node) => {
				this.nodeMap[node.uuid] = node.material.opacity;
			});
		},
		update: function () {
			this.traverseMesh((node) => {
				node.material.opacity = this.nodeMap[node.uuid] * this.data.opacityFactor;
				node.material.transparent = node.material.opacity < 1.0;
				node.material.needsUpdate = true;
			});
		},
		traverseMesh: function (func) {
			var mesh = this.el.getObject3D('mesh');
			console.log('mesh', mesh);

			if (!mesh) return;

			mesh.traverse((node) => {
				if (node.isMesh) {
					console.log(node.material);
					node.material.side = 1;
				}
			});
		}
	});

	onDestroy(() => {
		delete AFRAME.components['model-relative-opacity'];
	});
</script>

<Input label="scale" bind:value={modelScale} />
<Input label="modelx" bind:value={modelPos} />

<a-scene embedded renderer="colorManagement: true">
	<a-light position="0 2 -1.9" intensity="2" type="point" />

	<a-entity
		model-relative-opacity
		position="-10 -2 -20"
		scale="0.01 0.01 0.01"
		bind:this={model}
		id="model"
	/>
	<a-sphere material="opacity: 0.5" position="0 1.25 -5" radius="1.25" color="#EF2D5E" />
	<a-entity id="entity" position="0 2 -2" />
</a-scene>

<style>
	:global(.canvas canvas) {
		width: calc(95vmin - 3rem) !important;
		height: calc(95vmin - 3rem) !important;
		border-radius: 1.5rem;
		margin: 1rem auto;
	}
</style>
