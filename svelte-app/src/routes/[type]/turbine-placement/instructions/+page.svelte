<script>
	import { assets } from '$app/paths';
	import LabeledButton from '$lib/components/LabeledButton.svelte';
	import { mdiChevronRight } from '@mdi/js';
	import { Accordion, AccordionItem } from 'flowbite-svelte';

	const path = `${assets}/instructional-images/turbine-placement/`;

	export let dragInstructions = [
		{
			title: '',
			description: '',
			imagepath: path + 'click_for_turbine.svg'
		},
		{
			title: 'Click to add a turbine',
			description:
				'The amount of turbines you can place is determined by the difficulty level. Click on a turbine to remove it.',
			imagepath: path + 'click_result.svg'
		},
		{
			title: 'A turbine cannot be placed near to a crater',
			description: 'The radius around the crater is determined by the difficulty level.',
			imagepath: path + 'click_crater_radius.svg'
		},
		{
			title: 'A turbine cannot be placed near to another turbine',
			description: 'The radius around the turbines is determined by the difficulty level.',
			imagepath: path + 'click_turbine_radius.svg'
		}
	];
</script>

<div class="prose mx-auto mt-12 p-4">
	<h1>Turbine placement</h1>
	<Accordion class="not-prose">
		<AccordionItem open>
			<span slot="header">Turbine Placement</span>
			{#each dragInstructions as instruction}
				<div class="instruction">
					<img
						class="h-80 w-full object-contain"
						src={instruction.imagepath}
						alt="instructional image for {instruction.title}"
					/>
					<div class="text-gray-900">{instruction.title}</div>
					<div class="mb-4 text-sm text-gray-500 dark:text-gray-400">{instruction.description}</div>
				</div>
			{/each}
		</AccordionItem>
	</Accordion>

	<div class="not-prose mt-2 flex justify-end gap-4">
		<LabeledButton href="/ar/turbine-placement" icon={mdiChevronRight}>
			Place turbines
		</LabeledButton>
	</div>
</div>
