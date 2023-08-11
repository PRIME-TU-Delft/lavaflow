<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import Dropdown from '$lib/components/Dropdown.svelte';
	import { mdiReplay } from '@mdi/js';
	import * as gm from 'gammacv';
	import { onMount } from 'svelte';

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
	let image_name: keyof typeof images = 'IMG_8155.jpeg';

	let width: number; // Width of the canvas (window)
	let height: number; // Height of the canvas (window)

	let outputCanvas: HTMLCanvasElement; // Canvas to draw the output
	let gmSession: gm.Session; // GammaCV session

	// Hyperparameters to tweak
	const PARAMS = {
		HAS_PCLINE: true,
		MAX_LINES: 16
	};

	/**
	 * Define the pipeline
	 * @param input Input tensor
	 */
	function getPipeLine(input: gm.Tensor) {
		let pipeline = gm.dilate(input, [12, 12]);
		pipeline = gm.grayscale(pipeline);
		pipeline = gm.gaussianBlur(pipeline, 3, 3);
		pipeline = gm.sobelOperator(pipeline);
		pipeline = gm.cannyEdges(pipeline, 0.25, 0.75);

		if (PARAMS.HAS_PCLINE) {
			pipeline = gm.pcLines(pipeline);
		}
		return pipeline;
	}

	/**
	 * Draw the lines on the canvas
	 * @param input - Input tensor
	 * @param output - Output tensor
	 */
	function drawLines(input: gm.Tensor, output: gm.Tensor) {
		gm.canvasClear(outputCanvas);

		let lines = [];
		const lineContext = new gm.Line();
		const maxP = Math.max(input.shape[0], input.shape[1]);

		for (let i = 0; i < output.size / 4; i += 1) {
			const y = Math.floor(i / output.shape[1]);
			const x = i - y * output.shape[1];
			const value = output.get(y, x, 0);
			const x0 = output.get(y, x, 1);
			const y0 = output.get(y, x, 2);

			if (value > 0.0) {
				lines.push([value, x0, y0]);
			}
		}

		lines = lines.sort((b, a) => a[0] - b[0]); // Sort by relevance
		lines = lines.slice(0, PARAMS.MAX_LINES); // Pick the N most relevant lines

		for (let i = 0; i < lines.length; i += 1) {
			lineContext.fromParallelCoords(
				lines[i][1],
				lines[i][2],
				input.shape[1],
				input.shape[0],
				maxP,
				maxP / 2
			);

			gm.canvasDrawLine(outputCanvas, lineContext, 'rgba(0, 255, 0, 1.0)');
		}
	}

	/**
	 * (re-)Start the pipeline
	 */
	async function start() {
		const image_url = images[image_name as keyof typeof images] ?? image_url0;
		const input = await gm.imageTensorFromURL(image_url, 'uint8', [height, width, 4], true);

		let pipeline = getPipeLine(input); // define pipeline
		gmSession.init(pipeline); // initialize pipeline

		// allocate output tensor
		const output = gm.tensorFrom(pipeline);
		if (!output) return;

		// run pipeline
		gmSession.runOp(pipeline, 0, output);

		if (!outputCanvas) return;

		// Draw the output to the canvas - If we have lines, draw them, otherwise draw the output tensor
		if (PARAMS.HAS_PCLINE) {
			drawLines(input, output);
		} else {
			gm.canvasFromTensor(outputCanvas, output);
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

<div class="absolute top-2 left-2 z-10 flex gap-2">
	<Dropdown title={image_name} items={Object.keys(images)} let:item let:index>
		{#if image_name != item}
			<li>
				<Button on:click={() => changeImageTo(item)}>{index} - {item}</Button>
			</li>
		{/if}
	</Dropdown>

	<label for="showLines" class="btn btn-outline">
		Show lines
		<input
			class="checkbox checkbox-accent"
			id="showLines"
			type="checkbox"
			bind:checked={PARAMS.HAS_PCLINE}
			on:change={start}
		/>
	</label>

	<label for="lineCount" class="btn btn-outline">
		Max line count
		<input
			type="range"
			min="1"
			max="100"
			step="1"
			bind:value={PARAMS.MAX_LINES}
			on:change={start}
		/>
		{PARAMS.MAX_LINES}
	</label>

	{#if PARAMS.MAX_LINES != 16}
		<Button
			icon={mdiReplay}
			on:click={() => {
				PARAMS.MAX_LINES = 16;
				start();
			}}
		/>
	{/if}
</div>

<!-- Canvas with lines -->
<canvas style="position: absolute; top: 0;" {width} {height} bind:this={outputCanvas} />

<!-- Preview of the benchmark image -->
<img class="absolute top-0 right-0 h-64" alt="original" src={images[image_name]} />
