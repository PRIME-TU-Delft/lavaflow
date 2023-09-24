<script lang="ts">
	import * as d3 from 'd3';
	import { onMount } from 'svelte';
	import { data as rawData } from './contours';

	let width = 960;
	let height = 500;

	let svgEl: SVGSVGElement;
	let pathString: string | null;

	function handleZoom(e: any) {
		d3.select('svg g').attr('transform', e.transform);
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

		let testCoordinates = lastPolygon.geometry.coordinates
			.filter((cs) => {
				// Coordinates of length 1 are not do not have a inner and outer polygon
				// Coordinates of length 2 are the outer and inner polygon
				// Coordinates of length > 2 are holes in the polygon
				// return cs.length == 1;
				return cs.length == 2;
				// return cs.length > 2;
			})
			.map((cs) => [cs.at(0)])
			.map((cs) => cs.map((c) => c?.map((cc) => [cc[0] * 5, cc[1] * 5]) || []));

		let contourMultiPolygons: d3.ContourMultiPolygon = {
			value: lastPolygon.properties.max_v,
			coordinates: testCoordinates,
			type: 'MultiPolygon'
		};

		pathString = path(contourMultiPolygons);

		if (!pathString) return; // Todo handle error - No path string found

		d3.select(svgEl).call(zoomProtocol);
	});
</script>

<svelte:window bind:innerHeight={height} bind:innerWidth={width} />

<svg bind:this={svgEl} {width} {height}>
	<g style="stroke-width: 0.1px; stroke: white;">
		<path d={pathString} fill="none" stroke="white" stroke-width="2px" />
	</g>
</svg>
