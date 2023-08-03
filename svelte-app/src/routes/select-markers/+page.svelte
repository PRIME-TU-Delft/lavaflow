<script lang="ts">
	// import { goto } from '$app/navigation';
	import ActionButton from '$lib/components/ActionButton.svelte';
	import ActionMenu from '$lib/components/ActionMenu.svelte';
	import Menubar from '$lib/components/Menubar.svelte';
	// import LavaError from '$lib/data/LavaError';
	import type Draggable from '$lib/data/draggable';
	// import { hc_curves, hc_hierarchy } from '$lib/data/hardCoded';
	// import { contourLines } from '$lib/stores/contourLineStore';
	import imageStore from '$lib/stores/imageStore';
	import sizeStore from '$lib/stores/sizeStore';
	import { mdiChevronRight, mdiHelp } from '@mdi/js';
	// import P5Transform from './P5Transform.svelte';
	// import { extractSelectedArea, imageToContoursGammaCV } from './imageToContour';

	let points: [Draggable, Draggable, Draggable, Draggable] | [] = [];
	// let error: LavaError | null = null;
	let resetKey: number = 0;

	let perspectiveRemovedImage: HTMLCanvasElement;

	$: hasStores = !!($imageStore && $sizeStore.height && $sizeStore.width);

	function applySelection() {
		// Update the perspective-image inside the rendered canvas
		updatePerspectiveRemovedImage();
		// const opencvError = imageToContoursGammaCV(perspectiveRemovedImage);
		// if (opencvError) {
		// 	error = new LavaError('Something went wrong while detecting curves', opencvError);
		// 	return;
		// }
		// error = null;
	}

	function continueWithDefaultMap() {
		// const { curves, hierarchy } = { curves: hc_curves, hierarchy: hc_hierarchy };
		// const [hc_width, hc_height] = [1000, 800];
		// contourLines.setup({
		// 	curves,
		// 	hierarchy,
		// 	size: { width: hc_width, height: hc_height }
		// });
		// sizeStore.set({ width: hc_width, height: hc_height });
		// goto('/preview');
	}

	function resetPoints() {
		resetKey = Math.random();
	}

	function updatePerspectiveRemovedImage() {
		if (!perspectiveRemovedImage) return;

		console.log('Updating canvas');
		if (points.length !== 4) {
			// TODO: error = new LavaError('Please select 4 points', 'You need to select 4 points to continue');
			return;
		}
		type Draggbles = [Draggable, Draggable, Draggable, Draggable];

		// extractSelectedArea(points.slice() as Draggbles, perspectiveRemovedImage);
	}
</script>

<Menubar back="/capture" title="Select markers">
	<ActionButton secondary on:click={resetPoints}>Reset</ActionButton>
</Menubar>

<ActionMenu>
	<!-- {#if error}
        TODO: Show error message
		<ErrorMessage {error} hasActions on:dismiss={() => (error = null)}>
			<Button class="w-full" outline color="red" href="capture">Rescan image</Button>
			<Button class="w-full grow" outline color="red" on:click={continueWithDefaultMap}>
				Continue with pre-build contours
			</Button>
		</ErrorMessage>
	{/if} -->
	<ActionButton secondary href="/select-markers/instructions" icon={mdiHelp}>
		<!-- TODO: fix instructions -->
		Show instruction
	</ActionButton>
	<ActionButton disabled={!hasStores} icon={mdiChevronRight} on:click={applySelection}>
		Apply selection
	</ActionButton>
</ActionMenu>
