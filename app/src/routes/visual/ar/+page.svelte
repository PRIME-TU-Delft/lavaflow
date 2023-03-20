<script lang="ts">
	import 'aframe';
	import VisualiseUi from '../VisualiseUI.svelte';
	import Scene from './Scene.svelte';

	const size = [0.025, 0.0125, 0.025] as [number, number, number];
</script>

<VisualiseUi title="Visual ar" baseUrl="/ar" let:showLava>
	<!-- TODO: customise ui buttons -->
	<a-scene webxr="optionalFeatures: dom-overlay; overlayElement: #overlay;">
		<Scene {size} {showLava} />

		<a-camera position="0 3 3" />
	</a-scene>
</VisualiseUi>
