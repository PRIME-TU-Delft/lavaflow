<script>
	import { Canvas } from '@threlte/core';
	import VisualiseUI from '../VisualiseUI.svelte';
	import Scene from './Scene.svelte';
</script>

<VisualiseUI title="Visual 3d" baseUrl="/3d" let:showLava>
	<Canvas>
		<Scene {showLava} />
	</Canvas>
</VisualiseUI>
