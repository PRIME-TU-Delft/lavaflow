<script lang="ts">
	import { onDestroy, onMount } from 'svelte';

	let stream: MediaStream;
	let loading = true;

	const obtenerVideoCamara = async () => {
		try {
			loading = true;
			stream = await navigator.mediaDevices.getUserMedia({
				video: {
					facingMode: 'environment'
				}
			});
			loading = false;
		} catch (error) {
			console.log(error);
		}
	};

	onMount(async () => {
		await obtenerVideoCamara();
		console.log('mounted', stream);
	});

	onDestroy(() => {
		if (!stream) return;

		stream.getTracks().forEach((track) => track.stop());
	});
</script>

<slot {loading} {stream} />
