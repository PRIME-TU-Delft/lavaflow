<script>
	import { Canvas } from '@threlte/core';
	import VisualiseUi from '../VisualiseUI.svelte';
	import Scene from './Scene.svelte';
</script>

<VisualiseUi title="Visual 3d" baseUrl="/3d" let:showLava>
	<Canvas>
		<Scene {showLava} />
	</Canvas>
</VisualiseUi>
