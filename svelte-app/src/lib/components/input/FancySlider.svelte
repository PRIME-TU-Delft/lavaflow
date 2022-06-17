<script>
	export let value = 0;
	export let from = -180;
	export let to = 180;
	export let step = 10;
</script>

<div class="slider">
	<div><slot>Slide me</slot></div>
	<input type="range" bind:value min={from} max={to} {step} />
</div>

<style lang="scss">
	.slider {
		width: 20rem;
		max-width: 90vw;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		white-space: nowrap;
		border-radius: 0.5rem;
		padding: 0.5rem 1rem;

		background: #e15f55;
	}

	input[type='range'] {
		-webkit-appearance: none;
		width: 100%;
		background: transparent;
		display: block;
		border: none;
		position: relative;
		margin: 0;

		&:after {
			content: '';
			height: 1px;
			width: 100%;
			position: absolute;
			top: 50%;
			background: black;
			transform: translateY(-50%);
			border: 1px solid black;
			border-radius: 100vmax;
			z-index: 0;
		}

		&::-webkit-slider-thumb {
			-webkit-appearance: none;
			height: 1rem;
			width: 2rem;
			border-radius: 3px;
			background: url(https://upload.wikimedia.org/wikipedia/commons/thumb/f/fd/Braille_%C3%89.svg/86px-Braille_%C3%89.svg.png);
			background-size: contain;
			background-repeat: no-repeat;
			background-position: center center;
			border: 1px solid;
			padding: 1rem;
			cursor: pointer;
			z-index: 1;
		}
	}
</style>
