<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import { mdiChevronLeft } from '@mdi/js';
	// @ts-ignore
	import Icon from 'mdi-svelte';

	export let title = '';

	function goBack() {
		history.back();
	}
</script>

<div class="page">
	<div class="background">
		<slot name="background" />
		<div class="backdrop" />
	</div>

	<div class="foreground">
		<header>
			<slot name="headerButton">
				<Button on:click={goBack}>
					<Icon path={mdiChevronLeft} color="var(--text-color)" />
					Back</Button
				>
			</slot>
			<div class="title">
				{title}
			</div>
		</header>

		<main><slot /></main>

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
		height: var(--vh);
		z-index: -1;
	}
	.backdrop {
		backdrop-filter: blur(4rem);
		-webkit-backdrop-filter: blur(4rem);
		z-index: 1;
	}

	.foreground {
		position: relative;
		background: coral;
		width: calc(100vw - 2rem);
		max-width: 30rem;
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

	main {
		height: 100%;
		border-radius: 1rem;
		overflow: hidden;
	}

	footer {
		position: absolute;
		width: calc(100% - 1rem);
		left: 0.5rem;
		bottom: -1.5rem;
	}
</style>
