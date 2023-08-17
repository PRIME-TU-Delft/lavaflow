<script lang="ts">
	import { goto } from '$app/navigation';
	import ActionButton from '$lib/components/ActionButton.svelte';
	import ActionMenu from '$lib/components/ActionMenu.svelte';
	import Button from '$lib/components/Button.svelte';
	import Instructions from '$lib/components/Instructions.svelte';
	import Menubar from '$lib/components/Menubar.svelte';
	import imageStore from '$lib/stores/imageStore';
	import { mdiHelp } from '@mdi/js';
	import * as gm from 'gammacv';
	import { Icon } from 'mdi-svelte-ts';
	import { onMount } from 'svelte';
	import { extractSelectedArea } from './extractSelectedArea';
	import { dragInstructions, posErrorInstructions } from './instructions';
	import P5Overlay from './p5/P5Overlay.svelte';

	let width: number; // Width of the window
	let height: number; // Height of the window

	let gmSession: gm.Session; // GammaCV session

	async function handleCrop() {
		// 1. Transform image with corners
		const transformedCanvas = await extractSelectedArea($imageStore, gmSession);

		transformedCanvas.classList.add('w-full', 'h-full', 'absolute', 'left-0', 'top-0', 'z-10');

		// 1.5. Show preview of transformed image
		document.body.appendChild(transformedCanvas);

		transformedCanvas.onclick = () => {
			transformedCanvas.remove();
		};

		// 2. Extract contours from image
		// 3. Save to store
		// 4. Redirect to preview page
	}

	onMount(() => {
		if (!$imageStore) goto('./capture');

		if (!gmSession) gmSession = new gm.Session(); // Setup the GM session
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
	<div class="absolute left-0 top-0 z-10 no-touch select-none">
		{#if $imageStore?.corners}
			{@const { width, height } = $imageStore.imageProportions}
			<P5Overlay {width} {height} bind:suggestedCorners={$imageStore.corners} />
		{/if}
	</div>

	{#if $imageStore?.corners}
		<img class="w-full h-full" alt="captured figure" src={$imageStore?.imageUrl} />
	{/if}

	<Menubar back="./capture" title="Crop">
		<svelte:fragment slot="backTitle">Rescan</svelte:fragment>

		<!-- TODO: add reset button -->
	</Menubar>

	<ActionMenu>
		<ActionButton fullwidth href="./capture">Rescan</ActionButton>

		<label for="my-drawer" class="drawer-button btn btn-secondary">
			<Icon path={mdiHelp} />
			Instructions
		</label>

		<Button on:click={handleCrop}>Confirm crop</Button>
	</ActionMenu>
</Instructions>
