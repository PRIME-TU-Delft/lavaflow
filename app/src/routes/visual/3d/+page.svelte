<script lang="ts">
	import ActionButton from '$lib/components/ActionButton.svelte';
	import ActionMenu from '$lib/components/ActionMenu.svelte';
	import Menubar from '$lib/components/Menubar.svelte';
	import { difficultyStore } from '$lib/stores/difficultyStore';
	import { gltfStore } from '$lib/stores/gltfStore';
	import { targetLocations } from '$lib/stores/locationStore';
	import { mdiChevronRight } from '@mdi/js';
	import { Canvas } from '@threlte/core';
	import { Button } from 'flowbite-svelte';
	import EruptionScore from '../EruptionScore.svelte';
	import Scene from './Scene.svelte';

	$: deltaTubines = $difficultyStore.min_steam_turbines - $targetLocations.length;

	let eruptionScore: number | null = null;

	function startEruption() {
		// TODO: add eruption score
		eruptionScore = Math.floor(Math.random() * 1000);
	}
</script>

<Menubar title="Visual 3d" back="/preview" />

<div class="h-full w-full">
	<Canvas>
		{#if $gltfStore?.gltf_url}
			<Scene gltfModel={$gltfStore.gltf_url} />
		{/if}
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

		<ActionButton secondary on:click={startEruption}>Start eruption</ActionButton>
	{:else}
		<EruptionScore score={eruptionScore}>
			<Button href="/capture" outline red>Rescan volcano</Button>
			<Button href="/3d/turbine-placement" outline red>Place other turbines</Button>
		</EruptionScore>
	{/if}
</ActionMenu>
