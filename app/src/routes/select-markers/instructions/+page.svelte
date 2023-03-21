<script>
	import { assets } from '$app/paths';
	import LabeledButton from '$lib/components/LabeledButton.svelte';
	import Section from '$lib/components/Section.svelte';
	import { mdiChevronLeft, mdiChevronRight } from '@mdi/js';
	import { Accordion, AccordionItem } from 'flowbite-svelte';

	const path = `${assets}/instructional-images/select-markers/`;
	const alt="instructional image";

	export let dragInstructions = [
		{
			title: 'A correctly bordered drawing ',
			description:
				'Drag the shapes of the selection box to the shapes on the page. This will crop the image and apply transformations to compensate for any perspective. A preview of your transformed drawing can be seen at the top right of the \"dragging\" page.',
			imagepath: path + "correct.svg"
		},
		{
			title: '',
			description:
				"",
			imagepath: path + "correct_but_smaller.svg"
		},
		{
			title: 'More correct Examples',
			description:
				"So long as your level curves are in the selection box, the shapes do not need to match exactly, nor do the corners need to align.",
			imagepath: path + "correct_but_rotated.svg"
		},
		{
			title: 'Include whole level curves',
			description:
				"Accidentally cutting off a section of your level curves will cause the conversion to a 3D model to fail.",
			imagepath: path + "selection_through_drawing.svg"
		},
		{
			title: 'Select only your level curves',
			description:
				"Accidentally selecting anything that is not a level curve (or does not form a closed shape) will cause the conversion to a 3D model to fail.",
			imagepath: path + "selection_off_page.svg"
		}

	];

	// TODO: add more error instructions for scanning
	export let posErrorInstructions = [
		{
			title: 'Check the preview',
			description:
				' A preview of your cropped and transformed drawing can be seen at the top right of the \"dragging\" page. If this shows that you are acidentally removing part of your image, or including things that are not part of your drawing, try adjusting the selection box.'
		},
		{
			title: 'General',
			description:
				'If moving markers does not work, try taking a new picture.'
		}
	];
</script>

<Section title="Capture instructions">
	<Accordion>
		<AccordionItem open>
			<span slot="header">Dragging</span>
			{#each dragInstructions as instruction}
				<div class="instruction">
					<img class="h-80 w-full object-contain" src={instruction.imagepath} {alt} />
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
