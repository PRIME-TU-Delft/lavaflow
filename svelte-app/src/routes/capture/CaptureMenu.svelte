<script>
	import ActionButton from '$lib/components/ActionButton.svelte';
	import { mdiHelp, mdiCamera } from '@mdi/js';
	import { Icon } from 'mdi-svelte-ts';

	export let loading = false;
</script>

<div class="flex w-full items-center justify-around gap-4 bg-red-50 p-10 md:rounded-lg md:p-4">
	<ActionButton secondary>
		<!-- Page content here -->
		<label for="my-drawer" class="drawer-button">
			<Icon path={mdiHelp} />
		</label>
	</ActionButton>

	<ActionButton {loading} on:click>
		<Icon path={mdiCamera} size={2} />
	</ActionButton>

	<ActionButton twClass="!cursor-default !p-4 opacity-0" />
</div>
