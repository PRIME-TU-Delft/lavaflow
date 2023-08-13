<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import Dropdown from '$lib/components/Dropdown.svelte';
	import { mdiReplay } from '@mdi/js';
	import * as gm from 'gammacv';
	import { onMount } from 'svelte';

	import image_url0 from './IMG_8155.jpeg';
	import image_url1 from './IMG_8156.jpeg';
	import image_url2 from './IMG_8157.jpeg';
	import image_url3 from './IMG_8158.jpeg';
	import image_url4 from './IMG_8159.jpeg';

	let images = {
		'IMG_8155.jpeg': image_url0,
		'IMG_8156.jpeg': image_url1,
		'IMG_8157.jpeg': image_url2,
		'IMG_8158.jpeg': image_url3,
		'IMG_8159.jpeg': image_url4
	};

	// Image to use as benchmark
	let image_name: keyof typeof images = 'IMG_8157.jpeg';

	let width: number; // Width of the canvas (window)
	let height: number; // Height of the canvas (window)

	let outputCanvas: HTMLCanvasElement; // Canvas to draw the output
	let gmSession: gm.Session; // GammaCV session

	// Hyperparameters to tweak
	const PARAMS = {
		HAS_PCLINE: true,
		MAX_LINES: 16
	};

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
	 * Draw the lines on the canvas
	 * @param input - Input tensor
	 * @param output - Output tensor
	 */
	function drawLines(input: gm.Tensor, output: gm.Tensor) {
		gm.canvasClear(outputCanvas);

		let lines: [number, number, number][] = [];
		const lineContext = new gm.Line();
		const maxP = Math.max(input.shape[0], input.shape[1]);

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

		lines = lines.sort((b, a) => a[0] - b[0]); // Sort by relevance
		lines = lines.slice(0, PARAMS.MAX_LINES); // Pick the N most relevant lines

		let allLineContexts: gm.Line[] = [];

		for (let i = 0; i < lines.length; i += 1) {
			const lineContext = new gm.Line();
			lineContext.fromParallelCoords(
				lines[i][1],
				lines[i][2],
				input.shape[1],
				input.shape[0],
				maxP,
				maxP / 2
			);

			// Gather all line contexts inside one array
			allLineContexts.push(lineContext);

			gm.canvasDrawLine(outputCanvas, lineContext, 'rgba(0, 255, 0, 1.0)');
		}

		//
		// APPROACH: MERGING INTERSECTIONS
		//

		let points: [number, number, number, boolean][] = [];

		for (let i = 0; i < allLineContexts.length; i++) {
			for (let j = i + 1; j < allLineContexts.length; j++) {
				const intersection = gm.Line.Intersection(allLineContexts[i], allLineContexts[j]);
				let dAngle = Math.abs(allLineContexts[i].angle - allLineContexts[j].angle);
				if (dAngle > 90) {
					dAngle = 180 - dAngle;
				}

				if (
					intersection &&
					dAngle > 20 &&
					intersection[0] >= 0 &&
					intersection[1] >= 0 &&
					intersection[0] <= input.shape[1] &&
					intersection[1] <= input.shape[0]
				) {
					gm.canvasDrawCircle(outputCanvas, intersection, 2, 'rgba(255, 0, 0, 1.0)');
					points.push([intersection[0], intersection[1], 1, true]);
				}
			}
		}

		// Cluster the points
		let pointsNextGen: [number, number, number, boolean][] = [];
		let numberOfMerges = 0;
		const maxRange = 100;

		do {
			pointsNextGen = [];
			numberOfMerges = 0;

			for (let i = 0; i < points.length; i++) {
				if (points[i][3] == false) continue;

				for (let j = 0; j < points.length; j++) {
					if (i == j) continue;

					// If these points lie closely enough together and are not part of another cluster thusfar
					// Merge them and add them to the new generation
					if (
						Math.abs(points[i][0] - points[j][0]) <= maxRange &&
						Math.abs(points[i][1] - points[j][1]) <= maxRange
					) {
						const pointCount = points[i][2] + points[j][2];
						const avgx = (points[i][0] * points[i][2] + points[j][0] * points[j][2]) / pointCount;
						const avgy = (points[i][1] * points[i][2] + points[j][1] * points[j][2]) / pointCount;
						pointsNextGen.push([avgx, avgy, pointCount, true]);

						points[i][3] = false;
						points[j][3] = false;

						numberOfMerges++;
					}
				}
			}

			for (let i = 0; i < points.length; i++) {
				if (points[i][3]) {
					pointsNextGen.push(points[i]);
				}
			}

			points = pointsNextGen;
		} while (numberOfMerges > 0);

		// Choose the four points with the largest base
		let intersections: [number, number][] = points
			.sort((b, a) => a[2] - b[2])
			// .slice(0, 4)
			.map((a) => a.slice(0, 2) as [number, number]);

		for (let i = 0; i < intersections.length; i++) {
			gm.canvasDrawCircle(
				outputCanvas,
				[intersections[i][0], intersections[i][1]],
				10,
				'rgba(0, 0, 255, 1.0)'
			);
		}

		//
		// PLACING THE RIGHT POINT IN THE RIGHT CORNER
		//

		let defaultBorderOffsetX = input.shape[1] / 4;
		let defaultBorderOffsetY = input.shape[0] / 4;

		let defaultTopLeft: [number, number] = [defaultBorderOffsetX, defaultBorderOffsetY];
		let defaultTopRight: [number, number] = [
			input.shape[1] - defaultBorderOffsetX,
			defaultBorderOffsetY
		];
		let defaultBottomLeft: [number, number] = [
			defaultBorderOffsetX,
			input.shape[0] - defaultBorderOffsetY
		];
		let defaultBottomRight: [number, number] = [
			input.shape[1] - defaultBorderOffsetX,
			input.shape[0] - defaultBorderOffsetY
		];

		let defaults: [number, number][] = [
			defaultTopLeft,
			defaultTopRight,
			defaultBottomLeft,
			defaultBottomRight
		];

		function minimize(
			prop: (x: number, y: number) => number,
			pts: [number, number][]
		): [number, number] {
			let min = prop(pts[0][0], pts[0][1]);
			let minArg = pts[0];

			for (let p of pts) {
				if (prop(p[0], p[1]) < min) {
					min = prop(p[0], p[1]);
					minArg = p;
				}
			}

			return minArg;
		}

		// Top left is the point with smallest x and y
		let topLeft = minimize(
			(x: number, y: number) => {
				return x * y;
			},
			[...intersections, ...defaults]
		);

		// Top right is the point with largest x and smallest y
		let topRight = minimize(
			(x: number, y: number) => {
				return (1 / x) * y;
			},
			[...intersections, ...defaults].filter((a) => a != topLeft)
		);

		// Bottom left is the point with smallest x and largest y
		let bottomLeft = minimize(
			(x: number, y: number) => {
				return (x * 1) / y;
			},
			[...intersections, ...defaults].filter((a) => a != topLeft && a != topRight)
		);

		// Bottom right is the point with largest x and largest y
		let bottomRight = minimize(
			(x: number, y: number) => {
				return ((1 / x) * 1) / y;
			},
			[...intersections, ...defaults].filter(
				(a) => a != topLeft && a != topRight && a != bottomLeft
			)
		);

		gm.canvasDrawCircle(outputCanvas, topLeft, 12, 'rgba(255, 0, 255, 1.0)');
		gm.canvasDrawCircle(outputCanvas, topRight, 12, 'rgba(255, 0, 255, 1.0)');
		gm.canvasDrawCircle(outputCanvas, bottomLeft, 12, 'rgba(255, 0, 255, 1.0)');
		gm.canvasDrawCircle(outputCanvas, bottomRight, 12, 'rgba(255, 0, 255, 1.0)');
	}

	/**
	 * (re-)Start the pipeline
	 */
	async function start() {
		console.log('hoi!');
		const image_url = images[image_name as keyof typeof images] ?? image_url0;
		const input = await gm.imageTensorFromURL(image_url, 'uint8', [height, width, 4], true);

		let pipeline = getPipeLine(input); // define pipeline
		gmSession.init(pipeline); // initialize pipeline

		// allocate output tensor
		const output = gm.tensorFrom(pipeline);
		if (!output) return;

		// run pipeline
		gmSession.runOp(pipeline, 0, output);

		if (!outputCanvas) return;

		// Draw the output to the canvas - If we have lines, draw them, otherwise draw the output tensor
		if (PARAMS.HAS_PCLINE) {
			drawLines(input, output);
		} else {
			gm.canvasFromTensor(outputCanvas, output);
		}
	}

	onMount(() => {
		if (!gmSession) gmSession = new gm.Session(); // Setup the GM session

		start();
	});

	function changeImageTo(item: string): void {
		image_name = item as keyof typeof images;
		start();
	}
</script>

<svelte:window bind:innerHeight={height} bind:innerWidth={width} />

<div class="absolute top-2 left-2 z-10 flex gap-2">
	<Dropdown title={image_name} items={Object.keys(images)} let:item let:index>
		{#if image_name != item}
			<li>
				<Button on:click={() => changeImageTo(item)}>{index} - {item}</Button>
			</li>
		{/if}
	</Dropdown>

	<label for="showLines" class="btn btn-outline">
		Show lines
		<input
			class="checkbox checkbox-accent"
			id="showLines"
			type="checkbox"
			bind:checked={PARAMS.HAS_PCLINE}
			on:change={start}
		/>
	</label>

	<label for="lineCount" class="btn btn-outline">
		Max line count
		<input
			type="range"
			min="1"
			max="100"
			step="1"
			bind:value={PARAMS.MAX_LINES}
			on:change={start}
		/>
		{PARAMS.MAX_LINES}
	</label>

	{#if PARAMS.MAX_LINES != 16}
		<Button
			icon={mdiReplay}
			on:click={() => {
				PARAMS.MAX_LINES = 16;
				start();
			}}
		/>
	{/if}
</div>

<!-- Canvas with lines -->
<canvas style="position: absolute; top: 0;" {width} {height} bind:this={outputCanvas} />

<!-- Preview of the benchmark image -->
<!-- <img class="absolute top-0 right-0 h-64" alt="original" src={images[image_name]} /> -->
