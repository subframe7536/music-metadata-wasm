import { readTag, writeTag } from "../dist/metadata";
import url from "../samples/mp3.mp3?url";

const _ = await fetch(url).then((res) => res.arrayBuffer())
const buffer = new Uint8Array(_)
const oldMetadata = readTag(buffer)
oldMetadata.title = 'test'
const newBuffer = writeTag(buffer, oldMetadata)
const data = readTag(newBuffer)

document.querySelector('div')!.innerHTML = data.pictures?.[0].mimeType || 'no cover'
