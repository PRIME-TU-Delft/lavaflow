<script lang="ts">
	export let title: string = '';
	export let bordered: boolean = false;
</script>

<section class="mx-auto max-w-4xl container p-4" class:bordered>
	{#if title}
		<h2 class="pb-4 text-xl text-red-900">
			{title}
		</h2>
	{/if}

	<slot />
</section>

<style lang="postcss">
	.bordered {
		@apply p-2 border-4 border-red-900 rounded;
	}
</style>
