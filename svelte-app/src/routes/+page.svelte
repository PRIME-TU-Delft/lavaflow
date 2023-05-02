<script lang="ts">
	import { onMount } from 'svelte';
	import init, * as wasm from 'wasm';

	onMount(() => {
		init().then(() => {
			wasm.greet('World');
		});
	});
</script>
