<script>
	import Button from '$lib/components/Button.svelte';
	import Icon from '$lib/components/Icon.svelte';

	import { mdiChevronRight, mdiChevronLeft } from '@mdi/js';

	export let to = '/';
	export let secondary = false;
	export let large = false;
	export let disabled = false;
	export let back = false;
</script>

<a href={to}>
	<Button {disabled} {large} {secondary}>
		<div class="button">
			{#if back}
				<Icon path={mdiChevronLeft} color="var(--text-color)" />
			{/if}

			<div class="text">
				<slot />
			</div>

			{#if !back}
				<Icon path={mdiChevronRight} color="var(--text-color)" />
			{/if}
		</div>
	</Button>
</a>

<style>
	.button {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.text {
		width: 100%;
	}
</style>
