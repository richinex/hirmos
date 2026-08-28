import { useReducer } from 'react'
import { Icon } from '@/components/Icon'
import { MethodCaveats } from '@/components/MethodCaveats'
import { button, field, label, num, segment } from '@/components/ui/recipes'
import { isNumericDuckDbType, type ColumnId, type DatasetProfile } from '@/domain/dataset'
import { assertNever, isNonEmpty } from '@/domain/dop'
import { STATIONARITY_METHODS } from '@/domain/methods'
import {
  describeReadinessProblem,
  initialPreprocessingDraft,
  newPreparedDatasetVersionId,
  newStationarityEvidenceId,
  newTransformRecipeId,
  readyPreprocessingRecipe,
  stepPreprocessing,
  transformSeries,
  type Frequency,
  type MissingnessDraft,
  type PreparedDatasetArtifact,
  type ReadyPreprocessingRecipe,
  type SeriesTransform,
  type StationarityEvidenceArtifact,
  type VariableStationarityEvidence,
} from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'

interface PreprocessingPanelProps {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly onPrepared: (artifact: PreparedDatasetArtifact) => void
  readonly onStationarityEvidence: (evidence: StationarityEvidenceArtifact) => void
}

const FREQUENCIES: readonly { readonly value: Frequency; readonly label: string }[] = [
  { value: 'daily', label: 'Daily' },
  { value: 'weekly', label: 'Weekly' },
  { value: 'monthly', label: 'Monthly' },
  { value: 'quarterly', label: 'Quarterly' },
  { value: 'yearly', label: 'Yearly' },
]

const TRANSFORMS: readonly { readonly value: SeriesTransform; readonly label: string; readonly detail: string }[] = [
  { value: { kind: 'levels' }, label: 'Keep levels', detail: 'Original units and observations.' },
  { value: { kind: 'difference', order: 1 }, label: 'First difference', detail: 'Change per sampling interval; removes one leading observation.' },
  { value: { kind: 'linear-detrend' }, label: 'Linear detrend', detail: 'Residual from an explicit intercept-and-time trend.' },
]

type MissingnessChoiceKind = 'unresolved' | 'tigramite-mask' | 'complete-interval' | 'imputation'

const MISSINGNESS_CHOICES: readonly MissingnessChoiceKind[] = [
  'unresolved',
  'tigramite-mask',
  'complete-interval',
  'imputation',
]

const transformIsSelected = (selected: SeriesTransform, candidate: SeriesTransform): boolean =>
  selected.kind === candidate.kind

const missingnessChoice = (kind: MissingnessDraft['kind'], cells: number): MissingnessDraft => {
  switch (kind) {
    case 'unresolved': return { kind, cells }
    case 'tigramite-mask': return {
      kind,
      cells,
      cutOff: 'methodDefault',
      propagateThroughMaxLag: false,
      maskType: 'xyz',
    }
    case 'complete-interval': return { kind, cells }
    case 'imputation': return {
      kind,
      cells,
      method: 'linearInterior',
      maxGap: 3,
      confirmedStructuralZero: false,
    }
    case 'not-present': return { kind }
    default: return assertNever(kind)
  }
}

const pValue = (value: number): string => value < 0.0001 ? '<0.0001' : value.toFixed(4)
const rawNumber = (value: number): string => String(value)
const criticalValues = (values: readonly number[]): string => values.map(rawNumber).join(' · ')

function preparedArtifact(
  recipe: ReadyPreprocessingRecipe,
  profile: DatasetProfile,
  observations: number,
): PreparedDatasetArtifact {
  const identity = {
    id: newPreparedDatasetVersionId(),
    recipe: newTransformRecipeId(),
    sourceProfile: profile.id,
    observations,
    columns: recipe.columns,
    missingness: recipe.missingness,
  }
  switch (recipe.kind) {
    case 'regular-series': return { ...identity, kind: 'prepared-time-series', sampling: recipe.sampling }
    case 'cross-sectional': return { ...identity, kind: 'prepared-cross-section', sampling: recipe.sampling }
    default: return assertNever(recipe)
  }
}

export function PreprocessingPanel({ source, profile, onPrepared, onStationarityEvidence }: PreprocessingPanelProps) {
  const [draft, dispatch] = useReducer(
    stepPreprocessing,
    profile,
    initialPreprocessingDraft,
  )
  const numericColumns = profile.columns.filter((column) => isNumericDuckDbType(column.duckdbType))
  const selectedIds: readonly ColumnId[] = draft.variables.kind === 'selected' ? draft.variables.columns : []
  const readiness = readyPreprocessingRecipe(draft)
  const timeSeriesSelected = draft.sampling.kind === 'regular-series' || draft.sampling.kind === 'regular-series-awaiting-time'
  const crossSectionSelected = draft.sampling.kind === 'cross-sectional'
  const currentFrequency = timeSeriesSelected ? draft.sampling.frequency : 'monthly'
  const currentTime = draft.sampling.kind === 'regular-series' ? draft.sampling.timeColumn : ''
  const missingnessChoices = crossSectionSelected
    ? MISSINGNESS_CHOICES.filter((kind) => kind !== 'tigramite-mask')
    : MISSINGNESS_CHOICES

  const createPreparedVersion = async () => {
    const recipe = readyPreprocessingRecipe(draft)
    if (!recipe.ok) return
    dispatch({ type: 'preparation-started' })
    try {
      const { materializeNumericColumnsInWorker } = await import('@/data/client')
      const matrix = await materializeNumericColumnsInWorker(source.file, profile, recipe.value.columns)
      if (!matrix.ok) {
        dispatch({ type: 'preparation-failed', detail: `Numeric materialization refused: ${matrix.error.kind}.` })
        return
      }
      if (matrix.value.missingCells > 0) {
        dispatch({ type: 'preparation-failed', detail: 'The selected prepared matrix still contains unresolved missing cells.' })
        return
      }

      const artifact = preparedArtifact(recipe.value, profile, matrix.value.rowCount)
      dispatch({ type: 'preparation-succeeded', artifact })
      onPrepared(artifact)
    } catch (cause: unknown) {
      dispatch({
        type: 'preparation-failed',
        detail: cause instanceof Error ? cause.message : String(cause),
      })
    }
  }

  const runDiagnostics = async () => {
    if (draft.preparation.kind !== 'succeeded' || draft.preparation.artifact.kind !== 'prepared-time-series') return
    const prepared = draft.preparation.artifact
    dispatch({ type: 'diagnostics-started', total: prepared.columns.length })
    try {
      const [{ materializeNumericColumnsInWorker }, { runStationarityBattery }] = await Promise.all([
        import('@/data/client'),
        import('@/analysis/client'),
      ])
      const matrix = await materializeNumericColumnsInWorker(source.file, profile, prepared.columns)
      if (!matrix.ok) {
        dispatch({ type: 'diagnostics-failed', detail: `Numeric materialization refused: ${matrix.error.kind}.` })
        return
      }
      if (matrix.value.missingCells > 0) {
        dispatch({ type: 'diagnostics-failed', detail: 'The selected diagnostic matrix still contains unresolved missing cells.' })
        return
      }

      const evidence: VariableStationarityEvidence[] = []
      for (const [columnIndex, column] of matrix.value.columns.entries()) {
        const start = columnIndex * matrix.value.rowCount
        const levels = matrix.value.values.slice(start, start + matrix.value.rowCount)
        const transformed = transformSeries(levels, draft.transform)
        const result = await runStationarityBattery(transformed)
        if (!result.ok) {
          dispatch({ type: 'diagnostics-failed', detail: `${column.name}: ${result.error.detail}` })
          return
        }
        evidence.push({ column: column.id, result: result.value })
        dispatch({ type: 'diagnostic-completed' })
      }
      if (!isNonEmpty(evidence)) {
        dispatch({ type: 'diagnostics-failed', detail: 'No stationarity evidence was produced.' })
        return
      }
      const [firstEvidence] = evidence
      const stationarityEvidence: StationarityEvidenceArtifact = {
        kind: 'stationarity-evidence',
        id: newStationarityEvidenceId(),
        preparedDataset: prepared.id,
        observations: firstEvidence.result.observations,
        transform: draft.transform,
        variables: evidence,
      }
      dispatch({
        type: 'diagnostics-succeeded',
        evidence: stationarityEvidence,
      })
      onStationarityEvidence(stationarityEvidence)
    } catch (cause: unknown) {
      dispatch({
        type: 'diagnostics-failed',
        detail: cause instanceof Error ? cause.message : String(cause),
      })
    }
  }

  return (
    <section aria-labelledby="preprocessing-title">
      <div className="mb-5 flex flex-wrap items-end justify-between gap-3">
        <div>
          <span className={label('text-signal')}>Preprocessing</span>
          <h2 id="preprocessing-title" className="mb-0 mt-2 text-heading text-ink">Prepare analysis data</h2>
        </div>
        <span className="text-body text-faint">Every choice becomes a new recipe version.</span>
      </div>

      <div className="grid gap-4 lg:grid-cols-2">
        <section className="rounded-xl border border-line bg-panel p-4" aria-labelledby="sampling-title">
          <span className={label('text-faint')}>01 · Observational structure</span>
          <h3 id="sampling-title" className="mb-3 mt-1 text-title font-medium text-ink">How are rows related?</h3>
          <div className="mb-3 flex flex-wrap gap-1 rounded-lg border border-hair bg-well p-1">
            <button
              type="button"
              className={segment(timeSeriesSelected)}
              aria-pressed={timeSeriesSelected}
              onClick={() => dispatch({ type: 'regular-series-selected' })}
            >
              Regular time series
            </button>
            <button
              type="button"
              className={segment(crossSectionSelected)}
              aria-pressed={crossSectionSelected}
              onClick={() => dispatch({ type: 'cross-section-selected' })}
            >
              Independent observations
            </button>
          </div>
          {timeSeriesSelected && (
            <div className="grid gap-3 sm:grid-cols-2">
              <label className="block text-body text-ink">
                Time column
                <select
                  className={field('text', 'mt-1')}
                  value={currentTime}
                  onChange={(event) => {
                    const column = profile.columns.find((candidate) => candidate.id === event.target.value)
                    if (column) dispatch({ type: 'time-column-selected', timeColumn: column.id })
                  }}
                >
                  <option value="">Choose column</option>
                  {profile.columns.map((column) => <option key={column.id} value={column.id}>{column.name}</option>)}
                </select>
              </label>
              <label className="block text-body text-ink">
                Frequency
                <select
                  className={field('text', 'mt-1')}
                  value={currentFrequency}
                  onChange={(event) => {
                    const frequency = FREQUENCIES.find((candidate) => candidate.value === event.target.value)
                    if (frequency) dispatch({ type: 'frequency-selected', frequency: frequency.value })
                  }}
                >
                  {FREQUENCIES.map((frequency) => <option key={frequency.value} value={frequency.value}>{frequency.label}</option>)}
                </select>
              </label>
            </div>
          )}
          {crossSectionSelected && (
            <p className="m-0 text-body text-muted">Rows are independent units. Their order will not be interpreted as time.</p>
          )}
          {draft.sampling.kind === 'unconfigured' && (
            <p className="m-0 text-body text-faint">Choose explicitly; Hirmos will not infer time from row order.</p>
          )}
        </section>

        <section className="rounded-xl border border-line bg-panel p-4" aria-labelledby="variables-title">
          <span className={label('text-faint')}>02 · Variables</span>
          <h3 id="variables-title" className="mb-3 mt-1 text-title font-medium text-ink">Analysis columns</h3>
          <div className="grid max-h-40 gap-1 overflow-y-auto sm:grid-cols-2">
            {numericColumns.map((column) => {
              const isTime = column.id === currentTime
              return (
                <label key={column.id} className="flex items-center gap-2 rounded-md px-2 py-1.5 text-body text-muted hover:bg-well">
                  <input
                    type="checkbox"
                    checked={selectedIds.includes(column.id)}
                    disabled={isTime}
                    onChange={() => dispatch({ type: 'variable-toggled', column: column.id })}
                  />
                  <span className={isTime ? 'text-faint' : 'text-ink'}>{column.name}</span>
                </label>
              )
            })}
          </div>
        </section>

        <section className="rounded-xl border border-line bg-panel p-4" aria-labelledby="missingness-title">
          <span className={label('text-faint')}>03 · Missing data</span>
          <h3 id="missingness-title" className="mb-3 mt-1 text-title font-medium text-ink">Resolution policy</h3>
          {draft.missingness.kind === 'not-present' ? (
            <p className="m-0 flex items-center gap-2 text-body text-muted">
              <Icon name="check_circle" size={16} className="text-ok" /> No null cells detected.
            </p>
          ) : (
            <div className="space-y-2">
              {missingnessChoices.map((kind) => (
                <label key={kind} className="flex items-start gap-2 text-body text-ink">
                  <input
                    type="radio"
                    name="missingness"
                    checked={draft.missingness.kind === kind}
                    onChange={() => dispatch({
                      type: 'missingness-selected',
                      resolution: missingnessChoice(kind, draft.missingness.kind === 'not-present' ? 0 : draft.missingness.cells),
                    })}
                  />
                  <span>{kind === 'unresolved' ? 'Leave unresolved' : kind === 'tigramite-mask' ? 'Tigramite-compatible exclusion' : kind === 'complete-interval' ? 'Complete contiguous interval' : 'Explicit imputation'}</span>
                </label>
              ))}
            </div>
          )}
        </section>

        <section className="rounded-xl border border-line bg-panel p-4" aria-labelledby="transform-title">
          <span className={label('text-faint')}>04 · Transformation</span>
          <h3 id="transform-title" className="mb-3 mt-1 text-title font-medium text-ink">Stationarity view</h3>
          {timeSeriesSelected ? (
            <>
              <div className="flex flex-wrap gap-1 rounded-lg border border-hair bg-well p-1">
                {TRANSFORMS.map((transform) => (
                  <button
                    key={transform.value.kind}
                    type="button"
                    className={segment(transformIsSelected(draft.transform, transform.value))}
                    aria-pressed={transformIsSelected(draft.transform, transform.value)}
                    onClick={() => dispatch({ type: 'transform-selected', transform: transform.value })}
                  >
                    {transform.label}
                  </button>
                ))}
              </div>
              <p className="mb-0 mt-2 text-body text-faint">
                {TRANSFORMS.find((transform) => transformIsSelected(draft.transform, transform.value))?.detail}
              </p>
            </>
          ) : (
            <p className="m-0 text-body text-faint">
              {crossSectionSelected ? 'Stationarity and temporal transforms do not apply to independent observations.' : 'Choose the observational structure first.'}
            </p>
          )}
        </section>
      </div>

      <section className="mt-4 rounded-xl border border-edge bg-panel p-4" aria-labelledby="preparation-run-title">
        <div className="flex flex-wrap items-center justify-between gap-3">
          <div>
            <h3 id="preparation-run-title" className="m-0 text-title font-medium text-ink">Prepared dataset</h3>
            <p className="mb-0 mt-1 text-body text-faint">Validate the selected structure and create an immutable version.</p>
          </div>
          <button
            type="button"
            className={button('signal')}
            disabled={!readiness.ok || draft.preparation.kind === 'running'}
            onClick={() => void createPreparedVersion()}
          >
            {draft.preparation.kind === 'running' ? 'Preparing…' : 'Create prepared version'}
          </button>
        </div>
        {!readiness.ok && <p role="status" className="mb-0 mt-3 text-body text-faint">{describeReadinessProblem(readiness.error)}</p>}
        {draft.preparation.kind === 'failed' && <p role="alert" className="mb-0 mt-3 text-body text-danger">{draft.preparation.detail}</p>}
        {draft.preparation.kind === 'succeeded' && (
          <p role="status" className="mb-0 mt-3 flex flex-wrap items-center gap-2 text-body text-muted">
            <Icon name="check_circle" size={16} className="text-ok" />
            {draft.preparation.artifact.kind === 'prepared-time-series' ? 'Prepared time series' : 'Prepared cross-section'} ·{' '}
            {draft.preparation.artifact.observations.toLocaleString()} observations
          </p>
        )}
      </section>

      {timeSeriesSelected && (
        <section className="mt-4 rounded-xl border border-line bg-panel p-4" aria-labelledby="diagnostics-title">
          <div className="flex flex-wrap items-center justify-between gap-3">
            <div>
              <h3 id="diagnostics-title" className="m-0 text-title font-medium text-ink">Stationarity evidence</h3>
              <p className="mb-0 mt-1 text-body text-faint">Optional ADF, KPSS, and Zivot–Andrews evidence. Preparation remains valid if evidence is inconclusive.</p>
            </div>
            <button
              type="button"
              className={button('quiet')}
              disabled={draft.preparation.kind !== 'succeeded' || draft.stationarity.kind === 'running'}
              onClick={() => void runDiagnostics()}
            >
              {draft.stationarity.kind === 'running' ? `Testing ${draft.stationarity.completed}/${draft.stationarity.total}` : 'Run stationarity tests'}
            </button>
          </div>
          {draft.preparation.kind !== 'succeeded' && (
            <p role="status" className="mb-0 mt-3 text-body text-faint">Create the structurally prepared version first.</p>
          )}
          <MethodCaveats methods={STATIONARITY_METHODS} />
          {draft.stationarity.kind === 'failed' && <p role="alert" className="mb-0 mt-3 text-body text-danger">{draft.stationarity.detail}</p>}
          {draft.stationarity.kind === 'succeeded' && (
          <div className="mt-4 overflow-x-auto">
            <p role="status" className="mb-3 mt-0 flex flex-wrap items-center gap-2 text-body text-muted">
              <Icon name="check_circle" size={16} className="text-ok" />
              Evidence run · {draft.stationarity.evidence.observations.toLocaleString()} observations · {
                draft.stationarity.evidence.transform.kind === 'levels'
                  ? 'levels'
                  : draft.stationarity.evidence.transform.kind === 'difference'
                    ? 'first difference'
                    : 'linear detrend'
              }
            </p>
            <table className="w-full border-collapse text-left text-body">
              <thead className="text-faint">
                <tr>
                  <th className="border-b border-hair px-2 py-2 font-normal">Variable</th>
                  <th className="border-b border-hair px-2 py-2 text-right font-normal">ADF p · c</th>
                  <th className="border-b border-hair px-2 py-2 text-right font-normal">KPSS p · c</th>
                  <th className="border-b border-hair px-2 py-2 text-right font-normal">ZA p · c+trend</th>
                </tr>
              </thead>
              <tbody>
                {draft.stationarity.evidence.variables.map((evidence) => {
                  const column = profile.columns.find((candidate) => candidate.id === evidence.column)
                  return (
                    <tr key={evidence.column} className="border-b border-hair last:border-0">
                      <td className="px-2 py-2 text-ink">{column?.name ?? evidence.column}</td>
                      <td className={num('px-2 py-2 text-right text-muted')}>{pValue(evidence.result.adf.constant.pValue)}</td>
                      <td className={num('px-2 py-2 text-right text-muted')}>{pValue(evidence.result.kpss.constant.pValue)}</td>
                      <td className={num('px-2 py-2 text-right text-muted')}>{pValue(evidence.result.zivotAndrews.levelAndTrend.pValue)}</td>
                    </tr>
                  )
                })}
              </tbody>
            </table>
            <div className="mt-4 space-y-2" aria-label="Stationarity raw evidence">
              {draft.stationarity.evidence.variables.map((evidence) => {
                const column = profile.columns.find((candidate) => candidate.id === evidence.column)
                const result = evidence.result
                const rows = [
                  {
                    name: 'ADF · constant',
                    statistic: result.adf.constant.statistic,
                    p: result.adf.constant.pValue,
                    fit: `lag ${result.adf.constant.usedLag} · n ${result.adf.constant.observations}`,
                    critical: result.adf.constant.criticalValues,
                  },
                  {
                    name: 'ADF · constant + trend',
                    statistic: result.adf.constantAndTrend.statistic,
                    p: result.adf.constantAndTrend.pValue,
                    fit: `lag ${result.adf.constantAndTrend.usedLag} · n ${result.adf.constantAndTrend.observations}`,
                    critical: result.adf.constantAndTrend.criticalValues,
                  },
                  {
                    name: 'KPSS · constant',
                    statistic: result.kpss.constant.statistic,
                    p: result.kpss.constant.pValue,
                    fit: `lag ${result.kpss.constant.usedLag}`,
                    critical: result.kpss.constant.criticalValues,
                  },
                  {
                    name: 'KPSS · constant + trend',
                    statistic: result.kpss.constantAndTrend.statistic,
                    p: result.kpss.constantAndTrend.pValue,
                    fit: `lag ${result.kpss.constantAndTrend.usedLag}`,
                    critical: result.kpss.constantAndTrend.criticalValues,
                  },
                  {
                    name: 'Zivot–Andrews · level',
                    statistic: result.zivotAndrews.level.statistic,
                    p: result.zivotAndrews.level.pValue,
                    fit: `base lag ${result.zivotAndrews.level.baseLags} · break ${result.zivotAndrews.level.breakIndex}`,
                    critical: result.zivotAndrews.level.criticalValues,
                  },
                  {
                    name: 'Zivot–Andrews · trend',
                    statistic: result.zivotAndrews.trend.statistic,
                    p: result.zivotAndrews.trend.pValue,
                    fit: `base lag ${result.zivotAndrews.trend.baseLags} · break ${result.zivotAndrews.trend.breakIndex}`,
                    critical: result.zivotAndrews.trend.criticalValues,
                  },
                  {
                    name: 'Zivot–Andrews · level + trend',
                    statistic: result.zivotAndrews.levelAndTrend.statistic,
                    p: result.zivotAndrews.levelAndTrend.pValue,
                    fit: `base lag ${result.zivotAndrews.levelAndTrend.baseLags} · break ${result.zivotAndrews.levelAndTrend.breakIndex}`,
                    critical: result.zivotAndrews.levelAndTrend.criticalValues,
                  },
                ] as const
                return (
                  <details key={evidence.column} className="rounded-lg border border-hair bg-well px-3 py-2">
                    <summary className="cursor-pointer text-body font-medium text-ink">
                      {column?.name ?? evidence.column} · complete numerical evidence
                    </summary>
                    <div className="mt-2 overflow-x-auto">
                      <table className="w-full border-collapse text-left text-body">
                        <thead className="text-faint">
                          <tr>
                            <th className="border-b border-hair px-2 py-2 font-normal">Specification</th>
                            <th className="border-b border-hair px-2 py-2 text-right font-normal">Statistic</th>
                            <th className="border-b border-hair px-2 py-2 text-right font-normal">p-value</th>
                            <th className="border-b border-hair px-2 py-2 font-normal">Fit</th>
                            <th className="border-b border-hair px-2 py-2 font-normal">Critical values · reference order</th>
                          </tr>
                        </thead>
                        <tbody>
                          {rows.map((row) => (
                            <tr key={row.name} className="border-b border-hair last:border-0">
                              <td className="px-2 py-2 text-ink">{row.name}</td>
                              <td className={num('px-2 py-2 text-right text-muted')}>{rawNumber(row.statistic)}</td>
                              <td className={num('px-2 py-2 text-right text-muted')}>{rawNumber(row.p)}</td>
                              <td className={num('px-2 py-2 text-muted')}>{row.fit}</td>
                              <td className={num('px-2 py-2 text-muted')}>{criticalValues(row.critical)}</td>
                            </tr>
                          ))}
                        </tbody>
                      </table>
                    </div>
                  </details>
                )
              })}
            </div>
            <p className="mb-0 mt-3 text-body text-faint">ADF null: unit root. KPSS null: stationarity. This table is evidence; it does not transform the source automatically.</p>
          </div>
          )}
        </section>
      )}
    </section>
  )
}
