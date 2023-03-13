<script lang="ts">
	import { AmbientLight, OrbitControls, PerspectiveCamera, T } from '@threlte/core';
	import { GLTF } from '@threlte/extras';
	import { Vector3 } from 'three';

	import { gltfStore } from '$lib/stores/gltfStore';
	import { turbineLocations } from '$lib/stores/locationStore';

	const scale = 0.05;
	$: turbineScale = new Vector3(scale * 0.1, scale * 0.2, scale * 0.1);
</script>

<PerspectiveCamera position={new Vector3(10, 20, 10)} fov={25}>
	<!-- <OrbitControls maxPolarAngle={Math.PI / 2} minDistance={10} maxDistance={40} target={{ y: 2 }} /> -->
	<OrbitControls minDistance={10} maxDistance={40} target={{ y: 2 }} />
</PerspectiveCamera>

<AmbientLight />

<T.AxesHelper scale={[10, 10, 10]} />

{#if $gltfStore.gltf_url}
	<GLTF position={new Vector3(-50 * scale, 0, -50 * scale)} {scale} url={$gltfStore.gltf_url} />
{/if}

{#each $turbineLocations.map((l) => gltfStore.getAlitituteAndGradient(l, scale)) as altAndGrad}
	<!-- <GLTF
		position={new Vector3(altAndGrad.x, altAndGrad.altitude * scale, altAndGrad.y)}
		scale={turbineScale || scale}
		url="/steam_turbine.glb"
	/> -->

	<T.Mesh
		position={[altAndGrad.x - 50 * scale, altAndGrad.altitude / 2, altAndGrad.y - 50 * scale]}
	>
		<T.BoxGeometry args={[0.1, altAndGrad.altitude, 0.1]} />
		<T.MeshBasicMaterial color="#444" />
	</T.Mesh>
{/each}

<T.Mesh receiveShadow rotation.x={-Math.PI / 2}>
	<T.CircleGeometry args={[5, 72]} />
	<T.MeshStandardMaterial color="rgb(30,15,20)" transparent opacity={0.1} />
</T.Mesh>
