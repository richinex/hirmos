import type { BundleData } from '@/domain/bundle'

/** Base64 for the optional source payload, built in chunks so a large file does not spread the call stack. */
export async function encodeSourceFile(file: File): Promise<BundleData> {
  const bytes = new Uint8Array(await file.arrayBuffer())
  let binary = ''
  const chunk = 0x8000
  for (let offset = 0; offset < bytes.length; offset += chunk)
    binary += String.fromCharCode(...bytes.subarray(offset, offset + chunk))
  return {
    kind: 'source-file',
    name: file.name,
    mediaType: file.type,
    lastModified: file.lastModified,
    bytes: file.size,
    base64: btoa(binary),
  }
}

export function decodeSourceFile(data: Extract<BundleData, { kind: 'source-file' }>): File {
  const binary = atob(data.base64)
  const bytes = new Uint8Array(binary.length)
  for (let index = 0; index < binary.length; index += 1) bytes[index] = binary.charCodeAt(index)
  return new File([bytes], data.name, { type: data.mediaType, lastModified: data.lastModified })
}

/** Hand the reader a file; the object URL lives just long enough for the browser to take it. */
export function downloadBlob(name: string, blob: Blob): void {
  const url = URL.createObjectURL(blob)
  const anchor = document.createElement('a')
  anchor.href = url
  anchor.download = name
  anchor.click()
  window.setTimeout(() => URL.revokeObjectURL(url), 1000)
}

export function downloadText(name: string, text: string, mediaType = 'application/json'): void {
  downloadBlob(name, new Blob([text], { type: mediaType }))
}
