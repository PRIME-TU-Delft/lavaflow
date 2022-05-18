<script lang="ts">
	import init, * as wasm from 'wasm';

	import { onMount, onDestroy } from 'svelte';

	import { hc_level_curves, hc_parent_relations } from '$lib/data/hardCoded';
	import type { BufferGeometry, Material, Mesh } from 'three';
	import { DoubleSide } from 'three';

	let mounted: boolean;
	let aframe: boolean;
	let ar: boolean;
	let gltfUrl: string;
	$: ready = aframe && ar && mounted;

	const arLoaded = () => (ar = true);
	const aframeLoaded = () => {
		aframe = true;

		AFRAME.registerComponent('double-render', {
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
	};

	onMount(async () => {
		await init();

		const tree = new wasm.OpenCVTree({
			pixels_per_curve: hc_level_curves,
			parent_relations: hc_parent_relations
		});

		const settings = new wasm.ModelGenerationSettings(5, 50, 50, 50, 1.0);
		const gltf = wasm.generate_3d_model(tree, settings, 2, 0.7, 0.7, 4, 1, 30, 30, 10);
		const gltfBlob = new Blob([gltf], { type: 'application/json' });
		gltfUrl = URL.createObjectURL(gltfBlob);
		mounted = true;
	});

	onDestroy(() => {
		delete AFRAME.components['double-render'];
	});
</script>

<svelte:head>
	{#if mounted}
		<script src="https://aframe.io/releases/1.0.0/aframe.min.js" on:load={aframeLoaded}></script>
		<script
			src="https://raw.githack.com/AR-js-org/AR.js/master/aframe/build/aframe-ar.js"
			on:load={arLoaded}></script>
	{/if}
</svelte:head>

{#if ready}
	<a-scene
		embedded
		arjs="trackingMethod: best; detectionMode: mono_and_matrix; matrixCodeType: 3x3;"
	>
		<a-marker type="barcode" value="1">
			<a-box position="0 0.5 0" material="opacity: 0.5;" color="red" />

			<a-entity
				double-render
				gltf-model="url({gltfUrl})"
				position="0 0.5 0"
				scale="0.003 0.003 0.003"
				rotation="0 -90 0"
				id="model"
			/>
		</a-marker>
		<a-entity camera />
	</a-scene>
{/if}

<style>
	:global(.a-canvas, #arjs-video) {
		display: block;
		width: 100% !important;
		height: 100% !important;
		margin: 0 !important;
		object-fit: contain !important;
	}
</style>
