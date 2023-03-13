<script lang="ts">
	import ActionButton from '$lib/components/ActionButton.svelte';
	import ActionMenu from '$lib/components/ActionMenu.svelte';
	import Menubar from '$lib/components/Menubar.svelte';
	import { difficultyStore } from '$lib/stores/difficultyStore';
	import { gltfStore } from '$lib/stores/gltfStore';
	import { turbineLocations } from '$lib/stores/locationStore';
	import { mdiChevronRight } from '@mdi/js';
	import { Canvas } from '@threlte/core';
	import { Button, ButtonGroup } from 'flowbite-svelte';
	import EruptionScore from '../EruptionScore.svelte';
	import Scene from './Scene.svelte';

	$: deltaTubines = $difficultyStore.min_steam_turbines - $turbineLocations.length;

	let eruptionScore: number | null = null;
	let minimiseScore = false;

	function startEruption() {
		// TODO: add real eruption score
		minimiseScore = false;
		eruptionScore = Math.floor(Math.random() * 1000);
	}
</script>

<Menubar title="Visual 3d" back="/preview" />

<div class="h-full w-full">
	<Canvas>
		<Scene />
	</Canvas>
</div>

<ActionMenu>
	{#if deltaTubines > 0}
		<ActionButton href={'/3d/turbine-placement'} icon={mdiChevronRight}>
			Place {deltaTubines} more turbines
		</ActionButton>
	{:else if !eruptionScore}
		<ActionButton secondary href={'/3d/turbine-placement'} icon={mdiChevronRight}>
			Move turbines
		</ActionButton>

		<!-- TODO: @Rens will create svg for eruptio icon -->
		<ActionButton on:click={startEruption}>Start eruption</ActionButton>
	{:else if !minimiseScore}
		<EruptionScore score={eruptionScore} on:dismiss={() => (minimiseScore = true)}>
			<Button color="red" href="/capture" outline>Rescan volcano</Button>
			<Button color="red" href="/3d/turbine-placement" outline>Move turbines</Button>
		</EruptionScore>
	{:else}
		<ButtonGroup class="mt-4 w-full">
			<ActionButton twClass="w-full" href={'/capure'}>rescan volcano</ActionButton>
			<ActionButton twClass="w-full" secondary href={'/3d/turbine-placement'}>
				Move turbines
			</ActionButton>
		</ButtonGroup>
	{/if}
</ActionMenu>
