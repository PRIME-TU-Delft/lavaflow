<script lang="ts">
	import Menubar from '$lib/components/Menubar.svelte';
	import Video from '$lib/components/Video.svelte';
	import { Button, Chevron, Dropdown, DropdownItem } from 'flowbite-svelte';

	let deviceId = '';

	function getCameraLabel(cameraOptions: MediaDeviceInfo[]) {
		if (cameraOptions.length == 0) return 'No camera found';

		const camera = cameraOptions.find((camera) => camera.deviceId === deviceId);
		return camera?.label || 'Select other camera';
	}
</script>

<Video bind:deviceId let:cameraOptions>
	<Menubar back="capture/instructions" title="Capture">
		{#if cameraOptions.length > 0}
			{#key deviceId}
				<Button outline color="red"><Chevron>{getCameraLabel(cameraOptions)}</Chevron></Button>
			{/key}
			<Dropdown>
				{#each cameraOptions as camera}
					<DropdownItem on:click={() => (deviceId = camera.deviceId)}>{camera.label}</DropdownItem>
				{/each}
			</Dropdown>
		{/if}
	</Menubar>
</Video>
