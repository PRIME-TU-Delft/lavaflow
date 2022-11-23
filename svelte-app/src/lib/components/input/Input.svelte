<script lang="ts">
	export let label: string;
	export let name: string | null = null;
	export let value: any; // @eslint-disable-line @typescript-eslint/no-explicit-any

	const initialValue = value;

	const genRand = (len: number) => {
		return Math.random()
			.toString(36)
			.substring(2, len + 2);
	};

	$: id = name || label || 'input' + '-' + genRand(8);
</script>

<div class="input">
	<label for={id}>
		{label} ({initialValue})
	</label>
	<input {id} {name} type="number" bind:value />

	{#if value != initialValue}
		<button on:click={() => (value = initialValue)}>Reset</button>
	{/if}
</div>

<style>
	.input {
		display: flex;
		gap: 1rem;
		margin-block: 0.5rem;
		align-items: center;
		padding: 1rem;
	}

	label {
		display: block;
		width: 20rem;
	}

	input {
		width: 100%;
		background: transparent;
		color: white;
		border: 1px solid var(--primary-color);
		padding: 0.5rem;
		border-radius: 0.25rem;
	}
</style>
