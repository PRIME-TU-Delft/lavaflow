<script lang="ts">
	import ActionButton from '$lib/components/ActionButton.svelte';
	import ActionMenu from '$lib/components/ActionMenu.svelte';
	import Menubar from '$lib/components/Menubar.svelte';
	import { difficultyStore } from '$lib/stores/difficultyStore';
	import { turbineLocations } from '$lib/stores/locationStore';
	import { mdiChevronRight, mdiHelp, mdiReload } from '@mdi/js';
	import { Icon } from 'mdi-svelte-ts';
	import type { PageData } from './$types';
	import TurbinePlacement from './TurbinePlacement.svelte';
	import Button from '$lib/components/Button.svelte';

	export let data: PageData;

	$: deltaTubines = $difficultyStore.min_steam_turbines - $turbineLocations.length;

	function resetTurbines() {
		turbineLocations.clear();
	}
</script>

<Menubar back="/visual/{data.type}" title="Turbine placement">
	<div slot="backTitle">Back to {data.type.toUpperCase()}</div>
	<Button iconPrefix={mdiReload} outline on:click={resetTurbines}>Reset turbines</Button>
</Menubar>

<TurbinePlacement />

<ActionMenu>
	<!-- Instructions and difficulty selection -->
	<div class="join">
		<ActionButton twClass="w-full" href="turbine-placement/instructions" secondary icon={mdiHelp}>
			Instructions
		</ActionButton>
	</div>

	<!-- Navigation -->
	{#if deltaTubines > 0}
		<ActionButton disabled>
			Click map to place {deltaTubines} more turbines
		</ActionButton>
	{:else}
		<ActionButton href="/visual/{data.type}" icon={mdiChevronRight}>
			Go to eruption in {data.type.toUpperCase()}
		</ActionButton>
	{/if}
</ActionMenu>
