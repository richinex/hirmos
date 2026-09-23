import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { SelectionActions } from '@/components/ui/SelectionActions'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { field, fieldLabel } from '@/components/ui/recipes'
import type { ColumnId } from '@/domain/dataset'
import type { PanelInterventionConfiguration } from '@/domain/estimation'
type Configuration = Extract<PanelInterventionConfiguration, { primary: 'adjusted' }>
export function AdjustedDidControls({ configuration, candidates, onChange }: {
  readonly configuration: Configuration
  readonly candidates: readonly { readonly id: ColumnId; readonly name: string }[]
  readonly onChange: (value: Configuration) => void
}) {
  const spec = configuration.specification
  return <div className="grid gap-3">
    <div>
      <div className="flex items-center justify-between gap-2">
        <ParameterLabel className={fieldLabel} label={spec.kind === 'regression' ? 'Covariates' : 'Baseline covariates'} help={spec.kind === 'regression' ? 'Regression uses the selected measurements in each of the two periods. Avoid adjusting for variables caused by treatment.' : 'DR DiD uses only each unit’s pre-treatment measurements, with paired outcome differences. It fits linear outcome regression and unpenalized logistic propensity models.'} />
        <SelectionActions selectLabel="Select all DiD covariates" clearLabel="Clear DiD covariates" onSelectAll={() => onChange({ ...configuration, covariates: candidates.map(c => c.id) })} onClear={() => onChange({ ...configuration, covariates: [] })} />
      </div>
      <div role="group" aria-label="DiD covariates" className="mt-1 flex flex-wrap gap-2">
        {candidates.map(c => <label key={c.id} className="flex items-center gap-1.5 text-body text-ink"><input type="checkbox" checked={configuration.covariates.includes(c.id)} onChange={e => onChange({ ...configuration, covariates: e.target.checked ? [...configuration.covariates,c.id] : configuration.covariates.filter(id => id !== c.id) })} />{c.name}</label>)}
      </div>
    </div>
    {spec.kind === 'doublyRobust' && <div className="grid grid-cols-2 items-end gap-3">
      <label><span className={fieldLabel}>Folds</span><input aria-label="DiD folds" className={field('text','mt-1 w-full')} type="number" min={2} step={1} value={spec.folds} onChange={e => onChange({ ...configuration, specification: { ...spec, folds: Math.max(2,Math.trunc(Number(e.target.value)||2)) } })} /></label>
      <label><span className={fieldLabel}>Seed</span><input aria-label="DiD seed" className={field('text','mt-1 w-full')} type="number" min={0} max={4294967295} step={1} value={spec.seed} onChange={e => onChange({ ...configuration, specification: { ...spec, seed: Math.max(0,Math.min(4294967295,Math.trunc(Number(e.target.value)||0))) } })} /></label>
      <label><ParameterLabel className={fieldLabel} label="Propensity trimming" help="Estimated probabilities are clipped to this threshold and one minus it. No units are deleted; clipping does not repair absent overlap." /><input aria-label="DiD trimming" className={field('text','mt-1 w-full')} type="number" min={0} max={0.5} step="any" value={spec.trimming} onChange={e => { const value = Number(e.target.value); if (value > 0 && value < 0.5) onChange({ ...configuration, specification: { ...spec, trimming: value } }) }} /></label>
      <div><ParameterLabel className={fieldLabel} label="Normalization" help="Choose the DoubleML in-sample normalized weights or population normalization. This is part of the score specification." /><SegmentedControl ariaLabel="DiD normalization" className="mt-1" value={spec.normalization} onChange={normalization => onChange({ ...configuration, specification: { ...spec, normalization } })} options={[{ value:'in-sample',label:'In-sample' },{ value:'population',label:'Population' }]} /></div>
    </div>}
  </div>
}
