<script lang="ts">
	import ActionButton from '$lib/components/ActionButton.svelte';
	import ActionMenu from '$lib/components/ActionMenu.svelte';
	import Menubar from '$lib/components/Menubar.svelte';
	import { difficultyStore } from '$lib/stores/difficultyStore';
	import { gltfStore } from '$lib/stores/gltfStore';
	import { turbineLocations } from '$lib/stores/locationStore';
	import { mdiChevronRight } from '@mdi/js';
	import EruptionScore from './EruptionScore.svelte';

	$: deltaTubines = $difficultyStore.min_steam_turbines - $turbineLocations.length;

	export let baseUrl: string;
	export let title: string;

	let eruptionScore: number | null = null;
	let minimiseScore = false; // score minimised takes up less space to show the 3d scene better

	export let showLava = false; // Is lava gltf visible

	function startEruption() {
		showLava = true;
		minimiseScore = false;

		eruptionScore = gltfStore.computePlayerPoints(1000);
	}
</script>

<Menubar {title} back="/preview" />

<div id="overlay">
	<ActionMenu>
		<slot name="arActions" />

		{#if deltaTubines > 0}
			<ActionButton href="{baseUrl}/turbine-placement" icon={mdiChevronRight}>
				Place {deltaTubines} more turbines
			</ActionButton>
		{:else if !eruptionScore}
			<ActionButton secondary href="{baseUrl}/turbine-placement" icon={mdiChevronRight}>
				Place turbines
			</ActionButton>

			<ActionButton on:click={startEruption}>Start eruption</ActionButton>
		{:else if !minimiseScore}
			<EruptionScore score={eruptionScore} on:dismiss={() => (minimiseScore = true)}>
				<ActionButton href="/capture" secondary>Rescan volcano</ActionButton>
				<ActionButton href="{baseUrl}/turbine-placement" secondary>Place turbines</ActionButton>
			</EruptionScore>
		{:else}
			<div class="mt-4 w-full join">
				<ActionButton twClass="w-full" href={'/capure'}>Rescan volcano</ActionButton>
				<ActionButton twClass="w-full" secondary href="{baseUrl}/turbine-placement">
					Move turbines
				</ActionButton>
			</div>
		{/if}
	</ActionMenu>
</div>

<div class="h-full w-full">
	<slot {showLava} />
</div>
