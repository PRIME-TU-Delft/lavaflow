<script lang="ts">
	import NavigationButton from '$lib/components/NavigationButton.svelte';

	import { contourLines } from '$lib/stores/contourLineStore';

	import { gltfStore, targetLocations } from '$lib/stores/gltfStore';
	import { hc_curves, hc_hierarchy } from '$lib/data/hardCoded';

	import { onMount } from 'svelte';

	onMount(async () => {
		console.log($targetLocations);

		if (!$gltfStore) {
			contourLines.set({
				curves: hc_curves,
				hierarchy: hc_hierarchy,
				size: { width: 850, height: 950 }
			});

			await gltfStore.setup($contourLines);
			gltfStore.build();
			console.warn('gltf is loaded from hardcoded data', $gltfStore);
		}
	});
</script>

<svelte:head>
	<!-- {#if mounted}Add {/if} -->
</svelte:head>

<a-scene embedded background="color: #ddf">
	<div class="button backButton">
		<NavigationButton back to="/scan/mapscanning">Rescan image</NavigationButton>
	</div>

	<div class="button placeTargets">
		<NavigationButton to="/targetplacement">Place targets</NavigationButton>
	</div>

	<a-entity light="color: #ddf; intensity: 1.5" position="-1 1 0" />
	<a-entity light="color: #ddf; intensity: 1.5" position="3 1 -4" />
	<a-entity light="type: ambient; color: #fff" />

	{#if $gltfStore}
		<a-entity position="1 -1 -3" scale="0.05 0.025 0.05" rotation="0 -90 0">
			<a-entity gltf-model="url({$gltfStore})" />

			{#each $targetLocations.map((l) => gltfStore.getAlitituteAndGradient(l)) as altAndGrad}
				<a-entity
					gltf-model="url(steam_turbine.glb)"
					scale="0.1 0.2 0.1"
					position="{altAndGrad.x} {altAndGrad.altitude} {altAndGrad.y}"
					rotation="0 0 0"
					animation-mixer
				/>
				<a-box
					width="2"
					depth="2"
					height={altAndGrad.altitude / 2}
					position="
						{altAndGrad.x} 
						{altAndGrad.altitude / 2} 
						{altAndGrad.y}"
					color="#90352C"
					scale="1 2 1"
				/>
			{/each}
		</a-entity>
	{:else}
		<a-entity
			gltf-model="url(output20.gltf)"
			scale="0.0001 0.04 0.0001"
			position="1 1 -3"
			rotation="0 -90 0"
		/>
	{/if}

	<a-camera look-controls />
</a-scene>

<style>
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
