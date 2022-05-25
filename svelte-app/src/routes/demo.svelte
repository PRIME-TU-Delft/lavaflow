<script lang="ts">
	import NavigationButton from '$lib/components/NavigationButton.svelte';

	import init, * as wasm from 'wasm';

	import { onMount, onDestroy } from 'svelte';

	import { hc_curves, hc_hierarchy } from '$lib/data/hardCoded';
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
						// (node as Mesh<BufferGeometry, Material>).material.opacity = 0.8;
					}
				});
			}
		});
	};

	onMount(async () => {
		await init();

		const tree = new wasm.OpenCVTree({
			pixels_per_curve: hc_curves,
			parent_relations: hc_hierarchy
		});

		console.log(hc_curves, hc_hierarchy);

		console.log(tree);

		// const api = new wasm.ModelConstructionApi();
		// api.base(tree);
		// api.set_basic_parameters(100, 100, 10);

		// const gltf = api.build();

		// console.log({ api });

		// const gltfBlob = new Blob([gltf], { type: 'application/json' });
		// gltfUrl = URL.createObjectURL(gltfBlob);
		// mounted = true;
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
		<div class="backButton">
			<NavigationButton back to="/scan/mapscanning">Rescan image</NavigationButton>
		</div>
		<!-- <a-entity light="type: ambient; color: #CCC" /> -->

		<a-marker type="barcode" value="1">
			<a-light position="0 3 0" intensity="2" type="point" />
			<a-light position="-3 3 4" intensity="2" type="point" />

			<a-box position="0 1 0" material="opacity: 0.5;" color="red" />

			<a-entity
				double-render
				gltf-model="url({gltfUrl})"
				position="1 0 -1"
				scale="0.0038 0.0038 0.0038"
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

	.backButton {
		position: absolute;
		top: 1rem;
		left: 1rem;
		z-index: 1;
		width: 15rem;
		max-width: calc(100vw - 2rem);
	}
</style>
