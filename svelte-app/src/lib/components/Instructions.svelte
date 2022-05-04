<script lang="ts">
	interface Instruction {
		title: string;
		description: string;
	}

	let instructions: Instruction[] = [
		{
			title: '3 markers visible',
			description: 'To recognize the level curves, we need to have 3 markers visible'
		},
		{
			title: '3 markers visible',
			description: 'To recognize the level curves, we need to have 3 markers visible'
		},
		{
			title: '3 markers visible',
			description: 'To recognize the level curves, we need to have 3 markers visible'
		},
		{
			title: '3 markers visible',
			description: 'To recognize the level curves, we need to have 3 markers visible'
		},
		{
			title: '3 markers visible',
			description: 'To recognize the level curves, we need to have 3 markers visible'
		}
	];
</script>

<div class="instructions">
	{#each instructions as instruction}
		<div class="instruction">
			<div class="title">{instruction.title}</div>
			<div class="description">{instruction.description}</div>
		</div>
	{/each}
</div>

<style lang="scss">
	.instructions {
		padding: 3rem 1rem 1rem;
		color: vat(--text-color);
		display: flex;
		flex-direction: column;
		gap: 1rem;
		height: 100%;
		overflow: auto;

		.instruction .title {
			font-size: 1.2rem;
		}
	}
</style>
