import { z } from 'zod'
import { err, ok, type Result } from '@/domain/dop'

/**
 * Send Arrow IPC files to Python and receive an Arrow IPC stream for the result.
 * The fixed format names prevent callers from confusing the two encodings.
 */

export const PYODIDE_VERSION = '314.0.6'
/** The runtime and its wheels live under the app's own origin: R2 behind a Pages function in production, the Pyodide CDN behind the dev server. */
export const PYODIDE_INDEX_URL = `${self.location.origin}/pyodide/${PYODIDE_VERSION}/`
export const PYTHON_PACKAGES = ['numpy', 'pandas', 'pyarrow'] as const

export interface PythonInput { readonly format: 'arrow-file'; readonly bytes: Uint8Array }
export interface PythonOutput { readonly format: 'arrow-stream'; readonly bytes: Uint8Array }

export type PythonCommand =
  | { readonly kind: 'start' }
  | { readonly kind: 'run'; readonly request: string; readonly code: string; readonly inputs: readonly PythonInput[] }

export type PythonEvent =
  | { readonly kind: 'loading'; readonly detail: string }
  | { readonly kind: 'ready'; readonly python: string }
  | { readonly kind: 'start-failed'; readonly detail: string }
  | { readonly kind: 'ran'; readonly request: string; readonly prepared: PythonOutput; readonly rows: number; readonly columns: readonly string[]; readonly stdout: string }
  | { readonly kind: 'run-failed'; readonly request: string; readonly detail: string; readonly stdout: string }

const bytes = z.custom<Uint8Array>((value) => value instanceof Uint8Array)

export const pythonEventSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('loading'), detail: z.string() }),
  z.object({ kind: z.literal('ready'), python: z.string() }),
  z.object({ kind: z.literal('start-failed'), detail: z.string() }),
  z.object({ kind: z.literal('ran'), request: z.string(), prepared: z.object({ format: z.literal('arrow-stream'), bytes }), rows: z.number().int().nonnegative(), columns: z.array(z.string()).min(1), stdout: z.string() }),
  z.object({ kind: z.literal('run-failed'), request: z.string(), detail: z.string(), stdout: z.string() }),
])

export const parsePythonEvent = (value: unknown): Result<PythonEvent, { readonly detail: string }> => {
  const parsed = pythonEventSchema.safeParse(value)
  return parsed.success ? ok(parsed.data) : err({ detail: parsed.error.message })
}
