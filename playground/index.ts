import { MetaFile } from '../dist'
import url from '../samples/flac.flac?url'
// import url from '../samples/wav.wav?url'
// import url from '../samples/mp3.mp3?url'
// impot url from '../samples/rate.mp3?url'

console.time('total')

const _ = await fetch(url).then(res => res.arrayBuffer())
const buffer = new Uint8Array(_)
const oldMetadata = new MetaFile(buffer)
oldMetadata.title = 'test'
oldMetadata.rate = '2'

oldMetadata.save()
const newBuffer = oldMetadata.buffer
oldMetadata.dispose()

const data = new MetaFile(newBuffer)
console.log(data.title)
console.log(data.rate)
document.querySelector('div')!.innerHTML = data.pictures?.[0].mimeType || 'no cover'

console.timeEnd('total')
