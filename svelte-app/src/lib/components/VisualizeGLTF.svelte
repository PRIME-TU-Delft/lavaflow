<script lang="ts">
	import * as THREE from 'three';
	import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader.js';
	import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';

	export let gltf: string;

	const scene = new THREE.Scene();
	const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
	const renderer = new THREE.WebGLRenderer();
	const controls = new OrbitControls(camera, renderer.domElement);

	renderer.setFaceCulling(false);
	renderer.setClearColor(0xbbbfc9, 1);
	renderer.setSize(window.innerWidth, window.innerHeight);
	renderer.shadowMap.enabled = true;
	document.body.appendChild(renderer.domElement); // TODO fix this with svelte magic

	const geometry = new THREE.BoxGeometry();
	const material = new THREE.MeshBasicMaterial({ color: 0xff0000 });

	camera.position.y = 150;
	camera.position.x = 100;
	camera.position.z = 100;
	controls.update();

	const light = new THREE.PointLight(0xffffff, 1, 0);
	light.position.set(100, 150, 100);
	scene.add(light);

	const light2 = new THREE.AmbientLight(0x999999, 1); // soft white light
	scene.add(light2);

	let loader = new GLTFLoader();

	function loadGLTF() {
		loader.load(
			'data:text/plain;base64,' + btoa(gltf), // Data URI with the GLTF file's content
			// called when the resource is loaded
			function (gltf: any) {
				gltf.scene.scale.set(0.1, 0.1, 0.1); // Scale the model
				gltf.scene.children[0].material.side = THREE.DoubleSide; // "Invert" the mode sides

				gltf.scene.traverse((child: any) => {
					if (child.isMesh) child.material.flatShading = THREE.SmoothShading;
					if (child.material) child.material.metalness = 0.5;
				});

				gltf.scene.traverse(function (node: any) {
					if (node.isMesh || node.isLight) node.castShadow = true;
					if (node.isMesh || node.isLight) node.receiveShadow = true;
				});

				gltf.scene.position.x = -60;
				gltf.scene.position.z = -50;

				scene.add(gltf.scene);

				gltf.animations; // Array<THREE.AnimationClip>
				gltf.scene; // THREE.Group
				gltf.scenes; // Array<THREE.Group>
				gltf.cameras; // Array<THREE.Camera>
				gltf.asset; // Object
			},
			// called while loading is progressing
			function (xhr: any) {
				console.log((xhr.loaded / xhr.total) * 100 + '% loaded');
				console.log('xhr.total = ' + xhr.total);
			},
			// called when loading has errors
			function (error: any) {
				console.log('An error happened');
			}
		);
	}

	$: gltf && loadGLTF();

	function animate() {
		requestAnimationFrame(animate);
		controls.update();
		renderer.render(scene, camera);
	}
	animate();
</script>
