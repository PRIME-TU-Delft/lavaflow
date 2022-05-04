<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import Page from '$lib/components/Page.svelte';
	import VideoStream from '$lib/components/VideoStream.svelte';
	import Video from '$lib/components/Video.svelte';

	import { mdiAlertCircleOutline } from '@mdi/js';
	import { mdiBookOpenVariant } from '@mdi/js';
</script>

<VideoStream let:loading let:stream>
	<Page title="Map scanning">
		<div slot="background">
			<Video {loading} {stream} />
		</div>

		<Video style="border-radius: 1rem;" {loading} {stream}>
			<h1>loading...</h1>
		</Video>

		<div slot="footer">
			<Button secondary icon={mdiBookOpenVariant}>Read scan instructions</Button>
			<Button icon={mdiAlertCircleOutline}>
				<span>No keystones found</span>
				<span slot="content">
					To recognize the level curves, we need to have 3 markers visible
				</span>
			</Button>
		</div>
	</Page>
</VideoStream>
