<script lang="ts">
	/**
	 * This view is where the user will place target on the map.
	 */
	import Button from '$lib/components/Button.svelte';
	import NavigationButton from '$lib/components/NavigationButton.svelte';
	import P5TargetPlacement from '$lib/components/p5/P5TargetPlacement.svelte';
	import Page from '$lib/components/Page.svelte';

	import { contourLines } from '$lib/stores/contourLineStore';
	import { targetLocations } from '$lib/stores/gltfStore';
	import Draggable from '$lib/data/draggable';

	import { onMount } from 'svelte';
	import { mdiPin, mdiTrashCan } from '@mdi/js';

	let targetSelected = -1;

	function addTarget(x: number, y: number) {
		const newTarget = new Draggable(x, y, 20);
		newTarget.enableSelection();
		targetLocations.add(newTarget);
	}

	function removeTarget() {
		if (targetSelected == -1) return;
		targetLocations.remove(targetSelected);
		targetSelected = -1;
	}

	onMount(() => {});
</script>

<Page title="Placing steam turbines" let:foregroundHeight let:foregroundWidth>

	<div slot="background" style="background:#aaa;" />

	{#if $contourLines}
		<div class="sketch">
			<P5TargetPlacement
				bind:targetSelected
				{foregroundHeight}
				{foregroundWidth}
				curves={$contourLines.curves}
			/>
		</div>
	{:else}
		<NavigationButton to="/scan/mapscanning">No image found. Go to map scanning</NavigationButton>
	{/if}

	<div slot="footer">
		{#if $contourLines}
			{#if targetSelected != -1}
				<Button secondary icon={mdiTrashCan} on:click={removeTarget}>
					Remove steam turbine #{targetSelected}
				</Button>
			{/if}
			<Button icon={mdiPin} on:click={() => addTarget(foregroundWidth / 2, foregroundHeight / 2)}>
				Add steam turbine
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
