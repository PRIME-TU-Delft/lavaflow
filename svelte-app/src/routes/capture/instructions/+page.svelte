<script>
	import { goto } from '$app/navigation';
	import { assets } from '$app/paths';
	import ActionButton from '$lib/components/ActionButton.svelte';
	import ActionMenu from '$lib/components/ActionMenu.svelte';
	import { contourLines } from '$lib/stores/contourLineStore';
	import { hc_curves, hc_hierarchy } from '$lib/stores/hardCoded';
	import sizeStore from '$lib/stores/sizeStore';
	import { mdiChevronRight } from '@mdi/js';
	import { Timeline, TimelineItem } from 'flowbite-svelte';

	//img paths
	const drawing = `${assets}/instructional-images/drawing/`;
	const scanning = `${assets}/instructional-images/scanning/`;

	export let drawingInstructions = [
		{
			title: 'A correctly drawn set of level curves',
			description:
				'Be as creative as you like! Just make sure not to make any of the following mistakes',
			imagepath: [drawing + 'correct.svg']
		},
		{
			title: 'Curves should not overlap',
			description:
				'The curves should have sufficient margin between them, overlapping or close lines are harder to interpret',
			imagepath: [drawing + 'overlapping_lines.svg']
		},
		{
			title: 'Draw with a dark pen',
			description:
				'To recognise your level curves, we prefer a dark and thick marker. Using a lighter pen will likely not work',
			imagepath: [drawing + 'line_visibility.svg']
		},
		{
			title: 'Close your curves',
			description:
				'Dont draw any open ended line. Open ended lines will be ignored or misinterpreted.',
			imagepath: [drawing + 'disconnected_line.svg']
		},
		{
			title: 'Draw in the box',
			description:
				'To make the next step easier, do not draw outside of the square shown on the page',
			imagepath: [drawing + 'correct_in_box.svg', drawing + 'out_of_box.svg']
		},
		{
			title: 'Do not draw loose ends',
			description:
				'Try to keep your curves neat, as having dangling lines can lead to unintended behaviour.',
			imagepath: [drawing + 'loose_ends.svg']
		}
	];

	export let scanningInstructions = [
		{
			title: 'A correct scan',
			description: 'Try to capture an image where your level curves are the focus of the image.',
			imagepath: [scanning + 'correct.svg']
		},
		{
			title: 'Orientation of the scan',
			description:
				'The orientation of your level curves in the camera field highly influences scanning. This example is not parallel, but will probably still work. For best the best result try to scan the paper as close to parallel as possible. This is easiest if you put your paper on a table instead of holding it. ',
			imagepath: [scanning + 'correct_tilted.svg']
		},
		{
			title: 'Four corners visible',
			description: 'To scan the level curves, we need to have the four corners of the page visible',
			imagepath: [scanning + 'out_of_frame.svg']
		},
		{
			title: 'Drawing should not be too far',
			description: 'Scanning works best from a distance of 0.5 meters from the piece of paper.',
			imagepath: [scanning + 'page_too_small.svg']
		},
		{
			title: 'Shadows on page',
			description:
				'Level curves are hard to distinguish from the background in a low light setting. Try to use natural daylight or a proper ceiling light for the best results. Be wary of sharp or dark shadows on the drawing, as they might be interpreted as part of your drawing.',
			imagepath: [scanning + 'shadow_on_page.svg']
		}
	];

	function continueWithDefaultMap() {
		const { curves, hierarchy } = { curves: hc_curves, hierarchy: hc_hierarchy };
		const [hc_width, hc_height] = [1000, 800];

		contourLines.setup({
			curves,
			hierarchy,
			size: { width: hc_width, height: hc_height }
		});

		sizeStore.set({ width: hc_width, height: hc_height });

		goto('/preview');
	}
</script>

<div class="prose mx-auto my-16 p-4">
	<h2>Capture instructions</h2>
	<Timeline>
		{#each drawingInstructions as instruction}
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

	<h2>Scan instructions</h2>
	<Timeline>
		{#each scanningInstructions as instruction}
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
</div>

<ActionMenu>
	<ActionButton secondary icon={mdiChevronRight} on:click={continueWithDefaultMap}>
		Start with prescanned image
	</ActionButton>
	<ActionButton href="/capture" icon={mdiChevronRight}>Start scanning (recommended)</ActionButton>
</ActionMenu>
