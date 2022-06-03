<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { mdiChevronLeft, mdiClose } from '@mdi/js';

	import Button from '$lib/components/Button.svelte';
	import Icon from '$lib/components/Icon.svelte';

	export let title = '';
	export let closeButton = false;

	let foregroundWidth: number;
	let foregroundHeight: number;

	const dispatch = createEventDispatcher();
</script>

<div class="page">
	<div class="background">
		<slot name="background" />
		<div class="backdrop" />
	</div>

	<div class="foreground" bind:clientWidth={foregroundWidth} bind:clientHeight={foregroundHeight}>
		<header>
			<slot name="headerButton">
				{#if closeButton}
					<Button on:click={() => dispatch('close')}>
						<Icon path={mdiClose} color="var(--text-color)" />
						Close
					</Button>
				{:else}
					<Button on:click={() => history.back()}>
						<Icon path={mdiChevronLeft} color="var(--text-color)" />
						Back
					</Button>
				{/if}
			</slot>
			<div class="title">
				{title}
			</div>
		</header>

		<main><slot {foregroundWidth} {foregroundHeight} /></main>

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
		padding-block: 1.5rem 1rem;
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
		backdrop-filter: blur(4rem);
		-webkit-backdrop-filter: blur(4rem);
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
		display: flex;
		justify-content: space-between;
		gap: 1rem;
		z-index: 10;
	}

	:global(.page header button) {
		width: initial;
	}

	.title {
		background: var(--secondary-color);
		color: white;
		width: 100%;
		max-width: 20rem;
		padding: 0.75rem 1.1rem;

		font-size: 1rem;
		text-align: left;
		border: none;
		margin-block: 0.5rem;
		border-radius: 0.3rem;
		cursor: pointer;
	}

	main {
		height: 100%;
		border-radius: 1rem;
		overflow: hidden;
		display: grid;
		justify-content: center;
		align-items: center;
	}

	footer {
		position: absolute;
		width: calc(100% - 1rem);
		max-width: 30rem;
		margin: 0 auto;
		left: 50%;
		transform: translateX(-50%);
		bottom: -1.5rem;
	}
</style>
