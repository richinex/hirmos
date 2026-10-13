// Data-Oriented Programming primitives, translated from the rust_dop reference and Kiehl's
// Data-Oriented Programming. The `dop-audit` skill checks code against DOP.md, which cites these.

/**
 * The compiler-enforced exhaustive `match`. End every `switch` over a discriminated union with
 * `default: return assertNever(x)`, so adding a variant fails to compile at every incomplete switch.
 */
export function assertNever(x: never): never {
  throw new Error(`Unhandled variant: ${JSON.stringify(x)}`)
}

/** The human message inside any thrown shape. Providers throw plain objects as well as Errors, and
 * `String(obj)` gives "[object Object]", which once reached a user-facing error box verbatim. Digs one
 * level in to catch the `{error: {message}}` envelope OpenAI uses, then falls back to JSON. */
export const errMessage = (e: unknown): string => {
  if (e instanceof Error) return e.message
  if (e && typeof e === 'object') {
    const o = e as Record<string, unknown>
    for (const k of ['message', 'error']) {
      const v = o[k]
      if (typeof v === 'string' && v) return v
      if (v && typeof v === 'object' && typeof (v as Record<string, unknown>).message === 'string') {
        return (v as Record<string, unknown>).message as string
      }
    }
    try { return JSON.stringify(e) } catch { return '[unstringifiable error object]' }
  }
  return String(e)
}

/** A fallible result as a tagged union. Never throw across a boundary; put failure in the type. */
export type Result<T, E> = { ok: true; value: T } | { ok: false; error: E }
export const Ok = <T>(value: T): Result<T, never> => ({ ok: true, value })
export const Err = <E>(error: E): Result<never, E> => ({ ok: false, error })

/** "At least one" encoded in the type, so `head` is total and length guards disappear. */
export type NonEmptyArray<T> = readonly [T, ...T[]]
export const isNonEmpty = <T>(a: readonly T[]): a is NonEmptyArray<T> => a.length > 0
export const head = <T>(a: NonEmptyArray<T>): T => a[0]

/**
 * Brand a validated string so a raw `string` cannot be passed where a domain value is required.
 * TypeScript is structural, so the phantom brand is what breaks `string === NodeId`. Type-only, so
 * it costs nothing at runtime.
 */
export type Brand<T, B extends string> = T & { readonly __brand: B }
export const brand = <B extends Brand<string, string>>(s: string): B => s as B
