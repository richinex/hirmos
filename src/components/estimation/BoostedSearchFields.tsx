import type { ReactNode } from 'react'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { field, fieldHint, fieldLabel } from '@/components/ui/recipes'
import { boostedCandidateCount, type BoostedGridChoice } from '@/domain/estimation'
import { ESTIMATION_PARAMETER_HELP } from '@/domain/parameterHelp'
import { formatCount } from '@/lib/format/number'
import { BoostedGridAxisField } from './BoostedGridAxisField'

interface BoostedSearchFieldsProps {
  readonly grid: BoostedGridChoice
  readonly onChange: (grid: BoostedGridChoice) => void
  readonly seedHelp: string
  /** A control that belongs with the folds and the seed, such as how the chosen model scores the rows. */
  readonly scoring?: ReactNode
}

/** What the search tries, with the size of the search beside it, then how each candidate is scored. */
export function BoostedSearchFields({ grid, onChange, seedHelp, scoring }: BoostedSearchFieldsProps) {
  const update = (next: Partial<BoostedGridChoice>) => onChange({ ...grid, ...next })
  const candidates = boostedCandidateCount(grid)
  return (
    <div className="grid gap-4">
      <div>
        <div className="flex items-baseline justify-between gap-3">
          <span className={fieldLabel}>Search grid</span>
          <span className={fieldHint} role="status">{`${formatCount(candidates).text} candidate${candidates === 1 ? '' : 's'}`}</span>
        </div>
        <div className="mt-1 grid gap-3 @md/panel:grid-cols-3">
          <BoostedGridAxisField axis="learningRate" label="Learning rates" help={ESTIMATION_PARAMETER_HELP.propensity.learningRates} values={grid.learningRate} onChange={(learningRate) => update({ learningRate })} />
          <BoostedGridAxisField axis="maxDepth" label="Tree depths" help={ESTIMATION_PARAMETER_HELP.propensity.maxDepths} values={grid.maxDepth} onChange={(maxDepth) => update({ maxDepth })} />
          <BoostedGridAxisField axis="nEstimators" label="Tree counts" help={ESTIMATION_PARAMETER_HELP.propensity.nEstimators} values={grid.nEstimators} onChange={(nEstimators) => update({ nEstimators })} />
        </div>
      </div>
      <div>
        <span className={fieldLabel}>How each candidate is scored</span>
        <div className="mt-1 grid gap-3 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4">
          {scoring}
          <label className="block"><ParameterLabel className={fieldHint} label="Search folds" help={ESTIMATION_PARAMETER_HELP.propensity.splits} /><input type="number" min={2} max={20} aria-label="Search folds" className={field('text', 'mt-1 w-full')} value={grid.splits} onChange={(event) => update({ splits: Math.max(2, Math.min(20, Math.floor(Number(event.target.value) || 2))) })} /></label>
          <label className="block"><ParameterLabel className={fieldHint} label="Tree seed" help={seedHelp} /><input type="number" min={0} max={4294967295} aria-label="Tree seed" className={field('text', 'mt-1 w-full')} value={grid.seed} onChange={(event) => update({ seed: Math.max(0, Math.min(4294967295, Math.floor(Number(event.target.value) || 0))) })} /></label>
        </div>
      </div>
    </div>
  )
}
