import { Select } from '@/components/ui/Select'
import { ColumnChecklist } from '@/components/ui/ColumnChecklist'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { SettingsDisclosure } from '@/components/ui/SettingsDisclosure'
import { field, fieldHint, fieldLabel, fieldRow } from '@/components/ui/recipes'
import { DEFAULT_FOREST_ANALYSIS, type ForestAnalysis } from '@/domain/causalForestAnalysis'

interface AnalysisControlsProps {
  readonly value?: ForestAnalysis
  readonly columns: readonly { readonly id: string; readonly name: string }[]
  readonly onChange: (value: ForestAnalysis) => void
}

function ColumnSelect({ label, selected, columns, help, onChange }: {
  readonly label: string
  readonly selected: string
  readonly columns: AnalysisControlsProps['columns']
  readonly help: string
  readonly onChange: (column: string) => void
}) {
  return <div>
    <ParameterLabel label={label} className={fieldLabel} help={help} />
    <Select aria-label={label} value={selected} onChange={event => onChange(event.target.value)} className={field('text', 'mt-1 w-full')}>
      <option value="">Choose a column</option>{columns.map(column => <option key={column.id} value={column.id}>{column.name}</option>)}
    </Select>
  </div>
}

/** How the forest samples rows and averages its effects: these settings shape the fit, so they sit with the fit step. */
export function CausalForestSamplingControls({ value = DEFAULT_FOREST_ANALYSIS, columns, onChange }: AnalysisControlsProps) {
  const update = (patch: Partial<ForestAnalysis>) => onChange({ ...value, ...patch })
  const sample = value.sampling
  const weight = 'weighting' in sample ? sample.weighting : { kind: 'uniform' } as const
  return <SettingsDisclosure title="Sampling and aggregate inference" items={[{ icon: 'tune', text: value.averageMethod.toUpperCase() }]}>
    <div><ParameterLabel className={fieldLabel} label="Sampling units" help="With clustered observations, sample clusters when growing trees and account for clusters in aggregate uncertainty. Clusters must be independent of one another." />
      <SegmentedControl ariaLabel="Forest sampling units" value={sample.kind} onChange={kind => update({ sampling: kind === 'independent' ? { kind, weighting: weight } : kind === 'equal-clusters' ? { kind, column: '' } : { kind, column: '', equalize: false, weighting: weight } })}
        options={[{ value: 'independent', label: 'Independent rows' }, { value: 'clustered', label: 'Clusters' }, { value: 'equal-clusters', label: 'Equal cluster weights' }]} />
    </div>
    {sample.kind !== 'independent' && <ColumnSelect label="Cluster identifier" selected={sample.column} columns={columns} onChange={column => update({ sampling: { ...sample, column } })} help="Rows with the same identifier belong to one cluster. Use a numeric identifier in the prepared data." />}
    {sample.kind !== 'equal-clusters' && <>
      <SegmentedControl className="justify-self-start" ariaLabel="Forest sample weighting" value={weight.kind} options={[{ value: 'uniform', label: 'Uniform row weights' }, { value: 'column', label: 'Weight column' }]}
        onChange={kind => update({ sampling: { ...sample, weighting: kind === 'uniform' ? { kind } : { kind, column: '' } } })} />
      {weight.kind === 'column' && <ColumnSelect label="Sample weight" selected={weight.column} columns={columns} onChange={column => update({ sampling: { ...sample, weighting: { kind: 'column', column } } })} help="Nonnegative observation weights. The total weight must be positive. These are not propensity scores." />}
    </>}
    {sample.kind === 'equal-clusters' && <p className={fieldHint}>Each cluster receives equal weight. Trees sample the same number of observations from each sampled cluster. Sample weights cannot also be supplied.</p>}
    <div><ParameterLabel className={fieldLabel} label="Aggregate estimator" help="AIPW uses augmented inverse-probability weighting. TMLE is available for binary ATE, ATT and ATC with independent, unweighted observations." />
      <SegmentedControl ariaLabel="Forest aggregate estimator" value={value.averageMethod} onChange={averageMethod => update({ averageMethod })}
        options={[{ value: 'aipw', label: 'AIPW' }, { value: 'tmle', label: 'TMLE' }]} />
    </div>
  </SettingsDisclosure>
}

/** The summaries of a fitted forest's heterogeneity: a linear projection, a prioritization rule, and cluster-score comparisons. */
export function CausalForestAnalysisControls({ value = DEFAULT_FOREST_ANALYSIS, columns, onChange }: AnalysisControlsProps) {
  const update = (patch: Partial<ForestAnalysis>) => onChange({ ...value, ...patch })
  return <>
    <SettingsDisclosure title="Best linear projection" items={[{ icon: 'tune', text: value.projection.kind === 'none' ? 'Not requested' : 'Requested' }]}>
      <p className={fieldHint}>Summarize conditional effects with a linear projection on selected characteristics. The coefficients describe effect heterogeneity, not effects of intervening on those characteristics.</p>
      <SegmentedControl className="justify-self-start" ariaLabel="Forest linear projection" value={value.projection.kind} options={[{ value: 'none', label: 'Not requested' }, { value: 'linear', label: 'Estimate projection' }]}
        onChange={kind => update({ projection: kind === 'none' ? { kind } : { kind, columns: [], overlap: false, covariance: 'hc3' } })} />
      {value.projection.kind === 'linear' && <>
        <ColumnChecklist title="Projection covariates" help="Select numeric characteristics. The projection includes an intercept." columns={columns} selected={value.projection.columns}
          onChange={columns => { if (value.projection.kind === 'linear') update({ projection: { ...value.projection, columns: [...columns] } }) }} />
        <SegmentedControl className="justify-self-start" ariaLabel="Projection population" value={value.projection.overlap ? 'overlap' : 'all'} options={[{ value: 'all', label: 'All rows' }, { value: 'overlap', label: 'Overlap-weighted' }]}
          onChange={kind => { if (value.projection.kind === 'linear') update({ projection: { ...value.projection, overlap: kind === 'overlap' } }) }} />
        <SegmentedControl className="justify-self-start" ariaLabel="Projection covariance" value={value.projection.covariance} options={(['hc0', 'hc1', 'hc2', 'hc3'] as const).map(value => ({ value, label: value.toUpperCase() }))}
          onChange={covariance => { if (value.projection.kind === 'linear') update({ projection: { ...value.projection, covariance } }) }} />
      </>}
    </SettingsDisclosure>
    <SettingsDisclosure title="Treatment prioritization" items={[{ icon: 'tune', text: value.ranking.kind === 'none' ? 'Not requested' : 'RATE' }]}>
      <p className={fieldHint}>RATE evaluates a treatment-prioritization rule. Supply scores constructed independently of this evaluation sample. Larger scores mean higher treatment priority.</p>
      <SegmentedControl className="justify-self-start" ariaLabel="Forest prioritization analysis" value={value.ranking.kind} options={[{ value: 'none', label: 'Not requested' }, { value: 'external', label: 'Evaluate a rule' }]}
        onChange={kind => update({ ranking: kind === 'none' ? { kind } : { kind, columns: [''], rationale: '', target: 'autoc', quantiles: [.1,.2,.3,.4,.5,.6,.7,.8,.9,1], replications: 200, seed: 42 } })} />
      {value.ranking.kind === 'external' && <>
        <ColumnSelect label="Priority score" selected={value.ranking.columns[0] ?? ''} columns={columns} onChange={column => { if (value.ranking.kind === 'external') update({ ranking: { ...value.ranking, columns: [column, ...value.ranking.columns.slice(1)] } }) }} help="Use a prespecified rule or scores learned from separate training data, not effects fitted on this evaluation sample." />
        <ColumnSelect label="Comparison priority score" selected={value.ranking.columns[1] ?? ''} columns={columns} onChange={column => { if (value.ranking.kind === 'external') update({ ranking: { ...value.ranking, columns: [value.ranking.columns[0] ?? '', ...(column === '' ? [] : [column])] } }) }} help="Optionally compare a second independent rule. The difference uses paired uncertainty from the same evaluation observations." />
        <label><ParameterLabel className={fieldLabel} label="Priority independence rationale" help="Record where the priority rule came from and why its construction did not use this evaluation sample. This statement records an assumption; it does not verify independence." />
          <textarea aria-label="Priority independence rationale" className={field('text', 'mt-1 w-full')} value={value.ranking.rationale} onChange={event => { if (value.ranking.kind === 'external') update({ ranking: { ...value.ranking, rationale: event.target.value } }) }} />
        </label>
        <SegmentedControl className="justify-self-start" ariaLabel="RATE target" value={value.ranking.target} options={[{ value: 'autoc', label: 'AUTOC' }, { value: 'qini', label: 'QINI' }]}
          onChange={target => { if (value.ranking.kind === 'external') update({ ranking: { ...value.ranking, target } }) }} />
        <div className={fieldRow.two}>{(['replications', 'seed'] as const).map(key => <label key={key}><span className={fieldLabel}>{key === 'seed' ? 'RATE seed' : 'RATE bootstrap replications'}</span>
          <input type="number" className={field('text', 'mt-1 w-full')} aria-label={key === 'seed' ? 'RATE seed' : 'RATE bootstrap replications'} value={value.ranking.kind === 'external' ? value.ranking[key] : ''} min={key === 'seed' ? 0 : 2}
            onChange={event => { if (value.ranking.kind === 'external') update({ ranking: { ...value.ranking, [key]: event.target.value === '' ? NaN : Number(event.target.value) } }) }} /></label>)}</div>
      </>}
    </SettingsDisclosure>
    <SettingsDisclosure title="Cluster-score moderation" items={[{ icon: 'tune', text: value.moderation === undefined ? 'Not requested' : 'Requested' }]}>
      <p className={fieldHint}>Compare doubly robust treatment-effect scores averaged within clusters. Between-cluster comparisons use Welch tests above versus at or below the median and an ANOVA across tertiles. A within-cluster comparison uses a paired test of the high-minus-low score differences. These require binary treatment and equal cluster weights.</p>
      <SegmentedControl className="justify-self-start" ariaLabel="Cluster-score moderation" value={value.moderation === undefined ? 'off' : 'on'} options={[{ value: 'off', label: 'Not requested' }, { value: 'on', label: 'Compare clusters' }]}
        onChange={kind => { if (kind === 'off') { const { moderation: _, ...rest } = value; onChange(rest) } else update({ moderation: { between: [], within: [] } }) }} />
      {value.moderation !== undefined && <>
        <ColumnChecklist title="Between-cluster characteristics" help="Groups are defined by the mean of each characteristic within a cluster. Each cluster contributes one mean score." columns={columns} selected={value.moderation.between}
          onChange={between => { if (value.moderation !== undefined) update({ moderation: { ...value.moderation, between: [...between] } }) }} />
        <ColumnSelect label="Within-cluster characteristic" selected={value.moderation.within[0]?.column ?? ''} columns={columns} onChange={column => {
          if (value.moderation !== undefined) update({ moderation: { ...value.moderation, within: column === '' ? [] : [{ column, threshold: value.moderation.within[0]?.threshold ?? 0 }] } })
        }} help="Optional. Compare observations at or above a fixed threshold with those below it within every cluster. Every cluster must contain both groups." />
        {value.moderation.within[0] !== undefined && <label><span className={fieldLabel}>Within-cluster threshold</span>
          <input aria-label="Within-cluster threshold" type="number" className={field('text', 'mt-1 w-full')} value={value.moderation.within[0].threshold}
            onChange={event => { const within = value.moderation?.within[0]; if (within !== undefined && value.moderation !== undefined) update({ moderation: { ...value.moderation, within: [{ ...within, threshold: event.target.value === '' ? NaN : Number(event.target.value) }] } }) }} />
        </label>}
        <p className={fieldHint}>These p-values are not adjusted for multiple comparisons. Record prespecified hypotheses and distinguish them from exploratory comparisons.</p>
      </>}
    </SettingsDisclosure>
  </>
}
