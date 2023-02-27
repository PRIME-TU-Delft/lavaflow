import preprocess from 'svelte-preprocess';
import adapter from '@sveltejs/adapter-auto';
import { preprocessThrelte } from '@threlte/preprocess';
import { vitePreprocess } from '@sveltejs/kit/vite';
import seqPreprocessor from 'svelte-sequential-preprocessor';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: seqPreprocessor([
		[
			vitePreprocess(),
			preprocess({
				postcss: true
			})
		],
		preprocessThrelte()
	]),

	kit: {
		adapter: adapter()
	}
};

export default config;
