<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import Dropdown from '$lib/components/Dropdown.svelte';
	import * as gm from 'gammacv';
	import { onMount } from 'svelte';
	import { handleCapture, type Corners, defualtCorners } from '../../capture/handleCapture';
	import P5Overlay from './p5/P5Overlay.svelte';

	import image_url0 from './IMG_8155.jpeg';
	import image_url1 from './IMG_8156.jpeg';
	import image_url2 from './IMG_8157.jpeg';
	import image_url3 from './IMG_8158.jpeg';
	import image_url4 from './IMG_8159.jpeg';

	let images = {
		'IMG_8155.jpeg': image_url0,
		'IMG_8156.jpeg': image_url1,
		'IMG_8157.jpeg': image_url2,
		'IMG_8158.jpeg': image_url3,
		'IMG_8159.jpeg': image_url4
	};

	// Image to use as benchmark
	let image_name: keyof typeof images = 'IMG_8157.jpeg';

	let width: number; // Width of the canvas (window)
	let height: number; // Height of the canvas (window)

	let outputCanvas: HTMLCanvasElement; // Canvas to draw the output
	let gmSession: gm.Session; // GammaCV session
	let suggestedCorners: Corners; // Corners suggested by the pipeline

	let showDebugLines = true;

	/**
	 * (re-)Start the pipeline
	 */
	async function start() {
		const image_url = images[image_name as keyof typeof images] ?? image_url0;
		const input = await gm.imageTensorFromURL(image_url, 'uint8', [height, width, 4], true);

		// const defaultCorners = defualtCorners(input);

		// Draw the output to the canvas - If we have lines, draw them, otherwise draw the output tensor
		gm.canvasClear(outputCanvas);

		if (showDebugLines) {
			suggestedCorners = handleCapture(input, gmSession, outputCanvas);
		} else {
			suggestedCorners = handleCapture(input, gmSession);
		}
	}

	onMount(() => {
		if (!gmSession) gmSession = new gm.Session(); // Setup the GM session

		start();
	});

	function changeImageTo(item: string): void {
		image_name = item as keyof typeof images;
		start();
	}
</script>

<svelte:window bind:innerHeight={height} bind:innerWidth={width} />

<div class="absolute top-2 left-2 z-50 flex gap-2">
	<Dropdown title={image_name} items={Object.keys(images)} let:item let:index>
		{#if image_name != item}
			<li>
				<Button on:click={() => changeImageTo(item)}>{index} - {item}</Button>
			</li>
		{/if}
	</Dropdown>

	<label for="debugLines" class="btn">
		Show debug lines
		<input
			id="debugLines"
			class="checkbox"
			type="checkbox"
			bind:checked={showDebugLines}
			on:change={start}
		/>
	</label>
</div>

<!-- P5 Overlay -->
<div class="absolute left-0 top-0 z-20">
	{#if suggestedCorners}
		<P5Overlay bind:width bind:height {suggestedCorners} />
	{/if}
</div>

<!-- Canvas with lines -->
<canvas class="absolute left-o top-0 z-10" {width} {height} bind:this={outputCanvas} />

<!-- Preview of the benchmark image -->
<img class="absolute top-0 right-0 h-full w-full z-0" alt="original" src={images[image_name]} />
