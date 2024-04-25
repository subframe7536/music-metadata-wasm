import { spawnSync } from 'node:child_process'
import { rm } from 'node:fs/promises'

const outputDir = './dist'

export async function build(mode) {
    spawnSync(
        'wasm-pack',
        [
            'build',
            `--${mode}`,
            '--out-name',
            'metadata',
            '--out-dir',
            outputDir,
            '--no-pack'
        ],
        { stdio: 'inherit' }
    )
    await rm(`${outputDir}/.gitignore`)
}

process.argv[2] === 'release' ? await build('dev') : await build('release')