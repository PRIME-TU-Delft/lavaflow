<script lang="ts">
	import { onDestroy, onMount } from 'svelte';

	export let deviceId: string = '';

	let stream: MediaStream;
	let loading = true;
	let error: string;
	let cameraOptions: MediaDeviceInfo[] = [];

	const obtainVideoCamara = async () => {
		loading = true;

		// If the are still active streams -> stop them
		if (stream) stream.getTracks().forEach((track) => track.stop());

		// If there is a deviceId -> get the stream | else -> get the default camera
		const constraints = deviceId
			? { deviceId: { exact: deviceId } }
			: { facingMode: 'environment' };

		try {
			stream = await navigator.mediaDevices.getUserMedia({ video: constraints });

			loading = false;
			error = '';
		} catch (err) {
			error = ('' + err) as string;
		}
	};

	// When devideId updated -> optain a new video stream
	$: deviceId && obtainVideoCamara();

	onMount(async () => {
		await obtainVideoCamara();

		// List all cameras
		const devices = await navigator.mediaDevices.enumerateDevices();
		cameraOptions = devices.filter((device) => device.kind == 'videoinput');
	});

	// When component is destoryed -> close all video streams
	onDestroy(() => {
		if (!stream) return;

		stream.getTracks().forEach((track) => track.stop());
	});
</script>

<slot {loading} {stream} {error} {cameraOptions} />
