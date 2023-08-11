<script lang="ts">
	import * as gm from 'gammacv';
	import { onMount } from 'svelte';

	let width: number;
	let height: number;
	let tickLoop: number;

	let outputCanvas: HTMLCanvasElement;

	let gmStream: gm.CaptureVideo;
	let gmSession: gm.Session;

	function getPipeLine(input: gm.Tensor) {
		let pipeline = gm.grayscale(input);
		// pipeline = gm.downsample(pipeline, 2);
		pipeline = gm.gaussianBlur(pipeline, 4, 4);
		pipeline = gm.cannyEdges(pipeline, 0.25, 0.75);
		// pipeline = gm.pcLines(pipeline, 2, 2, 2);
		return pipeline;
	}

	function tick(
		input: gm.Tensor<gm.TensorDataView>,
		output: gm.Tensor<gm.TensorDataView>,
		pipeline: gm.Operation,
		frame = 0
	) {
		gmStream.getImageBuffer(input);
		gmSession.runOp(pipeline, frame, output);

		if (outputCanvas) {
			gm.canvasFromTensor(outputCanvas, output);
		}
		tickLoop = requestAnimationFrame(() => tick(input, output, pipeline, frame + 1));
	}

	function start() {
		gmStream.start();

		// allocate memeory for storing a frame and calculations output
		const input = new gm.Tensor('uint8', [height, width, 4]);

		let pipeline = getPipeLine(input); // define pipeline
		gmSession.init(pipeline); // initialize pipeline

		// allocate output tensor
		const output = gm.tensorFrom(pipeline);
		if (!output) return;

		tick(input, output, pipeline);
	}

	onMount(() => {
		if (tickLoop) cancelAnimationFrame(tickLoop);

		// Setup the GM session
		if (!gmSession) gmSession = new gm.Session();

		gmStream = new gm.CaptureVideo(width, height);
	});

	$: outputCanvas && width && height && start();
</script>

<svelte:window bind:innerHeight={height} bind:innerWidth={width} />

<canvas style="position: absolute; top: 0;" {width} {height} bind:this={outputCanvas} />
