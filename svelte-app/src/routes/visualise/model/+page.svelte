<script lang="ts">
	import 'aframe';
	import 'aframe-extras';

	import Button from '$lib/components/Button.svelte';
	import SceneViewer from '$lib/components/aframe/SceneViewer.svelte';
	import FancySlider from '$lib/components/input/FancySlider.svelte';
	import { mdiMagnifyMinus, mdiMagnifyPlus } from '@mdi/js';

	let rotation = 0;
	let scale: [number, number, number] = [0.05, 0.025, 0.05];

	function increaseScale() {
		scale = scale.map((axis) => axis * 1.1) as [number, number, number];
	}

	function decreaseScale() {
		scale = scale.map((axis) => axis / 1.1) as [number, number, number];
	}
</script>

<SceneViewer {rotation} {scale} />

<div class="rotationSlider">
	<FancySlider from={-200} to={200} step={10} bind:value={rotation} />
	<div class="iconButton">
		<Button icon={mdiMagnifyPlus} on:click={increaseScale} />
	</div>
	<div class="iconButton">
		<Button icon={mdiMagnifyMinus} on:click={decreaseScale} />
	</div>
</div>

<style>
	.rotationSlider {
		position: absolute;
		bottom: 1rem;
		left: 50%;
		transform: translateX(-50%);
		z-index: 100;
		display: flex;
		gap: 0.5rem;
		max-width: 90vw;
	}
</style>
