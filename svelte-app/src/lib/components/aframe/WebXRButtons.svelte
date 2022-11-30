<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import Icon from '$lib/components/Icon.svelte';

	import { targetLocations } from '$lib/stores/locationStore';
	import { gltfStore } from '$lib/stores/gltfStore';

	import { mdiLavaLamp } from '@mdi/js';
	import { goto } from '$app/navigation';
	import type { Scene } from 'aframe';

	export let sceneEl: Scene;
	export let lavaVisible = false;
	export let points = 0;

	const MAX_POINTS = 1000;
	async function closeNativeAr() {
		await sceneEl.exitVR();
	}

	async function gotoPlaceTargets() {
		await closeNativeAr();
		goto('/targetplacement/ar');
	}

	async function gotoPreview() {
		await closeNativeAr();
		goto('/scan/preview');
	}

	function revealLava() {
		// Compute the amount of points the user obtained
		// The rust api exports a function that can be used to efficiently compute the player's points.
		// Use this api-call to compute the points
		points = gltfStore.computePlayerPoints(MAX_POINTS);

		lavaVisible = true;
	}
</script>

<div class="backButton">
	<Button on:click={gotoPreview}>Back to preview</Button>
</div>

<div class="button rightButton">
	<Button on:click={gotoPlaceTargets}>Place targets</Button>

	{#if $targetLocations.length > 0}
		{#if lavaVisible}
			<Button secondary on:click={() => (lavaVisible = false)}>Hide lava</Button>
			<Button green>Energy: {points} / {MAX_POINTS} MW</Button>
		{:else}
			<Button on:click={revealLava}>
				<span>Reveal lava</span>
				<Icon path={mdiLavaLamp} />
			</Button>
		{/if}
	{:else}
		<Button disabled secondary>Place targets to begin</Button>
	{/if}
</div>

<style>
	.rightButton {
		width: fit-content;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}
</style>