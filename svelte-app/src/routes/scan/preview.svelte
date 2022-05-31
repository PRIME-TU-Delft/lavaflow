<script>
	/**
	 * This is where the user inspects the level curves that are drawn, to check if the user wants to rescan the image or not
	 */
	import Button from '$lib/components/Button.svelte';
	import Page from '$lib/components/Page.svelte';
	import Image from '$lib/components/Image.svelte';
	import NavigationButton from '$lib/components/NavigationButton.svelte';

	import { perspectiveImage } from '$lib/stores/imageStore';
	import { contourLines } from '$lib/stores/contourLineStore';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	import init, * as wasm from 'wasm';

	let gltfLoaded = false;

	function gotoVisualise() {
		goto('/demo');
	}

	onMount(async () => {
		if (!$perspectiveImage || !$contourLines.curves || !$contourLines.hierarchy) {
			return goto('/scan/mapscanning');
		}

		await init();

		const tree = new wasm.OpenCVTree({
			pixels_per_curve: $contourLines.curves,
			parent_relations: $contourLines.hierarchy
		});

		const api = new wasm.ModelConstructionApi();
		api.base(tree);

		const model_construction_result = api.build();

		console.log(model_construction_result);

		gltfLoaded = true;
	});
</script>

<Page title="image transformation">
	<NavigationButton slot="headerButton" to="/scan/maptransform" back>
		Redraw borders
	</NavigationButton>

	<div slot="background">
		<Image src={$perspectiveImage} alt="background" />
	</div>

	<Image src={$perspectiveImage} alt="foreground" />

	<div slot="footer">
		<Button loading={!gltfLoaded} on:click={gotoVisualise}>
			<span>Visualise</span>
		</Button>
	</div>
</Page>
