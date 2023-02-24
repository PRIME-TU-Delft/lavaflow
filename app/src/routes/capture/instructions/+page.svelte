<script>
	import LabeledButton from '$lib/components/LabeledButton.svelte';
	import Section from '$lib/components/Section.svelte';
	import { mdiChevronRight } from '@mdi/js';
	import { Accordion, AccordionItem, Button } from 'flowbite-svelte';

	export let drawingInstructions = [
		{
			title: 'Curves should have sufficient margin',
			description:
				'The curves should have sufficient margin between them, overlapping or close lines are hard to interpret'
		},
		{
			title: 'Drawing with dark pen',
			description:
				'To recognise your level curves, we prefer a dark and thick marker. Using a lighter pen will likely not work'
		},
		{
			title: 'Close your curves',
			description: 'To avoid false readings, close your curves, dont leave any open-ended lines'
		}
	];

	export let scanningInstructions = [
		{
			title: 'Four corners visible',
			description: 'To scan the level curves, we need to have the four corners visible'
		},
		{
			title: 'Drawing should not be too far',
			description:
				'Scanning works best from a distance of 0.5 meters from the piece of paper. For best the best result try to scan the paper as close to parallel as possible. This is easiest if you put your drawing on a table instead of holding it.'
		},
		{
			title: 'Proper lighting',
			description:
				'Level curves are hard to distinguish from the background in a low light setting. Try to use natural daylight or a propper ceiling light for the best results. Be careful of sharp or dark shadows on the drawing, as they might get interpreted as part of your drawing.'
		}
	];
</script>

<Section title="Capture instructions">
	<Accordion>
		<AccordionItem open>
			<span slot="header">Drawing</span>
			{#each drawingInstructions as instruction}
				<div class="instruction">
					<div class="text-gray-900">{instruction.title}</div>
					<div class="mb-4 text-sm text-gray-500 dark:text-gray-400">{instruction.description}</div>
				</div>
			{/each}
		</AccordionItem>
		<AccordionItem>
			<span slot="header">Scanning</span>
			{#each scanningInstructions as instruction}
				<div class="instruction">
					<div class="text-gray-900">{instruction.title}</div>
					<div class="mb-4 text-sm text-gray-500 dark:text-gray-400">{instruction.description}</div>
				</div>
			{/each}
		</AccordionItem>
	</Accordion>

	<div class="mt-2 flex justify-end">
		<a href="/capture">
			<LabeledButton icon={mdiChevronRight}>Start scanning</LabeledButton>
		</a>
	</div>
</Section>
