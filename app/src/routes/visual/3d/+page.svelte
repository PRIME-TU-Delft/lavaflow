<script lang="ts">
	import { goto } from '$app/navigation';
	import ActionButton from '$lib/components/ActionButton.svelte';
	import ActionMenu from '$lib/components/ActionMenu.svelte';
	import Menubar from '$lib/components/Menubar.svelte';
	import { gltfStore } from '$lib/stores/gltfStore';
	import { Canvas } from '@threlte/core';
	import { onMount } from 'svelte';
	import Scene from './Scene.svelte';

	let gltfModel: string;

	onMount(async () => {
		if (!$gltfStore?.gltf_url) {
			// TODO: display error and ask if user wants to rescan or continue with default mountain
			return goto('/preview');
		}
	});
</script>

<Menubar title="Visual 3d" back="/preview" />

<div class="h-full w-full">
	<Canvas>
		{#if $gltfStore?.gltf_url}
			<Scene gltfModel={$gltfStore.gltf_url} />
		{/if}
	</Canvas>
</div>

<ActionMenu>
	<!-- TODO: make buttons work -->
	<ActionButton secondary>Start eruption</ActionButton>
	<ActionButton>Place turbines</ActionButton>
</ActionMenu>
