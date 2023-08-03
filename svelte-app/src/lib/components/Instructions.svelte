<script lang="ts">
	import { onMount } from 'svelte';
	import Button from './Button.svelte';

	export let title: string;
	export let instructions: { [key: string]: Instruction[] };

	let drawerToggle: HTMLInputElement;

	interface Instruction {
		title: string;
		description: string;
		imagepath: string[];
	}

	function closeNextTime() {
		drawerToggle.checked = false;

		localStorage.setItem('hideInstructions' + title, 'true');
	}

	onMount(() => {
		if (localStorage.getItem('hideInstructions' + title) === 'true') {
			drawerToggle.checked = false;
		} else {
			drawerToggle.checked = true;
		}
	});
</script>

<input bind:this={drawerToggle} id="my-drawer" type="checkbox" class="drawer-toggle" />
<div class="drawer-content">
	<slot />
</div>

<div class="drawer-side z-50">
	<label for="my-drawer" class="drawer-overlay" />
	<ul
		class="menu z-50 flex-nowrap p-4 md:w-96 h-full overflow-x-hidden overflow-y-auto bg-base-200 text-base-content"
	>
		<li class="prose">
			{#each Object.entries(instructions) as [title, instructionList]}
				<h2 class="menu-title text-2xl">{title}</h2>

				<div class="flex flex-col gap-4">
					{#each instructionList as instruction, idx}
						<div class="card card-compact max-w-xl bg-base-300 shadow-xl">
							<div class="card-body">
								<h3 class="card-title">{idx + 1}. {instruction.title}</h3>
								<p>{instruction.description}</p>
							</div>

							{#each instruction.imagepath as src, index}
								<figure class="bg-white m-0">
									<img
										class="h-80 w-full object-contain"
										{src}
										alt="instructional image for {instruction.title} {index}"
									/>
								</figure>
							{/each}
						</div>
					{/each}
				</div>
			{/each}
		</li>

		<li>
			<div class="join">
				<Button twClass="btn-secondary" on:click={closeNextTime}>Close and hide next time</Button>
				<label for="my-drawer" class="drawer-button btn btn-primary flex flex-nowrap">
					<span>Close</span>
				</label>
			</div>
		</li>
	</ul>
</div>
