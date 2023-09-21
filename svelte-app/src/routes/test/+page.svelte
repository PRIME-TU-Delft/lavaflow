<script lang="ts">
	import * as d3 from 'd3';
	import { onMount } from 'svelte';

	let width = 960;
	let height = 500;
	const scale = 80;
	const q = 6; // The level of detail, e.g., sample every 6 pixels in x and y.

	let svgEl: SVGSVGElement;
	let gEl: SVGGElement;

	const thresholds = Array.from({ length: 2 }, (_, i) => (i - 6) / 5);

	function createGrid() {
		const x0 = -q / 2,
			x1 = width + q;
		const y0 = -q / 2,
			y1 = height + q;

		// const n = Math.ceil((x1 - x0) / q);
		// const m = Math.ceil((y1 - y0) / q);
		const n = 43;
		const m = 43;
		const data: number[] = dataRaw.flat();

		return { data, n, m };
	}

	const grid = createGrid();

	function handleZoom(e: any) {
		d3.select('svg g').attr('transform', e.transform);
	}

	const zoomProtocol = d3
		.zoom<SVGSVGElement, unknown>()
		.scaleExtent([1 / 3, 3])
		.on('zoom', handleZoom);

	onMount(() => {
		let path = d3.geoPath();

		const contours: d3.ContourMultiPolygon[] = [];
		for (const v of thresholds) {
			let contour = d3.contours().size([grid.n, grid.m]).contour(grid.data, v);

			contour.coordinates = contour.coordinates.map((coords) => {
				return coords.map((c) => c.map(([x, y]) => [x * scale, y * scale]));
				// 	coords.map((c) => c.map(([x, y]) => [x * 1000, y * 1000]));
				// 	console.log({ coords });
				// 	return coords;
			});
			console.log(contour);

			contours.push(contour);

			// path(contour);
		}

		d3.select(svgEl).call(zoomProtocol);

		d3.select(gEl).selectAll('path').data(contours).enter().append('path').attr('d', d3.geoPath());
	});
</script>

<svg bind:this={svgEl} {width} {height}>
	<g style="stroke-width: 0.01px" bind:this={gEl} />
</svg>
