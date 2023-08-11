<script lang="ts">
	import Instructions from '$lib/components/Instructions.svelte';
	import Video from '$lib/components/Video.svelte';
	import Menubar from '$lib/components/Menubar.svelte';
	import Dropdown from '$lib/components/Dropdown.svelte';
	import ActionMenu from '$lib/components/ActionMenu.svelte';
	import CaptureMenu from './CaptureMenu.svelte';
	import { drawingInstructions, scanningInstructions } from './instructions';
	import { onMount } from 'svelte';
	import * as gm from 'gammacv';
	import { PARAMS, calculateIntersectionPoints } from './line_intersections';

	let deviceId: string; // camera id
	let width: number;
	let height: number;
	let loadingNextPage = false;

	let tickLoop: number;
	let gmSession: gm.Session;
	let gmStream: gm.CaptureVideo;

	let outputCanvas: HTMLCanvasElement;

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
	 * Tick Method
	 * @param input
	 * @param output
	 * @param pipeline
	 * @param frame
	 */
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

		// Calculate the intersection points
		// if (PARAMS.HAS_PCLINE) {
		// 	const intersectionPoints = calculateIntersectionPoints(input, output);
		// }

		tickLoop = requestAnimationFrame(() => tick(input, output, pipeline, frame + 1));
	}

	/**
	 * Start method
	 * @param dId - device id (camera id) determines what camera to use
	 */
	function start(dId?: string) {
		// If the start method was called before, start a new one
		if (tickLoop) cancelAnimationFrame(tickLoop);

		gmStream.start(dId);

		// allocate memeory for storing a frame and calculations output
		const input = new gm.Tensor('uint8', [height, width, 4]);

		let pipeline = getPipeLine(input); // define pipeline
		gmSession.init(pipeline); // initialize pipeline

		// allocate output tensor
		const output = gm.tensorFrom(pipeline);
		if (!output) return;

		// tick(input, output, pipeline);
	}

	/**
	 * Change the camera id
	 */
	function setCameraId(label: any): any {
		throw new Error('Function not implemented.');
	}

	/**
	 * Handler: capture button
	 */
	async function handleCapture() {
		// allocate memeory for storing a frame and calculations output
		let input: gm.Tensor<gm.TensorDataView> = new gm.Tensor('uint8', [height, width, 4]);

		gmStream.getImageBuffer(input);

		let canvasContext = outputCanvas.getContext('2d');
		if (!canvasContext) return;

		gmStream.getImageBufferTo(input, canvasContext, width, height);

		input = await gm.imageTensorFromURL(outputCanvas.toDataURL('image/png'), 'uint8');

		let pipeline = getPipeLine(input); // define pipeline
		gmSession.init(pipeline); // initialize pipeline

		// allocate output tensor
		const output = gm.tensorFrom(pipeline);
		if (!output) return;
		gmSession.runOp(pipeline, 0, output);

		// Calculate the intersection points
		const intersectionPoints = calculateIntersectionPoints(input, output);

		console.log(intersectionPoints);
	}

	//
	// LIFETIME HANDLERS
	//

	/**
	 * OnMount
	 */
	onMount(() => {
		if (tickLoop) cancelAnimationFrame(tickLoop);

		// Setup the GM session
		if (!gmSession) gmSession = new gm.Session();

		gmStream = new gm.CaptureVideo(width, height);
		start();
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
		<canvas style="position: absolute; top: 0;" {width} {height} bind:this={outputCanvas} />

		<Menubar back="./" title="Capture">
			<!-- If more than one camera are available, display a dropdown to allow the user to choose -->
			{#if cameraOptions.length > 1}
				<Dropdown items={cameraOptions} title={deviceId || 'Select other camera'} let:item={camera}>
					<li>
						<button on:click={() => setCameraId(camera.label)}>{camera.label}<button /></button>
					</li>
				</Dropdown>
			{/if}
		</Menubar>

		<ActionMenu>
			<CaptureMenu loading={loadingNextPage} on:click={handleCapture} />
		</ActionMenu>
	</Video>
</Instructions>
