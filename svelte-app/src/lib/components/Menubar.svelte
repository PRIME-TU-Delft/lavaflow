<script lang="ts">
	import { mdiChevronLeft } from '@mdi/js';
	import Button from './Button.svelte';

	export let title = '';
	export let back = '';
</script>

<svelte:head>
	<title>Lava flow: {title}</title>
</svelte:head>

<div class="navbar bg-base-100 absolute top-0 z-40 no-touch">
	<div class="flex-1 flex gap-4">
		{#if back}
			<a href={back}>
				<Button twClass="btn-sm" iconPrefix={mdiChevronLeft} outline>
					{#if $$slots.backTitle}
						<slot name="backTitle" />
					{:else}
						Back
					{/if}
				</Button>
			</a>
		{/if}
		{#if title}
			<div class="flex-1">
				<h1 class="text-xl text-secondary">{title}</h1>
			</div>
		{/if}
	</div>
	<div class="flex-none">
		<slot />
	</div>
</div>
