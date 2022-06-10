<script lang="ts">
	import { gltfStore, targetLocations } from '$lib/stores/gltfStore';

	export let scale: [number, number, number] = [0.05, 0.025, 0.05];

	$: scaleString = scale.join(' ');
</script>

{#if $gltfStore}
	<a-entity position="1 -1 -3" scale={scaleString} rotation="0 -90 0">
		<!-- Place "soccer" spotlights on each corner of the model -->
		<a-entity light="color: #ddf; intensity: 1.5" position="-100 100 -100" />
		<a-entity light="color: #ddf; intensity: 1.5" position="100 100 -100" />
		<a-entity light="color: #ddf; intensity: 1.5" position="-100 100 100" />
		<a-entity light="color: #ddf; intensity: 1.5" position="100 100 100" />

		<!-- Mountain -->
		<a-entity gltf-model="url({$gltfStore})" />

		<!-- Loop over each target location. However, first each Draggable item is converted to an AltitudeAndGradient Object -->
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
				position="{altAndGrad.x} {altAndGrad.altitude / 2} {altAndGrad.y}"
				color="#90352C"
				scale="1 2 1"
			/>
		{/each}
	</a-entity>
{/if}
