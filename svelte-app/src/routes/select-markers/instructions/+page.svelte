<script>
	import { assets } from '$app/paths';
	import ActionButton from '$lib/components/ActionButton.svelte';
	import ActionMenu from '$lib/components/ActionMenu.svelte';
	import LabeledButton from '$lib/components/LabeledButton.svelte';
	import { mdiChevronLeft, mdiChevronRight } from '@mdi/js';

	const path = `${assets}/instructional-images/select-markers/`;
	const alt = 'instructional image';

	export let dragInstructions = [
		{
			title: 'A correctly bordered drawing ',
			description:
				'Drag the shapes of the selection box to the shapes on the page. This will crop the image and apply transformations to compensate for any perspective. A preview of your transformed drawing can be seen at the top right of the "marker selection" page.',
			imagepath: [path + 'correct.svg']
		},
		{
			title: 'More correct Examples',
			description:
				'So long as your level curves are in the selection box, the shapes do not need to match exactly, nor do the corners need to align.',
			imagepath: [path + 'correct_but_smaller.svg', path + 'correct_but_rotated.svg']
		},
		{
			title: 'Include whole level curves',
			description:
				'Accidentally cutting off a section of your level curves will cause the conversion to a 3D model to fail.',
			imagepath: [path + 'selection_through_drawing.svg']
		},
		{
			title: 'Select only your level curves',
			description:
				'Accidentally selecting anything that is not a level curve (or does not form a closed shape) will cause the conversion to a 3D model to fail.',
			imagepath: [path + 'selection_off_page.svg']
		}
	];

	// TODO: add more error instructions for scanning
	export let posErrorInstructions = [
		{
			title: 'Check the preview',
			description:
				' A preview of your cropped and transformed drawing can be seen at the top right of the "marker selection" page. If it shows that you are accidentally removing part of your image or including things that are not part of your drawing, try adjusting the selection box untill this is no longer the case.'
		},
		{
			title: 'New capture',
			description:
				'If moving the corners of the selcetion box does not work, try taking a new picture.'
		}
	];
</script>

<div class="prose mx-auto my-16 p-4">
	<h2>Dragging instructions</h2>
	<!-- <Timeline>
		{#each dragInstructions as instruction}
			<TimelineItem title={instruction.title}>
				<div class="prose">
					<p>{instruction.description}</p>

					{#each instruction.imagepath as src, index}
						<img
							class="h-80 w-full object-contain"
							{src}
							alt="instructional image for {instruction.title} {index}"
						/>
					{/each}
				</div>
			</TimelineItem>
		{/each}
	</Timeline>

	<h2>Posible error resolve instructions</h2>
	<Timeline>
		{#each posErrorInstructions as instruction}
			<TimelineItem title={instruction.title}>
				<div class="prose">
					<p>{instruction.description}</p>
				</div>
			</TimelineItem>
		{/each}
	</Timeline> -->
</div>

<ActionMenu>
	<ActionButton secondary href="/capture" icon={mdiChevronLeft}>Re-capture image</ActionButton>
	<ActionButton href="/select-markers" icon={mdiChevronRight}>Continue selecting</ActionButton>
</ActionMenu>
