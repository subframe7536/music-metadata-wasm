import { spawnSync } from 'node:child_process'
import { rm } from 'node:fs/promises'

const outputDir = './dist'

export async function run(releaseMode) {
    try {
        await rm(outputDir, { recursive: true })
    } catch { }
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
}

run(process.argv[2] === '--release')
