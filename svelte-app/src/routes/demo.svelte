<script lang="ts">
	import { Temporal } from '@js-temporal/polyfill';
	import init, * as wasm from 'wasm';
	import { hc_level_curves, hc_parent_relations } from '$lib/data/hardCoded';
	import Button from '$lib/components/Button.svelte';
	import VisualizeGLTF from '$lib/components/VisualizeGLTF.svelte';

	let tree: wasm.OpenCVTree;
	let settings: wasm.ModelGenerationSettings;
	let gltf: string;

	let repetitions = 2;
	let strength_positive = 0.7;
	let strength_negative = 0.7;
	let coverage = 4;
	let svc_weight = 100;

	let rows = 30;
	let columns = 30;
	let contour_margin = 10.0;

	let loadingGLTF = false;

	const getStringSize = (s: string) => new Blob([s]).size;

	function regenerateGLTF() {
		loadingGLTF = true;

		const startTime = Temporal.Now.instant();
		gltf = wasm.generate_3d_model(
			tree,
			settings,
			repetitions,
			strength_positive,
			strength_negative,
			coverage,
			svc_weight,
			rows,
			columns,
			contour_margin
		);
		const endTime = Temporal.Now.instant();

		console.log(getStringSize(gltf));

		alert(startTime.until(endTime).toString());

		loadingGLTF = false;
	}

	async function initWasm() {
		await init();

		tree = new wasm.OpenCVTree({
			pixels_per_curve: hc_level_curves,
			parent_relations: hc_parent_relations
		});
		settings = new wasm.ModelGenerationSettings(5.0, 50, 50, 50.0, 1.0);
	}
</script>

{#await initWasm()}
	<h1>WAITING ON WASM</h1>
{:then test}
	<h1>DEMO TIME | loading gltf: {loadingGLTF}</h1>
	{#if settings && tree}
		<Button disabled={loadingGLTF} on:click={regenerateGLTF}>Regenerate GLTF</Button>
	{/if}

	{#if gltf}
		<h1>GLTF is loaded</h1>
		<VisualizeGLTF {gltf} />
	{/if}
{/await}
