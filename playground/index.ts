import { parseMetadata } from '../lib'
import url from '../samples/flac.flac?url'
// import url from '../samples/mp3.mp3?url'

console.time('total')

const _ = await fetch(url).then(res => res.arrayBuffer())
const buffer = new Uint8Array(_)
const oldMetadata = parseMetadata(buffer)
oldMetadata.updateTag('title', 'test')

const newBuffer = oldMetadata.flush()
oldMetadata.dispose()

const data = parseMetadata(newBuffer)
console.log(data.tag('title'))
document.querySelector('div')!.innerHTML = data.tag('pictures')?.[0].mimeType || 'no cover'

console.timeEnd('total')
