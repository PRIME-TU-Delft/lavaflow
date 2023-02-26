<script lang="ts">
	import { goto } from '$app/navigation';
	import ActionButton from '$lib/components/ActionButton.svelte';
	import ActionMenu from '$lib/components/ActionMenu.svelte';
	import Menubar from '$lib/components/Menubar.svelte';
	import P5Transform from '$lib/components/p5/P5Transform.svelte';
	import type Draggable from '$lib/data/draggable';
	import LavaError from '$lib/data/LavaError';
	import { contourLines } from '$lib/stores/contourLineStore';
	import { hc_curves, hc_hierarchy } from '$lib/stores/hardCoded';
	import imageStore from '$lib/stores/imageStore';
	import sizeStore from '$lib/stores/sizeStore';
	import { mdiBookInformationVariant, mdiChevronRight } from '@mdi/js';
	import { Button } from 'flowbite-svelte';
	import { onMount } from 'svelte';
	import ErrorMessage from './ErrorMessage.svelte';
	import imageToCountours from './imageToContour';

	let points: Draggable[] = [];
	let error: LavaError | null = null;

	onMount(() => {
		// If no raw image in cache, go back to scan/mapscanning
		if (!$imageStore || !$sizeStore) goto('/capture');
	});

	function applySelection() {
		// TODO do openCV stuff
		const cvError = imageToCountours(points);

		if (cvError) {
			error = new LavaError('Something went wrong detecting curves', cvError);
			return;
		}

		error = null;
	}

	function continueWithDefaultMap() {
		const { curves, hierarchy } = { curves: hc_curves, hierarchy: hc_hierarchy };
		const [hc_width, hc_height] = [800, 667];

		contourLines.setup({
			curves,
			hierarchy,
			size: { width: hc_width, height: hc_height }
		});

		sizeStore.set({ width: hc_width, height: hc_height });

		goto('/preview');
	}
</script>

<Menubar back="/capture" title="Select markers">
	<!-- TODO: implement todo -->
	<Button outline color="red">Reset</Button>
</Menubar>

<P5Transform bind:points on:error={(e) => (error = e.detail.error)} />

<img
	style="display:none"
	height={$sizeStore.height * 2}
	width={$sizeStore.width * 2}
	id="foregroundImage"
	src={$imageStore}
	alt="background"
/>

<ActionMenu>
	{#if error}
		<ErrorMessage {error} on:dismiss={() => (error = null)} on:preBuild={continueWithDefaultMap} />
	{/if}
	<ActionButton secondary href="/select-markers/instructions" icon={mdiBookInformationVariant}>
		Show instruction
	</ActionButton>
	<ActionButton icon={mdiChevronRight} on:click={applySelection}>Apply selection</ActionButton>
</ActionMenu>
