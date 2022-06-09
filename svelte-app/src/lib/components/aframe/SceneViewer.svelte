<script lang="ts">
	import AframeModels from '$lib/components/aframe/AframeModels.svelte';
	import NavigationButton from '$lib/components/NavigationButton.svelte';

	export let arMode = false;
</script>

<a-scene
	class:arMode
	renderer="logarithmicDepthBuffer: true;"
	arjs="trackingMethod: best; sourceType: webcam;debugUIEnabled: false;"
	embedded
	vr-mode-ui="enabled: false"
>
	<div class="button backButton">
		<NavigationButton back to="/scan/preview">Back to preview</NavigationButton>
	</div>

	<div class="button placeTargets">
		<NavigationButton to="/targetplacement">Place targets</NavigationButton>
	</div>

	<a-entity light="type: ambient; color: #fff" />

	<!-- If AR is enabled -> wrap model in  -->
	{#if arMode}
		<a-marker preset="hiro">
			<a-box position="0 0 -1" rotation="0 0 0" color="red" />

			<AframeModels />
		</a-marker>
	{:else}
		<AframeModels />
	{/if}

	<a-camera />
</a-scene>

<style>
	.arMode {
		position: inherit !important;
	}

	.button {
		position: absolute;
		width: 15rem;
		max-width: calc(50vw - 2rem);
		z-index: 1;
	}

	.backButton {
		top: 1rem;
		left: 1rem;
	}

	.placeTargets {
		top: 1rem;
		right: 1rem;
	}
</style>
