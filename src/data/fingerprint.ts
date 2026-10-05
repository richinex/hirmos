import { createSHA256 } from 'hash-wasm'
import { err, ok, type Result } from '@/domain/dop'
import { sourceFingerprint, type SourceFingerprint } from '@/domain/dataset'

export type FingerprintProblem = { readonly kind: 'fingerprint-failed'; readonly detail: string }

/** Stream the browser File through a maintained incremental SHA-256 implementation. The source is
 * never assembled into one ArrayBuffer, so fingerprinting does not turn a large file into a peak
 * allocation before DuckDB has had a chance to project or filter it. */
export async function fingerprintFile(
  file: File,
): Promise<Result<SourceFingerprint, FingerprintProblem>> {
  try {
    const hasher = await createSHA256()
    hasher.init()
    const reader = file.stream().getReader()
    while (true) {
      const chunk = await reader.read()
      if (chunk.done) break
      hasher.update(chunk.value)
    }
    const digest = hasher.digest('hex')
    if (typeof digest !== 'string') {
      return err({ kind: 'fingerprint-failed', detail: 'SHA-256 returned a non-text digest.' })
    }
    const parsed = sourceFingerprint(digest)
    return parsed.ok
      ? ok(parsed.value)
      : err({ kind: 'fingerprint-failed', detail: 'SHA-256 returned an invalid digest.' })
  } catch (cause) {
    return err({
      kind: 'fingerprint-failed',
      detail: cause instanceof Error ? cause.message : String(cause),
    })
  }
}
