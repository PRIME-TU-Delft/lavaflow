<script lang="ts">
	import ActionButton from '$lib/components/ActionButton.svelte';
	import LabeledButton from '$lib/components/LabeledButton.svelte';
	import VStack from '$lib/components/VStack.svelte';
	import { mdiChevronDoubleRight, mdiDownload } from '@mdi/js';
	import { onMount } from 'svelte';
	import Header from './Header.svelte';
	import UserProfile from './UserProfile.svelte';

	onMount(() => {
		if (localStorage) {
			localStorage.clear();
		}
	});
</script>

<div class="logo-container">
	<a
		href="https://www.tudelft.nl/ewi/over-de-faculteit/afdelingen/applied-mathematics/studeren/prime"
		rel="noopener noreferrer"
		target="_blank"
		class="link left-4"
	>
		<img class="logo-left" src="PRIME logo tekst met symbolen web.png" alt="prime logo" />
	</a>

	<a href="https://www.tudelft.nl" rel="noopener noreferrer" target="_blank" class="link right-4">
		<img class="logo-right" src="TU Delft logo uitgelijnd web.png" alt="TU Delft logo" />
	</a>
</div>

<Header />

<VStack>
	<div class="prose mx-auto mt-12 p-8">
		The climate crisis is upon us! Lava tends to very precisely follow the steepest downwards
		direction, when it flows down the hills of a volcano. Its intense heat makes for a great
		opportunity to generate electricity for nearby cities. Your job is to predict where the lava
		will flow and place steam turbines on its paths. The steam turbines generate higher amounts of
		electricity as the lava reaches closer. Save the world by using this amazing sustainable
		energy-source!

		<div class="mt-4 flex justify-end">
			<ActionButton twClass="no-underline" href="/capture" icon={mdiChevronDoubleRight}>
				Start lava app
			</ActionButton>
		</div>
	</div>

	<div class="prose mx-auto mt-8 px-4">
		<h2>Instructions:</h2>
		<div class="not-prose rounded-xl bg-red-100 p-4">
			<LabeledButton href="/Template.pdf" outline icon={mdiDownload}>
				<div slot="label">
					Alongside this applet, a printable template was designed on which you can draw your
					level-curves.
				</div>
				Download template
			</LabeledButton>

			<!-- TODO: fix this condition -->
			{#if true}
				<LabeledButton
					href="https://apps.apple.com/nl/app/webxr-viewer/id1295998056"
					outline
					icon={mdiDownload}
				>
					<div class="mt-4" slot="label">
						We have detected that your device does not support webXR natively. The app is usable,
						however we would recommend you to download a special AR browser.
					</div>
					Download webXR browser
				</LabeledButton>
			{/if}
		</div>
	</div>

	<div class="prose mx-auto mt-12 mb-12 rounded-xl border-2 border-red-200 p-4">
		<h2>Credits</h2>

		<div class="">
			<UserProfile
				name="Abel de Bruijn"
				image="adebruijn.jpeg"
				title="Developer: User Interface and Augmented Reality"
			/>
			<UserProfile
				name="Rens Dur"
				image="rdur.jpeg"
				title="Developer: Model Construction and Image Recognition"
			/>
			<UserProfile name="Pauline Hengst" title="Developer: Model Construction and Model Graphics" />
			<UserProfile
				name="Julia van der Kris"
				image="jvanderkris.jpg"
				title="Developer: WebAssembly/deployment and Model Generation"
			/>
			<UserProfile
				name="Jonas van Marrewijk"
				image="jvanmarrewijk.jpeg"
				title="Developer emeritus: Image Recognition"
			/>
		</div>
	</div>
</VStack>

<style>
	.logo-container {
		position: absolute;
		left: 50vw;
		top: 0.5rem;

		transform: translateX(-50%);
		-webkit-transform: translateX(-50%);

		width: 67rem;
		height: 6rem;

		z-index: 5;
	}

	.logo-container img.logo-left {
		position: absolute;
		left: 0;
		top: 0;

		height: 100%;
	}

	.logo-container img.logo-right {
		position: absolute;
		right: 0;
		top: 0;

		height: 100%;
	}

	@media (max-width: 1130px) {
		.logo-container {
			position: relative;
			left: 50vw;
			top: 0.5rem;

			transform: translateX(-50%);
			-webkit-transform: translateX(-50%);

			width: 95vw;
			height: 4rem;

			z-index: 5;
		}

		.logo-container img.logo-left {
			position: absolute;
			left: 0;
			top: 0;

			height: 90%;
		}

		.logo-container img.logo-right {
			position: absolute;
			right: 0;
			top: 0;

			height: 90%;
		}
	}
</style>
