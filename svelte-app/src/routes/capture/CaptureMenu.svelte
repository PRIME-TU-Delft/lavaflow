<script>
	import { mdiHelp, mdiCamera } from '@mdi/js';
	import { Button, Spinner } from 'flowbite-svelte';
	import { Icon } from 'mdi-svelte-ts';

	export let loading = false;
	export let disabled = false;
</script>

<div class="flex w-full items-center justify-around gap-4 bg-red-50 p-10 md:rounded-lg md:p-4 ">
	<Button href="capture/instructions" class="h-min !p-4" outline color="red" pill>
		<Icon path={mdiHelp} size={1.2} />
	</Button>

	{#if loading}
		<Button class="!p-4 hover:bg-transparent" outline color="red">
			<Spinner class="h-8 w-8" color="red" />
		</Button>
	{:else}
		<Button {disabled} class="!p-4" outline color="red" on:click>
			<Icon path={mdiCamera} size={2} />
		</Button>
	{/if}

	<Button class="!cursor-default !p-4 opacity-0" />
</div>
