<script lang="ts">
	import Button from './Button.svelte';

	export let icon = '';
	export let loading = false;
	export let secondary = false;
	export let disabled = false;
	export let href = '';
	export let twClass = '';
	export let fullwidth = false;
</script>

{#if loading}
	<Button {fullwidth} {secondary}>
		<div class="flex gap-2 items-center">
			<span class="loading loading-spinner loading-md" />
			<slot name="loading">Loading...</slot>
		</div>
	</Button>
{:else if href}
	<a class="no-underline" {href}>
		<Button {fullwidth} {twClass} {icon} {secondary} {disabled}>
			<slot />
		</Button>
	</a>
{:else}
	<Button {fullwidth} {twClass} {icon} {secondary} {disabled} on:click>
		<slot />
	</Button>
{/if}
