<script lang="ts">
	/**
	 * Stylistic component for an image.
	 */

	export let src: string; // Image source
	export let alt: string; // Image alt text
</script>

<img {src} {alt} />

<style>
	img {
		height: 100%;
		width: 100%;
		object-fit: cover;
	}
</style>
