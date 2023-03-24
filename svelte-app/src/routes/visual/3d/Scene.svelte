<script lang="ts">
	import { AmbientLight, OrbitControls, PerspectiveCamera, T } from '@threlte/core';
	import { GLTF } from '@threlte/extras';
	import { Vector3 } from 'three';

	import { gltfStore } from '$lib/stores/gltfStore';
	import { turbineLocations, type Turbine } from '$lib/stores/locationStore';

	export let showLava = false;

	const scale = 0.05;

	/**
	 * Convert a Turbine object to an AltitudeAndGradient object, and scale to correct proportions
	 * @param t - Turbine object
	 */
	function getAltAndGrad(t: Turbine) {
		const altAndGrad = gltfStore.getAltitudeAndGradient(t);

		altAndGrad.x *= scale;
		altAndGrad.x -= 50 * scale;

		altAndGrad.y *= scale;
		altAndGrad.y -= 50 * scale;

		altAndGrad.altitude *= scale;

		return altAndGrad;
	}
</script>

<PerspectiveCamera position={new Vector3(10, 20, 10)} fov={25}>
	<OrbitControls maxPolarAngle={Math.PI / 2} minDistance={10} maxDistance={40} target={{ y: 2 }} />
</PerspectiveCamera>

<AmbientLight />

{#if $gltfStore.gltf_url}
	<GLTF position={new Vector3(-50 * scale, 0, -50 * scale)} {scale} url={$gltfStore.gltf_url} />
{/if}

{#if showLava && $gltfStore.lava_gltf_url}
	<GLTF
		position={new Vector3(-50 * scale, 0, -50 * scale)}
		{scale}
		url={$gltfStore.lava_gltf_url}
	/>
{/if}

{#each $turbineLocations.map(getAltAndGrad) as altAndGrad}
	<T.Mesh position={[altAndGrad.x, altAndGrad.altitude / 2, altAndGrad.y]}>
		<T.BoxGeometry args={[0.1, altAndGrad.altitude, 0.1]} />
		<T.MeshBasicMaterial color="#444" />
	</T.Mesh>

	<GLTF
		position={new Vector3(altAndGrad.x, altAndGrad.altitude, altAndGrad.y)}
		scale={scale * 0.2}
		url="/steam_turbine.glb"
	/>
{/each}

<T.Mesh receiveShadow rotation.x={-Math.PI / 2}>
	<T.CircleGeometry args={[5, 72]} />
	<T.MeshStandardMaterial color="rgb(30,15,20)" transparent opacity={0.1} />
</T.Mesh>
