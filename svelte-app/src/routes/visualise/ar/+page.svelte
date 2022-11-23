<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import WebXRButtons from '$lib/components/aframe/WebXRButtons.svelte';
	import AframeModels from '$lib/components/aframe/AframeModels.svelte';

	import { gltfStore } from '$lib/stores/gltfStore';
	import { debugMode } from '$lib/stores/debugStore';
	import { contourLines } from '$lib/stores/contourLineStore';
	import { hc_curves, hc_hierarchy } from '$lib/data/hardCoded';
	import { difficultyStore } from '$lib/stores/difficultyStore';

	import 'aframe';
	import type { Scene } from 'aframe';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { mdiCubeScan } from '@mdi/js';

	let sceneEl: Scene;
	let lavaVisible = false;
	let arMode = false;

	let defaultScale = [0.05, 0.025, 0.05];
	let arScale = 8;

	$: scale = defaultScale.map((axis) => (arMode ? axis / arScale : axis)) as [
		number,
		number,
		number
	];
	let rotation: number = 0;

	onMount(async () => {
		if ($gltfStore) return; // When gltf store is loaded -> don't (re)load again

		// If debug mode is disabled and there are no contourlines -> goto scanning of paper
		if (!$debugMode && !$contourLines) {
			goto('/scan/mapscanning');
			return;
		}

		if (!$contourLines) {
			// If debug mode is enabled -> load hardcoded data
			console.warn('SCENE VIEWER: gltf is loaded from hardcoded data');
			contourLines.setup({
				curves: hc_curves,
				hierarchy: hc_hierarchy,
				size: { width: 800, height: 960 }
			});
		}

		// TODO: Optimisation this should only be needed when the user is in debug mode
		await gltfStore.setup($contourLines, $difficultyStore.lava_forking);
		gltfStore.build($contourLines);
	});
</script>

<a-scene
	bind:this={sceneEl}
	vr-mode-ui="enterVRButton: #myEnterVRButton; enterARButton: #myEnterARButton"
	webxr="optionalFeatures: dom-overlay; overlayElement: #overlay;"
>
	<AframeModels {scale} {rotation} lava_revealed={lavaVisible} />

	<a-camera position="0 3 3" />
</a-scene>

<div id="overlay">
	<WebXRButtons {sceneEl} bind:lavaVisible />
</div>

<!-- svelte-ignore a11y-missing-content -->
<a id="myEnterVRButton" style="display:none" />

<!-- svelte-ignore a11y-invalid-attribute -->
<a id="myEnterARButton" href="">
	<Button icon={mdiCubeScan}>Native AR Mode</Button>
</a>

<style>
	#overlay {
		display: flex;
		justify-content: space-between;
		align-content: start;
		gap: 0.25rem;
		flex-wrap: wrap;
	}

	#myEnterARButton {
		position: absolute;
		bottom: 1rem;
		right: 1rem;
	}
</style>