<script lang="ts">
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';
	import LavaError from '$lib/data/LavaError';

	export let score: number | null = null;
	let scoreTitle: string = 'You are a volcano god!';

	$: {
		if (score === null || score > 900) {
			scoreTitle = 'You are a volcano god!';
		} else if (score > 800) {
			scoreTitle = 'You are a volcano master!';
		} else if (score > 700) {
			scoreTitle = 'You are a volcano expert!';
		} else if (score > 600) {
			scoreTitle = 'You are a volcano pro!';
		} else if (score > 500) {
			scoreTitle = 'You are a volcano enthusiast!';
		} else if (score > 400) {
			scoreTitle = 'You are a volcano beginner!';
		} else if (score > 300) {
			scoreTitle = 'You are a volcano novice!';
		} else if (score > 200) {
			scoreTitle = 'You are a volcano rookie!';
		}
	}
</script>

{#if score !== null}
	<ErrorMessage error={new LavaError(scoreTitle, `You scored ${score}/1000 points`)}>
		<slot />
	</ErrorMessage>
{/if}
