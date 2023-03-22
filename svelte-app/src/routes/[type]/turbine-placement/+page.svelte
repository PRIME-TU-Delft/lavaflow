<script lang="ts">
	import ActionButton from '$lib/components/ActionButton.svelte';
	import ActionMenu from '$lib/components/ActionMenu.svelte';
	import Menubar from '$lib/components/Menubar.svelte';
	import { difficultyStore } from '$lib/stores/difficultyStore';
	import { turbineLocations } from '$lib/stores/locationStore';
	import { mdiChevronRight, mdiHelp, mdiReload, mdiStairsUp } from '@mdi/js';
	import { Button, ButtonGroup } from 'flowbite-svelte';
	import { Icon } from 'mdi-svelte-ts';
	import type { PageData } from './$types';
	import TurbinePlacement from './TurbinePlacement.svelte';

	export let data: PageData;

	$: deltaTubines = $difficultyStore.min_steam_turbines - $turbineLocations.length;

	function resetTurbines() {
		turbineLocations.clear();
	}
</script>

<Menubar back="/visual/{data.type}" title="Turbine placement">
	<div slot="backTitle">Back to {data.type}</div>
	<Button class="flex items-center gap-2" outline color="red" on:click={resetTurbines}>
		<Icon path={mdiReload} />
		Reset turbines
	</Button>
</Menubar>

<TurbinePlacement />

<ActionMenu>
	<!-- Instructions and difficulty selection -->
	<ButtonGroup>
		<ActionButton twClass="w-full" href="turbine-placement/instructions" secondary icon={mdiHelp}>
			Instructions
		</ActionButton>
	</ButtonGroup>

	<!-- Navigation -->
	{#if deltaTubines > 0}
		<ActionButton disabled>
			Click map to place {deltaTubines} more turbines
		</ActionButton>
	{:else}
		<ActionButton href="/visual/{data.type}" icon={mdiChevronRight}>
			Start eruption in {data.type}
		</ActionButton>
	{/if}
</ActionMenu>
