<script context="module" lang="ts">
	export const prerender = true;
</script>

<script>
	import { onMount } from 'svelte';
	import { difficultyStore } from '$lib/stores/difficultyStore';

	import NavigationButton from '$lib/components/NavigationButton.svelte';
	import Dropdown from '$lib/components/input/Dropdown.svelte';

	onMount(async () => {
		if ($difficultyStore) return; // When gltf store is loaded -> don't (re)load again

		await difficultyStore.setup();
		difficultyStore.build();
	});

	function updateDifficulty() {
		difficultyStore.set_difficulty_to($difficultyStore.name);
	}
</script>

<div class="tudelftLogo" />

<header>
	<div class="img" />
	<div class="img backdrop" />
	<div class="title">
		<div class="logo" />
		<h2>AR lavaFlow</h2>
	</div>
</header>

<main>
	<div class="introduction">
		Select a difficulty mode.
	</div>

	{#if $difficultyStore}
		<Dropdown secondary bind:value={$difficultyStore.name} options={$difficultyStore.options} let:option on:change={updateDifficulty}>
			Difficulty: {option}
		</Dropdown>
		<NavigationButton large to="scan/mapscanning">Start flowing</NavigationButton>
	{/if}
	<!-- <NavigationButton large to="/" secondary={true}>Teacher corner</NavigationButton> -->
</main>

<style lang="scss">
	.tudelftLogo {
		position: fixed;
		top: 1rem;
		left: 1rem;
		background: url(/TU_P5_white.png);
		background-size: contain;
		background-repeat: no-repeat;
		height: 4rem;
		width: 100%;
		z-index: 1000;
	}

	header {
		position: relative;
		height: 20rem;
		width: 100%;

		.logo {
			height: 6rem;
			background: url(/primeLogo.svg) no-repeat;
			background-size: cover;
		}

		h2 {
			font-size: 1.5em;
		}

		.img {
			position: absolute;
			bottom: 0;
			background: url(/volcano.jpg);
			background-size: cover;
			background-position: center bottom;
			left: 50%;
			transform: translateX(-50%);
			height: 40rem;
			width: 40rem;
			z-index: 0;
			border-radius: 50%;
		}

		.backdrop {
			background: #dc494936;
			backdrop-filter: blur(1px) saturate(120%);
		}

		.title {
			position: absolute;
			top: 50%;
			left: 50%;
			transform: translate(-50%, -50%);
			color: white;
			text-align: center;
			font-size: 1.5em;
		}
	}

	main {
		display: block;
		max-width: 30rem;
		margin: 1rem auto;
		padding: 1rem;
	}

	.introduction {
		margin: 1rem 0;
		font-size: 1.1rem;
		line-height: 1.5rem;
	}
</style>
