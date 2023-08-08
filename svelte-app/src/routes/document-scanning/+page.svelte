<script lang="ts">
	import Video from '$lib/components/Video.svelte';
	import Menubar from '$lib/components/Menubar.svelte';

	import * as gm from 'gammacv';
	import Dropdown from '$lib/components/Dropdown.svelte';

	import cv from 'opencv-ts';

	let height: number; // window height
	let width: number; // window width

	let deviceId: string; // Id that references the current camera
	let session: gm.Session; // GammaCV session
	let linesSession: gm.Session; // GammaCV session, specifically for the lines
	let stream: gm.CaptureVideo; // Video stream
	let bwPipeline: gm.Operation;
	let linesPipeline: gm.Operation;

	let videoSource: HTMLVideoElement;

	let tickLoop: number;

	let linesCanvasEl: HTMLCanvasElement; // Canvas element to hold the lines
	let outputCanvasEl: HTMLCanvasElement; // Canvas element to hold the output

	const params = {
		D_COEF: 2.0,
		LAYERS: 2,
		LINE_COUNT: 10
	};

	// Helper function: ones
	function ones(num: number) {
		let result = [];

		for (let i = 0; i < num; i++) {
			result.push(1);
		}

		return result;
	}

	function documentDetectionWithOpenCv(canvas: HTMLCanvasElement) {
		// Create an OpenCv matrix from the contents of the canvas
		const img = cv.imread(canvas);

		const kernel = cv.matFromArray(5, 5, cv.CV_32FC1, ones(5 * 5));
		cv.morphologyEx(
			img,
			img,
			cv.MORPH_CLOSE,
			kernel,
			new cv.Point(0, 0),
			3,
			cv.BORDER_DEFAULT,
			new cv.Scalar(0)
		);

		cv.imshow(canvas, img);
	}

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
		pipeline = gm.adaptiveThreshold(pipeline, 1);

		return pipeline;
	}

	function getLinesPipeline(input: gm.Tensor<gm.TensorDataView>) {
		const params = {
			D_COEF: 2.0,
			LAYERS: 2,
			LINE_COUNT: 10
		};

		let pipeline = gm.grayscale(input);
		pipeline = gm.downsample(pipeline, params.D_COEF);
		pipeline = gm.gaussianBlur(pipeline, 3, 3);
		pipeline = gm.sobelOperator(pipeline);
		pipeline = gm.cannyEdges(pipeline, 0.25, 0.75);
		pipeline = gm.pcLines(pipeline, params.LAYERS, 2, 2);
		return pipeline;
	}

	function drawLines(
		input: gm.Tensor<gm.TensorDataView>,
		output: gm.Tensor<gm.TensorDataView>,
		canvasProcessed: HTMLCanvasElement
	) {
		let lines = [];

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

		// Sort the lines from best to worst and pick the first LINE_COUNT lines
		lines = lines.sort((b, a) => a[0] - b[0]);
		lines = lines.slice(0, params.LINE_COUNT);

		const lineContext = new gm.Line();

		const maxP = Math.max(input.shape[0], input.shape[1]);
		for (let i = 0; i < lines.length; i += 1) {
			lineContext.fromParallelCoords(
				lines[i][1] * params.D_COEF,
				lines[i][2] * params.D_COEF,
				input.shape[1],
				input.shape[0],
				maxP,
				maxP / 2
			);

			gm.canvasDrawLine(canvasProcessed, lineContext, 'rgba(0, 255, 0, 1.0)');
		}
	}

	function tick(
		input: gm.Tensor<gm.TensorDataView>,
		canvas: HTMLCanvasElement,
		contexts: CanvasRenderingContext2D[],
		frame: number = 0
	) {
		tickLoop = requestAnimationFrame(() => tick(input, canvas, contexts, frame + 1));

		//
		// Part 1: Transform to BW image
		//
		stream.getImageBuffer(input);

		// Create an output tensor
		const gammacvOutputTensor = gm.tensorFrom(bwPipeline);

		// Guard against null of gammacvOutputTensor
		if (!gammacvOutputTensor) return;

		// Run the pipeline on this session
		session.runOp(bwPipeline, frame, gammacvOutputTensor);

		// Fill the provided canvas with the output of this operation
		gm.canvasFromTensor(canvas, gammacvOutputTensor);

		contexts[0].drawImage(canvas, 0, 0, width, height);

		//
		// Part 2: Display the lines that are extracted by GammaCV
		//

		// const gammacvLinesTensor = gm.tensorFrom(linesPipeline);

		// if (!gammacvLinesTensor) return;

		// // gm.tensorClone(gammacvOutputTensor, gammacvLinesTensor);

		// // Run the pipeline on this session
		// linesSession.runOp(linesPipeline, frame, gammacvLinesTensor);

		// // Fill the right canvas with the output of this operation
		// gm.canvasFromTensor(canvas, gammacvLinesTensor);

		// contexts[1].drawImage(canvas, 0, 0, width, height);

		// drawLines(gammacvOutputTensor, gammacvLinesTensor, linesCanvasEl);
	}

	function start(previewCanvas: HTMLCanvasElement, deviceId?: string) {
		// if (!previewCanvas) return;
		// if (!session) session = new gm.Session();
		// if (!linesSession) linesSession = new gm.Session();
		// if (tickLoop) cancelAnimationFrame(tickLoop);
		// stream = new gm.CaptureVideo(width, height);
		// stream.start(deviceId);
		// const context1 = linesCanvasEl.getContext('2d');
		// context1?.drawImage(videoSource, 0, 0, width, height);
		// const input = new gm.Tensor('uint8', [height, width, 4]);
		// bwPipeline = getDocumentDetectionPipeline(input); // define bw pipeline
		// linesPipeline = getLinesPipeline(input);
		// // Initialise the sessions
		// session.init(bwPipeline); // initialize bw pipeline
		// linesSession.init(linesPipeline);
		// const canvas = gm.canvasCreate(width, height);
		// const context0 = previewCanvas.getContext('2d');
		// const context1 = linesCanvasEl.getContext('2d');
		// if (!context0 || !context1) return;
		// tick(input, canvas, [context0, context1]);
		// canvas.remove();
		// documentDetectionWithOpenCv(linesCanvasEl);
	}

	function setCameraId(label: string) {
		deviceId = label;

		if (stream) stream.stop();

		start(outputCanvasEl, deviceId);
	}
</script>

<svelte:window bind:innerHeight={height} bind:innerWidth={width} />

<Video bind:deviceId let:cameraOptions bind:videoSource>
	<Menubar title="Document detection">
		{#if cameraOptions.length > 1}
			<Dropdown title={deviceId || 'Select other camera'}>
				{#each cameraOptions as camera}
					<li>
						<button on:click={() => setCameraId(camera.label)}>{camera.label}<button /></button>
					</li>
				{/each}
			</Dropdown>
		{/if}
	</Menubar>

	{cameraOptions.map((c) => c.label)}

	<!-- Lines canvas -->
	<canvas {width} {height} bind:this={linesCanvasEl} />

	<!-- Output canvas -->
	{#if linesCanvasEl}
		<canvas {width} {height} bind:this={outputCanvasEl} use:start={deviceId} />
	{/if}
</Video>
