<script>
	import { browser } from '$app/environment';
	import { onMount } from 'svelte';
	import '../app.postcss';
	import { fade } from 'svelte/transition';

	let darkMode = false;
	let loaded = false;

	onMount(() => {
		darkMode = browser && window?.matchMedia('(prefers-color-scheme: dark)').matches;

		const themeName = darkMode ? 'dark' : 'light';
		document.getElementsByTagName('html')[0].setAttribute('data-theme', themeName);

		window?.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
			darkMode = e.matches;

			const themeName = darkMode ? 'dark' : 'light';
			document.getElementsByTagName('html')[0].setAttribute('data-theme', themeName);
		});

		loaded = true;
	});
</script>

{#if loaded}
	<div in:fade>
		<slot />
	</div>
{/if}
