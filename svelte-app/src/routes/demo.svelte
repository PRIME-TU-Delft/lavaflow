<script lang="ts">
	import NavigationButton from '$lib/components/NavigationButton.svelte';

	import { gltfUrl } from '$lib/stores/gltfStore';
	import { onMount } from 'svelte';
	import { set_attributes } from 'svelte/internal';

	let mounted: boolean;
	let aframe: boolean;
	let sceneEl: HTMLElement;
	$: ready = (aframe || window?.AFRAME) && mounted;

	function aframeLoaded() {
		aframe = true;

		console.log(window.AFRAME);
		window.AFRAME.registerComponent('lava-paths', {
			init: function () {
				const lavaPaths: [number, number, number][][] = [
					[
						[3, 0, -5],
						[-3, 2, 5],
						[3, 0, -5]
					],
					[
						[4, 4, 7],
						[1, 1, 3],
						[4, 2, 2],
						[3, 3, -1]
					],
					[
						[1, 1, 3],
						[0, -2, -4],
						[1, 1, -2],
						[1, 2, -1],
						[2, 2, 0]
					],
					[
						[1, 1, -2],
						[1, 4, -2],
						[12, 1, -1]
					]
				];

				for (let i = 0; i < lavaPaths.length; i++) {
					const points = lavaPaths[i];

					const curve = document.createElement('a-curve');
					curve.setAttribute('id', 'track' + i);

					points.forEach((point) => {
						const [x, y, z] = point;

						const aPoint = document.createElement('a-curve-point');
						aPoint.setAttribute('position', x + ' ' + z + ' ' + y);
						curve.appendChild(aPoint);
					});

					// Create track following each curve
					const track = document.createElement('a-entity');
					track.setAttribute('id', 'lava' + i);
					track.setAttribute(
						'clone-along-curve',
						'curve: #track' + i + '; spacing: 0.2; rotation: 90 0 0;'
					);
					track.setAttribute('geometry', 'primitive:cylinder; height:0.33; radius:0.2; color:red');

					this.el.appendChild(curve);
					this.el.appendChild(track);
				}
			}
		});
	}

	onMount(async () => {
		mounted = true;
	});
</script>

<svelte:head>
	{#if mounted && !window?.AFRAME}
		<script src="https://aframe.io/releases/1.0.0/aframe.min.js" on:load={aframeLoaded}></script>
	{/if}
</svelte:head>

{#if ready}
	<a-scene embedded>
		<div class="backButton">
			<NavigationButton back to="/scan/mapscanning">Rescan image</NavigationButton>
		</div>

		<a-box position="0 0 0" material="opacity: 0.5;" color="red" />

		<!-- <a-entity
			gltf-model="url({$gltfUrl})"
			position="3 0 -5"
			scale="0.00038 0.1 0.00038"
			rotation="0 -90 0"
			material="opacity: 0.5;"
			id="model"
		/> -->

		<a-curve id="track0">
			<a-curve-point position="3 -5 0"></a-curve-point>
			<a-curve-point position="-3 5 2"></a-curve-point>
			<a-curve-point position="5 -5 5"></a-curve-point>
		</a-curve>

		<a-draw-curve curveref="#track0" material="shader: line; color: white;"></a-draw-curve>
		<a-entity id="lava0" clone-along-curve="curve: #track0; spacing: 0.2; rotation: 90 0 0;" geometry="primitive:cylinder; height:0.33; radius:0.2; color:red"></a-entity>

		<a-camera look-controls />
	</a-scene>
{/if}

<style>
	.backButton {
		position: absolute;
		top: 1rem;
		left: 1rem;
		z-index: 1;
		width: 15rem;
		max-width: calc(100vw - 2rem);
	}
</style>
