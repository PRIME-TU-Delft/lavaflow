<script lang="ts">
	import { goto } from '$app/navigation';
	import ActionButton from '$lib/components/ActionButton.svelte';
	import ActionMenu from '$lib/components/ActionMenu.svelte';
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';
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
	import imageToCountours from './imageToContour';

	let points: [Draggable, Draggable, Draggable, Draggable] | [] = [];
	let error: LavaError | null = null;

	onMount(() => {
		// If no raw image in cache, go back to scan/mapscanning
		// TODO: move to load() function
		if (!$imageStore || !$sizeStore) goto('/capture');
	});

	function applySelection() {
		if (points.length !== 4) {
			error = new LavaError('Please select 4 points', 'You need to select 4 points to continue');
			return;
		}
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
	<!-- TODO: implement click -->
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
		<ErrorMessage {error} hasActions on:dismiss={() => (error = null)}>
			<Button class="w-full" outline color="red" href="capture">Rescan image</Button>
			<Button class="w-full grow" outline color="red" on:click={continueWithDefaultMap}>
				Continue with pre-build contours
			</Button>
		</ErrorMessage>
	{/if}
	<ActionButton secondary href="/select-markers/instructions" icon={mdiBookInformationVariant}>
		Show instruction
	</ActionButton>
	<ActionButton icon={mdiChevronRight} on:click={applySelection}>Apply selection</ActionButton>
</ActionMenu>
