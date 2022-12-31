import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';

const config: UserConfig = {
	plugins: [wasmPack(['../wasm']), sveltekit()],
	optimizeDeps: {
		exclude: ['../wasm']
	},
	build: {
		target: 'es2020'
	}
};

export default config;
