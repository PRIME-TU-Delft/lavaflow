<script lang="ts">
	export let instructions: { [key: string]: Instruction[] };

	interface Instruction {
		title: string;
		description: string;
		imagepath: string[];
	}
	[];
</script>

<input id="my-drawer" type="checkbox" class="drawer-toggle" />
<div class="drawer-side">
	<label for="my-drawer" class="drawer-overlay" />
	<ul class="menu p-4 w-80 h-full bg-base-200 text-base-content">
		<!-- Sidebar content here -->

		{#each Object.entries(instructions) as [title, instructionList]}
			<h2>{title}</h2>

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
	</ul>
</div>
