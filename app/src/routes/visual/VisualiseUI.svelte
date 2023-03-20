<script lang="ts">
	import ActionButton from '$lib/components/ActionButton.svelte';
	import ActionMenu from '$lib/components/ActionMenu.svelte';
	import Menubar from '$lib/components/Menubar.svelte';
	import { difficultyStore } from '$lib/stores/difficultyStore';
	import { gltfStore } from '$lib/stores/gltfStore';
	import { turbineLocations } from '$lib/stores/locationStore';
	import { mdiChevronRight } from '@mdi/js';
	import { Button, ButtonGroup } from 'flowbite-svelte';
	import EruptionScore from './EruptionScore.svelte';

	$: deltaTubines = $difficultyStore.min_steam_turbines - $turbineLocations.length;

	export let baseUrl: string;
	export let title: string;

	let eruptionScore: number | null = null;
	let minimiseScore = false; // score minimised takes up less space to show the 3d scene better

	let showLava = false; // Is lava gltf visible

	function startEruption() {
		showLava = true;
		minimiseScore = false;

		eruptionScore = gltfStore.computePlayerPoints(1000);
	}
</script>

<Menubar {title} back="/preview" />

<div class="h-full w-full">
	<slot {showLava} />
</div>

<ActionMenu>
	{#if deltaTubines > 0}
		<ActionButton href="{baseUrl}/turbine-placement" icon={mdiChevronRight}>
			Place {deltaTubines} more turbines
		</ActionButton>
	{:else if !eruptionScore}
		<ActionButton secondary href="{baseUrl}/turbine-placement" icon={mdiChevronRight}>
			Move turbines
		</ActionButton>

		<!-- TODO: @Rens will create svg for eruption icon -->
		<ActionButton on:click={startEruption}>Start eruption</ActionButton>
	{:else if !minimiseScore}
		<EruptionScore score={eruptionScore} on:dismiss={() => (minimiseScore = true)}>
			<Button color="red" href="/capture" outline>Rescan volcano</Button>
			<Button color="red" href="{baseUrl}/turbine-placement" outline>Move turbines</Button>
		</EruptionScore>
	{:else}
		<ButtonGroup class="mt-4 w-full">
			<ActionButton twClass="w-full" href={'/capure'}>rescan volcano</ActionButton>
			<ActionButton twClass="w-full" secondary href="{baseUrl}/turbine-placement">
				Move turbines
			</ActionButton>
		</ButtonGroup>
	{/if}
</ActionMenu>
