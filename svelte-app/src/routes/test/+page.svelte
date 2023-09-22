<script lang="ts">
	import * as d3 from 'd3';
	import { onMount } from 'svelte';
	import { data as rawData } from './contours';

	let width = 960;
	let height = 500;
	const scale = 80;
	const q = 6; // The level of detail, e.g., sample every 6 pixels in x and y.

	let svgEl: SVGSVGElement;
	let gEl: SVGGElement;

	const thresholds = Array.from({ length: 1 }, (_, i) => i / 2);
	const value = (x: number, y: number) => Math.sin(x + y) * Math.sin(x - y);

	function createGrid() {
		const x0 = -q / 2,
			x1 = width + q;
		const y0 = -q / 2,
			y1 = height + q;
		const n = Math.ceil((x1 - x0) / q);
		const m = Math.ceil((y1 - y0) / q);
		const data = new Array(n * m);
		for (let j = 0; j < m; ++j) {
			for (let i = 0; i < n; ++i) {
				data[j * n + i] = value(i * q * scale, j * q * scale);
			}
		}

		return { data, n, m };
	}

	const grid = createGrid();

	function handleZoom(e: any) {
		d3.select('svg g').attr('transform', e.transform);
	}

	const zoomProtocol = d3
		.zoom<SVGSVGElement, unknown>()
		.scaleExtent([1 / 3, 20])
		.on('zoom', handleZoom);

	let contourList: string[] = [];
	onMount(() => {
		let path = d3.geoPath();

		// const contours = d3.contours().size([grid.n, grid.m]);

		const multiPolies: d3.ContourMultiPolygon[] = [];

		type Position = [number, number];
		type D3ContourMultiPolygon = {
			value: number;
			coordinates: Position[][][];
			type: 'MultiPolygon';
		};

		const contourMultiPolygons: d3.ContourMultiPolygon[] = rawData.features.map((f) => {
			return {
				value: f.properties.max_v,
				coordinates: f.geometry.coordinates,
				type: 'MultiPolygon'
			};
		});

		for (const polyPath of contourMultiPolygons) {
			const pathString = path(polyPath);

			if (!pathString) continue;

			contourList = [...contourList, pathString];
		}

		// for (const v of thresholds) {
		// 	const pathString = path(contours.contour(grid.data, v));

		// 	if (!pathString) continue;

		// 	contourList = [...contourList, pathString];
		// }

		console.log(contourList);

		d3.select(svgEl).call(zoomProtocol);
	});
</script>

<svelte:window bind:innerHeight={height} bind:innerWidth={width} />

<svg bind:this={svgEl} {width} {height}>
	<g style="stroke-width: 0.1px; stroke: white;" bind:this={gEl}>
		{#each contourList as contour}
			<path d={contour} fill="none" stroke="white" />
		{/each}
	</g>
</svg>
