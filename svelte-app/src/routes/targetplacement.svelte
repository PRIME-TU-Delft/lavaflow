<script lang="ts">
	/**
	 * This view is where the user will place target on the map.
	 */
	import Button from '$lib/components/Button.svelte';
	import NavigationButton from '$lib/components/NavigationButton.svelte';
	import P5TargetPlacement from '$lib/components/p5/P5TargetPlacement.svelte';
	import Page from '$lib/components/Page.svelte';
	import Modal from '$lib/components/Modal.svelte';
	import Instructions from '$lib/components/InstructionsTargets.svelte';

	import { contourLines } from '$lib/stores/contourLineStore';
	import { targetLocations } from '$lib/stores/gltfStore';
	import Draggable from '$lib/data/draggable';

	import { onMount } from 'svelte';
	import { mdiPin, mdiTrashCan, mdiBookOpenVariant } from '@mdi/js';
	import { difficultyStore } from '$lib/stores/difficultyStore';

	let targetSelected = -1;
	let instructionVisible = false;

	const toggleInstruction = () => (instructionVisible = !instructionVisible);
	
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

<Modal title="transformation instructions" closeButtons = "top" bind:visible={instructionVisible}>
	<Instructions />
</Modal>

<Page title="Placing steam turbines" let:foregroundHeight let:foregroundWidth>
	<div slot="background" style="background:#aaa;" />

	{#if $contourLines?.curves && $difficultyStore}
	<div class="sketch">
		<P5TargetPlacement
			bind:targetSelected
			{foregroundHeight}
			{foregroundWidth}
			curves={$contourLines.curves}
		/>
	</div>
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
		{:else}
			<NavigationButton back to="/scan/mapscanning">
				No image found. Go to map scanning
			</NavigationButton>
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
