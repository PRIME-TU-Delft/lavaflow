<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import SceneViewer from '$lib/components/aframe/SceneViewer.svelte';

	import { mdiChevronLeft, mdiChevronRight } from '@mdi/js';

	import '@ar-js-org/ar.js';
</script>

<!-- Load scene in arMode -->
<SceneViewer arMode>
	<div class="headerButton" slot="overlay">
		<div class="button backButton">
			<!-- Custom back button -->
			<a sveltekit:reload href="/scan/preview">
				<Button icon={mdiChevronLeft}>Back to preview</Button>
			</a>
		</div>

		<div class="button placeTargets">
			<a sveltekit:reload href="/targetplacement">
				<Button>
					<span>Place targets</span>

					<Icon path={mdiChevronRight} />
				</Button>
			</a>
		</div>
	</div>
</SceneViewer>

<style>
	.button {
		position: absolute;
		width: 15rem;
		max-width: calc(50vw - 2rem);
		z-index: 1;
	}

	.backButton {
		top: 1rem;
		left: 1rem;
	}

	.placeTargets {
		top: 1rem;
		right: 1rem;
	}
</style>
