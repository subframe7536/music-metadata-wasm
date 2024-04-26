import { defineConfig } from 'tsup'

export default defineConfig({
    entry: ['./lib/index.ts'],
    dts: true,
    clean: true,
    format: ['esm'],
    minify: true,
    loader: {
        '.wasm': 'copy',
    },
})
