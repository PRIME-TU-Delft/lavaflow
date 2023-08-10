<script lang="ts">
	import Video from '$lib/components/Video.svelte';
	import Menubar from '$lib/components/Menubar.svelte';
	import Dropdown from '$lib/components/Dropdown.svelte';
	import * as gm from 'gammacv';

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

	// Helper function: ones and zeros
	function ones(num: number) {
		return Array.from({ length: num }).map(() => 1);
	}

	function zeros(num: number) {
		return Array.from({ length: num }).map(() => 0);
	}

	async function documentDetection(canvas: HTMLCanvasElement, frame: number = 0) {
		// Read the data from the stream
		let tensor = new gm.Tensor('uint8', [height, width, 4]);
		gmStream.getImageBuffer(tensor);

		// Define the image processing pipeline

		// Dilation
		let pipeline = gm.dilate(tensor, [20, 20]);
		// Normalisation
		pipeline = gm.gaussianBlur(pipeline, 10, 10);
		pipeline = gm.norm(pipeline, 'l2');
		pipeline = gm.threshold(pipeline, 0.3, 0);
		// pipeline = gm.colorSegmentation(pipeline, 2);

		// Extract the tensor output
		const pipelineTensor: any = gm.tensorFrom(pipeline);

		// Create and initialize the GammaCV session, to acquire GPU acceperation
		gmSession.init(pipeline);

		// Run the canny-edges operation
		gmSession.runOp(pipeline, frame, pipelineTensor);

		gm.canvasFromTensor(canvas, pipelineTensor);
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
