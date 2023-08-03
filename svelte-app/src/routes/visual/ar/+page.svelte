<script lang="ts">
	import 'aframe';
	import type { Scene as AScene } from 'aframe';
	import VisualiseUI from '../VisualiseUI.svelte';
	import Scene from './Scene.svelte';
	import Button from '$lib/components/Button.svelte';

	let scale = 2;
	let rotate = 0;
	let sceneEl: AScene;

	const baseUrl = '/ar';

	async function closeAR() {
		await sceneEl.exitVR();
		scale = 2;
	}
</script>

<VisualiseUI title="Visual AR" {baseUrl} let:showLava>
	<a-scene
		bind:this={sceneEl}
		vr-mode-ui="enterVRButton: #myEnterVRButton; enterARButton: #myEnterARButton"
		webxr="optionalFeatures: dom-overlay; overlayElement: #overlay;"
	>
		<Scene rotation={rotate} {scale} {showLava} />

		<a-camera position="0 3 3" />
	</a-scene>

	<div slot="arActions">
		<div class="flex gap-2 bg-white/70 backdrop:blur-lg p-2 rounded no-touch">
			<div>Rotate:</div>
			<input type="range" min="-200" max="200" bind:value={rotate} class="range range-primary" />
		</div>

		<div class="hidden" id="myEnterVRButton">Open vr</div>

		<Button
			twClass="enterAR w-full mt-2"
			on:click={() => (scale = 0.5)}
			id="myEnterARButton"
			fullwidth
		>
			Enter AR Mode
		</Button>

		<Button twClass="closeAR hidden w-full mt-2" on:click={closeAR} fullwidth>Close AR Mode</Button>
	</div>
</VisualiseUI>

<style>
	:global(#overlay:xr-overlay .enterAR) {
		display: none !important;
	}

	:global(#overlay:xr-overlay .closeAR) {
		display: initial !important;
	}
</style>
