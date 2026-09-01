import { Select } from '@/components/ui/Select'
import { describePanelDataProblem } from '@/domain/panel'
import { RadioList } from '@/components/ui/RadioList'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { useReducer, useState } from 'react'
import { Icon } from '@/components/Icon'
import { Alert } from '@/components/ui/Alert'
import { MethodCaveats } from '@/components/MethodCaveats'
import { SeriesStructureCard } from './SeriesStructureCard'
import { GrangerCard } from './GrangerCard'
import { PreparedSeriesPreview } from './PreparedSeriesPreview'
import type { GrangerEvidenceArtifact } from '@/domain/granger'
import { describeMissingnessRefusal, describeResolutionRecord, resolutionCommandFor, type MissingnessResolutionRecord } from '@/domain/missingness'
import { button, field, fieldLabel, label, num, table, td, th, tr } from '@/components/ui/recipes'
import { cellPadding, SortHeader, useTableDensity } from '@/components/table/primitives'
import { createColumnHelper, flexRender, getCoreRowModel, getSortedRowModel, useReactTable, type SortingState } from '@tanstack/react-table'
import { cn } from '@/lib/utils'
import { isNumericDuckDbType, type ColumnId, type DatasetProfile } from '@/domain/dataset'
import { assertNever, err, isNonEmpty, ok, type Result } from '@/domain/dop'
import { STATIONARITY_METHODS } from '@/domain/methods'
import {
  describeSeriesTransform,
  initialPreprocessingDraft,
  newPreparedDatasetVersionId,
  newStationarityEvidenceId,
  newTransformRecipeId,
  readyPreprocessingRecipe,
  seriesTransformFor,
  stepPreprocessing,
  transformSeries,
  transformWarmup,
  type Frequency,
  type MissingnessDraft,
  type PreparedDatasetArtifact,
  type ReadyPreprocessingRecipe,
  type SeriesTransform,
  type StationarityEvidenceArtifact,
  type VariableStationarityEvidence,
} from '@/domain/preprocessing'
import { assessStationarity, decisiveEvidence, describeStationarityAssessment, type StationarityAssessment, type StationarityTestRef } from '@/domain/stationarityAssessment'
import { describeSeasonalAdjustment, seasonalPeriodOf } from '@/domain/seasonal'
import type { SelectedSource } from '@/domain/workflow'
import { formatP, formatStatistic } from '@/lib/format/number'
import type { PanelStructureEvidence } from '@/domain/panel'

interface PreprocessingPanelProps {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly onPrepared: (artifact: PreparedDatasetArtifact) => void
  readonly onStationarityEvidence: (evidence: StationarityEvidenceArtifact) => void
  /** The recorded battery, so a reopened project shows the diagnostic as done before the panel runs one. */
  readonly stationarity: StationarityEvidenceArtifact | null
  /** The workflow's prepared version, so a reopened project shows its diagnostics before the form is touched. */
  readonly preparedVersion: PreparedDatasetArtifact | null
  readonly grangerEvidence: readonly GrangerEvidenceArtifact[]
  readonly onGrangerEvidence: (evidence: GrangerEvidenceArtifact) => void
}

type Diagnostic = 'stationarity' | 'structure' | 'granger'

/** A switch label with a fixed slot for the done glyph, so the knob's measured width does not change when a check appears. */
function DiagnosticLabel({ text, done }: { readonly text: string; readonly done: boolean }) {
  return (
    <span className="inline-flex items-center gap-1">
      {text}
      <Icon name="check" size={12} className={cn('shrink-0 text-ok', !done && 'invisible')} />
      {done && <span className="sr-only">, done</span>}
    </span>
  )
}

const FREQUENCIES: readonly { readonly value: Frequency; readonly label: string }[] = [
  { value: 'daily', label: 'Daily' },
  { value: 'weekly', label: 'Weekly' },
  { value: 'monthly', label: 'Monthly' },
  { value: 'quarterly', label: 'Quarterly' },
  { value: 'yearly', label: 'Yearly' },
]

const TRANSFORMS: readonly { readonly value: SeriesTransform; readonly label: string; readonly detail: string }[] = [
  { value: { kind: 'levels' }, label: 'Keep levels', detail: 'Use the recorded values in their original units.' },
  { value: { kind: 'difference', order: 1 }, label: 'First difference', detail: 'Use the change from the previous interval; one leading observation is removed.' },
  { value: { kind: 'linear-detrend' }, label: 'Linear detrend', detail: 'Subtract a fitted intercept and linear time trend; use deviations from that trend.' },
]

type MissingnessChoiceKind = 'unresolved' | 'lag-aware-exclusion' | 'complete-interval' | 'imputation'
type LagAwareExclusionDraft = Extract<MissingnessDraft, { readonly kind: 'lag-aware-exclusion' }>

const MISSINGNESS_CHOICES: readonly MissingnessChoiceKind[] = [
  'unresolved',
  'lag-aware-exclusion',
  'complete-interval',
  'imputation',
]

const TONE_CLASS = { ok: 'text-ok', warn: 'text-warn', danger: 'text-danger', muted: 'text-muted' } as const

const ruleLabel = (ref: StationarityTestRef): string => `${ref.test === 'zivot-andrews' ? 'ZA' : ref.test.toUpperCase()} ${ref.specification}${ref.series === 'first-difference' ? ' Δ' : ''} ${pValue(ref.pValue)}`

/** The interpreted route for one series with the tests that decided it. */
function StationarityVerdict({ assessment }: { readonly assessment: StationarityAssessment }) {
  const described = describeStationarityAssessment(assessment)
  const decisive = decisiveEvidence(assessment)
  return (
    <div className="min-w-[14rem]" aria-label={`Verdict for ${described.verdict}`}>
      <span className={`text-body font-medium ${TONE_CLASS[described.tone]}`}>{described.verdict}</span>
      <span className="block text-label text-faint">{described.route}</span>
      <span className={num('block text-micro text-faint')}>{decisive.map(ruleLabel).join(' · ')}</span>
    </div>
  )
}

const transformIsSelected = (selected: SeriesTransform, candidate: SeriesTransform): boolean =>
  selected.kind === candidate.kind

const missingnessChoice = (kind: MissingnessDraft['kind'], cells: number): MissingnessDraft => {
  switch (kind) {
    case 'unresolved': return { kind, cells }
    case 'lag-aware-exclusion': return {
      kind,
      cells,
      history: { kind: 'method-default' },
      gapInfluence: { kind: 'direct-only' },
      analysisExclusions: { kind: 'ignore' },
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

const pValue = (value: number): string => formatP(value, { withLabel: false }).text

interface TestStatisticRow {
  readonly name: string
  readonly statistic: number
  readonly p: number
  readonly fit: string
  readonly critical: readonly number[]
}

const statisticHelper = createColumnHelper<TestStatisticRow>()

const statisticColumns = [
  statisticHelper.accessor('name', { header: 'Specification', cell: (context) => <span className="text-ink">{context.getValue()}</span> }),
  statisticHelper.accessor('statistic', { header: 'Statistic', meta: { align: 'right' }, cell: (context) => <span className="text-muted">{rawNumber(context.getValue())}</span> }),
  statisticHelper.accessor('p', { header: 'p-value', meta: { align: 'right' }, cell: (context) => <span className="text-muted">{rawNumber(context.getValue())}</span> }),
  statisticHelper.accessor('fit', { header: 'Fit', cell: (context) => <span className="text-muted">{context.getValue()}</span> }),
  statisticHelper.accessor('critical', { header: 'Critical values · reference order', enableSorting: false, cell: (context) => <span className="text-muted">{criticalValues(context.getValue())}</span> }),
]

function TestStatisticsTable({ rows, density }: { readonly rows: readonly TestStatisticRow[]; readonly density: Parameters<typeof cellPadding>[0] }) {
  const [sorting, setSorting] = useState<SortingState>([])
  const statisticsTable = useReactTable({
    data: rows as TestStatisticRow[],
    columns: statisticColumns,
    state: { sorting },
    onSortingChange: setSorting,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
  })
  const padding = cellPadding(density)
  return (
    <div className="figure-strip mt-2 overflow-x-auto">
      <table className={table}>
        <thead>
          {statisticsTable.getHeaderGroups().map((group) => (
            <tr key={group.id}>
              {group.headers.map((header) => (
                <SortHeader
                  key={header.id}
                  sorted={header.column.getIsSorted()}
                  canSort={header.column.getCanSort()}
                  onToggle={() => header.column.toggleSorting()}
                  align={(header.column.columnDef.meta as { readonly align?: 'left' | 'right' } | undefined)?.align ?? 'left'}
                >
                  {flexRender(header.column.columnDef.header, header.getContext())}
                </SortHeader>
              ))}
            </tr>
          ))}
        </thead>
        <tbody>
          {statisticsTable.getRowModel().rows.map((row) => (
            <tr key={row.id} className={tr()}>
              {row.getVisibleCells().map((cell) => {
                const align = (cell.column.columnDef.meta as { readonly align?: 'left' | 'right' } | undefined)?.align
                return <td key={cell.id} className={td(cn(padding, align === 'right' && num('whitespace-nowrap text-right')))}>{flexRender(cell.column.columnDef.cell, cell.getContext())}</td>
              })}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}

const rawNumber = (value: number): string => formatStatistic('raw', value).text
const criticalValues = (values: readonly number[]): string => values.map(rawNumber).join(' · ')

function preparedArtifact(
  recipe: ReadyPreprocessingRecipe,
  profile: DatasetProfile,
  observations: number,
  resolution: MissingnessResolutionRecord,
  panel: PanelStructureEvidence | null,
): Result<PreparedDatasetArtifact, { readonly kind: 'panel-evidence-missing' }> {
  const identity = {
    id: newPreparedDatasetVersionId(),
    recipe: newTransformRecipeId(),
    sourceProfile: profile.id,
    observations,
    columns: recipe.columns,
    missingness: recipe.missingness,
    resolution,
  }
  switch (recipe.kind) {
    case 'regular-series': return ok({
      ...identity,
      kind: 'prepared-time-series',
      sampling: recipe.sampling,
      seasonalAdjustment: recipe.seasonalAdjustment,
      seriesTransforms: recipe.seriesTransforms,
    })
    case 'cross-sectional': return ok({ ...identity, kind: 'prepared-cross-section', sampling: recipe.sampling, seasonalAdjustment: { kind: 'none' } })
    case 'regular-panel': {
      if (panel === null) return err({ kind: 'panel-evidence-missing' })
      return ok({ ...identity, kind: 'prepared-panel', sampling: recipe.sampling, panel, seasonalAdjustment: { kind: 'none' } })
    }
    default: return assertNever(recipe)
  }
}

export function PreprocessingPanel({ source, profile, onPrepared, onStationarityEvidence, stationarity, preparedVersion, grangerEvidence, onGrangerEvidence }: PreprocessingPanelProps) {
  const [diagnostic, setDiagnostic] = useState<Diagnostic>('stationarity')
  const [structureChecked, setStructureChecked] = useState(false)
  const [draft, dispatch] = useReducer(
    stepPreprocessing,
    profile,
    initialPreprocessingDraft,
  )
  const numericColumns = profile.columns.filter((column) => isNumericDuckDbType(column.duckdbType))
  const [density] = useTableDensity()
  const selectedIds: readonly ColumnId[] = draft.variables.kind === 'selected' ? draft.variables.columns : []
  const readiness = readyPreprocessingRecipe(draft)
  const timeSeriesSelected = draft.sampling.kind === 'regular-series' || draft.sampling.kind === 'regular-series-awaiting-time'
  // The version this panel just made, or the one the project reopened with.
  const preparedCurrent = draft.preparation.kind === 'succeeded' ? draft.preparation.artifact : preparedVersion
  const preparedTimeSeries = preparedCurrent !== null && preparedCurrent.kind === 'prepared-time-series' ? preparedCurrent : null
  const stationarityEvidence: StationarityEvidenceArtifact | null = draft.stationarity.kind === 'succeeded'
    ? draft.stationarity.evidence
    : stationarity !== null && preparedCurrent !== null && stationarity.preparedDataset === preparedCurrent.id ? stationarity : null
  const panelSelected = draft.sampling.kind === 'regular-panel' || draft.sampling.kind === 'regular-panel-awaiting-keys'
  const crossSectionSelected = draft.sampling.kind === 'cross-sectional'
  const currentFrequency = timeSeriesSelected || panelSelected ? draft.sampling.frequency : 'monthly'
  const currentTime = draft.sampling.kind === 'regular-series' || draft.sampling.kind === 'regular-panel' || draft.sampling.kind === 'regular-panel-awaiting-keys' ? draft.sampling.timeColumn ?? '' : ''
  const currentUnit = draft.sampling.kind === 'regular-panel' || draft.sampling.kind === 'regular-panel-awaiting-keys' ? draft.sampling.unitColumn ?? '' : ''
  const seasonalPeriod = timeSeriesSelected ? seasonalPeriodOf(draft.sampling.frequency) : null
  const lagExclusion = draft.missingness.kind === 'lag-aware-exclusion' ? draft.missingness : null
  const columnName = (column: ColumnId): string => profile.columns.find((candidate) => candidate.id === column)?.name ?? column
  const missingnessChoices = crossSectionSelected
    ? MISSINGNESS_CHOICES.filter((kind) => kind !== 'lag-aware-exclusion')
    : MISSINGNESS_CHOICES

  const createPreparedVersion = async () => {
    const recipe = readyPreprocessingRecipe(draft)
    if (!recipe.ok) return
    dispatch({ type: 'preparation-started' })
    try {
      let panelStructure: PanelStructureEvidence | null = null
      if (recipe.value.kind === 'regular-panel') {
        const { inspectPanelInWorker } = await import('@/data/client')
        const inspected = await inspectPanelInWorker(source.file, profile, recipe.value.sampling.unitColumn, recipe.value.sampling.timeColumn)
        if (!inspected.ok) { dispatch({ type: 'preparation-failed', detail: describePanelDataProblem(inspected.error) }); return }
        panelStructure = inspected.value
        if (panelStructure.missingUnitKeys > 0 || panelStructure.missingTimeKeys > 0) {
          dispatch({ type: 'preparation-failed', detail: `Panel keys have ${panelStructure.missingUnitKeys} missing unit values and ${panelStructure.missingTimeKeys} missing time values. Fill or drop those rows.` }); return
        }
        if (panelStructure.duplicateKeys > 0) { dispatch({ type: 'preparation-failed', detail: `${panelStructure.duplicateKeys} unit–time keys repeat. Remove the duplicate rows.` }); return }
        if (!panelStructure.balanced) { dispatch({ type: 'preparation-failed', detail: `The panel is unbalanced: ${panelStructure.observations} rows for ${panelStructure.units} units × ${panelStructure.periods} periods. Complete every unit–period cell.` }); return }
      }
      const { materializeNumericColumnsInWorker } = await import('@/data/client')
      const matrix = await materializeNumericColumnsInWorker(source.file, profile, recipe.value.columns)
      if (!matrix.ok) {
        dispatch({ type: 'preparation-failed', detail: 'The selected numeric columns could not be read. Check their types and missing-value settings.' })
        return
      }
      const command = resolutionCommandFor(recipe.value.missingness)
      let resolution: MissingnessResolutionRecord = { kind: 'none' }
      let observations = matrix.value.rowCount
      if (matrix.value.missingCells > 0 && command === null) {
        dispatch({ type: 'preparation-failed', detail: 'Missing values remain in the selected columns. Choose a missing-value policy.' })
        return
      }
      if (recipe.value.kind === 'regular-panel' && matrix.value.missingCells > 0) {
        dispatch({ type: 'preparation-failed', detail: 'Complete the missing values separately within each unit, then import the balanced panel again.' })
        return
      }
      if (command !== null) {
        const { resolveMissingnessInWorker } = await import('@/analysis/client')
        const cells = matrix.value.rowCount * matrix.value.columns.length
        const validity = new Uint8Array(cells)
        for (let index = 0; index < cells; index += 1) validity[index] = (matrix.value.validity[index >> 3] >> (index & 7)) & 1
        const resolved = await resolveMissingnessInWorker(matrix.value.values, matrix.value.rowCount, matrix.value.columns.length, validity, command)
        if (!resolved.ok) {
          dispatch({ type: 'preparation-failed', detail: resolved.error.detail })
          return
        }
        const outcome = resolved.value.outcome
        if (outcome.kind === 'refused') {
          const reasons = outcome.reasons.map(describeMissingnessRefusal).join(' ')
          const runs = outcome.unresolvedRuns.slice(0, 4).map((run) => `${matrix.value.columns[run.column]?.name ?? run.column} rows ${run.start + 1} to ${run.end}`).join('; ')
          dispatch({ type: 'preparation-failed', detail: `${reasons}${runs.length > 0 ? ` (${runs}${outcome.unresolvedRuns.length > 4 ? '; …' : ''})` : ''} Raise the gap limit, choose another policy, or keep the complete interval.` })
          return
        }
        if (command.kind === 'completeInterval') {
          resolution = { kind: 'window', start: outcome.windowStart, endExclusive: outcome.windowEnd, sourceRows: matrix.value.rowCount }
          observations = outcome.windowEnd - outcome.windowStart
        } else {
          resolution = { kind: 'imputed', method: command.method, maxGap: command.maxGap, cells: outcome.imputedCells.length }
        }
      }

      if (recipe.value.kind === 'regular-series') {
        const leadingRows = transformWarmup(recipe.value.seriesTransforms)
        if (observations <= leadingRows) {
          dispatch({ type: 'preparation-failed', detail: 'First differencing needs at least two retained observations. Choose a longer interval or keep the series in levels.' })
          return
        }
        observations -= leadingRows
      }

      const artifact = preparedArtifact(recipe.value, profile, observations, resolution, panelStructure)
      if (!artifact.ok) { dispatch({ type: 'preparation-failed', detail: 'The unit and time columns were not saved. Select both panel keys and create the prepared dataset version again.' }); return }
      dispatch({ type: 'preparation-succeeded', artifact: artifact.value })
      onPrepared(artifact.value)
    } catch (cause: unknown) {
      dispatch({
        type: 'preparation-failed',
        detail: cause instanceof Error ? cause.message : String(cause),
      })
    }
  }

  const runDiagnostics = async () => {
    if (preparedTimeSeries === null) return
    const prepared = preparedTimeSeries
    dispatch({ type: 'diagnostics-started', total: prepared.columns.length })
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, { runStationarityBattery }] = await Promise.all([
        import('@/data/prepared'),
        import('@/analysis/client'),
      ])
      const matrix = await materialisePrepared(source, profile, prepared, prepared.columns)
      if (!matrix.ok) {
        dispatch({ type: 'diagnostics-failed', detail: describePreparedMaterialisationProblem(matrix.error) })
        return
      }

      const evidence: VariableStationarityEvidence[] = []
      for (const [columnIndex, column] of matrix.value.columns.entries()) {
        const start = columnIndex * matrix.value.rowCount
        const levels = matrix.value.values.slice(start, start + matrix.value.rowCount)
        const levelsBattery = await runStationarityBattery(levels.slice())
        if (!levelsBattery.ok) {
          dispatch({ type: 'diagnostics-failed', detail: `${column.name}: ${levelsBattery.error.detail}` })
          return
        }
        const differencedBattery = await runStationarityBattery(transformSeries(levels, { kind: 'difference', order: 1 }))
        const differenced = differencedBattery.ok ? differencedBattery.value : null
        const viewed = draft.diagnosticTransform.kind === 'levels'
          ? ok(levelsBattery.value)
          : draft.diagnosticTransform.kind === 'difference' && differenced !== null
            ? ok(differenced)
            : await runStationarityBattery(transformSeries(levels, draft.diagnosticTransform))
        if (!viewed.ok) {
          dispatch({ type: 'diagnostics-failed', detail: `${column.name}: ${viewed.error.detail}` })
          return
        }
        evidence.push({ column: column.id, result: viewed.value, levels: levelsBattery.value, differenced, assessment: assessStationarity(levelsBattery.value, differenced) })
        dispatch({ type: 'diagnostic-completed' })
      }
      if (!isNonEmpty(evidence)) {
        dispatch({ type: 'diagnostics-failed', detail: 'The stationarity tests returned no results. Check the selected numeric columns and run the tests again.' })
        return
      }
      const [firstEvidence] = evidence
      const stationarityEvidence: StationarityEvidenceArtifact = {
        kind: 'stationarity-evidence',
        id: newStationarityEvidenceId(),
        preparedDataset: prepared.id,
        observations: firstEvidence.result.observations,
        diagnosticTransform: draft.diagnosticTransform,
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
    <section aria-labelledby="preprocessing-title" className="@container/panel">
      <div className="mb-5 flex flex-wrap items-end justify-between gap-3">
        <div>
          <span className={label('text-faint')}>Prepare data</span>
          <h2 id="preprocessing-title" className="mb-0 mt-2 text-heading text-ink">Set the analysis dataset</h2>
        </div>
        <span className="max-w-[52ch] text-body text-faint">Set how rows are organised, handle missing values, and choose any time-series transformations.</span>
      </div>

      <div className="grid gap-4 @3xl/panel:grid-cols-2">
        <section className="@container/card rounded-xl border border-hair bg-panel p-4" aria-labelledby="sampling-title">
          <span className={label('text-faint')}>How rows are organised</span>
          <h3 id="sampling-title" className="mb-3 mt-1 text-title font-medium text-ink">Choose the observation structure</h3>
          <RadioList
            className="mb-3"
            legend="Observation structure"
            legendHidden
            value={timeSeriesSelected ? 'regular-series' : panelSelected ? 'regular-panel' : crossSectionSelected ? 'cross-section' : null}
            onChange={(next) => dispatch({ type: next === 'regular-series' ? 'regular-series-selected' : next === 'regular-panel' ? 'regular-panel-selected' : 'cross-section-selected' })}
            options={[
              { value: 'regular-series', label: 'Regular time series', hint: 'One row per time step, ordered by its temporal key' },
              { value: 'regular-panel', label: 'Panel', hint: 'Several units observed repeatedly; requires unit and temporal keys' },
              { value: 'cross-section', label: 'Independent observations', hint: 'Rows are exchangeable; no time ordering' },
            ]}
          />
          {(timeSeriesSelected || panelSelected) && (
            <div className="grid gap-3 @md/card:grid-cols-2">
              {panelSelected && (
                <label className="block text-body text-ink">
                  Unit column
                  <Select className={field('text', 'mt-1')} value={currentUnit} onChange={(event) => { const column = profile.columns.find((candidate) => candidate.id === event.target.value); if (column) dispatch({ type: 'unit-column-selected', unitColumn: column.id }) }}>
                    <option value="">Choose column</option>
                    {profile.columns.filter((column) => column.id !== currentTime).map((column) => <option key={column.id} value={column.id}>{column.name}</option>)}
                  </Select>
                </label>
              )}
              <label className="block text-body text-ink">
                Time column
                <Select
                  className={field('text', 'mt-1')}
                  value={currentTime}
                  onChange={(event) => {
                    const column = profile.columns.find((candidate) => candidate.id === event.target.value)
                    if (column) dispatch({ type: 'time-column-selected', timeColumn: column.id })
                  }}
                >
                  <option value="">Choose column</option>
                  {profile.columns.filter((column) => column.id !== currentUnit).map((column) => <option key={column.id} value={column.id}>{column.name}</option>)}
                </Select>
              </label>
              <label className="block text-body text-ink">
                Frequency
                <Select
                  className={field('text', 'mt-1')}
                  value={currentFrequency}
                  onChange={(event) => {
                    const frequency = FREQUENCIES.find((candidate) => candidate.value === event.target.value)
                    if (frequency) dispatch({ type: 'frequency-selected', frequency: frequency.value })
                  }}
                >
                  {FREQUENCIES.map((frequency) => <option key={frequency.value} value={frequency.value}>{frequency.label}</option>)}
                </Select>
              </label>
            </div>
          )}
          {crossSectionSelected && (
            <p className="m-0 text-body text-muted">Rows are independent units. Their order does not represent time.</p>
          )}
          {draft.sampling.kind === 'unconfigured' && (
            <p className="m-0 text-body text-faint">Select the structure that describes one row.</p>
          )}
        </section>

        <section className="@container/card rounded-xl border border-hair bg-panel p-4" aria-labelledby="variables-title">
          <div className="mb-3 flex flex-wrap items-end justify-between gap-3">
            <div>
              <span className={label('text-faint')}>Variables</span>
              <h3 id="variables-title" className="mb-0 mt-1 text-title font-medium text-ink">Select analysis columns</h3>
            </div>
            <div className="flex items-center gap-2">
              <button type="button" className={button('quiet')} onClick={() => numericColumns.forEach((column) => { if (column.id !== currentTime && column.id !== currentUnit && !selectedIds.includes(column.id)) dispatch({ type: 'variable-toggled', column: column.id }) })}>Select all</button>
              <button type="button" className={button('quiet')} onClick={() => selectedIds.forEach((column) => dispatch({ type: 'variable-toggled', column }))}>Clear</button>
            </div>
          </div>
          <div className="grid max-h-40 gap-1 overflow-y-auto @md/card:grid-cols-2">
            {numericColumns.map((column) => {
              const isKey = column.id === currentTime || column.id === currentUnit
              return (
                <label key={column.id} className="flex items-center gap-2 rounded-md px-2 py-1.5 text-body text-muted hover:bg-well">
                  <input
                    type="checkbox"
                    checked={selectedIds.includes(column.id)}
                    disabled={isKey}
                    onChange={() => dispatch({ type: 'variable-toggled', column: column.id })}
                  />
                  <span className={isKey ? 'text-faint' : 'text-ink'}>{column.name}</span>
                </label>
              )
            })}
          </div>
        </section>

        <section className="@container/card rounded-xl border border-hair bg-panel p-4 @3xl/panel:col-span-2" aria-labelledby="missingness-title">
          <span className={label('text-faint')}>Missing values</span>
          <h3 id="missingness-title" className="mb-3 mt-1 text-title font-medium text-ink">Choose how to handle missing data</h3>
          {draft.missingness.kind === 'not-present' ? (
            <p className="m-0 flex items-center gap-2 text-body text-muted">
              <Icon name="check_circle" size={16} className="text-ok" /> No missing values detected.
            </p>
          ) : panelSelected ? (
            <Alert tone="danger" live={false}>
              <p className="m-0">Fill the missing values separately within each unit before preparing this panel.</p>
              <p className="mb-0 mt-1 text-muted">Interpolation and complete-interval operations on this screen do not cross unit boundaries.</p>
            </Alert>
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
                  <span>{kind === 'unresolved' ? 'Leave unresolved' : kind === 'lag-aware-exclusion' ? 'Lag-aware sample exclusion' : kind === 'complete-interval' ? 'Complete contiguous interval' : 'Explicit imputation'}</span>
                </label>
              ))}
              {draft.missingness.kind === 'complete-interval' && (
                <p className="mb-0 mt-1 pl-6 text-body text-faint">Keeps the longest run of rows where every selected column is observed; rows before and after are dropped and recorded.</p>
              )}
              {lagExclusion !== null && (
                <div className="mt-2 grid gap-3 pl-6 @md/card:grid-cols-2">
                  <label className="block text-body text-ink">
                    <span className={fieldLabel}>Leading history</span>
                    <Select
                      className={field('text', 'mt-1')}
                      value={lagExclusion.history.kind}
                      onChange={(event) => {
                        const history: LagAwareExclusionDraft['history'] = event.target.value === 'minimum-for-features'
                          ? { kind: 'minimum-for-features' }
                          : event.target.value === 'fixed-warmup'
                            ? { kind: 'fixed-warmup', observations: 1 }
                            : { kind: 'method-default' }
                        dispatch({ type: 'missingness-selected', resolution: { ...lagExclusion, history } })
                      }}
                    >
                      <option value="method-default">Method default</option>
                      <option value="minimum-for-features">Only required feature history</option>
                      <option value="fixed-warmup">Fixed warm-up</option>
                    </Select>
                  </label>
                  <label className="block text-body text-ink">
                    <span className={fieldLabel}>Gap influence</span>
                    <Select
                      className={field('text', 'mt-1')}
                      value={lagExclusion.gapInfluence.kind}
                      onChange={(event) => {
                        const gapInfluence: LagAwareExclusionDraft['gapInfluence'] = event.target.value === 'following-guard'
                          ? { kind: 'following-guard', steps: 1 }
                          : { kind: 'direct-only' }
                        dispatch({ type: 'missingness-selected', resolution: { ...lagExclusion, gapInfluence } })
                      }}
                    >
                      <option value="direct-only">Affected reference only</option>
                      <option value="following-guard">Guard following references</option>
                    </Select>
                  </label>
                  {lagExclusion.history.kind === 'fixed-warmup' && (
                    <label className="block text-body text-ink">
                      <span className={fieldLabel}>Warm-up observations</span>
                      <input
                        type="number"
                        min={0}
                        max={1000}
                        className={field('text', 'mt-1 w-24')}
                        value={lagExclusion.history.observations}
                        onChange={(event) => dispatch({
                          type: 'missingness-selected',
                          resolution: {
                            ...lagExclusion,
                            history: { kind: 'fixed-warmup', observations: Math.max(0, Math.min(1000, Number(event.target.value) || 0)) },
                          },
                        })}
                      />
                    </label>
                  )}
                  {lagExclusion.gapInfluence.kind === 'following-guard' && (
                    <label className="block text-body text-ink">
                      <span className={fieldLabel}>Following references</span>
                      <input
                        type="number"
                        min={1}
                        max={1000}
                        className={field('text', 'mt-1 w-24')}
                        value={lagExclusion.gapInfluence.steps}
                        onChange={(event) => dispatch({
                          type: 'missingness-selected',
                          resolution: {
                            ...lagExclusion,
                            gapInfluence: { kind: 'following-guard', steps: Math.max(1, Math.min(1000, Number(event.target.value) || 1)) },
                          },
                        })}
                      />
                    </label>
                  )}
                  <p className="mb-0 text-body text-faint @md/card:col-span-2">The time grid stays intact. Compatible lagged methods apply this policy while constructing their analysis samples.</p>
                </div>
              )}
              {draft.missingness.kind === 'imputation' && (
                <div className="mt-2 grid gap-3 pl-6 @md/card:grid-cols-[auto_auto] sm:items-end">
                  <div>
                    <span className={fieldLabel}>Method</span>
                    <SegmentedControl
                      className="mt-1"
                      ariaLabel="Imputation method"
                      value={draft.missingness.kind === 'imputation' ? draft.missingness.method : null}
                      onChange={(method) => dispatch({ type: 'missingness-selected', resolution: { ...(draft.missingness as Extract<MissingnessDraft, { kind: 'imputation' }>), method } })}
                      options={[{ value: 'linearInterior', label: 'Linear inside the series' }, { value: 'forwardFill', label: 'Carry forward' }, { value: 'structuralZero', label: 'Structural zero' }]}
                    />
                  </div>
                  {draft.missingness.method !== 'structuralZero' ? (
                    <label className="block text-body text-ink"><span className={fieldLabel}>Longest gap to fill</span><input type="number" min={1} max={1000} aria-label="Longest gap to fill" className={field('text', 'mt-1 w-24')} value={draft.missingness.maxGap} onChange={(event) => dispatch({ type: 'missingness-selected', resolution: { ...(draft.missingness as Extract<MissingnessDraft, { kind: 'imputation' }>), maxGap: Math.max(1, Math.min(1000, Number(event.target.value) || 1)) } })} /></label>
                  ) : (
                    <label className="flex items-start gap-2 text-body text-ink"><input type="checkbox" checked={draft.missingness.confirmedStructuralZero} onChange={(event) => dispatch({ type: 'missingness-selected', resolution: { ...(draft.missingness as Extract<MissingnessDraft, { kind: 'imputation' }>), confirmedStructuralZero: event.target.checked } })} /><span>Confirm that each missing value represents a true zero.</span></label>
                  )}
                </div>
              )}
            </div>
          )}
        </section>

        {(timeSeriesSelected || panelSelected) && (
        <section className="@container/card rounded-xl border border-hair bg-panel p-4 @3xl/panel:col-span-2" aria-labelledby="transform-title">
          <span className={label('text-faint')}>Time-series values</span>
          <h3 id="transform-title" className="mb-1 mt-1 text-title font-medium text-ink">Prepare the analysis scale</h3>
          {timeSeriesSelected ? (
            <>
              <p className="mb-0 mt-1 text-body text-faint">The saved version resolves missing values, removes any selected seasonal component, then applies each column’s transformation.</p>
              <div className="mt-4">
                <label className="flex items-start gap-2 text-body text-ink">
                  <input
                    type="checkbox"
                    checked={draft.seasonal.kind === 'stl'}
                    disabled={seasonalPeriod === null}
                    onChange={(event) => dispatch({ type: 'seasonal-adjustment-selected', seasonal: event.target.checked ? { kind: 'stl', columns: [], robust: false } : { kind: 'none' } })}
                  />
                  <span>
                    Remove the seasonal component with seasonal-trend decomposition using loess (STL){seasonalPeriod === null ? ' (no period for yearly rows)' : ` at period ${seasonalPeriod}`}
                    <span className="block text-faint">Choose columns with a recurring seasonal pattern. The saved recipe records the period and whether the robust fit was used.</span>
                  </span>
                </label>
                {draft.seasonal.kind === 'stl' && (
                  <div className="mt-2 grid gap-2 pl-6">
                    <div className="grid gap-1 @md/card:grid-cols-2" role="group" aria-label="Columns to adjust seasonally">
                      {selectedIds.map((column) => {
                        const name = profile.columns.find((candidate) => candidate.id === column)?.name ?? column
                        const seasonal = draft.seasonal
                        return (
                          <label key={column} className="flex items-center gap-2 rounded-md px-2 py-1 text-body text-ink hover:bg-well">
                            <input
                              type="checkbox"
                              checked={seasonal.kind === 'stl' && seasonal.columns.includes(column)}
                              onChange={(event) => seasonal.kind === 'stl' && dispatch({ type: 'seasonal-adjustment-selected', seasonal: { ...seasonal, columns: event.target.checked ? [...seasonal.columns, column] : seasonal.columns.filter((candidate) => candidate !== column) } })}
                            />
                            <span>{name}</span>
                          </label>
                        )
                      })}
                      {selectedIds.length === 0 && <p className="m-0 text-body text-faint">Select analysis columns first.</p>}
                    </div>
                    <label className="flex items-center gap-2 text-body text-ink">
                      <input type="checkbox" checked={draft.seasonal.robust} onChange={(event) => draft.seasonal.kind === 'stl' && dispatch({ type: 'seasonal-adjustment-selected', seasonal: { ...draft.seasonal, robust: event.target.checked } })} />
                      <span>Robust fit (down-weights outliers)</span>
                    </label>
                  </div>
                )}
              </div>
              <div className="mt-4 border-t border-hair pt-3">
                <div className="rounded-md border border-line bg-panel" role="group" aria-label="Transformations by column">
                  <div className="flex flex-wrap items-center justify-between gap-2 border-b border-hair px-3 py-1.5">
                    <span className="text-body text-muted">All columns</span>
                    <div className="flex flex-wrap items-center gap-1 pr-[5px]" role="group" aria-label="Set transformation for all selected columns">
                      {TRANSFORMS.map((transform) => (
                        <button key={transform.value.kind} type="button" className={button('quiet', 'px-2 py-1 text-label')} onClick={() => dispatch({ type: 'all-series-transforms-selected', transform: transform.value })}>
                          {transform.label}
                        </button>
                      ))}
                    </div>
                  </div>
                  <div className="divide-y divide-hair">
                    {selectedIds.map((column) => {
                      const name = columnName(column)
                      const selected = seriesTransformFor(draft.seriesTransforms, column)
                      const definition = TRANSFORMS.find((candidate) => transformIsSelected(selected, candidate.value))
                      return (
                        <div key={column} className="flex flex-wrap items-center justify-between gap-x-3 gap-y-1.5 px-3 py-2">
                          <div className="min-w-0">
                            <span className="block text-body text-ink">{name}</span>
                            {selected.kind !== 'levels' && <span className="block text-label text-faint">{definition?.detail}</span>}
                          </div>
                          <SegmentedControl
                            size="sm"
                            ariaLabel={`Transformation for ${name}`}
                            value={selected.kind}
                            onChange={(next) => {
                              const chosen = TRANSFORMS.find((candidate) => candidate.value.kind === next)
                              if (chosen) dispatch({ type: 'series-transform-selected', column, transform: chosen.value })
                            }}
                            options={TRANSFORMS.map((transform) => ({ value: transform.value.kind, label: transform.label }))}
                          />
                        </div>
                      )
                    })}
                    {selectedIds.length === 0 && <p className="m-0 px-3 py-2 text-body text-faint">Select analysis columns first.</p>}
                  </div>
                </div>
                {draft.seriesTransforms.some((record) => record.transform.kind === 'difference') && (
                  <p className="mb-0 mt-3 text-body text-faint">First difference replaces xₜ with xₜ − xₜ₋₁. The first retained row is removed from every column so timestamps remain aligned.</p>
                )}
              </div>
            </>
          ) : (
            <p className="m-0 text-body text-faint">This prepared panel keeps outcomes in levels. Any later lag, difference, or interpolation must operate separately within each unit.</p>
          )}
        </section>
        )}
      </div>

      {preparedTimeSeries !== null && <PreparedSeriesPreview key={preparedTimeSeries.id} source={source} profile={profile} prepared={preparedTimeSeries} />}

      {(timeSeriesSelected || preparedTimeSeries !== null) && (
        <section className="mt-4 rounded-xl border border-hair bg-panel p-4" aria-labelledby="diagnostics-title">
          <div className="flex flex-wrap items-center justify-between gap-3">
            <div>
              <h3 id="diagnostics-title" className="m-0 text-title font-medium text-ink">Diagnostics</h3>
              <p className="mb-0 mt-1 text-body text-faint">These tests do not change the prepared dataset.</p>
            </div>
            <SegmentedControl
              size="sm"
              ariaLabel="Diagnostic"
              value={diagnostic}
              onChange={setDiagnostic}
              options={[
                { value: 'stationarity', label: <DiagnosticLabel text="Stationarity" done={stationarityEvidence !== null} /> },
                { value: 'structure', label: <DiagnosticLabel text="Breaks and seasonality" done={structureChecked} /> },
                { value: 'granger', label: <DiagnosticLabel text="Granger" done={grangerEvidence.length > 0} /> },
              ]}
            />
          </div>
          {preparedTimeSeries === null && (
            <p role="status" className="mb-0 mt-3 text-body text-faint">Create a prepared dataset version to run these diagnostics.</p>
          )}
          <div hidden={diagnostic !== 'stationarity'} className="mt-4 border-t border-hair pt-4">
          <div className="flex flex-wrap items-center justify-between gap-3">
            <div>
              <h4 className="m-0 text-body font-medium text-ink">Stationarity tests</h4>
              <p className="mb-0 mt-1 text-body text-faint">ADF and KPSS assess the prepared values and their first difference; Zivot–Andrews allows one structural break.</p>
            </div>
            <button
              type="button"
              className={button('quiet')}
              disabled={preparedTimeSeries === null}
              aria-busy={draft.stationarity.kind === 'running'}
              onClick={draft.stationarity.kind === 'running' ? undefined : () => void runDiagnostics()}
            >
              Run stationarity tests
            </button>
          </div>
          <MethodCaveats methods={STATIONARITY_METHODS} />
          <div className="mt-3 flex flex-wrap items-end justify-between gap-3 rounded-lg border border-hair bg-well p-3">
            <div>
              <span className={fieldLabel}>Diagnostic scale</span>
              <p className="mb-0 mt-1 text-label text-faint">Changes the stationarity table only. It does not create another prepared dataset version.</p>
            </div>
            <SegmentedControl
              size="sm"
              ariaLabel="Stationarity diagnostic scale"
              value={draft.diagnosticTransform.kind}
              onChange={(next) => {
                const selected = TRANSFORMS.find((candidate) => candidate.value.kind === next)
                if (selected) dispatch({ type: 'diagnostic-transform-selected', transform: selected.value })
              }}
              options={TRANSFORMS.map((transform) => ({
                value: transform.value.kind,
                label: transform.value.kind === 'levels'
                  ? 'Prepared values'
                  : transform.value.kind === 'difference'
                    ? 'First difference of prepared values'
                    : 'Linear detrend of prepared values',
              }))}
            />
          </div>
          {draft.stationarity.kind === 'failed' && <Alert tone="danger" className="mt-3"><p className="m-0">{draft.stationarity.detail}</p></Alert>}
          {stationarityEvidence !== null && (
          <div className="mt-4">
            <p role="status" className="mb-3 mt-0 flex flex-wrap items-center gap-2 text-body text-muted">
              <Icon name="check_circle" size={16} className="text-ok" />
              Stationarity tests · {stationarityEvidence.observations.toLocaleString()} rows · {
                stationarityEvidence.diagnosticTransform.kind === 'levels'
                  ? 'prepared values'
                  : stationarityEvidence.diagnosticTransform.kind === 'difference'
                    ? 'first difference of prepared values'
                    : 'linear detrend of prepared values'
              }
            </p>
            <div className="figure-strip overflow-x-auto">
            <table className={table}>
              <thead>
                <tr>
                  <th className={th()}>Variable</th>
                  <th className={th('text-right')}>ADF p · c</th>
                  <th className={th('text-right')}>KPSS p · c</th>
                  <th className={th('text-right')}>Zivot–Andrews p · constant and trend</th>
                  <th className={th()}>Verdict</th>
                </tr>
              </thead>
              <tbody>
                {stationarityEvidence.variables.map((evidence) => {
                  const column = profile.columns.find((candidate) => candidate.id === evidence.column)
                  return (
                    <tr key={evidence.column} className={tr()}>
                      <td className={td(cn('text-ink', cellPadding(density)))}>{column?.name ?? evidence.column}</td>
                      <td className={td(cn(num('text-right text-muted'), cellPadding(density)))}>{pValue(evidence.result.adf.constant.pValue)}</td>
                      <td className={td(cn(num('text-right text-muted'), cellPadding(density)))}>{pValue(evidence.result.kpss.constant.pValue)}</td>
                      <td className={td(cn(num('text-right text-muted'), cellPadding(density)))}>{pValue(evidence.result.zivotAndrews.levelAndTrend.pValue)}</td>
                      <td className={td(cn('max-w-none whitespace-normal', cellPadding(density)))}><StationarityVerdict assessment={evidence.assessment} /></td>
                    </tr>
                  )
                })}
              </tbody>
            </table>
            </div>
            <div className="mt-4 space-y-2" aria-label="Stationarity raw evidence">
              {stationarityEvidence.variables.map((evidence) => {
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
                      {column?.name ?? evidence.column} · test statistics
                    </summary>
                    <TestStatisticsTable rows={rows} density={density} />
                  </details>
                )
              })}
            </div>
            <details className="mt-3">
              <summary className="cursor-pointer text-label text-muted">About these tests</summary>
              <p className="mb-0 mt-2 text-body text-faint">The ADF null hypothesis is a unit root. The KPSS null hypothesis is stationarity. These tests do not transform the source.</p>
            </details>
          </div>
          )}
          </div>
          <div hidden={diagnostic !== 'structure'} className="mt-4 border-t border-hair pt-4">
            {preparedTimeSeries !== null
              ? <SeriesStructureCard embedded source={source} profile={profile} prepared={preparedTimeSeries} onResult={() => setStructureChecked(true)} />
              : null}
          </div>
          <div hidden={diagnostic !== 'granger'} className="mt-4 border-t border-hair pt-4">
            {preparedTimeSeries !== null
              ? (
                <GrangerCard
                  embedded
                  source={source}
                  profile={profile}
                  prepared={preparedTimeSeries}
                  stationarity={stationarityEvidence}
                  evidence={grangerEvidence}
                  onEvidence={onGrangerEvidence}
                />
              )
              : null}
          </div>
        </section>
      )}

      {draft.preparation.kind === 'failed' && <Alert tone="danger" className="mt-4"><p className="m-0">{draft.preparation.detail}</p></Alert>}
      {readiness.ok && (
      <div className={cn('pop sticky bottom-3 z-(--z-sticky) ml-auto mt-4 flex w-fit max-w-full flex-wrap items-center justify-end gap-x-3 gap-y-1.5', draft.preparation.kind === 'succeeded' && 'float rounded-lg border border-hair bg-panel py-2 pl-3.5 pr-2')}>
        {draft.preparation.kind === 'succeeded'
            ? (
              <p role="status" className="m-0 flex flex-wrap items-center gap-1.5 text-body text-muted">
                <Icon name="check_circle" size={14} className="text-ok" />
                {draft.preparation.artifact.kind === 'prepared-time-series' ? 'Prepared time series' : draft.preparation.artifact.kind === 'prepared-panel' ? 'Prepared panel' : 'Prepared cross-section'} · <span className={num()}>{draft.preparation.artifact.observations.toLocaleString()} rows</span>
                {describeResolutionRecord(draft.preparation.artifact.resolution) !== null && <> · {describeResolutionRecord(draft.preparation.artifact.resolution)}</>}
                {describeSeasonalAdjustment(draft.preparation.artifact.seasonalAdjustment, columnName) !== null && <> · {describeSeasonalAdjustment(draft.preparation.artifact.seasonalAdjustment, columnName)}</>}
                {draft.preparation.artifact.kind === 'prepared-time-series' && draft.preparation.artifact.seriesTransforms.some((record) => record.transform.kind !== 'levels') && <> · {draft.preparation.artifact.seriesTransforms.filter((record) => record.transform.kind !== 'levels').map((record) => `${columnName(record.column)}: ${describeSeriesTransform(record.transform)}`).join(', ')}</>}
                {draft.preparation.artifact.kind === 'prepared-panel' && <> · {draft.preparation.artifact.panel.units.toLocaleString()} units × {draft.preparation.artifact.panel.periods.toLocaleString()} periods</>}
              </p>
            )
            : null}
        <button
          type="button"
          className={button('signal', draft.preparation.kind === 'succeeded' ? undefined : 'float')}
          aria-busy={draft.preparation.kind === 'running'}
          onClick={draft.preparation.kind === 'running' ? undefined : () => void createPreparedVersion()}
        >
          Create prepared dataset version
        </button>
      </div>
      )}
    </section>
  )
}
