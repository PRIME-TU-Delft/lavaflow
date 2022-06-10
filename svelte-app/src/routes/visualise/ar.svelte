<script lang="ts">
	import ArSandBox from '$lib/components/aframe/ArSandBox.svelte';

	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	import { gltfStore } from '$lib/stores/gltfStore';

	// Todo remove these dependencies
	import { contourLines } from '$lib/stores/contourLineStore';
	import { hc_curves, hc_hierarchy } from '$lib/data/hardCoded';

	function loadAr(e: Event) {
		const target = (e.target as HTMLIFrameElement)?.contentDocument?.body;

		console.log(target);
		if (!target) return;

		new ArSandBox({ target, props: { gltfStore } });
	}

	onMount(async () => {
		// if no gltf is stored goto scanning of paper
		// if (!$gltfStore) goto('/scan/mapscanning');

		// TODO remove these in production
		if (!$gltfStore) {
			contourLines.set({
				curves: hc_curves,
				hierarchy: hc_hierarchy,
				size: { width: 850, height: 950 }
			});

			await gltfStore.setup($contourLines);
			gltfStore.build();
			console.warn('gltf is loaded from hardcoded data', $gltfStore);
		}
	});
</script>

<iframe title="arjs sanbox" on:load={loadAr} />
