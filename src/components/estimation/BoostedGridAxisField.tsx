import { useEffect, useId, useState } from 'react'
import { field, fieldHint, fieldLabel } from '@/components/ui/recipes'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import type { NonEmptyArray } from '@/domain/dop'
import { describeBoostedGridProblem, formatBoostedGridAxis, parseBoostedGridAxis, type BoostedGridAxis, type BoostedGridProblem } from '@/domain/estimation'

interface BoostedGridAxisFieldProps {
  readonly axis: BoostedGridAxis
  readonly label: string
  readonly help: string
  readonly values: readonly number[]
  readonly onChange: (values: NonEmptyArray<number>) => void
}

/**
 * One axis of the boosted grid. A valid list reaches the configuration as it is typed; an invalid one
 * stays in the field with its reason, and leaving the field restores the values the run will search.
 */
export function BoostedGridAxisField({ axis, label, help, values, onChange }: BoostedGridAxisFieldProps) {
  const canonical = formatBoostedGridAxis(values)
  const [draft, setDraft] = useState(canonical)
  const [problem, setProblem] = useState<BoostedGridProblem | null>(null)
  const hint = useId()

  useEffect(() => setDraft(canonical), [canonical])

  return (
    <label className="block">
      <ParameterLabel className={fieldLabel} label={label} help={help} />
      <input
        type="text"
        inputMode="decimal"
        aria-label={label}
        aria-invalid={problem !== null}
        aria-describedby={hint}
        className={field('text', 'mt-1 w-full')}
        value={draft}
        onChange={(event) => {
          const next = event.target.value
          setDraft(next)
          const parsed = parseBoostedGridAxis(axis, next)
          if (parsed.ok) {
            setProblem(null)
            onChange(parsed.value)
          } else {
            setProblem(parsed.error)
          }
        }}
        onBlur={() => {
          setDraft(canonical)
          setProblem(null)
        }}
      />
      {problem !== null && <span id={hint} role="alert" className={fieldHint}>{describeBoostedGridProblem(axis, problem)}</span>}
    </label>
  )
}
