<script lang="ts">
	/**
	 * This component is a dropdown input
	 */
	export let value: any;
	export let options: any[];
</script>

<select bind:value on:change>
	{#each options as option}
		<option value={option}>{option}</option>
	{/each}
</select>

<style>
	select {
		background: var(--primary-color);
		color: var(--text-color);
		width: 100%;
		padding: 0.75rem 1.1rem;

		font-size: 1rem;
		text-align: left;
		border: none;
		margin-block: 0.5rem;
		border-radius: 0.3rem;
		cursor: pointer;
	}
</style>
