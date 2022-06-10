<script>
	import SceneViewer from '$lib/components/aframe/SceneViewer.svelte';

	import '@ar-js-org/ar.js';

	// TODO: load gltf contours from cache
</script>

<!-- <div class="iframe">
	<h1>AR view 123</h1>
</div> -->

<SceneViewer arMode />

<style>
	:global(.iframe h1) {
		color: green;
	}
</style>
