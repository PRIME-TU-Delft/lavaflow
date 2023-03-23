<script lang="ts">
	import { gltfStore } from '$lib/stores/gltfStore';
	import { turbineLocations } from '$lib/stores/locationStore';

	export let rotation = 0;
	export let size = [0.025, 0.0125, 0.025] as [number, number, number];
	export let showLava: boolean = false;
	export let scale = 0.5;

	$: scaleString = size.map((x) => x * scale).join(' ');
</script>

{#if $gltfStore}
	<!----------------------------------------------------------------
	--        				SUPER ENTITY																	--
	------------------------------------------------------------------>
	<a-entity scale={scaleString} rotation="0 {rotation} 0">
		<a-entity position="-50 0 -50">
			<!----------------------------------------------------------------
			--  Place "soccer" spotlights on each corner of the model				--
			------------------------------------------------------------------>
			<a-entity light="color: #ddf; intensity: 0.75" position="-100 150 -100" />
			<a-entity light="color: #ddf; intensity: 0.75" position="100 150 -100" />
			<a-entity light="color: #ddf; intensity: 0.75" position="-100 150 100" />
			<a-entity light="color: #ddf; intensity: 0.75" position="100 150 100" />

			<!----------------------------------------------------------------
			--                    MOUNTAIN																	--
			------------------------------------------------------------------>
			<a-entity gltf-model="url({$gltfStore.gltf_url})" />
			{#if showLava}
				<!--LAVA gltf model-->
				<a-entity gltf-model="url({$gltfStore.lava_gltf_url})" />
			{/if}

			<!----------------------------------------------------------------
			--       		        	TARGETS				                					  --
			--                                                              --
			-- Loop over each target location. First each          					--
			-- Draggable item is converted to an AltitudeAndGradient Object --
			------------------------------------------------------------------>
			{#each $turbineLocations.map((l) => gltfStore.getAlitituteAndGradient(l)) as altAndGrad}
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
	</a-entity>
{/if}
