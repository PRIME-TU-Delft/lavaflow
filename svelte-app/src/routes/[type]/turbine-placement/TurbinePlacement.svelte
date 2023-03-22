<script lang="ts">
	import P5PreviewCurves from '$lib/components/p5/P5PreviewCurves.svelte';
	import { contourLines } from '$lib/stores/contourLineStore';
	import { turbineLocations, craterLocation } from '$lib/stores/locationStore';
	import sizeStore from '$lib/stores/sizeStore';
	import Turbines from './Turbines.svelte';

	let { height: sHeight, width: sWidth } = $sizeStore;
	let [winHeight, winWidth] = [0, 0];
	let [cHeight, cWidth] = [0, 0];

	let overlayEl: HTMLDivElement;
	let offset: DOMRect;

	$: {
		if (winHeight + winWidth) {
			offset = overlayEl?.getBoundingClientRect();
		}
	}

	function placeDom(e: MouseEvent) {
		const x = (e.x - offset.left) / (cWidth / sWidth);
		const y = (e.y - offset.top) / (cHeight / sHeight);

		// If any turbine is open, close all and do not place new turbine
		const turbineIsOpen = $turbineLocations.find((t) => t.hasConfirmOpen == true);
		if (turbineIsOpen) {
			turbineLocations.closeAll();
			return;
		}

		// TODO: throw error
		if ($turbineLocations.length >= 10) return;

		// TODO: throw error | Check if turbine is at valid place
		if (x < 20 || x > sWidth - 20 || y < 20 || y > sHeight - 20) return;

		turbineLocations.add({ x, y, index: `${x.toFixed(2)}-${y.toFixed(2)}`, hasConfirmOpen: false });
	}
</script>

{#if $contourLines?.curves}
	{#key cHeight + cWidth}
		<div style="height:100%" bind:clientHeight={winHeight} bind:clientWidth={winWidth}>
			<!-- The overlay has the exact dimensions as the canvas and is the main source of interactivity -->
			<div
				bind:this={overlayEl}
				style="aspect-ratio: {sWidth}/{sHeight};"
				class="overlay"
				class:tall={winWidth / winHeight >= sWidth / sHeight}
				bind:clientHeight={cHeight}
				bind:clientWidth={cWidth}
				on:click={placeDom}
				on:keydown={() => console.log('key')}
			>
				<!-- TODO: get crater location from store -->
				<div
					on:click|stopPropagation
					on:keydown|stopPropagation
					class="crater"
					style="--pos-y: {($craterLocation[1] * cHeight) / sHeight}px;
								 --pos-x: {($craterLocation[0] * cWidth) / sWidth}px;"
				>
					Crater
				</div>

				<Turbines sizeMult={[cHeight / sHeight, cWidth / sWidth]} />
			</div>

			<P5PreviewCurves curves={$contourLines?.curves} />
		</div>
	{/key}
{/if}

<style lang="postcss">
	.overlay {
		--tw-shadow: 0 20px 25px 5px rgb(0 0 0 / 0.1), 0 -8px 10px 0 rgb(0 0 0 / 0.1) !important;
		@apply absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 overflow-hidden shadow-xl;
	}

	.overlay.tall {
		@apply h-full;
	}

	.overlay:not(.tall) {
		@apply w-full;
	}

	.crater {
		@apply absolute -translate-x-1/2 -translate-y-1/2 cursor-pointer select-none rounded-full bg-red-900 p-5 text-slate-100;
		left: var(--pos-x);
		top: var(--pos-y);
	}

	.crater:after {
		@apply absolute left-1/2 top-1/2 -translate-y-1/2 -translate-x-1/2 cursor-not-allowed rounded-full bg-red-900/30;
		content: '';
		width: 20vmin;
		height: 20vmin;
	}
</style>
