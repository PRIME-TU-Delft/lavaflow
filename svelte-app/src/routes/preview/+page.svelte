<script lang="ts">
	import { goto } from '$app/navigation';
	import ActionButton from '$lib/components/ActionButton.svelte';
	import ActionMenu from '$lib/components/ActionMenu.svelte';
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';
	import Menubar from '$lib/components/Menubar.svelte';
	import P5PreviewCurves from '$lib/components/p5/P5PreviewCurves.svelte';
	import { contourLines } from '$lib/stores/contourLineStore';
	import { difficultyStore } from '$lib/stores/difficultyStore';
	import { gltfStore } from '$lib/stores/gltfStore';
	import { onMount } from 'svelte';
	import type { PageData } from './$types';

	export let data: PageData;

	let gltfLoaded = false;
	let webXRSupport = false;

	onMount(async () => {
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

<Menubar back="/capture" title="preview">
	<div slot="backTitle">Re-scan</div>
</Menubar>

{#key $contourLines}
	<P5PreviewCurves curves={$contourLines?.curves} />
{/key}

<ActionMenu>
	{#if data.error}
		<ErrorMessage error={data.error} on:dismiss={() => goto('/preview')} />
	{/if}

	<ActionButton loading={!gltfLoaded} secondary href="/visual/3d">
		Show in 3D model
		<div slot="loading">Loading contour lines</div>
	</ActionButton>

	{#if !gltfLoaded}
		<!-- Filled with no content -->
	{:else if webXRSupport}
		<ActionButton loading={!gltfLoaded} href="/visual/ar">
			Show in AR {'(Recommended)'}
		</ActionButton>
	{:else}
		<ActionButton loading={!gltfLoaded} href="/visual/ar">
			Show in AR {'(no official support)'}
		</ActionButton>
	{/if}
</ActionMenu>
