<script lang="ts">
	import { gltfStore, targetLocations } from '$lib/stores/gltfStore';

	export let scale: [number, number, number] = [0.05, 0.025, 0.05];

	$: scaleString = scale.join(' ');
</script>

{#if $gltfStore}
	<a-entity position="1 -1 -3" scale={scaleString} rotation="0 -90 0">
		<!-- Place "soccer" spotlights on each corner of the model -->
		<a-entity light="color: #ddf; intensity: 0.75" position="-100 150 -100" />
		<a-entity light="color: #ddf; intensity: 0.75" position="100 150 -100" />
		<a-entity light="color: #ddf; intensity: 0.75" position="-100 150 100" />
		<a-entity light="color: #ddf; intensity: 0.75" position="100 150 100" />

		<!-- Mountain -->
		<a-entity gltf-model="url({$gltfStore.gltf_url})" />

		<!-- Loop over each target location. However, first each Draggable item is converted to an AltitudeAndGradient Object -->
		{#each $targetLocations.map((l) => gltfStore.getAlitituteAndGradient(l)) as altAndGrad}
			<a-entity
				gltf-model="url(/steam_turbine.glb)"
				scale="0.1 0.2 0.1"
				position="{altAndGrad.x} {altAndGrad.altitude} {altAndGrad.y}"
				rotation="0 0 0"
				animation-mixer
			/>
			<a-box
				width="1.4"
				depth="2.5"
				height={altAndGrad.altitude / 2}
				position="{altAndGrad.x} {altAndGrad.altitude / 2} {altAndGrad.y + 0.15}"
				color="#444"
				scale="1 2 1"
			/>
		{/each}
	</a-entity>
{/if}
