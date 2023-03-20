<script>
	import { Canvas } from '@threlte/core';
	import VisualiseUi from '../VisualiseUI.svelte';
	import Scene from './Scene.svelte';
</script>

<VisualiseUi title="Visual ar" baseUrl="/ar" let:showLava>
	<Canvas>
		<Scene {showLava} />
	</Canvas>
</VisualiseUi>
