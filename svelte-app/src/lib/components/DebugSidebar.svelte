<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import type { Options } from '$lib/types/Options';
	import Input from '$lib/components/Input.svelte';

	let sidebarOpen = true;

	interface GeneralSettings {
		rows: number;
		columns: number;
		border_size: number;
		svc_distance: number;
		smoothing_settings: Object[]; // update type
		catmull_clark_iterations: number;
	}

	let generalSettingsOptions: Partial<Options<GeneralSettings>> = {
		rows: {
			default: 5,
			from: 1,
			to: 10,
			step: 1
		},
		columns: {
			default: 5,
			from: 1,
			to: 10,
			step: 1
		},
		border_size: {
			default: 5,
			from: 1,
			to: 10,
			step: 1
		},
		svc_distance: {
			default: 5,
			from: 1,
			to: 10,
			step: 1
		},
		smoothing_settings: {
			default: [],
			from: [],
			to: [],
			step: 1
		}
	};

	console.log(generalSettingsOptions);
</script>

{#if !sidebarOpen}
	<div class="openSidebar">
		<Button on:click={() => (sidebarOpen = !sidebarOpen)}>Open sidebar</Button>
	</div>
{/if}

<aside class="sideBar" class:sidebarOpen>
	<Button secondary on:click={() => (sidebarOpen = !sidebarOpen)}>Close sidebar</Button>

	<h3>General Settings</h3>
	{#each Object.entries(generalSettingsOptions) as [key, option]}
		<Input label={key} bind:value={generalSettingsOptions[key].default} />
		({key}, {option.default}, [{option.from}, {option.to}])
	{/each}
</aside>

<style lang="scss">
	.openSidebar {
		width: 20rem;
		max-width: calc(100vw - 2rem);
		right: 1rem;
		top: 0.75rem;
		position: fixed;
	}

	aside {
		box-sizing: border-box;
		position: fixed;
		top: 0;
		right: 0;
		margin: 1rem;
		padding: 0.5rem;
		width: min(30rem, 100vw - 2rem);
		max-width: 90vw;
		height: calc(max(100vh, var(--vh)) - 2rem);
		border-radius: 0.75rem;
		background: var(--primary-color);
		z-index: 10;
		transform: translateX(calc(100% + 2rem));
		transition: transform 0.5s ease-in-out;

		&.sidebarOpen {
			transform: translateX(0);
		}
	}
</style>
