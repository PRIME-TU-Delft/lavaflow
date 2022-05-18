<script lang="ts">
	import init, * as wasm from 'wasm';
	import { hc_level_curves, hc_parent_relations } from '$lib/data/hardCoded';
	import Button from '$lib/components/Button.svelte';
	import VisualizeGLTF from '$lib/components/VisualizeGLTF.svelte';

	let tree: wasm.OpenCVTree;
	let gltf: string;

	let loadingGLTF = false;

	function regenerateGLTF() {
		loadingGLTF = true;

		const settings = new wasm.ModelGenerationSettings(5, 50, 50, 50, 1.0);
		console.log(settings.debug());
		loadingGLTF = false;

		gltf = wasm.generate_3d_model(tree, settings, 2, 0.7, 0.7, 4, 1, 30, 30, 10);
	}

	async function initWasm() {
		await init();

		tree = new wasm.OpenCVTree({
			pixels_per_curve: hc_level_curves,
			parent_relations: hc_parent_relations
		});
	}
</script>

{#await initWasm()}
	<h1>WAITING ON WASM</h1>
{:then}
	{#if gltf}
		<h1>GLTF is loaded</h1>
		{#key gltf}
			<VisualizeGLTF {gltf} />
		{/key}
	{:else if tree}
		<Button disabled={loadingGLTF} on:click={() => regenerateGLTF()}>Regenerate GLTF</Button>
	{/if}
{/await}
