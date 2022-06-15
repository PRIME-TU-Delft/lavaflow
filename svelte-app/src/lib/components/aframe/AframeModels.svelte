<script lang="ts">
	import { gltfStore, targetLocations } from '$lib/stores/gltfStore';
	import Draggable from '$lib/data/draggable';

	export let scale: [number, number, number];

	$: scaleString = scale.join(' ');
</script>

{#if $gltfStore}
	<!----------------------------------------------------------------
	--        				SUPER ENTITY							--
	------------------------------------------------------------------>
	<a-entity scale={scaleString} >

		<!----------------------------------------------------------------
		--  Place "soccer" spotlights on each corner of the model		--
		------------------------------------------------------------------>
		<a-entity light="color: #ddf; intensity: 0.75" position="-100 150 -100" />
		<a-entity light="color: #ddf; intensity: 0.75" position="100 150 -100" />
		<a-entity light="color: #ddf; intensity: 0.75" position="-100 150 100" />
		<a-entity light="color: #ddf; intensity: 0.75" position="100 150 100" />

		<!----------------------------------------------------------------
		--                    MOUNTAIN									--
		------------------------------------------------------------------>
		<a-entity gltf-model="url({$gltfStore.gltf_url})" />

		<!----------------------------------------------------------------
		--                    CRATERS								  --
		------------------------------------------------------------------>
		{#each $gltfStore.craters.map( (l) => gltfStore.getAlitituteAndGradient(new Draggable(l[0], l[1], 20), true) ) as altAndGrad}
			<a-cylinder
				radius="2"
				height={altAndGrad.altitude / 2}
				position="
				{altAndGrad.x} 
				{altAndGrad.altitude / 2} 
				{altAndGrad.y}"
				opacity="0.8"
				color="#ff4025"
				scale="1 2 1"
			/>
		{/each}

		<!----------------------------------------------------------------
		--       		        	TARGETS				                					  --
		--                                                              --
		-- Loop over each target location, However, first each          --
		-- Draggable item is converted to an AltitudeAndGradient Object --
		------------------------------------------------------------------>
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
				color="#ff4025"
				scale="1 2 1"
			/>
		{/each}
	</a-entity>
{/if}
