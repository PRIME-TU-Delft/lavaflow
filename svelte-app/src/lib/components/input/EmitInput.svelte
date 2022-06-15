<script lang="ts">
	/**
	 * This component extends input however it emits the value as a string instead as bind value
	 */
	import { createEventDispatcher } from 'svelte';

	import Input from '$lib/components/input/Input.svelte';

	type T = $$Generic;

	export let label: string;
	export let value: T;

	const dispatch = createEventDispatcher();

	$: value && dispatch('input', { value });
</script>

<Input {label} bind:value />
