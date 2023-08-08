<script lang="ts">
	import { turbineLocations, type Turbine } from '$lib/stores/locationStore';

	export let sizeMult: [number, number] = [1, 1];

	function removeTurbine(turbine: Turbine) {
		turbineLocations.remove(turbine.index);
	}

	function closeAllTurbineConfirms() {
		turbineLocations.closeAll();
	}

	function toggleDelete(turbine: Turbine) {
		closeAllTurbineConfirms();

		turbineLocations.toggleOpen(turbine.index);
	}
</script>

{#each $turbineLocations || [] as turbine, index}
	<button
		on:click|stopPropagation={() => toggleDelete(turbine)}
		on:keydown={() => toggleDelete(turbine)}
		class="turbine"
		class:top={turbine.hasConfirmOpen}
		style="--pos-x: {turbine.x * sizeMult[0]}px;
								 --pos-y: {turbine.y * sizeMult[1]}px;"
	>
		Turbine {index + 1}

		{#if turbine.hasConfirmOpen}
			<p class="mb-1 text-sm text-slate-400">Delete this turbine?</p>
			<div class="flex w-full gap-2 text-slate-900">
				<button class=" bg-red-200 p-2" on:click|stopPropagation={() => removeTurbine(turbine)}>
					Yes, delete
				</button>
				<button on:click|stopPropagation={closeAllTurbineConfirms} class="bg-white p-2">
					Cancel
				</button>
			</div>
		{/if}
	</button>
{/each}

<style lang="postcss">
	.turbine {
		@apply absolute -translate-x-1/2 -translate-y-1/2 cursor-pointer select-none rounded-sm bg-slate-900 p-2 text-slate-100;
		left: var(--pos-x);
		top: var(--pos-y);
	}

	.turbine:before {
		@apply absolute left-1/2 top-1/2 -z-10 -translate-y-1/2 -translate-x-1/2 cursor-not-allowed rounded-full bg-slate-900/30;
		content: '';
		width: 15vmin;
		height: 15vmin;
	}

	.turbine:hover {
		@apply scale-105 rounded-md bg-slate-700;
	}

	.turbine.top {
		@apply z-10;
	}
</style>
