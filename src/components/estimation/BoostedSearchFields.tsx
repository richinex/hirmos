import type { ReactNode } from 'react'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { SettingsDisclosure, type SettingsSummaryItem } from '@/components/ui/SettingsDisclosure'
import { field, fieldLabel, fieldRow } from '@/components/ui/recipes'
import { boostedCandidateCount, type BoostedGridChoice } from '@/domain/estimation'
import { ESTIMATION_PARAMETER_HELP } from '@/domain/parameterHelp'
import { formatCount } from '@/lib/format/number'
import { BoostedGridAxisField } from './BoostedGridAxisField'

interface BoostedSearchFieldsProps {
  readonly grid: BoostedGridChoice
  readonly onChange: (grid: BoostedGridChoice) => void
  readonly seedHelp: string
  readonly scoring?: { readonly control: ReactNode; readonly summary: SettingsSummaryItem }
}

export function BoostedSearchFields({ grid, onChange, seedHelp, scoring }: BoostedSearchFieldsProps) {
  const update = (next: Partial<BoostedGridChoice>) => onChange({ ...grid, ...next })
  const candidates = boostedCandidateCount(grid)
  const items: readonly SettingsSummaryItem[] = [
    { icon: 'apps', text: `${formatCount(candidates).text} candidate${candidates === 1 ? '' : 's'}` },
    { icon: 'view_week', text: `${grid.splits} folds` },
    { icon: 'tag', text: `seed ${grid.seed}` },
    ...(scoring === undefined ? [] : [scoring.summary]),
  ]
  return (
    <SettingsDisclosure title="Search grid" items={items}>
      <div className={fieldRow.three}>
        <BoostedGridAxisField axis="learningRate" label="Learning rates" help={ESTIMATION_PARAMETER_HELP.propensity.learningRates} values={grid.learningRate} onChange={(learningRate) => update({ learningRate })} />
        <BoostedGridAxisField axis="maxDepth" label="Tree depths" help={ESTIMATION_PARAMETER_HELP.propensity.maxDepths} values={grid.maxDepth} onChange={(maxDepth) => update({ maxDepth })} />
        <BoostedGridAxisField axis="nEstimators" label="Tree counts" help={ESTIMATION_PARAMETER_HELP.propensity.nEstimators} values={grid.nEstimators} onChange={(nEstimators) => update({ nEstimators })} />
      </div>
      <div className={fieldRow.three}>
        {scoring?.control}
        <label className="block"><ParameterLabel className={fieldLabel} label="Search folds" help={ESTIMATION_PARAMETER_HELP.propensity.splits} /><input type="number" min={2} max={20} aria-label="Search folds" className={field('text', 'mt-1 w-full')} value={grid.splits} onChange={(event) => update({ splits: Math.max(2, Math.min(20, Math.floor(Number(event.target.value) || 2))) })} /></label>
        <label className="block"><ParameterLabel className={fieldLabel} label="Tree seed" help={seedHelp} /><input type="number" min={0} max={4294967295} aria-label="Tree seed" className={field('text', 'mt-1 w-full')} value={grid.seed} onChange={(event) => update({ seed: Math.max(0, Math.min(4294967295, Math.floor(Number(event.target.value) || 0))) })} /></label>
      </div>
    </SettingsDisclosure>
  )
}
