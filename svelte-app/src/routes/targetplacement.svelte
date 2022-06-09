<script lang="ts">
	/**
	 * This view is where the user will place target on the map.
	 */
	import Button from '$lib/components/Button.svelte';
	import NavigationButton from '$lib/components/NavigationButton.svelte';
	import Image from '$lib/components/Image.svelte';
	import P5TargetPlacement from '$lib/components/p5/P5TargetPlacement.svelte';
	import Page from '$lib/components/Page.svelte';

	import { perspectiveImage, demoPerspectiveImage } from '$lib/stores/imageStore';
	import { targetLocations } from '$lib/stores/gltfStore';
	import Draggable from '$lib/data/draggable';

	import { onMount } from 'svelte';
	import { mdiPin, mdiTrashCan } from '@mdi/js';

	let targetSelected = -1;

	function addTarget(x: number, y: number) {
		const newTarget = new Draggable(x, y, 20);
		targetLocations.add(newTarget);
	}

	function removeTarget() {
		if (targetSelected == -1) return;
		targetLocations.remove(targetSelected);
		targetSelected = -1;
	}

	onMount(() => {
		if (!$perspectiveImage) perspectiveImage.set(demoPerspectiveImage);
	});
</script>

<Page title="image transformation" let:foregroundHeight let:foregroundWidth>
	<!-- TODO: have a genuine back button -->
	<NavigationButton slot="headerButton" to="demo" back>Back to visualise</NavigationButton>

	<div slot="background">
		<Image src={$perspectiveImage} alt="background" />
	</div>

	{#if $perspectiveImage}
		<div class="sketch">
			<P5TargetPlacement bind:targetSelected {foregroundHeight} {foregroundWidth} />
		</div>
	{:else}
		<NavigationButton to="/scan/mapscanning">No image found. Go to map scanning</NavigationButton>
	{/if}

	<div slot="footer">
		{#if $perspectiveImage}
			{#if targetSelected != -1}
				<Button secondary icon={mdiTrashCan} on:click={removeTarget}>
					Remove target #{targetSelected}
				</Button>
			{/if}
			<Button icon={mdiPin} on:click={() => addTarget(foregroundWidth / 2, foregroundHeight / 2)}>
				Add target
			</Button>
		{/if}
	</div>
</Page>

<style>
	.sketch {
		height: 100%;
		touch-action: none;
		user-select: none;
	}
</style>
