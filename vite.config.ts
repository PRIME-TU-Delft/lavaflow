import { defineConfig } from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
	root: "src",
	build: {
		minify: false
	},
	plugins: [wasmPack(['./wasm'])]
});
