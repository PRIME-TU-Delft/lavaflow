<script lang="ts">
	import { gltfStore } from '$lib/stores/gltfStore';
	import { Canvas } from '@threlte/core';
	import VisualiseUI from '../VisualiseUI.svelte';
	import Scene from './Scene.svelte';

	$: $gltfStore.gltf_url && !$gltfStore.usdz_url && gltfStore.getUSDZ();
</script>

<VisualiseUI title="Visual 3d" baseUrl="/3d" let:showLava>
	<div slot="arActions">
		{$gltfStore.gltf_url}
		{$gltfStore.usdz_url}
		{#if $gltfStore.usdz_url}
			<a id="link" rel="ar" href={$gltfStore.usdz_url}>{$gltfStore.usdz_url}</a>
			<a
				class="btn btn-secondary w-full"
				id="link"
				rel="ar"
				href={$gltfStore.usdz_url}
				download={$gltfStore.usdz_url}
			>
				QuickLook
			</a>
		{/if}
	</div>

	<Canvas>
		<Scene {showLava} />
	</Canvas>
</VisualiseUI>
