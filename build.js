import { spawnSync } from 'node:child_process'
import { rm } from 'node:fs/promises'
import { build } from 'tsup'

const outputDir = './lib/metadata'

export async function run(releaseMode) {
    spawnSync(
        'wasm-pack',
        [
            'build',
            `--${releaseMode ? 'release' : 'dev'}`,
            '--out-name',
            'index',
            '--out-dir',
            outputDir,
            '--no-pack',
        ],
        { stdio: 'inherit' },
    )
    try {
        await rm(`${outputDir}/.gitignore`)
    } catch { }

    if (releaseMode) {
        await build({
            entry: ['./lib/index.ts'],
            dts: true,
            clean: true,
            format: ['esm'],
            loader: {
                '.wasm': 'file',
            },
        })
    }
}

run(process.argv[2] === '--release')
