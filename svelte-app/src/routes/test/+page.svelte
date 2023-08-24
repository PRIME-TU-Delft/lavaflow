<script lang="ts">
	import * as gm from 'gammacv';
	import { onMount } from 'svelte';
	import image1 from './image.png';
	import image2 from './image2.png';
	import image3 from './image3.png';
	import { imageToContoursGammaCV } from '../crop/open-cv/imageToContours';
	import { contourLines } from '$lib/stores/contourLineStore';

	let images = {
		'image1.png': image1,
		'image2.png': image2,
		'image3.png': image3
	};

	let image_name: keyof typeof images = 'image3.png';
	let outputCanvas: HTMLCanvasElement;

	function BW_pipeline(input: gm.Tensor<gm.TensorDataView>) {
		const height = input.shape[0];
		const width = input.shape[1];

		// Normalization: add contrast, make colors seem deeper
		let pipeline = gm.grayscale(input);
		pipeline = gm.erode(pipeline, [10, 10]);
		pipeline = gm.gaussianBlur(pipeline, 3, 1);
		pipeline = gm.threshold(pipeline, 0.75);

		// pipeline = sharpen(pipeline, 32);

		return pipeline;
	}

	export function preProcessTensor(
		input: gm.Tensor,
		canvas: HTMLCanvasElement,
		gmSession: gm.Session
	): gm.Tensor {
		const pipeline = BW_pipeline(input);

		const outputTensor = gm.tensorFrom(pipeline);
		if (!outputTensor) throw new Error('Could not create output tensor');

		// Create and initialize the GammaCV session, to acquire GPU acceperation
		// Then run the perspective projection operation
		gmSession.init(pipeline);

		// Draw the output on the canvas
		gmSession.runOp(pipeline, 0, outputTensor);

		return outputTensor;
	}

	function imageToContours(outputCanvas: HTMLCanvasElement) {
		console.log(outputCanvas.width);
		const contourError = imageToContoursGammaCV(outputCanvas);
		if (contourError) return console.log(contourError);

		console.log('done', $contourLines);
	}

	onMount(async () => {
		const session = new gm.Session();

		const input = await gm.imageTensorFromURL(
			images[image_name as keyof typeof images],
			'uint8',
			[1000, 1000, 4],
			true
		);

		const resultTensor = preProcessTensor(input, outputCanvas, session);

		await gm.canvasFromTensor(outputCanvas, resultTensor);

		requestAnimationFrame(() => imageToContours(outputCanvas));
	});
</script>

<canvas bind:this={outputCanvas} width="1000" height="1000" />
<img src={images[image_name]} alt="original" />
