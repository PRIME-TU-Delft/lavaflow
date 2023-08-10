<script lang="ts">
	import Video from '$lib/components/Video.svelte';
	import Menubar from '$lib/components/Menubar.svelte';
	import Dropdown from '$lib/components/Dropdown.svelte';
	import * as gm from 'gammacv';
	import { onDestroy } from 'svelte';

	let height: number; // window height
	let width: number; // window width

	let deviceId: string; // Id that references the current camera
	let videoSource: HTMLVideoElement;
	let tickLoop: number;
	let linesCanvasEl: HTMLCanvasElement; // Canvas element to hold the lines

	// GammaCV variables
	let gmStream: gm.CaptureVideo;
	let gmSession: gm.Session;

	const params = {
		D_COEF: 2.0,
		LAYERS: 2,
		LINE_COUNT: 10
	};

	function documentDetection(canvas: HTMLCanvasElement, frame: number = 0) {
		if (!canvas) return;

		gm.canvasClear(canvas);

		// Read the data from the stream
		let tensor = new gm.Tensor('uint8', [height, width, 4]);
		gmStream.getImageBuffer(tensor);

		// Define the image processing pipeline
		let lines = [];
		const maxP = Math.max(tensor.shape[0], tensor.shape[1]);

		// Dilation
		let pipeline = gm.dilate(tensor, [20, 20]);
		// Normalisation
		pipeline = gm.gaussianBlur(pipeline, 10, 10);
		pipeline = gm.norm(pipeline, 'l2');
		pipeline = gm.threshold(pipeline, 0.5, 0);

		// Extract the tensor output
		const visualOutputTensor = gm.tensorFrom(pipeline);

		if (!visualOutputTensor) return;

		// Create and initialize the GammaCV session, to acquire GPU acceperation
		gmSession.init(pipeline);
		gmSession.runOp(pipeline, frame, visualOutputTensor);

		// Draw this black/white result to the canvas
		gm.canvasFromTensor(canvas, visualOutputTensor);

		// Add one last operation to the pipeline
		pipeline = gm.pcLines(pipeline, params.LAYERS, 2, 2);
		const linesOutputTensor = gm.tensorFrom(pipeline);
		if (!linesOutputTensor) return;

		gmSession.init(pipeline);
		gmSession.runOp(pipeline, frame, linesOutputTensor);

		// Extract the lines

		for (let i = 0; i < linesOutputTensor.size / 4; i += 1) {
			const y = Math.floor(i / linesOutputTensor.shape[1]);
			const x = i - y * linesOutputTensor.shape[1];
			const value = linesOutputTensor.get(y, x, 0);
			const x0 = linesOutputTensor.get(y, x, 1);
			const y0 = linesOutputTensor.get(y, x, 2);

			if (value > 0.0) {
				lines.push([value, x0, y0]);
			}
		}

		// Sort the lines and keep only the best lines
		lines = lines.sort((b, a) => a[0] - b[0]);
		lines = lines.slice(0, params.LINE_COUNT);

		// Draw the lines on the canvas
		const lineContext = new gm.Line();

		for (let i = 0; i < lines.length; i += 1) {
			lineContext.fromParallelCoords(
				lines[i][1] * params.D_COEF,
				lines[i][2] * params.D_COEF,
				tensor.shape[1],
				tensor.shape[0],
				maxP,
				maxP / 2
			);

			gm.canvasDrawLine(canvas, lineContext, 'rgba(0, 255, 0, 1.0)');
		}
	}

	/** Recursive function that runs each frame */
	function tick(canvas: HTMLCanvasElement, frame: number = 0) {
		tickLoop = requestAnimationFrame(() => tick(canvas, frame + 1));

		documentDetection(canvas, frame);
	}

	function start(
		previewCanvas: HTMLCanvasElement,
		deviceId?: string,
		videoSource?: HTMLVideoElement
	) {
		if (tickLoop) return;

		// Setup the GM stream
		gmStream = new gm.CaptureVideo(width, height);
		gmStream.start(deviceId);
		// Setup the GM session
		gmSession = new gm.Session();

		const context1 = previewCanvas.getContext('2d');

		if (!context1 || !videoSource)
			return console.error('No context or video source', context1, videoSource);

		context1.drawImage(videoSource, 0, 0, width, height);
		documentDetection(previewCanvas);

		tick(previewCanvas, 0);
	}

	function setCameraId(label: string) {
		deviceId = label;

		start(linesCanvasEl, deviceId);
	}

	onDestroy(() => {
		if (tickLoop) cancelAnimationFrame(tickLoop);
	});

	$: if (videoSource) {
		console.log('loaded');
		start(linesCanvasEl, deviceId, videoSource);
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

		<button class="btn" on:click={() => start(linesCanvasEl, deviceId, videoSource)}>Tick</button>
	</Menubar>

	<!-- Lines canvas -->
	<canvas style="position: absolute; top: 0;" {width} {height} bind:this={linesCanvasEl} />
</Video>
