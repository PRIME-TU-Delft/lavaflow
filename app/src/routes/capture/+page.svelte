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
	import * as gm from 'gammacv'

	export let data: PageData;

	let deviceId = '';
	let loadingNextPage = false;

	function getCameraLabel(cameraOptions: MediaDeviceInfo[]) {
		if (cameraOptions.length == 0) return 'No camera found';

		const camera = cameraOptions.find((camera) => camera.deviceId === deviceId);
		return camera?.label || 'Select other camera';
	}

	async function gotoTransform(videoSource: HTMLVideoElement | undefined) {
		loadingNextPage = true;

		if (!videoSource) return;

		// Create an atificial canvas element
		const canvas = document.createElement('canvas');
		canvas.height = videoSource.videoHeight;
		canvas.width = videoSource.videoWidth;
		const context = canvas.getContext('2d');

		// Take screenshot of video
		if (context) context.drawImage(videoSource, 0, 0, videoSource.videoWidth, videoSource.videoHeight);

		const image = canvas.toDataURL('image/png');

		// Transform the image (from imageStore) into a gammacv tensor
		const gammacvInputTensor = await gm.imageTensorFromURL(image, "uint8", [videoSource.videoHeight, videoSource.videoWidth, 4])

		// const cannyEdgesOperation = gm.cannyEdges(
		// 	gm.sobelOperator(
		// 		// gm.gaussianBlur(
		// 			gm.norm(
		// 				gm.erode(gammacvInputTensor, [5, 5])
		// 			, "l2")
		// 		// , 3, 1)
		// 	), 0.25, 0.75
		// )

		const cannyEdgesOperation =
			gm.adaptiveThreshold(
				gm.erode(
					gm.norm(gammacvInputTensor, "l2")
				, [3, 3])
			, 15, 20)

		// const cannyEdgesOperation = gm.norm(gammacvInputTensor, "l2")

		// Extract the tensor output
		const gammacvOutputTensor: any = gm.tensorFrom(cannyEdgesOperation)

		// Create and initialize the GammaCV session, to acquire GPU acceperation
		const gammacvSession = new gm.Session()
		gammacvSession.init(cannyEdgesOperation)

		// Run the canny-edges operation
		gammacvSession.runOp(cannyEdgesOperation, 0, gammacvOutputTensor);

		gm.canvasFromTensor(canvas, gammacvOutputTensor)

		imageStore.set(canvas.toDataURL('image/png'))

		canvas.remove();

		// Set image in (raw)image store
		sizeStore.set({ width: videoSource.videoWidth, height: videoSource.videoHeight });

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

		<CaptureMenu
			loading={!data.error && loadingNextPage}
			on:click={() => gotoTransform(videoSource)}
		/>
	</ActionMenu>
</Video>
