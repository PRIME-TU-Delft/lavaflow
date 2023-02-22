<script lang="ts">
	export let title: string = '';
	export let bordered: boolean = false;
</script>

<section class="mx-auto container p-4" class:bordered>
	{#if title}
		<h2 class="pb-4 text-xl text-red-900">
			{title}
		</h2>
	{/if}

	<slot />
</section>

<style>
	.bordered {
		@apply p-2 border-4 border-red-900 rounded;
	}
</style>
