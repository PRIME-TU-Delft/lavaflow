<script>
	import { mdiBookInformationVariant, mdiCamera } from '@mdi/js';
	import { Button, Spinner } from 'flowbite-svelte';
	import { Icon } from 'mdi-svelte-ts';

	export let loading = false;
</script>

<div
	class="w-full md:w-80 bg-red-50 fixed flex gap-4 justify-around items-center bottom-0 right-0 md:right-3 md:bottom-3 md:rounded-lg p-10 md:p-4 "
>
	<Button href="capture/instructions" class="!p-4 h-min" outline color="red" pill>
		<Icon path={mdiBookInformationVariant} size={1.2} />
	</Button>

	{#if loading}
		<Button class="!p-4 hover:bg-transparent" outline color="red">
			<Spinner class="h-8 w-8" color="red" />
		</Button>
	{:else}
		<Button class="!p-4" outline color="red" on:click>
			<Icon path={mdiCamera} size={2} />
		</Button>
	{/if}

	<Button class="opacity-0 !cursor-default !p-4" />
</div>
