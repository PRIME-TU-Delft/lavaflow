<script lang="ts">
	import { gltfStore } from '$lib/stores/gltfStore';

	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	import SceneViewer from '$lib/components/aframe/SceneViewer.svelte';

	onMount(() => {
		// if no gltf is stored goto scanning of paper
		if (!$gltfStore) goto('/scan/mapscanning');
	});
</script>

<SceneViewer />
