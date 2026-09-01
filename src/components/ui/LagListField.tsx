import { useEffect, useState } from 'react'
import { field, fieldHint, fieldLabel } from '@/components/ui/recipes'
import { err, isNonEmpty, ok, type NonEmptyArray, type Result } from '@/domain/dop'

export type LagListProblem =
  | { readonly kind: 'empty' }
  | { readonly kind: 'not-an-integer'; readonly token: string }
  | { readonly kind: 'outside-range'; readonly lag: number; readonly maximum: number }

export function parseLagList(text: string, maximum = 24): Result<NonEmptyArray<number>, LagListProblem> {
  const tokens = text.split(/[\s,]+/u).filter((token) => token.length > 0)
  if (!isNonEmpty(tokens)) return err({ kind: 'empty' })
  const parsed: number[] = []
  for (const token of tokens) {
    const lag = Number(token)
    if (!Number.isInteger(lag)) return err({ kind: 'not-an-integer', token })
    if (lag < 1 || lag > maximum) return err({ kind: 'outside-range', lag, maximum })
    if (!parsed.includes(lag)) parsed.push(lag)
  }
  parsed.sort((left, right) => left - right)
  return isNonEmpty(parsed) ? ok(parsed) : err({ kind: 'empty' })
}

function describeProblem(problem: LagListProblem): string {
  switch (problem.kind) {
    case 'empty': return 'Enter at least one lag.'
    case 'not-an-integer': return `“${problem.token}” is not a whole-number lag.`
    case 'outside-range': return `Lag ${problem.lag} is outside the supported range 1–${problem.maximum}.`
  }
}

interface LagListFieldProps {
  readonly label: string
  readonly lags: NonEmptyArray<number>
  readonly onChange: (lags: NonEmptyArray<number>) => void
  readonly maximum?: number
}

/**
 * Edits a non-empty, sorted set of positive lags. Invalid text remains local to the field, so the
 * scientific configuration can never contain an empty, duplicate, fractional, or out-of-range lag.
 */
export function LagListField({ label, lags, onChange, maximum = 24 }: LagListFieldProps) {
  const canonical = lags.join(', ')
  const [draft, setDraft] = useState(canonical)
  const [problem, setProblem] = useState<LagListProblem | null>(null)

  useEffect(() => setDraft(canonical), [canonical])

  return (
    <label className="block">
      <span className={fieldLabel}>{label}</span>
      <input
        type="text"
        inputMode="numeric"
        className={field('text', 'mt-1')}
        value={draft}
        aria-invalid={problem !== null}
        aria-describedby={`${label.replace(/\s+/gu, '-').toLowerCase()}-hint`}
        onChange={(event) => {
          const next = event.target.value
          setDraft(next)
          const parsed = parseLagList(next, maximum)
          if (parsed.ok) {
            setProblem(null)
            onChange(parsed.value)
          } else {
            setProblem(parsed.error)
          }
        }}
        onBlur={() => {
          const parsed = parseLagList(draft, maximum)
          if (parsed.ok) setDraft(parsed.value.join(', '))
        }}
      />
      <span id={`${label.replace(/\s+/gu, '-').toLowerCase()}-hint`} className={fieldHint}>
        {problem === null ? 'Comma-separated positive lags, for example 1, 7, 13.' : describeProblem(problem)}
      </span>
    </label>
  )
}
