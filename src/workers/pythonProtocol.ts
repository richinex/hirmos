import { z } from 'zod'
import { err, ok, type Result } from '@/domain/dop'

/**
 * The Python worker's protocol. The page sends a script with its inputs as CSV bytes; the worker
 * runs it under Pyodide with pandas and numpy and answers with the `prepared` table as CSV bytes.
 * CSV is the exchange format because pandas reads and writes it without pyarrow, which would add
 * ten megabytes to a runtime that already weighs twenty.
 */

export const PYODIDE_VERSION = '314.0.6'
/** Where the runtime and its wheels are fetched from; one constant, so moving them to R2 is one change. */
export const PYODIDE_INDEX_URL = `https://cdn.jsdelivr.net/pyodide/v${PYODIDE_VERSION}/full/`
export const PYTHON_PACKAGES = ['numpy', 'pandas'] as const

export type PythonCommand =
  | { readonly kind: 'start' }
  | { readonly kind: 'run'; readonly request: string; readonly code: string; readonly inputs: readonly Uint8Array[] }

export type PythonEvent =
  | { readonly kind: 'loading'; readonly detail: string }
  | { readonly kind: 'ready'; readonly python: string }
  | { readonly kind: 'start-failed'; readonly detail: string }
  | { readonly kind: 'ran'; readonly request: string; readonly prepared: Uint8Array; readonly rows: number; readonly columns: readonly string[]; readonly stdout: string }
  | { readonly kind: 'run-failed'; readonly request: string; readonly detail: string; readonly stdout: string }

const bytes = z.custom<Uint8Array>((value) => value instanceof Uint8Array)

export const pythonEventSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('loading'), detail: z.string() }),
  z.object({ kind: z.literal('ready'), python: z.string() }),
  z.object({ kind: z.literal('start-failed'), detail: z.string() }),
  z.object({ kind: z.literal('ran'), request: z.string(), prepared: bytes, rows: z.number().int().nonnegative(), columns: z.array(z.string()), stdout: z.string() }),
  z.object({ kind: z.literal('run-failed'), request: z.string(), detail: z.string(), stdout: z.string() }),
])

export const parsePythonEvent = (value: unknown): Result<PythonEvent, { readonly detail: string }> => {
  const parsed = pythonEventSchema.safeParse(value)
  return parsed.success ? ok(parsed.data) : err({ detail: parsed.error.message })
}
