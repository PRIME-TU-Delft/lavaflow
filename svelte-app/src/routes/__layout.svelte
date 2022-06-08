<script lang="ts">
	import { konami } from 'konami.js';

	import { debugMode } from '$lib/stores/debugStore';

	konami(() => {
		debugMode.update((mode) => !mode);
		console.log($debugMode);
	});
</script>

<slot />
