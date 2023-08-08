<script lang="ts">
	import Video from '$lib/components/Video.svelte';
	import * as gm from 'gammacv';

	let height: number; // window height
	let width: number; // window width

	let deviceId: string; // Id that references the current camera
	let session: gm.Session; // GammaCV session
	let stream: gm.CaptureVideo; // Video stream
	let bw_pipeline: gm.Operation;

	let tickLoop: number;

	let linesCanvas: HTMLCanvasElement; // Canvas element to hold the lines

	function getPipeline(input: gm.Tensor<gm.TensorDataView>) {
		// Normalization: add contrast, make colors seem deeper
		let pipeline = gm.norm(input, 'l2');
		// Erosion: erode into rectangles of shape 2x2 (best to see for yourself: https://gammacv.com/examples/erode)
		pipeline = gm.erode(pipeline, [2, 2]);
		// Adaptive Threshold: Black/white - make pixels black if they pass the threshold 20 within a certain box of size 10
		// (best to see for yourself: https://gammacv.com/examples/adaptive_threshold)
		pipeline = gm.adaptiveThreshold(pipeline, 10, 35);
		// Gaussian Blur: remove sharp edges
		pipeline = gm.gaussianBlur(pipeline, 3, 1);
		// Make the lines a bit thinner so the result from opencv's getContours is better
		pipeline = gm.threshold(pipeline, 0.3);

		return pipeline;
	}

	function tick(
		input: gm.Tensor<gm.TensorDataView>,
		canvas: HTMLCanvasElement,
		previewContext: CanvasRenderingContext2D,
		frame: number = 0
	) {
		tickLoop = requestAnimationFrame(() => tick(input, canvas, previewContext, frame + 1));

		//
		// Part 1: Transform to BW image
		//
		stream.getImageBuffer(input);

		// Create an output tensor
		const gammacvOutputTensor = gm.tensorFrom(bw_pipeline);

		// Guard against null of gammacvOutputTensor
		if (!gammacvOutputTensor) return;

		// Run the pipeline on this session
		session.runOp(bw_pipeline, frame, gammacvOutputTensor);

		// Fill the provided canvas with the output of this operation
		gm.canvasFromTensor(canvas, gammacvOutputTensor);

		previewContext.drawImage(canvas, 0, 0, width, height);

		//
		// Part 2: Display the lines that are extracted by GammaCV
		//
	}

	function start(previewCanvas: HTMLCanvasElement, deviceId?: string) {
		if (!session) session = new gm.Session();
		if (tickLoop) cancelAnimationFrame(tickLoop);

		stream = new gm.CaptureVideo(width, height);
		stream.start(deviceId);

		const input = new gm.Tensor('uint8', [height, width, 4]);
		bw_pipeline = getPipeline(input); // define bw pipeline
		session.init(bw_pipeline); // initialize bw pipeline

		const canvas = gm.canvasCreate(width, height);

		const previewContext = previewCanvas.getContext('2d');

		if (!previewContext) return;

		tick(input, canvas, previewContext);

		canvas.remove();
	}
</script>

<svelte:window bind:innerHeight={height} bind:innerWidth={width} />

<Video bind:deviceId let:cameraOptions let:videoSource>
	{cameraOptions.map((c) => c.label)}

	<!-- Lines canvas -->
	<canvas {width} {height} bind:this={linesCanvas} />

	<!-- Output canvas -->
	<canvas {width} {height} use:start={deviceId} />
</Video>
