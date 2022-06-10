<script lang="ts">
	/**
	 * This view is created to test and show the client the progess of the mountain construction
	 */

	import SceneViewer from '$lib/components/aframe/SceneViewer.svelte';

	import { contourLines } from '$lib/stores/contourLineStore';
	import { gltfStore } from '$lib/stores/gltfStore';

	import { hc_curves, hc_hierarchy } from '$lib/data/hardCoded';
	import { onMount } from 'svelte';

	onMount(async () => {
		if (!$gltfStore) {
			if (!$contourLines) {
				contourLines.set({
					curves: hc_curves,
					hierarchy: hc_hierarchy,
					size: { width: 850, height: 950 }
				});
			}

			await gltfStore.setup($contourLines);
			gltfStore.build();
			console.warn('gltf is loaded from hardcoded data', $gltfStore);
		}
	});
</script>

<SceneViewer />
