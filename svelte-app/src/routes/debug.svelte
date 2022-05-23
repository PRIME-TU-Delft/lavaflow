<script lang="ts">
	import Button from '$lib/components/Button.svelte';

	import init, * as wasm from 'wasm';
	import 'aframe';

	import { onMount, onDestroy, beforeUpdate, afterUpdate } from 'svelte';

	import { hc_level_curves, hc_parent_relations } from '$lib/data/hardCoded';
	import type { BufferGeometry, Material, Mesh } from 'three';
	import { DoubleSide } from 'three';

	let gltfUrl: string;
	let sidebarOpen = true;
	let ready = false;

	AFRAME.registerComponent('double-render', {
		dependencies: ['dynamic-gltf'],
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
	AFRAME.registerComponent('dynamic-gltf', {
		init: function () {
			const tree = new wasm.OpenCVTree({
				pixels_per_curve: hc_level_curves,
				parent_relations: hc_parent_relations
			});

			const settings = new wasm.ModelGenerationSettings(5, 50, 50, 50, 1.0);
			const gltf = wasm.generate_3d_model(tree, settings, 2, 0.7, 0.7, 4, 1, 30, 30, 10);
			const gltfBlob = new Blob([gltf], { type: 'application/json' });
			gltfUrl = URL.createObjectURL(gltfBlob);

			this.el.setAttribute('gltf-model', `url(${gltfUrl})`);
		}
	});

	onMount(async () => {
		await init();

		ready = true;
	});

	onDestroy(() => {
		delete AFRAME.components['double-render'];
		delete AFRAME.components['dynamic-gltf'];
	});
</script>

<div class="openSidebar">
	<Button on:click={() => (sidebarOpen = !sidebarOpen)}>Open sidebar</Button>
</div>

<aside class="sideBar" class:sidebarOpen />

{#if ready}
	<a-scene embedded>
		<a-light type="ambient" intensity="2" />
		<a-light position="0 3 0" intensity="2" type="point" />
		<a-light position="-10 3 10" intensity="2" type="point" />

		<a-box position="0 1 0" material="opacity: 0.5;" color="red" />

		<a-entity
			dynamic-gltf
			double-render
			position="5 0 -5"
			scale="0.0038 0.0038 0.0038"
			rotation="0 -90 0"
		/>
	</a-scene>
{/if}

<style lang="scss">
	.openSidebar {
		width: 20rem;
		max-width: calc(100vw - 2rem);
		left: 1rem;
		top: 1rem;
		position: fixed;
	}

	aside {
		position: fixed;
		top: 0;
		right: 0;
		width: 300px;
		height: 100vh;
		background: var(--primary);
		z-index: 10;
		transform: translateX(-100%);
		transition: transform 0.5s ease-in-out;
	}

	aside.sidebarOpen {
		transform: translateX(0);
	}
</style>
