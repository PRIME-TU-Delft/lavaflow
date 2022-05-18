<script lang="ts">
	import Input from '$lib/components/Input.svelte';

	import { onDestroy, onMount } from 'svelte';

	import type { BufferGeometry, Material, Mesh } from 'three';
	import { DoubleSide } from 'three';
	import { registerComponent, components } from 'aframe';

	export let gltf: string;

	let model: HTMLElement;
	let modelPos = -2; // y position of model

	/**
	 * Run on mount (when all DOM elements have laoded)
	 * Convert gltf string to (blob) url and set gltf-model attribute to model element
	 */
	onMount(() => {
		if (!gltf) return;

		const gltfBlob = new Blob([gltf], { type: 'application/json' });
		const gltfUrl = URL.createObjectURL(gltfBlob);

		model.setAttribute('gltf-model', `url(${gltfUrl})`);
	});

	/**
	 * Register aframe component to render a mesh on both sides
	 */
	registerComponent('double-render', {
		schema: { opacityFactor: { default: 0.5 } },
		init: function () {
			this.traverseMesh.bind(this);

			this.el.addEventListener('model-loaded', () => {
				this.traverseMesh();
			});
		},
		traverseMesh: function () {
			const mesh = this.el.getObject3D('mesh');

			if (!mesh) return;

			mesh.traverse((node) => {
				if (node.type == 'Mesh') {
					(node as Mesh<BufferGeometry, Material>).material.side = DoubleSide;
				}
			});
		}
	});

	/**
	 * Destoy double-render component when this svelte component is out of memory
	 */
	onDestroy(() => {
		delete components['double-render'];
	});
</script>

<Input label="move y" bind:value={modelPos} />

<a-scene embedded arjs="trackingMethod: best; detectionMode: mono_and_matrix; matrixCodeType: 3x3;">
	<!-- LIGHTS -->
	<a-light position="0 2 -1.9" intensity="2" type="point" />

	<a-marker type="barcode" value="0">
		<a-box color="red" position="0 0.5 0" />

		<!-- GLTF MODEL -->
		<a-entity
			double-render
			position="0 {modelPos} 0"
			scale="0.005 0.005 0.005"
			rotation="0 -90 0"
			bind:this={model}
			id="model"
		/>
	</a-marker>

	<!-- SPHERE FOR DEBUGGING -->
	<!-- <a-sphere material="opacity: 0.5" position="0 1.25 -5" radius="1.25" color="#EF2D5E" /> -->

	<a-entity camera />
</a-scene>

<style>
	:global(.a-canvas, #arjs-video) {
		display: block;
		width: 100% !important;
		height: 100% !important;
		margin: 0 !important;
		object-fit: contain !important;
	}
</style>
