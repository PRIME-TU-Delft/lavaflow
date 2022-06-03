import preprocess from 'svelte-preprocess';
import adapter from '@sveltejs/adapter-static';
import wasmPack from 'vite-plugin-wasm-pack';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: preprocess(),

	kit: {
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			fallback: 'index.html'
		}),

		vite: {
			plugins: [wasmPack(['./../wasm'])],
			optimizeDeps: {
				exclude: ['./../wasm']
			}
		}
	}
};

export default config;
