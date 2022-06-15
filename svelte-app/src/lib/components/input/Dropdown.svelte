<script lang="ts">
import { mdiChevronDown } from "@mdi/js";

	type T = $$Generic;

	/**
	 * This component is a dropdown input
	 */
	export let value: T;
	export let options: T[];
	export let secondary = false;
	export let large = false;
</script>

<select class:secondary class:large bind:value on:change>
	{#each options as option, index}
		<option value={option}>
			<slot {option} {index}>{option}</slot>
		</option>
	{/each}
</select>

<style>
	select {
		background: var(--primary-color);
		color: var(--text-color);
		width: 100%;
		padding: 0.75rem 1.1rem;

		font-family: "Roboto Slab";
		font-weight: 500;
		font-size: 1rem;
		text-align: left;
		border: none;
		margin-block: 0.5rem;
		border-radius: 0.3rem;
		cursor: pointer;
		position:relative;

		-webkit-appearance: none;
	}

	.secondary {
		background: var(--secondary-color);
		color: var(--text-color-secondary);
	}

	.large {
		padding: 1rem 1.5rem;
	}
</style>
