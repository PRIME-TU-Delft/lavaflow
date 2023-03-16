import adapter from '@sveltejs/adapter-auto';
import netlifyAdapter from '@sveltejs/adapter-netlify';
import { vitePreprocess } from '@sveltejs/kit/vite';
import { preprocessThrelte } from '@threlte/preprocess';
import preprocess from 'svelte-preprocess';
import seqPreprocessor from 'svelte-sequential-preprocessor';

console.log('Building for: ' + process.env.BUILD_ENV === 'netlify' ? 'Netlify' : 'NodeJS');

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
		adapter: process.env.BUILD_ENV === 'netlify' ? netlifyAdapter() : adapter()
	}
};

export default config;
