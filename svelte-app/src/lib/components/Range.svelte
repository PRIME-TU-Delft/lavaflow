<script lang="ts">
	/**
	 * This component is a range slider that can be used to select a value
	 */

	export let label: string;
	export let value: number;
	export let from: number;
	export let to: number;
	export let step: number;

	const initialValue = value;

	const genRand = (len: number) => {
		return Math.random()
			.toString(36)
			.substring(2, len + 2);
	};

	$: id = label || 'input' + '-' + genRand(8);
</script>

<div class="input">
	<label for={id}>
		{label} ({initialValue})
	</label>
	<input {id} name={label} type="range" min={from} max={to} {step} bind:value />

	{#if value != initialValue}
		<button on:click={() => (value = initialValue)}>Reset</button>
	{/if}
</div>
