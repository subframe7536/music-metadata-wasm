# music-metadata-wasm

read / write the music file metadata using wasm, powered by [`lofty-rs`](https://github.com/Serial-ATA/lofty-rs)

## Install

```sh
npm install music-metadata-wasm
```
```sh
yarn add music-metadata-wasm
```
```sh
pnpm add music-metadata-wasm
```

## Usage

```ts
import { MetaFile } from 'music-metadata-wasm'
import url from '../samples/flac.flac?url'

console.time('total')

const _ = await fetch(url).then(res => res.arrayBuffer())
const buffer = new Uint8Array(_)
const oldMetadata = new MetaFile(buffer)
oldMetadata.title = 'test'

oldMetadata.save()
const newBuffer = oldMetadata.buffer
oldMetadata.dispose()

const data = new MetaFile(newBuffer)
console.log(data.title)
document.querySelector('div')!.innerHTML = data.pictures?.[0].mimeType || 'no cover'

console.timeEnd('total')
```

## License

AGPL-3.0-only