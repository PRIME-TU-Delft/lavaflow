<script lang="ts">
	import { mdiChevronLeft } from '@mdi/js';
	import { Button } from 'flowbite-svelte';
	import { Icon } from 'mdi-svelte-ts';

	export let title = '';
	export let back = '';
</script>

<nav class="fixed top-0 left-0 z-10 flex w-full items-center justify-between bg-red-50 p-4">
	<div class="flex items-center gap-4">
		{#if back}
			<Button class="flex items-center gap-2 !p-2" href={back} outline color="red">
				<Icon path={mdiChevronLeft} />
				<slot name="backTitle" />
			</Button>
		{/if}

		{#if title}
			<h1 class="text-xl text-red-900">{title}</h1>
		{/if}
	</div>

	<slot />
</nav>
