import { defineConfig } from 'tsup'

export default defineConfig({
    entry: ['./lib/index.ts'],
    dts: true,
    clean: true,
    format: ['esm'],
    loader: {
        '.wasm': 'copy',
    },
})
