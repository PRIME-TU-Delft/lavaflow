<script lang="ts">
	import * as d3 from 'd3';
	import { onMount } from 'svelte';
	import { data as rawData } from './contours';

	let width = 960;
	let height = 500;

	let svgEl: SVGSVGElement;
	let pathString: string | null;
	let debug: Object;

	function handleZoom(e: any) {
		d3.select('svg g').attr('transform', e.transform);
	}

	function determineHierarchy(contours: d3.ContourMultiPolygon['coordinates']) {
		debug = contours.map((c) => c[0].length);

		const biggest = contours[2][0].map((p) => p as [number, number]);
		const secondBiggest = contours[3][0].map((p) => p as [number, number]);

		debug = d3.polygonContains(biggest, secondBiggest[0]);
	}

	const zoomProtocol = d3
		.zoom<SVGSVGElement, unknown>()
		.scaleExtent([1 / 3, 20])
		.on('zoom', handleZoom);

	onMount(() => {
		let path = d3.geoPath();

		const lastPolygon = rawData.features.at(-1);

		// Todo handle error - No polygons found in dataset
		// @do - lower threshold
		if (!lastPolygon) return;

		let testCoordinates: number[][][][] = lastPolygon.geometry.coordinates
			.filter((cs) => {
				// Coordinates of length 1 are not do not have a inner and outer polygon
				// Coordinates of length 2 are the outer and inner polygon
				// Coordinates of length > 2 are holes in the polygon
				// return true;
				// return cs.length == 1;
				return cs.length == 2;
				// return cs.length > 2;
			})
			//.map((cs) => [cs.at(0)]) // Select outer polygon
			.map((cs) => cs.filter((c) => c && c.length > 10)) // Filter out (too) small polygons
			.map((cs) => cs.map((c) => c?.map((cc) => [cc[0] * 5, cc[1] * 5]) || []));

		let contourMultiPolygons: d3.ContourMultiPolygon = {
			value: lastPolygon.properties.max_v,
			coordinates: testCoordinates,
			type: 'MultiPolygon'
		};

		pathString = path(contourMultiPolygons);

		determineHierarchy(testCoordinates);

		d3.select(svgEl).call(zoomProtocol);
	});
</script>

<svelte:window bind:innerHeight={height} bind:innerWidth={width} />

<h1>Debug</h1>
<pre>{JSON.stringify(debug, null, 2)}</pre>

<svg bind:this={svgEl} {width} {height}>
	<g style="stroke-width: 0.1px; stroke: white;">
		<path d={pathString} fill="none" stroke="black" stroke-width="2px" />
	</g>
</svg>
