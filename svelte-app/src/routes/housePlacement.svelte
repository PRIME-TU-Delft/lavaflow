<script lang="ts">
	/**
	 * This view is where the user will place houses on the map.
	 */
	import Page from '$lib/components/Page.svelte';
	import NavigationButton from '$lib/components/NavigationButton.svelte';
	import Image from '$lib/components/Image.svelte';

	import { perspectiveImage, demoPerspectiveImage } from '$lib/stores/imageStore';

	import { onMount } from 'svelte';

	onMount(() => {
		if (!$perspectiveImage) perspectiveImage.set(demoPerspectiveImage);
	});
</script>

<Page title="image transformation">
	<!-- TODO: Member the houses before routing -->
	<NavigationButton slot="headerButton" to="/demo" back>Back to visualise</NavigationButton>

	<div slot="background">
		<Image src={$perspectiveImage} alt="background" />
	</div>

	{#if $perspectiveImage}
		<Image src={$perspectiveImage} alt="foreground" />
	{:else}
		<NavigationButton to="/scan/mapscanning">No image found. Go to map scanning</NavigationButton>
	{/if}
</Page>
