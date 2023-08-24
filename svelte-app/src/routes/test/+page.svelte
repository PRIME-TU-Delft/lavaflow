<script lang="ts">
	import * as gm from 'gammacv';
	import { onMount } from 'svelte';
	import image1 from './image.png';
	import image2 from './image2.png';
	import image3 from './image3.png';
	import inverseKernel from './inverseKernel.glsl?raw';
	import sharpenKernel from './sharpenKernel.glsl?raw';
	import ImageTracer from 'imagetracerjs';

	function inverse(previous: gm.InputType) {
		const width = previous.shape[1];
		const height = previous.shape[0];

		return new gm.RegisterOperation('inverse')
			.Input('tSrc', 'uint8')
			.Output('uint8')
			.SetShapeFn(() => [height, width, 4])
			.LoadChunk('pickValue')
			.GLSLKernel(inverseKernel)
			.Compile({ tSrc: previous });
	}

	let images = {
		'image1.png': image1,
		'image2.png': image2,
		'image3.png': image3
	};

	function sharpen(previous: gm.InputType, factor: number) {
		const width = previous.shape[1];
		const height = previous.shape[0];

		return new gm.RegisterOperation('sharpen')
			.Input('tSrc', 'uint8')
			.Constant('SHARPEN_FACTOR', factor)
			.Output('uint8')
			.SetShapeFn(() => [height, width, 4])
			.LoadChunk('pickValue')
			.GLSLKernel(sharpenKernel)
			.Compile({ tSrc: previous });
	}

	let image_name: keyof typeof images = 'image3.png';
	let outputCanvas: HTMLCanvasElement;

	function BW_pipeline(input: gm.Tensor<gm.TensorDataView>) {
		const height = input.shape[0];
		const width = input.shape[1];

		// Normalization: add contrast, make colors seem deeper
		let pipeline = gm.grayscale(input);
		pipeline = gm.erode(pipeline, [10, 10]);

		pipeline = gm.gaussianBlur(pipeline, 3, 1);
		pipeline = gm.sobelOperator(pipeline);
		pipeline = gm.cannyEdges(pipeline, 0.25, 0.75);
		pipeline = gm.gaussianBlur(pipeline, 3, 3);
		pipeline = inverse(pipeline);
		pipeline = gm.gaussianBlur(pipeline, 3, 3);
		pipeline = gm.threshold(pipeline, 0.9, 1);

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

	onMount(async () => {
		const sessions = new gm.Session();

		const input = await gm.imageTensorFromURL(
			images[image_name as keyof typeof images],
			'uint8',
			[1000, 1000, 4],
			true
		);

		ImageTracer.imageToSVG(images[image_name as keyof typeof images], (svgstr: string) => {
			console.log(svgstr);
		});

		const resultTensor = preProcessTensor(input, outputCanvas, new gm.Session());

		gm.canvasFromTensor(outputCanvas, resultTensor);
	});
</script>

<canvas bind:this={outputCanvas} width="1000" height="1000" />
<img src={images[image_name]} alt="original" />
