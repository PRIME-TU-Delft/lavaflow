<script lang="ts">
	import LabeledButton from '$lib/components/LabeledButton.svelte';
	import Section from '$lib/components/Section.svelte';
	import VStack from '$lib/components/VStack.svelte';
	import { mdiDownload } from '@mdi/js';
	import Header from './Header.svelte';
</script>

<Header />

<VStack>
	<Section>
		The climate crisis is upon us! Lava tends to very precisely follow the steepest downwards
		direction, when it flows down the hills of a volcano. Its intense heat makes for a great
		opportunity to generate electricity for nearby cities. Your job is to predict where the lava
		will flow and place steam turbines on its paths. The steam turbines generate higher amounts of
		electricity as the lava reaches closer. Save the world by using this amazing sustainable
		energy-source!
	</Section>

	<Section>
		<!-- TODO: onclick -->
		<LabeledButton icon={mdiDownload}>
			<p slot="label">
				Alongside this applet, a printable template was designed on which you can draw your
				level-curves.
			</p>
			Download template
		</LabeledButton>

		<!-- TODO: onclick -->
		{#if true}
			<LabeledButton icon={mdiDownload}>
				<p slot="label">
					We have detected that your device does not support webXR natively. The app is usable,
					however we would recommend you to download a special AR browser.
				</p>
				Download webXR browser
			</LabeledButton>
		{/if}

		<div class="pt-3">
			<a href="/capture/instructions">Start game</a>
		</div>
	</Section>

	<Section title="Credits" bordered />
</VStack>
