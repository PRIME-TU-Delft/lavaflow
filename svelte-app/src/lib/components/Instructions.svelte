<script lang="ts">
	import { instructions } from '$lib/data/instructions';
</script>

<div class="instructions">
	<h2>Good example of contour map</h2>
	<div class="example" />

	<h2>Instructions</h2>
	{#each instructions as instruction}
		<div class="instruction">
			<div class="title">{instruction.title}</div>
			<div class="description">{instruction.description}</div>
		</div>
	{/each}
</div>

<style lang="scss">
	.example {
		height: 15rem;
		background: url('/example.png') no-repeat center center;
		background-size: contain;
	}

	.instructions {
		padding: 3rem 1rem 1rem;
		color: var(--text-color);
		height: 100%;
		overflow: auto;
		max-width: 30rem;
		margin: 0 auto;

		.instruction {
			margin: 0.5rem auto;
			.title {
				font-size: 1.2rem;
			}
		}
	}
</style>
