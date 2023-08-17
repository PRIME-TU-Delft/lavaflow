<script lang="ts">
	import Menubar from '$lib/components/Menubar.svelte';
	import ActionMenu from '$lib/components/ActionMenu.svelte';
	import { onMount } from 'svelte';
	import imageStore from '$lib/stores/imageStore';
	import P5Overlay from './p5/P5Overlay.svelte';
	import { goto } from '$app/navigation';
	import Button from '$lib/components/Button.svelte';
	import Instructions from '$lib/components/Instructions.svelte';
	import { dragInstructions, posErrorInstructions } from './instructions';
	import { Icon } from 'mdi-svelte-ts';
	import { mdiHelp } from '@mdi/js';

	let width: number;
	let height: number;

	function handleCrop() {
		// 1. Transform image with corners
		// 2. Extract contours from image
		// 3. Save to store
		// 4. Redirect to preview page
	}

	onMount(() => {
		if (!$imageStore) {
			goto('./capture');
		}
	});
</script>

<svelte:window bind:innerHeight={height} bind:innerWidth={width} />

<Instructions
	title="Crop instructions"
	instructions={{
		'Drag instructions': dragInstructions,
		'Scanning instructions': posErrorInstructions
	}}
>
	<!-- P5 Overlay -->
	<div class="absolute left-0 top-0 z-10 no-touch">
		{#if $imageStore?.corners}
			{@const { width, height } = $imageStore.imageProportions}
			<P5Overlay {width} {height} suggestedCorners={$imageStore.corners} />
		{/if}
	</div>

	{#if $imageStore?.corners}
		<img class="w-full h-full" alt="captured figure" src={$imageStore?.imageUrl} />
	{/if}

	<Menubar back="./capture" title="Crop" />

	<ActionMenu>
		<label for="my-drawer" class="drawer-button btn btn-secondary">
			<Icon path={mdiHelp} />
			Instructions
		</label>

		<Button on:click={handleCrop}>Confirm crop</Button>
	</ActionMenu>
</Instructions>
