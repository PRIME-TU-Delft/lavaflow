<script lang="ts">
	import { assets } from '$app/paths';
	import ActionButton from '$lib/components/ActionButton.svelte';
	import ActionMenu from '$lib/components/ActionMenu.svelte';
	import { mdiChevronRight } from '@mdi/js';
	import type { PageData } from './$types';

	const path = `${assets}/instructional-images/turbine-placement/`;

	export let data: PageData;

	export let dragInstructions = [
		{
			title: 'Click on an empty place to add a turbine',
			description: 'Click on a turbine again to remove it.',
			imagepath: [path + 'click_for_turbine.svg', path + 'click_result.svg']
		},
		{
			title: 'A turbine cannot be placed near a crater',
			description: '',
			imagepath: [path + 'click_crater_radius.svg']
		},
		{
			title: 'A turbine cannot be placed near another turbine',
			description: '',
			imagepath: [path + 'click_turbine_radius.svg']
		}
	];
</script>

<div class="prose mx-auto my-16 p-4">
	<h2>Turbine placement instructions</h2>
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
	</Timeline> -->
</div>

<ActionMenu>
	<ActionButton href="/{data.type}/turbine-placement" icon={mdiChevronRight}>
		Place turbines
	</ActionButton>
</ActionMenu>
