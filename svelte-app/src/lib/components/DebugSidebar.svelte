<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import EmitInput from '$lib/components/EmitInput.svelte';
	import Range from '$lib/components/Range.svelte';
	import Modal from '$lib/components/Modal.svelte';

	import SortableList from 'svelte-sortable-list';

	import type { Options, Entries } from '$lib/types/Options';
	import { hc_smooth_parameters, hc_smooth_parameter_types } from '$lib/data/hardCoded';
	import { SmoothParameters } from '$lib/data/smoothParameters';
	import Dropdown from './Dropdown.svelte';

	interface GeneralSettings {
		rows: number;
		columns: number;
		border_size: number;
		svc_distance: number;
		catmull_clark_iterations: number;
	}

	let sidebarOpen = true;
	let newSmoothModalOpen = false;
	let newSmoothParameterType: string = hc_smooth_parameter_types[0];
	let editSmoothModalOpen = false;
	let smoothParametersOpen: SmoothParameters;
	let indexEditing: number;

	let generalSettingsOptions: Options<GeneralSettings> = {
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
		catmull_clark_iterations: {
			default: 5,
			from: 1,
			to: 10,
			step: 1
		}
	};

	let smoothLayers: SmoothParameters[] = hc_smooth_parameters;
	function sortSmoothLayers(ev: any) {
		smoothLayers = ev.detail;
	}

	function openCreateModal() {
		newSmoothParameterType = hc_smooth_parameter_types[0];
		newSmoothModalOpen = true;
		// TODO
	}

	function addParameters() {
		smoothLayers = [...smoothLayers, smoothParametersOpen];
		newSmoothModalOpen = false;
		newSmoothParameterType = hc_smooth_parameter_types[0];
	}

	function removeParameter() {
		smoothLayers = smoothLayers.filter((_, i) => i !== indexEditing);
		editSmoothModalOpen = false;
	}

	/**
	 * Initialize the new smooth parameters
	 */
	function setNewParametersForType() {
		// If the `newSmoothParameterType` is not set
		if (newSmoothParameterType == hc_smooth_parameter_types[0]) return;

		smoothParametersOpen = SmoothParameters.create(newSmoothParameterType);

		console.log(smoothParametersOpen);
	}

	function openEditModalWith(smoothLayer: SmoothParameters, index: number) {
		indexEditing = index;
		editSmoothModalOpen = true;
		smoothParametersOpen = smoothLayer;
	}

	function setParameter(key: string, value: any) {
		smoothParametersOpen.setValue(key, value);

		smoothParametersOpen = smoothParametersOpen.clone();
	}

	function toggleBooleanParameter(key: string) {
		smoothParametersOpen.setValue(key, !smoothParametersOpen.getValue(key));
	}

	function editParametersForIndex() {
		smoothLayers[indexEditing] = smoothParametersOpen;
		editSmoothModalOpen = false;
	}

	function objToArr(obj: Options<GeneralSettings>) {
		return Object.entries(obj) as Entries<Options<GeneralSettings>>;
	}
</script>

{#if !sidebarOpen}
	<div class="openSidebar">
		<Button on:click={() => (sidebarOpen = !sidebarOpen)}>Open sidebar</Button>
	</div>
{/if}

<aside class="sideBar" class:sidebarOpen>
	<Button secondary on:click={() => (sidebarOpen = !sidebarOpen)}>Close sidebar</Button>

	<h3>General Settings</h3>
	{#each objToArr(generalSettingsOptions) as [key, option]}
		{#if 'from' in option}
			<Range
				label={key}
				from={option.from}
				to={option.to}
				step={option.step}
				bind:value={generalSettingsOptions[key].default}
			/>
		{:else}
			test: {key} {option}
		{/if}
	{/each}

	<SortableList list={smoothLayers} key="id" on:sort={sortSmoothLayers} let:item let:index>
		<Button on:click={() => openEditModalWith(item, index)}>
			{item.toString()}
		</Button>
	</SortableList>

	<Button secondary on:click={openCreateModal}>New smooth layer</Button>

	<Button secondary on:click={() => console.log('Regenerate model')}>Regenerate model</Button>
</aside>

<!-- EDIT SMOOTH ITEM-->
<Modal bind:visible={editSmoothModalOpen}>
	{#each smoothParametersOpen.toArray() as [key, value]}
		{#if typeof value == 'number'}
			<EmitInput label={key} {value} on:input={(e) => setParameter(key, e.detail.value)} />
		{:else if typeof value == 'boolean'}
			<Dropdown options={[true, false]} {value} on:change={() => toggleBooleanParameter(key)} />
		{/if}
	{/each}

	<Button secondary on:click={removeParameter}>Remove smooth layer</Button>
	<Button on:click={editParametersForIndex}>Set parameters</Button>
</Modal>

<!-- CREATE NEW SMOOTH ITEM -->
<Modal bind:visible={newSmoothModalOpen}>
	<Dropdown
		options={hc_smooth_parameter_types}
		bind:value={newSmoothParameterType}
		on:change={setNewParametersForType}
	/>

	{#if newSmoothParameterType != 'None' && smoothParametersOpen?.toArray()}
		{#each smoothParametersOpen.toArray() as [key, value]}
			{#if typeof value == 'number'}
				<EmitInput label={key} {value} on:input={(e) => setParameter(key, e.detail.value)} />
			{:else if typeof value == 'boolean'}
				<Dropdown options={[true, false]} {value} on:change={() => toggleBooleanParameter(key)} />
			{/if}
		{/each}

		<Button on:click={addParameters}>Set parameters</Button>
	{/if}
</Modal>

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
