<script lang="ts">
	import { goto } from '$app/navigation';
	import ActionMenu from '$lib/components/ActionMenu.svelte';
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';
	import Menubar from '$lib/components/Menubar.svelte';
	import Video from '$lib/components/Video.svelte';
	import imageStore from '$lib/stores/imageStore';
	import sizeStore from '$lib/stores/sizeStore';
	import { Button, Chevron, Dropdown, DropdownItem } from 'flowbite-svelte';
	import type { PageData } from './$types';
	import CaptureMenu from './CaptureMenu.svelte';

	export let data: PageData;

	let deviceId = '';
	let loadingNextPage = false;

	function getCameraLabel(cameraOptions: MediaDeviceInfo[]) {
		if (cameraOptions.length == 0) return 'No camera found';

		const camera = cameraOptions.find((camera) => camera.deviceId === deviceId);
		return camera?.label || 'Select other camera';
	}

	function gotoTransform(videoSource: HTMLVideoElement | undefined) {
		loadingNextPage = true;

		if (!videoSource) return;

		// Create an atificial canvas element
		const canvas = document.createElement('canvas');
		const context = canvas.getContext('2d');
		canvas.height = videoSource.videoHeight;
		canvas.width = videoSource.videoWidth;

		// Take screenshot of video
		if (context) context.drawImage(videoSource, 0, 0, canvas.width, canvas.height);
		const image = canvas.toDataURL('image/png');

		canvas.remove();

		// Set image in (raw)image store
		imageStore.set(image);
		sizeStore.set({ width: canvas.width, height: canvas.height });

		goto('/select-markers');
	}
</script>

<Video bind:deviceId let:cameraOptions let:videoSource>
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

	<ActionMenu>
		{#if data.error}
			<ErrorMessage error={data.error} on:dismiss={() => goto('/capture')} />
		{/if}

		<CaptureMenu loading={loadingNextPage} on:click={() => gotoTransform(videoSource)} />
	</ActionMenu>
</Video>
