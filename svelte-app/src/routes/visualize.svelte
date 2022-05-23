<script lang="ts">
	import init, * as wasm from 'wasm';
	import { hc_level_curves, hc_parent_relations } from '$lib/data/hardCoded';
	import VisualizeGLTF from '$lib/components/VisualizeGLTF.svelte';

	let gltf: string;

	async function initWasm() {
		await init();

		const tree = new wasm.OpenCVTree({
			pixels_per_curve: hc_level_curves,
			parent_relations: hc_parent_relations
		});

		const settings = new wasm.ModelGenerationSettings(5, 50, 50, 50, 1.0);
		gltf = wasm.generate_3d_model(tree, settings, 2, 0.7, 0.7, 4, 1, 30, 30, 10);
	}
</script>

{#await initWasm() then}
	{#if gltf}
		<VisualizeGLTF {gltf} />
	{/if}
{/await}
