import { MetaFile, type Picture } from './metadata'

export { MetaFile, Picture } from './metadata'

type AudioPropertyKeys = 'bitRate' | 'bitDepth' | 'channels' | 'duration' | 'sampleRate' | 'quality'
type AudioTagKeys = Exclude<keyof MetaFile, AudioPropertyKeys | 'buffer' | 'save' | 'free' | 'dispose'>

export function parseMetadata(buf: Uint8Array) {
    const metadata = new MetaFile(buf)

    return {
        tag: <T extends AudioTagKeys>(key: T) => metadata[key],
        property: <T extends AudioPropertyKeys>(key: T) => metadata[key],
        updateTag: <T extends AudioTagKeys>(key: T, value: MetaFile[T]) => {
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
 * convert {@link Picture} to URL with cleanup function
 * @param picture parsed picture
 */
export function getPictureURL(picture: Picture): [url: string, clean: VoidFunction] {
    const url = URL.createObjectURL(
        new Blob([picture.data.buffer], { type: picture.mimeType }),
    )
    return [url, () => URL.revokeObjectURL(url)]
}

/**
 * convert {@link Picture} to Base64
 * @param picture parsed picture
 */
export async function getPictureBase64(picture: Picture): Promise<string> {
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
