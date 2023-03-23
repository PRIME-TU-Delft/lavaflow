<script lang="ts">
	import type LavaError from '$lib/data/LavaError';
	import { mdiClose } from '@mdi/js';
	import { Button, ButtonGroup } from 'flowbite-svelte';
	import { Icon } from 'mdi-svelte-ts';
	import { createEventDispatcher } from 'svelte';
	import { fade } from 'svelte/transition';

	const dispatch = createEventDispatcher();

	export let error: LavaError;
	export let hasActions = false;

	const dismiss = () => dispatch('dismiss');
</script>

<div
	transition:fade={{ duration: 250 }}
	class="w-full rounded border-2 border-red-900 bg-red-50 p-4"
>
	<div class="flex items-center justify-between gap-2">
		<p class="w-auto overflow-hidden text-ellipsis whitespace-nowrap font-bold text-red-900">
			{error.title}
		</p>
		<Button outline class="!p-2" color="red" on:click={dismiss}>
			<Icon path={mdiClose} />
		</Button>
	</div>

	<p class="mt-1 text-red-800">{error.message}</p>

	{#if hasActions}
		<ButtonGroup class="mt-4 w-full">
			<slot />
		</ButtonGroup>
	{/if}
</div>
