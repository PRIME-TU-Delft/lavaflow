<script lang="ts">
	type T = $$Generic;

	export let items: T[] = [];
	export let title: string = 'Click';
</script>

<div class="dropdown">
	<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
	<label for="dropdown" tabindex="0" class="btn m-1">{title}</label>
	<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
	<ul
		tabindex="0"
		id="dropdown"
		class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52"
	>
		{#each items as item, index}
			<slot {item} {index}>
				<li>{item}</li>
			</slot>
		{/each}
	</ul>
</div>
