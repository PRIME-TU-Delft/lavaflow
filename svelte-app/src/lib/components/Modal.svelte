<script lang="ts">
	/**
	 * This component is a modal that can be opened and closed
	 */
	import Button from '$lib/components/Button.svelte';
	import { onMount } from 'svelte';

	export let visible = true;
	export let closeButtons: 'both' | 'top' | 'bottom' = 'both';
	export let title: string;

	function closeModal() {
		visible = false;

		localStorage.setItem(title, 'true');
	}

	onMount(() => {
		visible = !localStorage.getItem(title);
	});
</script>

{#if visible}
	<div class="background" on:click={closeModal}>
		<div class="modal" on:click|stopPropagation>
			{#if closeButtons == 'both' || closeButtons == 'top'}
				<Button on:click={closeModal}>Close {title}</Button>
			{/if}

			<slot />

			{#if closeButtons == 'both' || closeButtons == 'bottom'}
				<Button on:click={closeModal}>Close {title}</Button>
			{/if}
		</div>
	</div>
{/if}

<style lang="scss">
	.background {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background: rgba(0, 0, 0, 0.5);
		z-index: 11;
		cursor: pointer;

		.modal {
			width: 30rem;
			min-height: 30rem;
			max-width: 90vw;
			max-height: calc(var(--vh) * 0.9 - 5rem);
			overflow: scroll;
			padding: 1rem;
			border-radius: 0.5rem;
			background: var(--secondary-color);
			color: var(--text-color-secondary);
			position: absolute;
			top: 50%;
			left: 50%;
			transform: translate(-50%, -50%);
		}
	}
</style>
