<script lang="ts">
	import 'aframe';
	import type { Scene as AScene } from 'aframe';
	import { Button } from 'flowbite-svelte';
	import VisualiseUi from '../VisualiseUI.svelte';
	import Scene from './Scene.svelte';

	let scale = 2;
	let sceneEl: AScene;

	const baseUrl = '/ar';

	async function closeAR() {
		await sceneEl.exitVR();
		scale = 2;
	}
</script>

<VisualiseUi title="Visual AR" {baseUrl} let:showLava>
	<a-scene
		bind:this={sceneEl}
		vr-mode-ui="enterVRButton: #myEnterVRButton; enterARButton: #myEnterARButton"
		webxr="optionalFeatures: dom-overlay; overlayElement: #overlay;"
	>
		<Scene {scale} {showLava} />

		<a-camera position="0 3 3" />
	</a-scene>

	<div slot="arActions">
		<div class="hidden" id="myEnterVRButton">Open vr</div>

		<Button class="enterAR w-full" on:click={() => (scale = 0.5)} color="red" id="myEnterARButton">
			Enter AR Mode
		</Button>

		<Button class="closeAR hidden w-full" on:click={closeAR} color="red">Close AR Mode</Button>
	</div>
</VisualiseUi>

<style>
	:global(#overlay:xr-overlay .enterAR) {
		display: none !important;
	}

	:global(#overlay:xr-overlay .closeAR) {
		display: initial !important;
	}
</style>
