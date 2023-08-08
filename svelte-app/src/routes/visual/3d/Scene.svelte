<script lang="ts">
	import { T } from '@threlte/core';
	import { GLTF, OrbitControls } from '@threlte/extras';

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

<T.PerspectiveCamera position={[10, 5, 10]} fov={25} makeDefault>
	<OrbitControls
		maxPolarAngle={Math.PI / 2}
		minDistance={4}
		maxDistance={40}
		target={[0, 0.5, 0]}
	/>
</T.PerspectiveCamera>

<T.AmbientLight />

<T.Group scale={[1, 0.5, 1]}>
	{#if $gltfStore.gltf_url}
		<GLTF position={[-50 * scale, 0, -50 * scale]} {scale} url={$gltfStore.gltf_url} />
	{/if}

	{#if showLava && $gltfStore.lava_gltf_url}
		<GLTF position={[-50 * scale, 0, -50 * scale]} {scale} url={$gltfStore.lava_gltf_url} />
	{/if}

	{#each $turbineLocations.map(getAltAndGrad) as altAndGrad}
		<T.Mesh position={[altAndGrad.x, altAndGrad.altitude / 2, altAndGrad.y]}>
			<T.BoxGeometry args={[0.15, altAndGrad.altitude, 0.15]} />
			<T.MeshBasicMaterial color={[70, 70, 70]} />
		</T.Mesh>

		<GLTF
			position={[altAndGrad.x, altAndGrad.altitude, altAndGrad.y]}
			scale={[scale * 0.2, scale * 0.4, scale * 0.2]}
			url="/steam_turbine.glb"
		/>
	{/each}
</T.Group>

<T.Mesh receiveShadow rotation.x={-Math.PI / 2}>
	<T.CircleGeometry args={[5, 72]} />
	<T.MeshStandardMaterial color={[30, 15, 20]} transparent opacity={0.1} />
</T.Mesh>
