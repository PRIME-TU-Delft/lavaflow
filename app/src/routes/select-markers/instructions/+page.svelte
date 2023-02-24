<script>
	import LabeledButton from '$lib/components/LabeledButton.svelte';
	import Section from '$lib/components/Section.svelte';
	import { mdiChevronLeft, mdiChevronRight } from '@mdi/js';
	import { Accordion, AccordionItem } from 'flowbite-svelte';

	export let dragInstructions = [
		{
			title: 'How to select',
			description:
				"Drag the shapes on your screen to the corresponding shapes on the template. Make sure that you don't leave part of your drawing outside the selected area."
		}
	];

	// TODO: add more error instructions for scanning
	export let posErrorInstructions = [
		{
			title: 'General',
			description:
				'If your scan does not work, you can move the shapes slightly, which could help. Scanning works best if the draggable shapes are on the inside of the rectangle formed by the shapes on the template, rather than outside or on top of it. If moving markers does not work, try taking a new picture.'
		}
	];
</script>

<Section title="Capture instructions">
	<Accordion>
		<AccordionItem open>
			<span slot="header">Dragging</span>
			{#each dragInstructions as instruction}
				<div class="instruction">
					<div class="text-gray-900">{instruction.title}</div>
					<div class="mb-4 text-sm text-gray-500 dark:text-gray-400">{instruction.description}</div>
				</div>
			{/each}
		</AccordionItem>
		<AccordionItem>
			<span slot="header">Error resolving</span>
			{#each posErrorInstructions as instruction}
				<div class="instruction">
					<div class="text-gray-900">{instruction.title}</div>
					<div class="mb-4 text-sm text-gray-500 dark:text-gray-400">{instruction.description}</div>
				</div>
			{/each}
		</AccordionItem>
	</Accordion>

	<div class="mt-2 flex justify-end gap-4">
		<LabeledButton outline href="/capture" iconPrefix={mdiChevronLeft}>Re-scan</LabeledButton>
		<LabeledButton href="/select-markers" icon={mdiChevronRight}>Start selecting</LabeledButton>
	</div>
</Section>
