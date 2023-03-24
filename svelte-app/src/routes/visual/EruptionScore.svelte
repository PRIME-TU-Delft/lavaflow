<script lang="ts">
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';
	import LavaError from '$lib/data/LavaError';

	export let score: number | null = null;
	let scoreMessage: string = 'You are a volcano god!';

	$: {
		if (score === null || score > 900) {
			scoreMessage = 'You are a volcano god!, you can bend the lava to your will';
		} else if (score > 800) {
			scoreMessage = 'You are a volcano master!, you have the power of the volcano';
		} else if (score > 700) {
			scoreMessage = 'You are a volcano expert!, who should we call when you are gone?';
		} else if (score > 600) {
			scoreMessage =
				'You are a volcano pro!, you know where people are safe and where they are not';
		} else if (score > 500) {
			scoreMessage =
				'You are a volcano enthusiast! You can do better by placing the tubines closer to the lava';
		} else if (score > 400) {
			scoreMessage =
				'You are a volcano beginner! You can do better by placing the tubines closer to the lava';
		} else if (score > 300) {
			scoreMessage =
				'You are a volcano novice! You can do better by placing the tubines closer to the lava';
		} else {
			scoreMessage =
				'You are a volcano rookie! You can do better by placing the tubines closer to the lava';
		}
	}
</script>

{#if score !== null}
	<ErrorMessage
		hasActions
		error={new LavaError(`You scored ${score}/1000 points`, scoreMessage)}
		on:dismiss
	>
		<slot />
	</ErrorMessage>
{/if}
