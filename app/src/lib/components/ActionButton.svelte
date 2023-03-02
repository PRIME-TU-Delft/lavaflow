<script>
	import { Button, Spinner } from 'flowbite-svelte';
	import { Icon } from 'mdi-svelte-ts';

	export let icon = '';
	export let loading = false;
	export let secondary = false;
	export let disabled = false;
	export let href = '';
</script>

{#if loading}
	<Button outline={secondary} class="flex items-center justify-between !p-4" color="red">
		<Spinner />
		<slot name="loading">Loading...</slot>
	</Button>
{:else}
	<Button
		{href}
		outline={secondary}
		{disabled}
		class="flex items-center justify-between !p-4 {secondary ? 'bg-red-50/80' : ''}"
		on:click
		color="red"
	>
		<slot />

		{#if icon}
			<Icon path={icon} />
		{/if}
	</Button>
{/if}
