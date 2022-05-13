<script lang="ts">
	import { Temporal } from '@js-temporal/polyfill';
	import init, * as wasm from 'wasm';
	import {
		convertToObject,
		hc_level_curves,
		hc_parent_relations,
		initSettingsOptions,
		initSmoothOptions,
		type SettingsOptionNames,
		type SmoothOptionNames
	} from '$lib/data/hardCoded';
	import Button from '$lib/components/Button.svelte';
	import VisualizeGLTF from '$lib/components/VisualizeGLTF.svelte';
	import OptionsList from '$lib/components/OptionsList.svelte';

	let settings_options = initSettingsOptions();
	let smooth_options = initSmoothOptions();

	let tree: wasm.OpenCVTree;
	let gltf: string;

	let loadingGLTF = false;

	function regenerateGLTF() {
		loadingGLTF = true;

		const startTime = Temporal.Now.instant();
		const settings_mapped = convertToObject<SettingsOptionNames>(settings_options);
		const settings = new wasm.ModelGenerationSettings(
			settings_mapped.contour_margin,
			settings_mapped.columns,
			settings_mapped.rows,
			settings_mapped.altitude_step,
			settings_mapped.desired_dist
		);
		console.log(settings.debug());
		loadingGLTF = false;

		const smooth_mapped = convertToObject<SmoothOptionNames>(smooth_options);
		gltf = wasm.generate_3d_model(
			tree,
			settings,
			smooth_mapped.repetitions,
			smooth_mapped.strength_positive,
			smooth_mapped.strength_negative,
			smooth_mapped.coverage,
			smooth_mapped.svc_weight,
			30,
			30,
			10
		);
		const endTime = Temporal.Now.instant();
		alert(startTime.until(endTime).toString());
	}

	async function initWasm() {
		await init();

		tree = new wasm.OpenCVTree({
			pixels_per_curve: hc_level_curves,
			parent_relations: hc_parent_relations
		});
	}
</script>

<!-- SETTINGS -->
<OptionsList name="Settings" bind:options={settings_options} />

<!-- OPTIONS -->
<OptionsList name="Options" bind:options={smooth_options} />

{#await initWasm()}
	<h1>WAITING ON WASM</h1>
{:then}
	<h1>DEMO TIME | loading gltf: {loadingGLTF}</h1>
	{#if tree}
		<Button disabled={loadingGLTF} on:click={() => regenerateGLTF()}>Regenerate GLTF</Button>
	{/if}

	{#if gltf}
		<h1>GLTF is loaded</h1>
		<VisualizeGLTF {gltf} />
	{/if}
{/await}
