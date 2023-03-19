<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import ActionButton from '$lib/components/ActionButton.svelte';
	import ActionMenu from '$lib/components/ActionMenu.svelte';
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';
	import Menubar from '$lib/components/Menubar.svelte';
	import P5Transform from './P5Transform.svelte';
	import type Draggable from '$lib/data/draggable';
	import LavaError from '$lib/data/LavaError';
	import { contourLines } from '$lib/stores/contourLineStore';
	import { hc_curves, hc_hierarchy } from '$lib/stores/hardCoded';
	import imageStore from '$lib/stores/imageStore';
	import sizeStore from '$lib/stores/sizeStore';
	import { mdiChevronRight, mdiHelp } from '@mdi/js';
	import { Button } from 'flowbite-svelte';
	import { imageToContoursGammaCV, extractSelectedArea } from './imageToContour';

	let points: [Draggable, Draggable, Draggable, Draggable] | [] = [];
	let error: LavaError | null = null;

	$: hasStores = !!($imageStore && $sizeStore.height && $sizeStore.width);

	// async function applySelection() {
	// 	if (points.length !== 4) {
	// 		error = new LavaError('Please select 4 points', 'You need to select 4 points to continue');
	// 		return;
	// 	}
	// 	const cvError = await imageToCountours(points);

	// 	if (cvError) {
	// 		error = new LavaError('Something went wrong detecting curves', cvError);
	// 		return;
	// 	}

	// 	error = null;
	// }

	function applySelection() {

		// Update the perspective-image inside the rendered canvas
		updatePerspectiveRemovedImage()

		const opencvError = imageToContoursGammaCV(perspectiveRemovedImage)

		if (opencvError) {
			error = new LavaError("Something went wrong while detecting curves", opencvError);
			return;
		}

		error = null
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

	let perspectiveRemovedImage: HTMLCanvasElement;

	function updatePerspectiveRemovedImage() {
		console.log('Updating canvas');
		if (points.length !== 4) {
			error = new LavaError('Please select 4 points', 'You need to select 4 points to continue');
			return;
		}
		type Draggbles = [Draggable, Draggable, Draggable, Draggable];

		extractSelectedArea(points.slice() as Draggbles, perspectiveRemovedImage)
	}
</script>

<Menubar back="/capture" title="Select markers">
	<!-- TODO: implement click -->
	<Button outline color="red">Reset</Button>
</Menubar>

{#if hasStores}
	{#key $imageStore}
		<P5Transform
			on:pointsUpdated={updatePerspectiveRemovedImage}
			bind:points
			on:error={(e) => (error = e.detail.error)}
		/>

		<canvas
			id="perspectiveRemovedImage"
			class="absolute top-20 right-10 h-40 w-40 object-contain"
			width="800"
			height="800"
			bind:this={perspectiveRemovedImage}
		/>
		<img
			style="display:none;position:absolute;width:100px;height:100px;top:500px;right:0;"
			height={$sizeStore.height}
			width={$sizeStore.width}
			id="foregroundImage"
			src={$imageStore}
			alt="background"
		/>
		<canvas id="canvasOutput" />
	{/key}
{/if}

<ActionMenu>
	{#if error}
		<ErrorMessage {error} hasActions on:dismiss={() => (error = null)}>
			<Button class="w-full" outline color="red" href="capture">Rescan image</Button>
			<Button class="w-full grow" outline color="red" on:click={continueWithDefaultMap}>
				Continue with pre-build contours
			</Button>
		</ErrorMessage>
	{/if}
	<ActionButton secondary href="/select-markers/instructions" icon={mdiHelp}>
		Show instruction
	</ActionButton>
	<ActionButton disabled={!hasStores} icon={mdiChevronRight} on:click={applySelection}>
		Apply selection
	</ActionButton>
</ActionMenu>
