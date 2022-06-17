<script lang="ts">
	import 'aframe';
	import 'aframe-extras';

	import SceneViewer from '$lib/components/aframe/SceneViewer.svelte';
</script>

<SceneViewer {rotation} />

<div class="rotationSlider">
	<FancySlider from={-200} to={200} step={10} bind:value={rotation} />
</div>

<style>
	.rotationSlider {
		position: absolute;
		bottom: 1rem;
		left: 50%;
		transform: translateX(-50%);
		z-index: 100;
	}
</style>
