<script>
	/**
	 * This is where the user inspects the level curves that are drawn, to check if the user wants to rescan the image or not
	 */
	import Button from '$lib/components/Button.svelte';
	import Page from '$lib/components/Page.svelte';
	import Image from '$lib/components/Image.svelte';
	import NavigationButton from '$lib/components/NavigationButton.svelte';

	import { perspectiveImage } from '$lib/stores/imageStore';
	import { goto } from '$app/navigation';

	function gotoVisualise() {
		// TODO: take the positions of the markers and transform image

		goto('/demo');
	}
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
		<Button on:click={gotoVisualise}>
			<span>Visualise</span>
		</Button>
	</div>
</Page>
