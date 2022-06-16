<script lang="ts">
	import Icon from '$lib/components/Icon.svelte';
	import { mdiLoading } from '@mdi/js';

	export let loading = false;
	export let disabled = false;
	export let large = false;
	export let small = false;
	export let secondary = false;
	export let green = false;
	export let icon: string = '';
	export let noMargin = false;
</script>

<button
	disabled={disabled || loading}
	class:disabled={disabled || loading}
	class:noMargin
	class:large
	class:small
	class:secondary
	class:green
	on:click
>
	{#if loading}
		<div class="title">
			<Icon path={mdiLoading} spin={1} />
			Loading...
		</div>
	{:else}
		<div class="title">
			{#if icon}
				<Icon path={icon} />
			{/if}
			<slot />
		</div>

		{#if $$slots.content}
			<div class="content">
				<slot name="content" />
			</div>
		{/if}
	{/if}
</button>

<style>
	button {
		background: var(--primary-color);
		color: var(--text-color);
		width: 100%;
		padding: 0.75rem 1.1rem;

		font-family: 'Roboto Slab';
		font-weight: 700;
		font-size: 1rem;
		text-align: left;
		border: none;
		margin-block: 0.5rem;
		border-radius: 0.3rem;
		cursor: pointer;
		user-select: none;
	}

	button.noMargin {
		margin-block: 0;
	}

	button.disabled {
		background: grey;
	}

	.title {
		font-size: 1.1rem;
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	.content {
		margin-top: 0.25rem;
		font-size: 0.9rem;
	}

	.large {
		padding: 1rem 1.5rem;
	}

	.small {
		padding: 0.5rem 1.5rem;
	}

	.secondary {
		background: var(--secondary-color);
		color: var(--text-color-secondary);
	}

	.green {
		background: #308167;
		color: #fff;
	}

	:global(button > .title > span) {
		width: 100%;
	}
</style>
