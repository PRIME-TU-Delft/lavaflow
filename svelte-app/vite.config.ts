import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';

const config: UserConfig = {
	plugins: [wasmPack(['../wasm']), sveltekit()],
	optimizeDeps: {
		exclude: ['../wasm']
	}
};

export default config;
