<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import SceneViewer from '$lib/components/aframe/SceneViewer.svelte';

	import { mdiChevronLeft, mdiChevronRight } from '@mdi/js';

	import '@ar-js-org/ar.js';
</script>

<!-- Load scene in arMode -->
<SceneViewer arMode>
	<!-- Custom back button that will reload the page-->
	<a slot="backButton" sveltekit:reload href="/scan/preview">
		<Button icon={mdiChevronLeft}>Back to preview</Button>
	</a>

	<!-- Custom target button -->
	<a slot="targetButton" sveltekit:reload href="/targetplacement">
		<Button>
			<span>Place targets</span>

			<Icon path={mdiChevronRight} />
		</Button>
	</a>
</SceneViewer>
