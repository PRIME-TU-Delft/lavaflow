import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
	plugins: [wasmPack(['../wasm']), sveltekit()],
	ssr: {
		noExternal: ['three']
	},
	optimizeDeps: {
		exclude: ['../wasm']
	},
	build: {
		target: 'es2020'
	} // TODO: Ask @juliavdkris for more info why es2020 is needed
});
