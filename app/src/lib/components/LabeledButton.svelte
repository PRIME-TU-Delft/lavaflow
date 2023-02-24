<script lang="ts">
	import { Button } from 'flowbite-svelte';
	import { Icon } from 'mdi-svelte-ts';

	export let iconPrefix: string = '';
	export let icon: string = '';
</script>

<div>
	{#if $$slots.label}
		<div class="my-2 text-sm text-zinc-800">
			<slot name="label" />
		</div>
	{/if}

	<!-- TODO: implement own button -->
	<Button color="red" on:click>
		<p class="flex items-center gap-2">
			{#if iconPrefix}
				<Icon path={iconPrefix} />
			{/if}

			<slot />

			{#if icon}
				<Icon path={icon} />
			{/if}
		</p>
	</Button>
</div>
