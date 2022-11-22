<script lang="ts">
	import { mdiChevronLeft } from '@mdi/js';

	import Button from '$lib/components/Button.svelte';
	import Icon from '$lib/components/Icon.svelte';

	export let fullscreen = false;

	let foregroundWidth: number;
	let foregroundHeight: number;
</script>

<div class="page">
	<div class="backdrop" />
	<div class="background">
		<slot name="background" />
	</div>

	<div class="foreground" bind:clientWidth={foregroundWidth} bind:clientHeight={foregroundHeight}>
		<header>
			<slot name="headerButton">
				<Button on:click={() => history.back()}>
					<Icon path={mdiChevronLeft} color="var(--text-color)" />
					Back
				</Button>
			</slot>

			<slot name="options" />
		</header>

		<main class:fullscreen><slot {foregroundWidth} {foregroundHeight} /></main>

		<footer>
			<slot name="footer" />
		</footer>
	</div>
</div>

<style>
	.page {
		height: 100%;
		user-select: none;
		overflow: hidden;
		-moz-user-select: none;
		-khtml-user-select: none;
		-webkit-user-select: none;
		-o-user-select: none;
		touch-action: none;
	}

	.background {
		height: calc(100% - 1rem);
	}

	.backdrop {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background: var(--background-color);
		/* opacity: 0.5; */

		--tw-backdrop-blur: blur(2em);
		backdrop-filter: var(--tw-backdrop-blur);
		-webkit-backdrop-filter: blur(2em);
	}

	.foreground {
		position: absolute;
		background: coral;
		left: 50%;
		top: 50%;
		transform: translate(-50%, -50%);
		width: calc(100vw - 2rem);
		max-width: 60rem;
		height: calc(100vh - 5rem);
		height: calc(var(--vh) - 5rem);
		max-height: 50rem;
		border-radius: 1rem;
		border: 1px solid #ccc;
	}

	header {
		position: absolute;
		width: calc(100% - 1rem);
		top: -2rem;
		left: 0.5rem;
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
		z-index: 10;
	}

	:global(.page header button) {
		width: 15rem;
	}

	main {
		height: 100%;
		border-radius: 1rem;
		overflow: hidden;
	}

	.fullscreen {
		display: block;
		user-select: none;
		-moz-user-select: none;
		-khtml-user-select: none;
		-webkit-user-select: none;
		-o-user-select: none;
	}

	footer {
		position: absolute;
		width: calc(100% - 1rem);
		max-width: 30rem;
		margin: 0 auto;
		left: 50%;
		transform: translateX(-50%);
		bottom: -1.5rem;
		display: flex;
		gap: 0.5rem;
		flex-direction: column;
	}
</style>
