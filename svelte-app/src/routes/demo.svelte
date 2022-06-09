<script lang="ts">
	import NavigationButton from '$lib/components/NavigationButton.svelte';

	import { contourLines } from '$lib/stores/contourLineStore';

	import { gltfStore } from '$lib/stores/gltfStore';
	import { hc_curves, hc_hierarchy } from '$lib/data/hardCoded';

	import { onMount } from 'svelte';

	let i = 10;
	let j = 10;

	onMount(async () => {
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

		let magicAnimation = setInterval(() => {
			if (i >= 90) {
				// If i is at the end -> reset to 10 and increase j
				i = 1;
				j += 10;
			} else if (i >= 90 && j >= 90) {
				// stop interval if i and j are at the end
				i = 1;
				j = 1;
				return;
			}else {
				i += 3;
			}
		}, 500);
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
		<a-entity position="1 -1 -3" scale="0.05 0.05 0.05" rotation="0 -90 0">
			<a-entity gltf-model="url({$gltfStore})" />

			{#await gltfStore.getAlitituteAndGradient(i, j) then altAndGrad}
				<a-entity
					gltf-model="url(steam_turbine.glb)"
					scale="0.1 0.1 0.1"
					position="{i} {gltfStore.altitude_adjusted_to_gradient(altAndGrad)} {j}"
					rotation="0 0 0"
					animation-mixer
				/>
				<a-box width="2" depth="2" height="{gltfStore.altitude_adjusted_to_gradient(altAndGrad)}" position="{i} {gltfStore.altitude_adjusted_to_gradient(altAndGrad) / 2} {j}" color="gray"/>
			{/await}
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
