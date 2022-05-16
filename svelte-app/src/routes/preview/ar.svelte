<script>
	import { onMount } from 'svelte';
	import Aframe from '$lib/components/Aframe.svelte';

	let ready = false;

	onMount(async () => {
		ready = true;
	});
</script>

{#if ready}
	<Aframe />
{/if}
