<script>
	import Button from './Button.svelte';

	import { mdiChevronRight } from '@mdi/js';
	import { mdiChevronLeft } from '@mdi/js';
	import Icon from '$lib/components/Icon.svelte';

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

			<span class="text">
				<slot />
			</span>

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
		width: 100%;
	}

	.text {
		width: 100%;
	}
</style>
