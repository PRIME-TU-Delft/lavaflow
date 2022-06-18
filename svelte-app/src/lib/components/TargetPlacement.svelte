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
	import { craterLocations, targetLocations } from '$lib/stores/locationStore';
	import Draggable from '$lib/data/draggable';

	import { mdiPin, mdiTrashCan, mdiHelpCircle } from '@mdi/js';
	import { difficultyStore } from '$lib/stores/difficultyStore';

	let targetSelected = -1;
	let instructionVisible = false;

	function addTarget(x: number, y: number, viewportwidth: number, viewportheight: number) {
		// If the maximum amount of steam turbines has been reached, don't add more
		if ($targetLocations.length >= $difficultyStore.max_steam_turbines) return;

		const newTarget = new Draggable(x, y, 20);
		newTarget.enableSelection();

		for (let t of $targetLocations) {
			while (newTarget.isTooCloseTo(t.x, t.y, $difficultyStore.min_steam_turbine_separation)) {
				// This target is too close to another
				let x_dir = newTarget.x - t.x;
				let y_dir = newTarget.y - t.y;

				if (x_dir == 0) x_dir = 0.001;
				if (y_dir == 0) y_dir = 0.001;

				newTarget.x += x_dir;
				newTarget.y += y_dir;
			}
		}

		let is_too_close = false;
		let too_close_relative: Draggable = newTarget;

		while (true) {
			is_too_close = false;

			if (
				newTarget.x <= 0 ||
				newTarget.x >= viewportwidth ||
				newTarget.y <= 0 ||
				newTarget.y >= viewportheight
			) {
				newTarget.x = 20 + Math.random() * (viewportwidth - 40);
				newTarget.y = 20 + Math.random() * (viewportheight - 40);
			}

			for (let t of $targetLocations) {
				if (newTarget.isTooCloseTo(t.x, t.y, $difficultyStore.min_steam_turbine_separation)) {
					// This target is too close to another
					is_too_close = true;
					too_close_relative = t;
				}
			}

			for (let c of $craterLocations) {
				if (newTarget.isTooCloseTo(c[0], c[1], $difficultyStore.min_crater_distance)) {
					// This target is too close to a crater
					if (is_too_close == false) {
						too_close_relative = new Draggable(c[0], c[1], 20);
					}
					is_too_close = true;
				}
			}

			if (!is_too_close) break;

			let x_dir = newTarget.x - too_close_relative.x;
			let y_dir = newTarget.y - too_close_relative.y;

			if (x_dir == 0) x_dir = 0.001;
			if (y_dir == 0) y_dir = 0.001;

			newTarget.x += x_dir;
			newTarget.y += y_dir;
		}

		targetLocations.add(newTarget);
	}

	function removeTarget() {
		if (targetSelected == -1) return;
		targetLocations.remove(targetSelected);
		targetSelected = -1;
	}

	function clearTargets() {
		for (let i = $targetLocations.length - 1; i >= 0; i--) {
			targetSelected = i;
			removeTarget();
		}
	}

	function addMinimalAmountOfTurbines(foregroundWidth: number, foregroundHeight: number) {
		let added_turbines = false;

		// We'll add as many steam-turbines as necessary to reach the minimal amount
		while ($targetLocations.length < $difficultyStore.min_steam_turbines) {
			addTarget(foregroundWidth / 8, foregroundHeight / 8, foregroundWidth, foregroundHeight);
			added_turbines = true;
		}

		// If the while loop above was not able to add enough steam-turbines, we'll add one more
		if (!added_turbines) {
			addTarget(foregroundWidth / 8, foregroundHeight / 8, foregroundWidth, foregroundHeight - 100);
		}
	}
</script>

<Modal title="transformation instructions" closeButtons="top" bind:visible={instructionVisible}>
	<Instructions />
</Modal>

<Page let:foregroundHeight let:foregroundWidth>
	<div slot="background" style="background:#aaa;" />

	<div slot="headerButton"><slot /></div>

	{#if $contourLines?.curves && $difficultyStore && $targetLocations}
		<div class="sketch">
			<P5TargetPlacement
				bind:targetSelected
				{foregroundHeight}
				{foregroundWidth}
				curves={$contourLines.curves}
			/>
		</div>
	{/if}

	<svelte:fragment slot="footer">
		<Button
			secondary
			icon={mdiHelpCircle}
			on:click={() => (instructionVisible = !instructionVisible)}
		>
			Place turbine | Instructions
		</Button>

		{#if $contourLines}
			<div class="editTurbines">
				{#if $targetLocations.length > 0}
					<Button secondary icon={mdiTrashCan} on:click={clearTargets}>Clear all</Button>
				{/if}

				<Button
					icon={mdiPin}
					on:click={() => addMinimalAmountOfTurbines(foregroundWidth, foregroundHeight - 100)}
				>
					Add steam turbine
				</Button>
			</div>
		{:else}
			<NavigationButton back to="/scan/mapscanning">
				No image found. Go to map scanning
			</NavigationButton>
		{/if}
	</svelte:fragment>
</Page>

<style>
	.sketch {
		height: 100%;
		touch-action: none;
		user-select: none;
		-moz-user-select: none;
		-khtml-user-select: none;
		-webkit-user-select: none;
		-o-user-select: none;
	}

	.editTurbines {
		display: flex;
		gap: 0.5rem;
		overflow-x: auto;
	}
</style>
