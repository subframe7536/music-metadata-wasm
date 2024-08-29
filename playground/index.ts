import { MetaFile } from '../dist/bundler'
import url from '../samples/flac.flac?url'
// import url from '../samples/wav.wav?url'
// import url from '../samples/mp3.mp3?url'
// impot url from '../samples/rate.mp3?url'

console.time('total')

const _ = await fetch(url).then(res => res.arrayBuffer())
const buffer = new Uint8Array(_)
const oldMetadata = new MetaFile(buffer)
oldMetadata.title = 'test title'
oldMetadata.artist = 'test artist'
oldMetadata.album = 'test album'
oldMetadata.year = 2022
oldMetadata.genre = 'test genre'
oldMetadata.comment = 'test comment'
oldMetadata.composer = 'test composer'
oldMetadata.albumArtist = 'test albumArtist'
oldMetadata.lyricist = 'test lyricist'
oldMetadata.disc = 1
oldMetadata.track = 1
oldMetadata.discTotal = 1
oldMetadata.trackTotal = 1


oldMetadata.save()
const newBuffer = oldMetadata.buffer
oldMetadata.dispose()

const data = new MetaFile(newBuffer)
console.table({
    title: data.title,
    artist: data.artist,
    album: data.album,
    year: data.year,
    genre: data.genre,
    comment: data.comment,
    composer: data.composer,
    albumArtist: data.albumArtist,
    lyricist: data.lyricist,
    disc: data.disc,
    track: data.track,
    discTotal: data.discTotal,
    trackTotal: data.trackTotal,
})
document.querySelector('div')!.innerHTML = data.pictures?.[0].mimeType || 'no cover'

console.timeEnd('total')
