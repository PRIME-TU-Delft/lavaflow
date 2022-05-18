<script>
	/**
	 * This is where the user inspects the level curves that are drawn, to check if the user wants to rescan the image or not
	 */
	import Button from '$lib/components/Button.svelte';
	import Page from '$lib/components/Page.svelte';
	import Image from '$lib/components/Image.svelte';
	import NavigationButton from '$lib/components/NavigationButton.svelte';

	import { rawImage } from '$lib/stores/imageStore';
	import { goto } from '$app/navigation';

	function gotoPreview() {
		// TODO: take the positions of the markers and transform image

		goto('/scan/preview');
	}
</script>

<Page title="image transformation">
	<NavigationButton slot="headerButton" to="/scan/mapscanning" back>Rescan image</NavigationButton>

	<div slot="background">
		<Image src={$rawImage} alt="background" />
	</div>

	<Image src={$rawImage} alt="foreground" />

	<div slot="footer">
		<Button on:click={gotoPreview}>
			<span>Proceed</span>
		</Button>
	</div>
</Page>
