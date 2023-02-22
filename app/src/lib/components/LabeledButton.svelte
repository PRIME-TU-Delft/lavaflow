<script lang="ts">
	import { Button } from 'flowbite-svelte';
	import { Icon } from 'mdi-svelte-ts';

	export let iconPrefix: string = '';
	export let icon: string = '';
</script>

<div>
	{#if $$slots.label}
		<div class="text-sm text-zinc-800 my-2">
			<slot name="label" />
		</div>
	{/if}

	<!-- TODO: implement own button -->
	<Button on:click>
		<p class="flex gap-2 items-center">
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
