import { MetaFile, type MetaPicture } from './metadata'

export { MetaFile } from './metadata'
export type { MetaPicture } from './metadata'

type AudioPropertyKeys = 'bitRate' | 'bitDepth' | 'channels' | 'duration' | 'sampleRate'
type AudioTagKeys = Exclude<keyof MetaFile, AudioPropertyKeys | 'buffer' | 'save' | 'free' | 'dispose'>

export function parseMetadata(buf: Uint8Array) {
    const metadata = new MetaFile(buf)

    return {
        tag: <T extends AudioTagKeys>(key: T) => metadata[key],
        property: <T extends AudioPropertyKeys>(key: T) => metadata[key],
        set: <T extends AudioTagKeys>(key: T, value: MetaFile[T]) => {
            metadata[key] = value
        },
        flush: () => {
            metadata.save()
            return metadata.buffer
        },
        dispose: () => metadata.dispose(),
    }
}

/**
 * convert {@link IParsedPicture} to URL with cleanup function
 * @param picture parsed picture
 */
export function getPictureURL(picture: MetaPicture): [url: string, clean: VoidFunction] {
    const url = URL.createObjectURL(
        new Blob([picture.data.buffer], { type: picture.mimeType }),
    )
    return [url, () => URL.revokeObjectURL(url)]
}

type MimeType = string
type Base64String = string

/**
 * convert {@link IParsedPicture} to Base64
 * @param picture parsed picture
 */
export async function getPictureBase64(
    picture: MetaPicture,
): Promise<`data:${MimeType};base64,${Base64String}`> {
    if (!picture.mimeType) {
        throw new Error('mimeType is empty')
    }
    let reader = new FileReader()
    let promise = new Promise<string>(
        resolve => reader.onload = () => resolve(reader.result as string),
    )
    let type = 'application/octet-stream'
    let blob = new Blob([picture.data.buffer], { type })
    reader.readAsDataURL(blob)
    return (await promise).replace(type, picture.mimeType) as any
}
