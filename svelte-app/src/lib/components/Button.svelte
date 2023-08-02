<script lang="ts">
	import { Icon } from 'mdi-svelte-ts';
	import { twMerge } from 'tailwind-merge';

	export let iconPrefix: string = '';
	export let icon: string = '';
	export let outline: boolean = false;
	export let disabled: boolean = false;
	export let twClass: string = '';
	export let id: string = '';

	$: classes = twMerge('btn btn-primary', twClass);
</script>

<button {id} class={classes} class:btn-outline={outline} {disabled} on:click>
	<div class="flex items-center gap-2">
		{#if iconPrefix}
			<Icon path={iconPrefix} />
		{/if}

		<slot />

		{#if icon}
			<Icon path={icon} />
		{/if}
	</div>
</button>
