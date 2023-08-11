<script lang="ts">
	import { goto } from '$app/navigation';
	import ActionMenu from '$lib/components/ActionMenu.svelte';
	import Dropdown from '$lib/components/Dropdown.svelte';
	import Menubar from '$lib/components/Menubar.svelte';
	import Video from '$lib/components/Video.svelte';
	import imageStore from '$lib/stores/imageStore';
	import sizeStore from '$lib/stores/sizeStore';
	import * as gm from 'gammacv';
	import { onDestroy, onMount } from 'svelte';
	import CaptureMenu from './CaptureMenu.svelte';
	import Instructions from '$lib/components/Instructions.svelte';
	import { drawingInstructions, scanningInstructions } from './instructions';
	import { hc_curves, hc_hierarchy } from '$lib/data/hardCoded';
	import { contourLines } from '$lib/stores/contourLineStore';

	let width: number;
	let height: number;
	let deviceId: string;

	let stream: gm.CaptureVideo;
	let canvasProcessed: HTMLCanvasElement;
	let session: gm.Session;

	let previewCanvasEl: HTMLCanvasElement;

	let loadingNextPage: boolean = false;

	const params = {
		D_COEF: 2.0,
		LAYERS: 2,
		LINE_COUNT: 10
	};

	function getPipeline(input: gm.Tensor<gm.TensorDataView>) {
		let pipeline = gm.grayscale(input);
		pipeline = gm.downsample(pipeline, params.D_COEF);
		pipeline = gm.gaussianBlur(pipeline, 3, 3);
		pipeline = gm.sobelOperator(pipeline);
		pipeline = gm.cannyEdges(pipeline, 0.25, 0.75);
		pipeline = gm.pcLines(pipeline, params.LAYERS, 2, 2);
		return pipeline;
	}

	function BW_pipeline(input: gm.Tensor<gm.TensorDataView>) {
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

	const tick = (
		input: gm.Tensor<gm.TensorDataView>,
		output: gm.Tensor<gm.TensorDataView>,
		pipeline: gm.Operation,
		frame: number = 0
	) => {
		requestAnimationFrame(() => tick(input, output, pipeline, frame + 1));
		// Read current in to the tensor
		stream.getImageBuffer(input);

		// Retrieve the BW pipeline and use the input-tensor we just retrieved
		const bw_pipeline = BW_pipeline(input);

		// Clear the canvas
		gm.canvasClear(canvasProcessed);

		// Initialise the gm session with this pipeline
		session.init(bw_pipeline);

		// finaly run operation on GPU and then write result in to output tensor
		session.runOp(bw_pipeline, frame, output);

		// Extract the tensor output
		const gammacvOutputTensor: any = gm.tensorFrom(bw_pipeline);

		// draw result into canvas
		gm.canvasFromTensor(canvasProcessed, gammacvOutputTensor);

		// Draw this output to the canvas
		// gm.canvasFromTensor(previewCanvasEl, gammacvOutputTensor);

		return;

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

		const bwPipeline = BW_pipeline(input);
		session.runOp(pipeline, 0, output);
		gm.canvasFromTensor(previewCanvasEl, output);

		// if we would like to recalculated we need a frame update
		// frame += 1;
	};

	function start(devideId?: string) {
		stream = new gm.CaptureVideo(width, height);
		stream.start(devideId);
		canvasProcessed = gm.canvasCreate(width, height);
		canvasProcessed.getContext('2d', { willReadFrequently: true });
		// allocate memeory for storing a frame and calculations output
		const input = new gm.Tensor('uint8', [height, width, 4]);
		let pipeline = getPipeline(input); // define pipeline
		session.init(pipeline); // initialize pipeline
		const output = gm.tensorFrom(pipeline); // allocate output tensor
		if (!output) return;

		tick(input, output, pipeline);

		document.body.appendChild(canvasProcessed);
		canvasProcessed.classList.add('canvasProcessed');
		// canvasProcessed.style.position = 'absolute';
		// canvasProcessed.style.top = '0';
		// canvasProcessed.style.position = 'absolute';
		// canvasProcessed.style.top = '0';
		// canvasProcessed.style.left = '0';
	}

	function setCameraId(id: string) {
		deviceId = id;
		start(id);
	}

	function continueWithDefaultMap() {
		console.log('continueWithDefaultMap');
		const { curves, hierarchy } = { curves: hc_curves, hierarchy: hc_hierarchy };
		const [hc_width, hc_height] = [1000, 800];
		contourLines.setup({
			curves,
			hierarchy,
			size: { width: hc_width, height: hc_height }
		});
		sizeStore.set({ width: hc_width, height: hc_height });
		goto('/preview');
	}

	async function gotoTransform(videoSource: HTMLVideoElement | undefined) {
		console.log('gotoTransform');
		// TODO: Do actual capture

		loadingNextPage = true;

		if (
			!videoSource ||
			!videoSource.srcObject ||
			!videoSource.videoWidth ||
			!videoSource.videoHeight
		) {
			loadingNextPage = false;
			return;
		}

		// Create an atificial canvas element
		const canvas = document.createElement('canvas') as unknown as HTMLCanvasElement;
		canvas.height = videoSource.videoHeight;
		canvas.width = videoSource.videoWidth;
		const context = canvas.getContext('2d');

		// Take screenshot of video
		if (context)
			context.drawImage(videoSource, 0, 0, videoSource.videoWidth, videoSource.videoHeight);

		const image = canvas.toDataURL('image/png');

		// Transform the image (from imageStore) into a gammacv tensor
		const gammacvInputTensor = await gm.imageTensorFromURL(image, 'uint8', [
			videoSource.videoHeight,
			videoSource.videoWidth,
			4
		]);

		// Define the image processing pipeline

		// Normalization: add contrast, make colors seem deeper
		let pipeline = gm.norm(gammacvInputTensor, 'l2');
		// Erosion: erode into rectangles of shape 2x2 (best to see for yourself: https://gammacv.com/examples/erode)
		pipeline = gm.erode(pipeline, [2, 2]);
		// Adaptive Threshold: Black/white - make pixels black if they pass the threshold 20 within a certain box of size 10
		// (best to see for yourself: https://gammacv.com/examples/adaptive_threshold)
		pipeline = gm.adaptiveThreshold(pipeline, 10, 35);
		// Gaussian Blur: remove sharp edges
		pipeline = gm.gaussianBlur(pipeline, 3, 1);
		// Make the lines a bit thinner so the result from opencv's getContours is better
		pipeline = gm.threshold(pipeline, 0.3);

		// Extract the tensor output
		const gammacvOutputTensor: any = gm.tensorFrom(pipeline);

		// Create and initialize the GammaCV session, to acquire GPU acceperation
		const gammacvSession = new gm.Session();
		gammacvSession.init(pipeline);

		// Run the canny-edges operation
		gammacvSession.runOp(pipeline, 0, gammacvOutputTensor);

		gm.canvasFromTensor(canvas, gammacvOutputTensor);

		imageStore.set(canvas.toDataURL('image/png'));

		canvas.remove();

		// Set image in (raw)image store
		sizeStore.set({ width: videoSource.videoWidth, height: videoSource.videoHeight });

		goto('/select-markers');
	}

	onMount(() => {
		session = new gm.Session();

		start();
	});

	onDestroy(() => {
		if (stream) stream.stop();
		if (canvasProcessed) canvasProcessed.remove();
	});
</script>

<svelte:window bind:innerHeight={height} bind:innerWidth={width} />

<Instructions
	title="Capture instructions"
	instructions={{
		'Drawing instructions': drawingInstructions,
		'Scanning instructions': scanningInstructions
	}}
>
	<Video bind:deviceId let:cameraOptions let:videoSource>
		<canvas bind:this={previewCanvasEl} />

		<Menubar back="./" title="Capture">
			{#if cameraOptions.length > 0}
				<Dropdown title={deviceId || 'Select other camera'}>
					{#each cameraOptions as camera}
						<li>
							<button on:click={() => setCameraId(camera.label)}>{camera.label}<button /></button>
						</li>
					{/each}
				</Dropdown>
			{/if}
		</Menubar>

		<ActionMenu>
			{#if width && height}
				<button on:click={() => start(deviceId)}>Start capture</button>
			{/if}

			<CaptureMenu loading={loadingNextPage} on:click={() => gotoTransform(videoSource)} />
		</ActionMenu>
	</Video>
</Instructions>
