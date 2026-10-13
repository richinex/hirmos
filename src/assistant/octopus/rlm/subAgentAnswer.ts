// What one sub-agent came back with.
//
// This was a plain string with the outcome encoded as a marker ("[sub-agent failed: …]"), which the
// consumer had to parse back and could not: a sub-agent asked "did the extraction fail?" answers "the
// sub-agent failed" and reads as a crash.
//
// Three cases rather than ok/error, because "ran fine and found nothing" is not a failure and needs its
// own handling. `lm_query` still returns text: a single call raises through the callback bridge, but a
// batch cannot raise for one item without losing the rest, so its failures have to be data.
import { assertNever } from '../dop'

export type SubAgentAnswer =
  | { readonly kind: 'answered'; readonly answer: string }
  | { readonly kind: 'nothing'; readonly reason: string }
  | { readonly kind: 'failed'; readonly error: string }

/** What crosses into Python. A tagged dict rather than a flattened one, so model code branches on
 *  `a["kind"]` instead of sniffing an answer for markers we invented. */
export interface PySubAgentAnswer {
  readonly kind: 'answered' | 'nothing' | 'failed'
  /** Present on 'answered'. */
  readonly answer?: string
  /** Present on 'nothing' and 'failed': why there is no answer. */
  readonly reason?: string
}

export function toPyAnswer(a: SubAgentAnswer): PySubAgentAnswer {
  switch (a.kind) {
    case 'answered': return { kind: 'answered', answer: a.answer }
    case 'nothing': return { kind: 'nothing', reason: a.reason }
    case 'failed': return { kind: 'failed', reason: a.error }
    default: return assertNever(a)
  }
}

/** For a caller that only wants text, such as a log line. Exhaustive, so a new case cannot be quietly
 *  rendered as an empty string. */
export function answerText(a: SubAgentAnswer): string {
  switch (a.kind) {
    case 'answered': return a.answer
    case 'nothing': return `(nothing found: ${a.reason})`
    case 'failed': return `(failed: ${a.error})`
    default: return assertNever(a)
  }
}

export const failed = (error: string): SubAgentAnswer => ({ kind: 'failed', error })
