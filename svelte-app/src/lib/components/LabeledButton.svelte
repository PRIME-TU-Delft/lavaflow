<script lang="ts">
	import Button from './Button.svelte';

	export let iconPrefix: string = '';
	export let icon: string = '';
	export let href: string = '';
	export let outline: boolean = false;
	export let disabled: boolean = false;
	export let twClass: string = '';
</script>

<div>
	{#if $$slots.label}
		<div class="my-2 text-sm text-zinc-800">
			<slot name="label" />
		</div>
	{/if}

	{#if href}
		<a href={'' + href}>
			<Button {iconPrefix} {icon} {outline} {disabled} {twClass} on:click>
				<slot />
			</Button>
		</a>
	{:else}
		<Button {iconPrefix} {icon} {outline} {disabled} {twClass} on:click>
			<slot />
		</Button>
	{/if}
</div>
