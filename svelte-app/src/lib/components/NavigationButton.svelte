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
		{#if back}
			<Icon path={mdiChevronLeft} color="var(--text-color)" />
		{/if}

		<span class="text">
			<slot />
		</span>

		{#if !back}
			<Icon path={mdiChevronRight} color="var(--text-color)" />
		{/if}
	</Button>
</a>

<style>
	.text {
		width: 100%;
	}
</style>
