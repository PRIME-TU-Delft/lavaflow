<script lang="ts">
	import { konami } from 'konami.js';

	import { debugMode } from '$lib/stores/debugStore';
	import { onMount } from 'svelte';

	onMount(() => {
		konami(() => {
			debugMode.update((mode) => !mode);
			console.log($debugMode);
		});
	});
</script>

<slot />
