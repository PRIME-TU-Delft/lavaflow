import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';

const config: UserConfig = {
	plugins: [wasmPack(['../wasm']), sveltekit()],
	ssr: {
		noExternal: ['three', 'troika-three-text']
	},
	optimizeDeps: {
		exclude: ['../wasm']
	},
	build: {
		target: 'es2020'
	} // TODO: Ask @juliavdkris for more info why es2020 is needed
};

export default config;
