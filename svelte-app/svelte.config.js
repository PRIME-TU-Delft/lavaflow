import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/kit/vite';
import { preprocessThrelte } from '@threlte/preprocess';
import preprocess from 'svelte-preprocess';
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
