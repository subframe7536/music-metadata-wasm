import { type MetaPicture, Metadata } from './metadata'

export { MetaPicture, Metadata } from './metadata'

type MetaKeys = Exclude<keyof Metadata, 'free' | 'save'>

export function parseMetadata(buf: Uint8Array) {
    const metadata = new Metadata(buf)

    return {
        get: <T extends MetaKeys>(prop: T) => metadata[prop],
        set: <T extends MetaKeys>(prop: T, value: Metadata[T]) => {
            metadata[prop] = value
        },
        save: () => metadata.save(),
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
    let reader = new FileReader()
    let promise = new Promise<string>(
        resolve => reader.onload = () => resolve(reader.result as string),
    )
    let type = 'application/octet-stream'
    let blob = new Blob([picture.data.buffer], { type })
    reader.readAsDataURL(blob)
    return (await promise).replace(type, picture.mimeType) as any
}
