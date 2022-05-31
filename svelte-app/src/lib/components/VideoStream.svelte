<script lang="ts">
	import { onDestroy, onMount } from 'svelte';

	let stream: MediaStream;
	let loading = true;
	let error: string;

	const obtenerVideoCamara = async () => {
		try {
			loading = true;
			stream = await navigator.mediaDevices.getUserMedia({
				video: {
					facingMode: 'environment'
				}
			});
			loading = false;
			error = '';
		} catch (err) {
			error = ('' + err) as string;
		}
	};

	onMount(async () => {
		await obtenerVideoCamara();
	});

	onDestroy(() => {
		if (!stream) return;

		stream.getTracks().forEach((track) => track.stop());
	});
</script>

<slot {loading} {stream} {error} />
