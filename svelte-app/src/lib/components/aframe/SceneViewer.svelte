<script lang="ts">
	import AframeModels from '$lib/components/aframe/AframeModels.svelte';
	import NavigationButton from '$lib/components/NavigationButton.svelte';
	import Button from '$lib/components/Button.svelte';

	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	import { gltfStore } from '$lib/stores/gltfStore';
	import { contourLines } from '$lib/stores/contourLineStore';
	import { difficultyStore } from '$lib/stores/difficultyStore';
	import { hc_curves, hc_hierarchy } from '$lib/data/hardCoded';
	import { debugMode } from '$lib/stores/debugStore';
	import { targetLocations } from '$lib/stores/locationStore';

	export let arMode = false;
	export let scale: [number, number, number] = [0.05, 0.025, 0.05];

	let lava_revealed = false;
	let obtained_points = 0;
	let max_points = 1000;

	function revealLava() {
		// Compute the amount of points the user obtained
		// The rust api exports a function that can be used to efficiently compute the player's points.
		// Use this api-call to compute the points
		obtained_points = gltfStore.computePlayerPoints(max_points);

		lava_revealed = true;
	}

	function hideLava() {
		lava_revealed = false;
	}

	onMount(async () => {
		if ($gltfStore) return; // When gltf store is loaded -> don't (re)load again

		// If debug mode is disabled and there are no contourlines -> goto scanning of paper
		if (!$debugMode && !$contourLines) {
			goto('/scan/mapscanning');
			return;
		}

		// If debug mode is enabled -> load hardcoded data
		console.warn('SCENE VIEWER: gltf is loaded from hardcoded data');

		if (!$contourLines) {
			contourLines.set({
				curves: hc_curves,
				hierarchy: hc_hierarchy,
				size: { width: 800, height: 960 }
			});
		}

		await gltfStore.setup($contourLines, $difficultyStore.lava_forking);
		gltfStore.build($contourLines);
	});
</script>

<a-scene
	class:arMode
	embedded
	vr-mode-ui="enabled: false"
	gesture-detector
	arjs="trackingMethod: best; detectionMode: mono_and_matrix; matrixCodeType: 3x3;"
>
	<slot name="overlay">
		<div class="button backButton">
			<slot name="backButton">
				<NavigationButton back to="/scan/preview">Back to preview</NavigationButton>
			</slot>
		</div>


		<div class="button rightButton">
			<slot name="targetButton">
				<NavigationButton to="/targetplacement">Place targets</NavigationButton>
			</slot>
		
			{#if $targetLocations.length > 0}
				{#if lava_revealed}
					<Button secondary on:click={hideLava}>Hide lava</Button>
					<Button green>Points: {obtained_points} / {max_points}</Button>
				{:else}
					<Button on:click={revealLava}>Reveal lava</Button>
				{/if}
			{:else}
				<Button disabled secondary>Place targets to begin</Button>
			{/if}
		</div>

		<!-- <div class="button placeTargets">
			<NavigationButton to="/targetplacement">Place targets</NavigationButton>
		</div> -->
	</slot>

	<a-entity light="type: ambient; color: #fff" />

	<!-- If AR is enabled -> wrap model in  -->
	{#if arMode}
		<a-marker
			id="marker0"
			type="barcode"
			value="0"
			raycaster="objects: .clickable"
			emitevents="true"
			cursor="fuse: false; rayOrigin: mouse;"
		>
			{#if $debugMode}
				<!-- When debugging -> display blue cube on top of marker -->
				<a-box position="0 0 0" material="color: blue; opacity: 0.5;" />
			{/if}

			<a-entity class="clickable" gesture-handler>
				<AframeModels {scale} {lava_revealed} />
			</a-entity>
		</a-marker>

		<a-entity camera />
	{:else}
		<AframeModels {scale} {lava_revealed} />

		<a-camera position="4 2 7" />
	{/if}
</a-scene>

<style>
	.arMode {
		position: inherit !important;
	}

	.button {
		position: absolute;
		width: 15rem;
		max-width: calc(50vw - 2rem);
		z-index: 1;
	}

	.backButton {
		top: 1rem;
		left: 1rem;
	}

	.rightButton {
		top: 1rem;
		right: 1rem;
	}
</style>
