<script lang="ts">
	import Input from '$lib/components/Input.svelte';

	interface Option {
		name: string;
		value: number;
		description: string;
	}

	export let options: Option[];
	export let name: string;
	export let description: string = '';
</script>

<div class="optionsList">
	<h3>{name}</h3>
	{#if description}
		<p>{description}</p>
	{/if}
	{#each options as option}
		<Input label={option.name} bind:value={option.value} />
	{/each}
</div>

<style>
	.optionsList {
		width: 40rem;
		margin: 0 auto;
	}
</style>
