import { parseMetadata } from '../lib'
import url from '../samples/flac.flac?url'
// import url from '../samples/mp3.mp3?url'

const _ = await fetch(url).then(res => res.arrayBuffer())
const buffer = new Uint8Array(_)
const oldMetadata = parseMetadata(buffer)
oldMetadata.set('title', 'test')
const data = parseMetadata(oldMetadata.flush())
console.log(data.get('title'))
document.querySelector('div')!.innerHTML = data.get('pictures')?.[0].mimeType || 'no cover'
