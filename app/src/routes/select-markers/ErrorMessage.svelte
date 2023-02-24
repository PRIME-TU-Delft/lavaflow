<script lang="ts">
	import type LavaError from '$lib/data/LavaError';
	import { mdiClose } from '@mdi/js';
	import { Button, ButtonGroup } from 'flowbite-svelte';
	import { Icon } from 'mdi-svelte-ts';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	export let error: LavaError;

	const dismiss = () => dispatch('dismiss');
	const preBuild = () => dispatch('preBuild');
</script>

<div class="w-full rounded border-2 border-red-900 bg-red-50 p-4">
	<div class="flex items-center justify-between gap-2">
		<p class="w-auto overflow-hidden text-ellipsis whitespace-nowrap font-bold text-red-900">
			{error.title}
		</p>
		<Button outline class="!p-2" color="red" on:click={dismiss}>
			<Icon path={mdiClose} />
		</Button>
	</div>

	<p class="mt-1 text-red-800">{error.message}</p>

	<ButtonGroup class="mt-4 w-full">
		<Button class="w-full" outline color="red" href="capture">Rescan image</Button>
		<Button class="w-full grow" outline color="red" on:click={preBuild}>
			Continue with pre-build contours
		</Button>
	</ButtonGroup>
</div>
