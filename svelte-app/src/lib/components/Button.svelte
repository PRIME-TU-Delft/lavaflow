<script>
	import Icon from 'mdi-svelte';

	export let disabled = false;
	export let large = false;
	export let secondary = false;
	export let icon = false;
</script>

<button class:disabled class:large class:secondary on:click>
	<div class="title">
		{#if icon}
			<Icon path={icon} color="var(--text-color)" />
		{/if}

		<slot />
	</div>

	{#if $$slots.content}
		<div class="content">
			<slot name="content" />
		</div>
	{/if}
</button>

<style>
	button {
		background: var(--primary-color);
		color: var(--text-color);
		width: 100%;
		padding: 0.75rem 1.1rem;

		font-size: 1rem;
		text-align: left;
		border: none;
		margin-block: 0.5rem;
		border-radius: 0.3rem;
		cursor: pointer;
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

	.secondary {
		background: var(--secondary-color);
	}
</style>
