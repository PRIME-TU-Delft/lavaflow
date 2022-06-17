<script lang="ts">
	import { mdiChevronLeft } from '@mdi/js';

	import Button from '$lib/components/Button.svelte';
	import Icon from '$lib/components/Icon.svelte';

	export let title = '';
	export let fullscreen = false;

	let foregroundWidth: number;
	let foregroundHeight: number;
</script>

<div class="page">
	<div class="background">
		<slot name="background" />
		<div class="backdrop" />
	</div>

	<div class="foreground" bind:clientWidth={foregroundWidth} bind:clientHeight={foregroundHeight}>
		<header>
			<slot name="headerButton">
				<Button on:click={() => history.back()}>
					<Icon path={mdiChevronLeft} color="var(--text-color)" />
					Back
				</Button>
			</slot>
			<div class="title">
				{title}
			</div>

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
		height: 100vh;
		height: var(--vh);
		width: 100vw;
		overflow: hidden;
		display: grid;
		align-items: center;
		justify-items: center;
		user-select: none;
		-moz-user-select: none;
		-khtml-user-select: none;
		-webkit-user-select: none;
		-o-user-select: none;
		touch-action: none;
	}

	.background,
	.backdrop {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100vh;
		height: min(var(--vh), 100vh);
		z-index: -1;
	}
	.backdrop {
		background: rgba(255, 255, 255, 0.5);
		z-index: 1;
	}

	:global(.background > div) {
		height: 100%;
	}

	.foreground {
		position: relative;
		background: coral;
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

	.title {
		background: var(--secondary-color);
		color: white;
		width: 100%;
		max-width: 20rem;
		padding: 0.75rem 1.1rem;
		display: grid;
		align-items: center;

		font-size: 1rem;
		text-align: left;
		border: none;
		margin: 0 0 0 auto;
		border-radius: 0.3rem;
		cursor: pointer;
		user-select: none;
		-moz-user-select: none;
		-khtml-user-select: none;
		-webkit-user-select: none;
		-o-user-select: none;
	}

	main {
		height: 100%;
		border-radius: 1rem;
		overflow: hidden;
		display: grid;
		justify-content: center;
		align-items: center;
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

	/* if backdrop support: very transparent and blurred */
	@supports ((-webkit-backdrop-filter: none) or (backdrop-filter: none)) {
		.backdrop {
			-webkit-backdrop-filter: blur(2em);
			backdrop-filter: blur(2em);
		}
	}
</style>
