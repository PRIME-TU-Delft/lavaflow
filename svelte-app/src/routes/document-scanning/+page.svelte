<script lang="ts">
	import Video from '$lib/components/Video.svelte';
	import Menubar from '$lib/components/Menubar.svelte';
	import Dropdown from '$lib/components/Dropdown.svelte';
	import cv from 'opencv-ts';

	let height: number; // window height
	let width: number; // window width

	let deviceId: string; // Id that references the current camera
	let videoSource: HTMLVideoElement;
	let tickLoop: number;
	let linesCanvasEl: HTMLCanvasElement; // Canvas element to hold the lines

	const params = {
		D_COEF: 2.0,
		LAYERS: 2,
		LINE_COUNT: 10
	};

	// Helper function: ones
	function ones(num: number) {
		return Array.from({ length: num }).map(() => 1);
	}

	function zeros(num: number) {
		return Array.from({ length: num }).map(() => 0);
	}

	function documentDetectionWithOpenCv(canvas: HTMLCanvasElement) {
		// Create an OpenCv matrix from the contents of the canvas
		let img = cv.imread(canvas);

		// Step 1 - Remove details while keeping the edges
		let step1 = new cv.Mat();
		const kernel = cv.matFromArray(5, 5, cv.CV_32FC1, ones(5 * 5));
		cv.morphologyEx(
			img,
			step1,
			cv.MORPH_CLOSE,
			kernel,
			new cv.Point(0, 0),
			3,
			cv.BORDER_DEFAULT,
			new cv.Scalar(0)
		);

		// Step 2 - The Magic of GrabCut
		const mask = cv.matFromArray(width, height, cv.CV_8U, zeros(width * height)); // mask = np.zeros(img.shape[:2],np.uint8)
		const bgdModel = cv.matFromArray(1, 65, cv.CV_64F, zeros(65)); // bgdModel = np.zeros((1,65),np.float64)
		const fgdModel = cv.matFromArray(1, 65, cv.CV_64F, zeros(65)); // fgdModel = np.zeros((1,65),np.float64)
		const rect = new cv.Rect(20, 20, height - 20, width - 20); // rect = (20,20,img.shape[1]-20,img.shape[0]-20)
		// cv.grabCut(step1, mask, rect, bgdModel, fgdModel, 5, cv.GC_INIT_WITH_RECT); // cv2.grabCut(img,mask,rect,bgdModel,fgdModel,5,cv2.GC_INIT_WITH_RECT)
		// mask2 = np.where((mask==2)|(mask==0),0,1).astype('uint8')
		// img = img*mask2[:,:,np.newaxis]

		cv.imshow(canvas, step1);
	}

	/** Recursive function that runs each frame */
	function tick(frame: number = 0) {
		tickLoop = requestAnimationFrame(() => tick(frame + 1));
	}

	function start(
		previewCanvas: HTMLCanvasElement,
		deviceId?: string,
		videoSource?: HTMLVideoElement
	) {
		const context1 = previewCanvas.getContext('2d');

		if (!context1 || !videoSource)
			return console.error('No context or video source', context1, videoSource);

		context1.drawImage(videoSource, 0, 0, width, height);
		documentDetectionWithOpenCv(linesCanvasEl);
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
	<canvas {width} {height} bind:this={linesCanvasEl} />
</Video>
