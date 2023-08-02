<script lang="ts">
	import { Icon } from 'mdi-svelte-ts';
	import LabeledButton from './LabeledButton.svelte';

	export let icon = '';
	export let loading = false;
	export let secondary = false;
	export let disabled = false;
	export let href = '';
	export let twClass = '';
</script>

{#if loading}
	<LabeledButton outline={secondary}>
		<span class="loading loading-spinner loading-md" />
		<slot name="loading">Loading...</slot>
	</LabeledButton>
{:else}
	<LabeledButton {twClass} {href} outline={secondary} {disabled} on:click>
		<slot />

		{#if icon}
			<Icon path={icon} />
		{/if}
	</LabeledButton>
{/if}
