<script lang="ts">
	import { Icon } from 'mdi-svelte-ts';
	import { twMerge } from 'tailwind-merge';

	export let iconPrefix: string = '';
	export let icon: string = '';
	export let outline: boolean = false;
	export let disabled: boolean = false;
	export let twClass: string = '';
	export let id: string = '';

	$: classes = twMerge('btn btn-primary flex items-center justify-center gap-2', twClass);
</script>

<button {id} class={classes} class:btn-outline={outline} {disabled} on:click>
	{#if iconPrefix}
		<Icon path={iconPrefix} />
	{/if}

	{#if $$slots.default}
		<div>
			<slot />
		</div>
	{/if}

	{#if icon}
		<Icon path={icon} />
	{/if}
</button>
