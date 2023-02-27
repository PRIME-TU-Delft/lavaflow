<script lang="ts">
	import { AmbientLight, OrbitControls, PerspectiveCamera, T } from '@threlte/core';
	import { GLTF } from '@threlte/extras';
	import { Vector3 } from 'three';

	export let gltfModel: string = '';
</script>

<PerspectiveCamera position={new Vector3(10, 20, 10)} fov={25}>
	<OrbitControls maxPolarAngle={Math.PI / 2} minDistance={10} maxDistance={40} target={{ y: 2 }} />
</PerspectiveCamera>

<AmbientLight />

<GLTF position={new Vector3(-2.5, 0, -2.5)} scale={0.05} url={gltfModel} />

<T.Mesh receiveShadow rotation.x={-Math.PI / 2}>
	<T.CircleGeometry args={[5, 72]} />
	<T.MeshStandardMaterial color="rgb(30,15,20)" />
</T.Mesh>
