<script lang="ts">
	import { mdiChevronLeft } from '@mdi/js';
	import LabeledButton from './LabeledButton.svelte';

	export let title = '';
	export let back = '';
</script>

<nav class="fixed top-0 left-0 z-10 flex w-full items-center justify-between bg-red-50 p-4">
	<div class="flex items-center gap-4">
		{#if back}
			<LabeledButton iconPrefix={mdiChevronLeft} href={back} outline>
				<slot name="backTitle" />
			</LabeledButton>
		{/if}

		{#if title}
			<h1 class="text-xl text-red-900">{title}</h1>
		{/if}
	</div>

	<slot />
</nav>
