<script lang="ts">
	import Video from '$lib/components/Video.svelte';
	import Menubar from '$lib/components/Menubar.svelte';
	import Dropdown from '$lib/components/Dropdown.svelte';
	import cv2 from 'opencv-ts';

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

	// Helper function: ones and zeros
	function ones(num: number) {
		return Array.from({ length: num }).map(() => 1);
	}

	function zeros(num: number) {
		return Array.from({ length: num }).map(() => 0);
	}

	function documentDetectionWithOpenCv(canvas: HTMLCanvasElement) {
		// Create an OpenCv matrix from the contents of the canvas
		let img = cv2.imread(canvas);

		// Step 1 - Remove details while keeping the edges
		const kernel = cv2.matFromArray(5, 5, cv2.CV_32FC1, ones(5 * 5));
		cv2.morphologyEx(
			img,
			img,
			cv2.MORPH_CLOSE,
			kernel,
			new cv2.Point(0, 0),
			3,
			cv2.BORDER_DEFAULT,
			new cv2.Scalar(0)
		);

		// Step 2 - The Magic of GrabCut
		const mask = cv2.matFromArray(width, height, cv2.CV_8U, zeros(width * height)); // mask = np.zeros(img.shape[:2],np.uint8)
		const bgdModel = cv2.matFromArray(1, 65, cv2.CV_64F, zeros(65)); // bgdModel = np.zeros((1,65),np.float64)
		const fgdModel = cv2.matFromArray(1, 65, cv2.CV_64F, zeros(65)); // fgdModel = np.zeros((1,65),np.float64)
		const rect = new cv2.Rect(20, 20, width - 40, height - 40); // rect = (20,20,img.shape[1]-20,img.shape[0]-20)
		cv2.grabCut(img, mask, rect, bgdModel, fgdModel, 5, cv2.GC_INIT_WITH_RECT); // cv2.grabCut(img,mask,rect,bgdModel,fgdModel,5,cv2.GC_INIT_WITH_RECT)

		//
		// Draw the foreground after grabCut
		//

		// For every pixel in the Mat
		for (let i = 0; i < img.rows; i++) {
			for (let j = 0; j < img.cols; j++) {
				// If the pixel value in 'mask' is either 0 or 2
				if (mask.ucharPtr(i, j)[0] == 0 || mask.ucharPtr(i, j)[0] == 2) {
					img.ucharPtr(i, j)[0] = 0;
					img.ucharPtr(i, j)[1] = 0;
					img.ucharPtr(i, j)[2] = 0;
				}
			}
		}

		// Draw the grab rectangle
		const whiteColor = new cv2.Scalar(0, 0, 255);
		const point1 = new cv2.Point(rect.x, rect.y);
		const point2 = new cv2.Point(rect.x + rect.width, rect.y + rect.height);
		cv2.rectangle(img, point1, point2, whiteColor);

		cv2.imshow(canvas, img);

		// Delete all objects
		img.delete();
		mask.delete();
		bgdModel.delete();
		fgdModel.delete();
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
		documentDetectionWithOpenCv(previewCanvas);
	}

	function setCameraId(label: string) {
		deviceId = label;

		// start(linesCanvasEl, deviceId);
	}

	$: if (videoSource) {
		console.log('loaded');
		// start(linesCanvasEl, deviceId, videoSource);
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
