<script lang="ts">
	export let src: string;
	export let alt: string;
</script>

<img {src} {alt} />

<style>
	img {
		height: 100%;
		width: 100%;
		object-fit: cover;
	}
</style>
