<script lang="ts">
	/**
	 * This component is a modal that can be opened and closed
	 */
	import Button from '$lib/components/Button.svelte';

	export let visible = false;
</script>

{#if visible}
	<div class="background" on:click={() => (visible = !visible)}>
		<div class="modal" on:click|stopPropagation>
			<Button on:click={() => (visible = !visible)}>Close modal</Button>
			<slot />
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
			max-height: 90vh;
			overflow: scroll;
			padding: 1rem;
			border-radius: 0.5rem;
			background: var(--secondary-color);
			position: absolute;
			top: 50%;
			left: 50%;
			transform: translate(-50%, -50%);
		}
	}
</style>
