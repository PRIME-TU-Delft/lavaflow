<script lang="ts">
	import { mdiChevronLeft } from '@mdi/js';
	import { Button } from 'flowbite-svelte';
	import { Icon } from 'mdi-svelte-ts';

	export let title = '';
	export let back = '';
</script>

<nav class="fixed top-0 left-0 w-full p-4 bg-red-50 flex justify-between items-center">
	<div class="flex gap-4 items-center">
		{#if back}
			<Button class="!p-2" href={back} outline color="red">
				<Icon path={mdiChevronLeft} />
			</Button>
		{/if}

		{#if title}
			<h1 class="text-xl text-red-900">{title}</h1>
		{/if}
	</div>

	<slot />
</nav>
