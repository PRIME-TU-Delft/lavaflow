<script lang="ts">
	import { goto } from '$app/navigation';
	import ActionButton from '$lib/components/ActionButton.svelte';
	import ActionMenu from '$lib/components/ActionMenu.svelte';
	import Menubar from '$lib/components/Menubar.svelte';
	import P5PreviewCurves from '$lib/components/p5/P5PreviewCurves.svelte';
	import { contourLines } from '$lib/stores/contourLineStore';
	import { difficultyStore } from '$lib/stores/difficultyStore';
	import { gltfStore } from '$lib/stores/gltfStore';
	import { onMount } from 'svelte';

	let gltfLoaded = false;
	let webXRSupport = false;

	onMount(async () => {
		// TODO: move to load() function
		if (!$contourLines.curves || !$contourLines.hierarchy) {
			return goto('/capture');
		}

		console.log($contourLines.curves);

		await gltfStore.setup($contourLines, $difficultyStore.lava_forking);
		gltfStore.build($contourLines);

		webXRSupport = (await navigator?.xr?.isSessionSupported('immersive-ar')) ?? false;

		gltfLoaded = true;
	});
</script>

<Menubar title="preview" />

{#key $contourLines}
	<P5PreviewCurves curves={$contourLines?.curves} />
{/key}

<ActionMenu>
	<ActionButton loading={!gltfLoaded} secondary on:click={() => goto('/visual/3d')}>
		Show in 3d model
	</ActionButton>

	{#if webXRSupport}
		<ActionButton loading={!gltfLoaded} on:click={() => goto('/visual/ar')}>
			Show in AR {'(Recommended)'}
		</ActionButton>
	{:else}
		<ActionButton loading={!gltfLoaded}>Show in AR {'(no official support)'}</ActionButton>
	{/if}
</ActionMenu>
