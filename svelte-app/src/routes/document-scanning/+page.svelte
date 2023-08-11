<script lang="ts">
	import Video from '$lib/components/Video.svelte';
	import Menubar from '$lib/components/Menubar.svelte';
	import Dropdown from '$lib/components/Dropdown.svelte';
	import * as gm from 'gammacv';
	import { onDestroy } from 'svelte';
	import { mdiApproximatelyEqual } from '@mdi/js';

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
		DRAW_PC_LINES: true,
		D_COEF: 1.0,
		LAYERS: 2,
		LINE_COUNT: 20
	};

	function documentDetection(canvas: HTMLCanvasElement, frame: number = 0) {
		if (!canvas) return;

		gm.canvasClear(canvas);

		// Define the image processing pipeline
		// Read the data from the stream
		let input = new gm.Tensor('uint8', [height, width, 4]);
		gmStream.getImageBuffer(input);

		let lines: [number, number, number][] = [];
		const maxP = Math.max(input.shape[0], input.shape[1]);

		let pipeline = gm.downsample(input, 1, 'nearest');

		pipeline = gm.dilate(pipeline, [12, 12]);
		pipeline = gm.grayscale(pipeline);
		pipeline = gm.gaussianBlur(pipeline, 3, 3);
		pipeline = gm.sobelOperator(pipeline);
		pipeline = gm.cannyEdges(pipeline, 0.25, 0.75);

		// Dilation
		// pipeline = gm.dilate(pipeline, [12, 12]);
		// pipeline = gm.threshold(pipeline, 0.6, 0);

		// Normalisation
		// pipeline = gm.norm(pipeline, 'l2');
		// pipeline = gm.cannyEdges(pipeline, 0.25, 0.75);
		// pipeline = gm.threshold(pipeline, 0.5, 0);

		if (params.DRAW_PC_LINES) {
			pipeline = gm.pcLines(pipeline, params.LAYERS, 2, 2);
		}

		// Extract the tensor output
		const output = gm.tensorFrom(pipeline);
		if (!output) return;

		// Create and initialize the GammaCV session, to acquire GPU acceperation
		gmSession.init(pipeline);
		gmSession.runOp(pipeline, frame, output);

		pipeline.destroy();

		// Draw this black/white result to the canvas
		gm.canvasFromTensor(canvas, output);

		// Add one last operation to the pipeline
		// let pipeline = createPipeline();
		// pipeline = gm.pcLines(pipeline, params.LAYERS, 2, 2);

		// const linesOutputTensor = gm.tensorFrom(pipeline);
		// if (!linesOutputTensor) return;

		// gmSession.init(pipeline);
		// gmSession.runOp(pipeline, frame, linesOutputTensor);

		// Extract the lines
		if (params.DRAW_PC_LINES) {
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

			lines = lines.sort((b, a) => a[0] - b[0]);
			lines = lines.slice(0, params.LINE_COUNT);

			let allLineContexts: gm.Line[] = [];

			for (let i = 0; i < lines.length; i += 1) {
				const lineContext = new gm.Line();
				lineContext.fromParallelCoords(
					lines[i][1] * params.D_COEF,
					lines[i][2] * params.D_COEF,
					input.shape[1],
					input.shape[0],
					maxP,
					maxP / 2
				);

				// Gather all line contexts inside one array
				allLineContexts.push(lineContext);

				gm.canvasDrawLine(canvas, lineContext, 'rgba(0, 255, 0, 1.0)');
			}

			// Merge lines that lie closely together, based on similar angle and position
			const angleMargin = 20;
			const distanceMargin = 10;
			function linesAreSimilar(line1: gm.Line, line2: gm.Line): boolean {
				// Req1: the angles must be similar
				if (Math.abs(line1.angle - line2.angle) > angleMargin) {
					return false;
				}

				// Req2: the lines must lie close together
				if (
					Math.abs(line1.x1 - line2.x1) > distanceMargin &&
					Math.abs(line1.x2 - line2.x2) > distanceMargin &&
					Math.abs(line1.y1 - line2.y1) > distanceMargin &&
					Math.abs(line1.y2 - line2.y2) > distanceMargin
				) {
					return false;
				}

				return true;
			}

			function avgx0(toMergeNext: number[]): number {
				if (toMergeNext.length == 0) return 0;

				let sum = 0;

				for (let i of toMergeNext) {
					sum += lines[i][1] * params.D_COEF;
				}

				return sum / toMergeNext.length;
			}

			function avgy0(toMergeNext: number[]): number {
				if (toMergeNext.length == 0) return 0;

				let sum = 0;

				for (let i of toMergeNext) {
					sum += lines[i][2] * params.D_COEF;
				}

				return sum / toMergeNext.length;
			}

			function mergeLines(toMergeNext: number[]): gm.Line {
				let result = new gm.Line();

				result.fromParallelCoords(
					avgx0(toMergeNext),
					avgy0(toMergeNext),
					input.shape[1],
					input.shape[0],
					maxP,
					maxP / 2
				);

				return result;
			}

			let mergedLineContexts = [];
			let toMergeNext = [];
			let merged: number[] = [];

			for (let i = 0; i < allLineContexts.length; i++) {
				if (i in merged) continue;

				// We're going to look for lines to merge with line number i,
				// also be sure to clear toMergeNext at this point
				toMergeNext = [i];

				for (let j = 0; j < allLineContexts.length; j++) {
					if (i == j || j in merged) continue;

					// Check if these lines are similar
					if (linesAreSimilar(allLineContexts[i], allLineContexts[j])) {
						// Add j to the list of lines to merge
						toMergeNext.push(j);
					}
				}

				// Perform the merge
				mergedLineContexts.push(mergeLines(toMergeNext));

				// Use the 'merged' array to indicate that these lines have taken part in a merge
				merged.push(...toMergeNext);
			}

			// Draw the merged lines to the canvas
			for (let line of mergedLineContexts) {
				gm.canvasDrawLine(canvas, line, 'rgba(0, 0, 255, 1.0)');
			}
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
