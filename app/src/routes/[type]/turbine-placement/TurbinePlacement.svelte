<script lang="ts">
	import P5PreviewCurves from '$lib/components/p5/P5PreviewCurves.svelte';

	import { contourLines } from '$lib/stores/contourLineStore';

	// import { craterLocations } from '$lib/stores/locationStore';
	import sizeStore from '$lib/stores/sizeStore';
	// import { difficultyStore } from '$lib/stores/difficultyStore';

	interface Turbine {
		x: number;
		y: number;
		index: string;
		hasConfirmOpen: boolean;
	}

	export let turbines: Turbine[] = [];

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

	function toggleConfirm(turbine: Turbine) {
		turbines = turbines.map((t) => {
			if (t.index === turbine.index) {
				return { ...t, hasConfirmOpen: !t.hasConfirmOpen };
			} else {
				return { ...t, hasConfirmOpen: false };
			}
		});
	}

	function removeTurbine(turbine: Turbine) {
		turbines = turbines.filter((t) => t.index !== turbine.index);
	}

	function placeDom(e: MouseEvent) {
		const x = (e.x - offset.left) / (cWidth / sWidth);
		const y = (e.y - offset.top) / (cHeight / sHeight);

		// Check if turbine is at valid place | TODO: throw error
		if (x < 20 || x > sWidth - 20 || y < 20 || y > sHeight - 20) return;

		turbines = [
			...turbines,
			{ x, y, index: `${x.toFixed(2)}-${y.toFixed(2)}`, hasConfirmOpen: false }
		];
	}
</script>

{#if $contourLines?.curves}
	<div style="height:100%" bind:clientHeight={winHeight} bind:clientWidth={winWidth}>
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
			<h1>{cHeight} {cWidth} {offset?.top} {offset?.left}</h1>

			<div
				on:click|stopPropagation
				on:keydown|stopPropagation
				class="crater"
				style="--pos-y: {(410 * cHeight) / sHeight}px;
								 --pos-x: {(670 * cWidth) / sWidth}px;"
			>
				Crater
			</div>

			{#each turbines as turbine, index}
				<div
					on:click|stopPropagation={() => toggleConfirm(turbine)}
					on:keydown={() => toggleConfirm(turbine)}
					class="turbine"
					class:top={turbine.hasConfirmOpen}
					style="--pos-y: {(turbine.y * cHeight) / sHeight}px;
								 --pos-x: {(turbine.x * cWidth) / sWidth}px;"
				>
					Turbine {index + 1}

					{#if turbine.hasConfirmOpen}
						<div class="mt-2 flex w-full gap-2 text-slate-900">
							<button
								class=" bg-red-200 p-2"
								on:click|stopPropagation={() => removeTurbine(turbine)}
							>
								Delete
							</button>
							<button class="bg-white p-2">Cancel</button>
						</div>
					{/if}
				</div>
			{/each}
		</div>

		<P5PreviewCurves curves={$contourLines?.curves} />
	</div>
{/if}

<style lang="postcss">
	.overlay {
		@apply absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 overflow-hidden;
	}

	.overlay.tall {
		@apply h-full bg-green-500/50;
	}

	.overlay:not(.tall) {
		@apply w-full;
	}

	.turbine,
	.crater {
		--size: 5vmin;
		@apply absolute -translate-x-1/2 -translate-y-1/2 cursor-pointer rounded-sm bg-slate-900 p-2 text-slate-100;
		left: calc(var(--pos-x));
		top: calc(var(--pos-y));
	}

	.turbine:before {
		@apply absolute left-1/2 top-1/2 -z-10 -translate-y-1/2 -translate-x-1/2 rounded-full bg-slate-900/30;
		content: '';
		width: 15vmin;
		height: 15vmin;
	}

	.crater {
		@apply rounded-full bg-red-900 p-5;
	}

	.crater:after {
		@apply absolute left-1/2 top-1/2 -translate-y-1/2 -translate-x-1/2 cursor-not-allowed rounded-full bg-red-900/30;
		content: '';
		width: 20vmin;
		height: 20vmin;
	}

	.turbine:hover {
		@apply scale-105 rounded-md bg-slate-700 transition-all;
	}

	.turbine.top {
		@apply z-10;
	}
</style>
