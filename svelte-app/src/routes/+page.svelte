<script context="module" lang="ts">
	export const prerender = true;
</script>

<script>
	import { difficultyStore, cacheDifficultyStore } from '$lib/stores/difficultyStore';

	import NavigationButton from '$lib/components/NavigationButton.svelte';
	import Button from '$lib/components/Button.svelte';
	import Dropdown from '$lib/components/input/Dropdown.svelte';
	import MeetTheDevelopers from '$lib/components/MeetTheDevelopers.svelte';

	import { difficulty_modes } from '$lib/data/difficultyModes';
	import { onMount } from 'svelte';
	import { dev } from '$app/environment';
	import { craterLocations, targetLocations } from '$lib/stores/locationStore';
	import { contourLines } from '$lib/stores/contourLineStore';

	let header_height = 20;
	let main_height = 0;
	let landing_page_container_height = 0;
	let get_started_container_height = 0;
	let meet_developers_container_height = 0;
	let page_shift_top = 0;
	let copyright_height = 35;

	function initialize_scrolling_animation() {
		goto_landing_page();
	}

	function goto_landing_page() {
		header_height = 20;
		page_shift_top = 0;
		main_height = landing_page_container_height + copyright_height;
	}

	function goto_get_started_page() {
		header_height = 10;
		page_shift_top = -1;
		main_height = get_started_container_height + copyright_height;
	}

	function goto_meet_developers_page() {
		header_height = 10;
		page_shift_top = -2;
		main_height = meet_developers_container_height + copyright_height;
	}

	onMount(() => {
		difficultyStore.clear();

		if (dev) return;

		targetLocations.clear(); // clear target locations
		craterLocations.clear(); // clear crater locations
		contourLines.clear(); // clear contour lines
	});

	// Listen for the initialisation of the clientHeight of the landing page (first page) and then update accordingly.
	$: landing_page_container_height, initialize_scrolling_animation();
</script>

<header style="height:{header_height}rem;">
	<div class="logo tudelftLogo" />
	<div class="logo primelogo" />
	<div class="img" />
	<div class="img backdrop" />
	<div class="title">
		<h2>AR Lava Flow</h2>
	</div>
</header>

<main style="height:calc({main_height}px + {header_height}rem);">
	<div
		class="landing_page_container"
		style="margin-top:{header_height}rem;top:calc({page_shift_top} * ({landing_page_container_height}px + {header_height}rem)); "
		bind:clientHeight={landing_page_container_height}
	>
		<div class="introduction">
			The climate crisis is upon us! Lava tends to very precisely follow the steepest downwards
			direction, when it flows down the hills of a volcano. Its intense heat makes for a great
			opportunity to generate electricity for nearby cities. Your job is to predict where the lava
			will flow and place steam turbines on its paths. The steam turbines generate higher amounts of
			electricity as the lava reaches closer. Save the world by using this amazing sustainable
			energy-source!
		</div>

		<div class="get_started_button">
			<Button large on:click={goto_get_started_page}>Get started</Button>
		</div>
	</div>

	<div
		class="get_started_container"
		style="margin-top:{header_height}rem;top:calc({page_shift_top} * ({landing_page_container_height}px + {header_height}rem) + {page_shift_top +
			1} * 2 * {header_height}rem);"
		bind:clientHeight={get_started_container_height}
	>
		<Button secondary small on:click={goto_landing_page}>Back to starting page</Button>

		<div class="difficulty_container">
			<div class="difficulty_title">
				<h3>Select a difficulty mode</h3>
				<p>With increasing difficulty, you'll have to think harder to get your points.</p>

				{#if $difficultyStore}
					<div class="difficulty_description_container">
						<h4>{$difficultyStore.name}</h4>
						<p class="min_turbines">
							You must place at least
							<span class="min_turbines_bold">{$difficultyStore.min_steam_turbines}</span>
							steam turbines.
						</p>
						<p>{$difficultyStore.description}</p>
					</div>
				{/if}
			</div>

			{#if $difficultyStore}
				<Dropdown
					secondary={true}
					large={true}
					bind:value={$difficultyStore}
					options={difficulty_modes}
					let:option
					on:change={() => cacheDifficultyStore($difficultyStore)}
				>
					Difficulty: {option.name}
				</Dropdown>
			{/if}
		</div>

		<div class="start_flowing_button">
			<NavigationButton large to="scan/mapscanning">Start flowing</NavigationButton>
		</div>

		<div class="meet_the_developers">
			<p on:click={goto_meet_developers_page} on:keydown={goto_meet_developers_page}>
				Meet the developers
			</p>
		</div>
	</div>

	<div
		class="meet_developers_container"
		style="margin-top:{header_height}rem;top:calc({page_shift_top +
			1} * ({landing_page_container_height +
			get_started_container_height}px + 2*{header_height}rem)); "
		bind:clientHeight={meet_developers_container_height}
	>
		<Button secondary small on:click={goto_get_started_page}>Back to the game</Button>

		<MeetTheDevelopers />
	</div>
</main>

<div class="copyright" style="height:{copyright_height}px">
	<p>Copyright &copy; 2022 PRIME, TU Delft. All rights reserved.</p>
</div>

<style lang="scss">
	main {
		position: absolute;
		left: 0;
		top: 0;

		min-height: calc(var(--vh) - 2rem);

		overflow: hidden;

		display: block;
		width: calc(100vw - 2rem);
		max-width: 30rem;
		padding: 1rem;

		margin-left: 50vw;
		transform: translate(-50%, 0);
	}

	header {
		position: relative;
		// height: 20rem;
		width: 100vw;
		overflow: hidden;
		transition: height 1s ease-in-out;
		-webkit-transition: height 1s ease-in-out;
		z-index: 2;

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
		.tudelftLogo {
			position: absolute;
			top: 1.3rem;
			left: calc(max(1rem, 50vw - 17rem));
			background: url(/TU_P5_white.png);
			background-size: contain;
			background-repeat: no-repeat;
			width: 6rem;
			height: 4rem;
			z-index: 1000;
		}

		.primelogo {
			position: absolute;
			top: 1rem;
			left: calc(min(100vw - 1rem - 6rem, 50vw + 11rem));
			background: url(/primeLogo.svg);
			background-size: contain;
			background-repeat: no-repeat;
			width: 6rem;
			height: 3rem;
			z-index: 1000;
		}

		.backdrop {
			background: #dc494936;
			backdrop-filter: blur(1px) saturate(120%);
		}

		.title {
			position: absolute;
			width: 100vw;
			top: 50%;
			left: 50%;
			transform: translate(-50%, -50%);
			color: white;
			text-align: center;
			font-size: 1.5em;
		}
	}

	.landing_page_container {
		position: relative;
		height: auto;

		transition: top 1s ease-in-out, margin-top 1s ease-in-out, height 1s ease-in-out;
		-webkit-transition: top 1s ease-in-out, margin-top 1s ease-in-out, height 1s ease-in-out;

		.introduction {
			margin: 1rem 0;
			font-size: 1.1rem;
			line-height: 1.5rem;

			position: relative;
		}

		.get_started_button {
			position: relative;
		}
	}

	.get_started_container {
		position: relative;
		height: auto;

		transition: top 1s ease-in-out, margin-top 1s ease-in-out, height 1s ease-in-out;
		-webkit-transition: top 1s ease-in-out, margin-top 1s ease-in-out, height 1s ease-in-out;

		.difficulty_title {
			position: relative;
			margin-top: 2rem;
			margin-bottom: 3rem;
			top: 50%;
			left: 50%;
			transform: translate(-50%, 0);
			color: white;
			text-align: center;

			h3 {
				margin: 0;
			}

			h4 {
				margin: 0;
				margin-top: 2rem;
			}

			p {
				margin: 0;
			}

			p.min_turbines {
				font-family: 'Roboto Slab';
				font-weight: 200;

				margin-top: 10px;
				margin-bottom: 10px;
			}

			span.min_turbines_bold {
				font-weight: 400;
			}

			.difficulty_description_container {
				height: 6rem;
			}
		}

		.difficulty_container {
			position: relative;
		}

		.start_flowing_button {
			position: relative;
		}
	}

	.meet_the_developers {
		margin-top: 20px;

		p {
			color: #eee;
			text-decoration: underline;
			text-align: center;
		}

		p:hover {
			cursor: pointer;
		}
	}

	.meet_developers_container {
		position: relative;
		height: auto;

		transition: top 1s ease-in-out, margin-top 1s ease-in-out, height 1s ease-in-out;
		-webkit-transition: top 1s ease-in-out, margin-top 1s ease-in-out, height 1s ease-in-out;
	}

	.copyright {
		position: fixed;
		width: 100vw;

		background: #1a1717;

		left: 0;
		bottom: 0;

		p {
			margin: 0;
			padding: 0;

			font-family: 'Roboto Slab';
			font-weight: 200;
			font-size: 10pt;
			color: #999;

			width: 100vw;

			text-align: center;
		}
	}
</style>
