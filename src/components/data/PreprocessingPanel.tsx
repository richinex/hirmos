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
import { CountSeriesCard } from './CountSeriesCard'
import { PreparedSeriesPreview } from './PreparedSeriesPreview'
import type { GrangerEvidenceArtifact } from '@/domain/granger'
import type { CountSeriesModelArtifact } from '@/domain/countSeries'
import { describeResolutionRecord, type MissingnessResolutionRecord } from '@/domain/missingness'
import { button, field, fieldLabel, label, num, table, td, th, tr } from '@/components/ui/recipes'
import { cellPadding, SortHeader, useTableDensity } from '@/components/table/primitives'
import { createColumnHelper, flexRender, getCoreRowModel, getSortedRowModel, useReactTable, type SortingState } from '@tanstack/react-table'
import { cn } from '@/lib/utils'
import { isNumericDuckDbType, type ColumnId, type DatasetProfile } from '@/domain/dataset'
import { assertNever, err, isNonEmpty, ok, type Result } from '@/domain/dop'
import { STATIONARITY_METHODS } from '@/domain/methods'
import {
  describeSeriesTransform,
  describeReadinessProblem,
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
import { assessStationarity, decisiveEvidence, describeStationarityAssessment, describeStationarityConflict, type StationarityAssessment, type StationarityTestRef } from '@/domain/stationarityAssessment'
import { describeSeasonalAdjustment, seasonalPeriodOf } from '@/domain/seasonal'
import type { SelectedSource } from '@/domain/workflow'
import { formatP, formatStatistic } from '@/lib/format/number'
import type { PanelStructureEvidence } from '@/domain/panel'
import {
  aggregationFor,
  aggregationsForColumns,
  describeResampling,
  describeResamplingProblem,
  effectiveFrequency,
  resampledMatrixFromEvidence,
  type ResamplingAggregation,
  type ResamplingRecord,
} from '@/domain/resampling'

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
  readonly countSeriesModels: readonly CountSeriesModelArtifact[]
  readonly onCountSeriesModel: (artifact: CountSeriesModelArtifact) => void
}

type Diagnostic = 'stationarity' | 'structure' | 'granger' | 'count-series'

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

const RESAMPLING_AGGREGATIONS: readonly { readonly value: ResamplingAggregation; readonly label: string }[] = [
  { value: 'mean', label: 'Mean' },
  { value: 'sum', label: 'Sum' },
  { value: 'median', label: 'Median' },
  { value: 'minimum', label: 'Minimum' },
  { value: 'maximum', label: 'Maximum' },
  { value: 'first', label: 'First value' },
  { value: 'last', label: 'Last value' },
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

const specificationName = (specification: StationarityTestRef['specification']): string => {
  switch (specification) {
    case 'c': return 'constant'
    case 'ct': return 'constant and trend'
    case 'level': return 'level break'
    case 'trend': return 'trend break'
    case 'levelAndTrend': return 'level and trend break'
    default: return assertNever(specification)
  }
}

const ruleLabel = (ref: StationarityTestRef): string => {
  const test = ref.test === 'zivot-andrews' ? 'Zivot–Andrews' : ref.test.toUpperCase()
  const scale = ref.series === 'first-difference' ? 'first difference' : 'prepared values'
  return `${test} · ${specificationName(ref.specification)} · ${scale}: p ${pValue(ref.pValue)}`
}

/** The interpreted route for one series with the tests that decided it. */
function StationarityVerdict({ assessment }: { readonly assessment: StationarityAssessment }) {
  const described = describeStationarityAssessment(assessment)
  const decisive = decisiveEvidence(assessment)
  // An inconclusive verdict is only actionable with the disagreement that produced it: which
  // specification the two tests fell out over, and which way each of them went.
  const conflicts = assessment.kind === 'inconclusive' ? assessment.conflicts.map(describeStationarityConflict) : []
  return (
    <div className="min-w-[14rem]" aria-label={`Verdict for ${described.verdict}`}>
      <span className={`text-body font-medium ${TONE_CLASS[described.tone]}`}>{described.verdict}</span>
      {conflicts.map((conflict) => <span key={conflict} className="block text-label text-warn">{conflict}</span>)}
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
  readonly critical: readonly { readonly level: string; readonly value: number }[]
}

const statisticHelper = createColumnHelper<TestStatisticRow>()

const statisticColumns = [
  statisticHelper.accessor('name', { header: 'Specification', cell: (context) => <span className="text-ink">{context.getValue()}</span> }),
  statisticHelper.accessor('statistic', { header: 'Statistic', meta: { align: 'right' }, cell: (context) => <span className="text-muted">{rawNumber(context.getValue())}</span> }),
  statisticHelper.accessor('p', { header: 'p-value', meta: { align: 'right' }, cell: (context) => <span className="text-muted">{rawNumber(context.getValue())}</span> }),
  statisticHelper.accessor('fit', { header: 'Fit', cell: (context) => <span className="text-muted">{context.getValue()}</span> }),
  statisticHelper.accessor('critical', { header: 'Critical values', enableSorting: false, cell: (context) => <span className="text-muted">{criticalValues(context.getValue())}</span> }),
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
const criticalValues = (values: readonly { readonly level: string; readonly value: number }[]): string =>
  values.map(({ level, value }) => `${level}: ${rawNumber(value)}`).join(' · ')

const labelledCriticalValues = (
  test: 'adf' | 'kpss' | 'zivot-andrews',
  values: readonly number[],
): readonly { readonly level: string; readonly value: number }[] => {
  const levels = test === 'kpss' ? ['10%', '5%', '2.5%', '1%'] : ['1%', '5%', '10%']
  return values.map((value, index) => ({ level: levels[index] ?? '?', value }))
}

function preparedArtifact(
  recipe: ReadyPreprocessingRecipe,
  profile: DatasetProfile,
  observations: number,
  resolution: MissingnessResolutionRecord,
  resampling: ResamplingRecord,
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
      sampling: { ...recipe.sampling, frequency: effectiveFrequency(recipe.sampling.frequency, recipe.resampling) },
      resampling,
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

export function PreprocessingPanel({ source, profile, onPrepared, onStationarityEvidence, stationarity, preparedVersion, grangerEvidence, onGrangerEvidence, countSeriesModels, onCountSeriesModel }: PreprocessingPanelProps) {
  const [diagnostic, setDiagnostic] = useState<Diagnostic>('stationarity')
  const [structureChecked, setStructureChecked] = useState(false)
  const [draft, dispatch] = useReducer(
    stepPreprocessing,
    profile,
    initialPreprocessingDraft,
  )
  const numericColumns = profile.columns.filter((column) => isNumericDuckDbType(column.duckdbType))
  const [stationarityOpen, setStationarityOpen] = useState(true)
  const [savedRecipe, setSavedRecipe] = useState<string | null>(null)
  const [density] = useTableDensity()
  const selectedIds: readonly ColumnId[] = draft.variables.kind === 'selected' ? draft.variables.columns : []
  const readiness = readyPreprocessingRecipe(draft)
  const recipeDirty = readiness.ok && JSON.stringify(readiness.value) !== savedRecipe
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
  const outputFrequency = timeSeriesSelected ? effectiveFrequency(draft.sampling.frequency, draft.resampling) : null
  const seasonalPeriod = outputFrequency === null ? null : seasonalPeriodOf(outputFrequency)
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
      const { materializeNumericColumnsInWorker, materializeTimeSeriesColumnsInWorker } = await import('@/data/client')
      const matrix = recipe.value.kind === 'regular-series'
        ? await materializeTimeSeriesColumnsInWorker(source.file, profile, recipe.value.sampling.timeColumn, recipe.value.columns)
        : await materializeNumericColumnsInWorker(source.file, profile, recipe.value.columns)
      if (!matrix.ok) {
        const detail = matrix.error.kind === 'time-value-unparseable'
          ? `${matrix.error.name} contains an unparseable time at sorted row ${matrix.error.row + 1}.`
          : matrix.error.kind === 'duplicate-time-value'
            ? `${matrix.error.name} repeats at sorted row ${matrix.error.row + 1}. A regular time series needs one row per time point.`
            : 'The selected columns could not be read. Check their types and missing-value settings.'
        dispatch({ type: 'preparation-failed', detail })
        return
      }
      if (recipe.value.kind === 'regular-panel' && matrix.value.missingCells > 0) {
        dispatch({ type: 'preparation-failed', detail: 'Complete the missing values separately within each unit, then import the balanced panel again.' })
        return
      }

      const { describePreparedMaterialisationProblem, resolveNullableInput } = await import('@/data/prepared')
      const resolved = await resolveNullableInput(matrix.value, recipe.value.missingness)
      if (!resolved.ok) { dispatch({ type: 'preparation-failed', detail: describePreparedMaterialisationProblem(resolved.error) }); return }
      let observations = resolved.value.matrix.rowCount
      let resampling: ResamplingRecord = { kind: 'none' }

      if (recipe.value.kind === 'regular-series') {
        if (recipe.value.resampling.kind === 'daily-downsample') {
          const timeAxis = resolved.value.matrix.timeAxis
          if (timeAxis?.kind !== 'calendar') { dispatch({ type: 'preparation-failed', detail: 'Weekly and monthly resampling require a date or timestamp column. An ordinal time key can order rows but cannot define calendar bins.' }); return }
          const input = { ...resolved.value.matrix, timestamps: timeAxis.timestamps }
          const aggregations = aggregationsForColumns(input.columns, recipe.value.resampling)
          if (!aggregations.ok) { dispatch({ type: 'preparation-failed', detail: describeResamplingProblem(aggregations.error) }); return }
          const { runPandasResampling } = await import('@/analysis/client')
          const evidence = await runPandasResampling(input.timestamps, input.values, input.rowCount, input.columns.length, recipe.value.resampling.targetFrequency, recipe.value.resampling.incompleteBins, aggregations.value, input.imputedCells)
          if (!evidence.ok) { dispatch({ type: 'preparation-failed', detail: evidence.error.detail }); return }
          const grouped = resampledMatrixFromEvidence(input, recipe.value.resampling, evidence.value)
          if (!grouped.ok) { dispatch({ type: 'preparation-failed', detail: describeResamplingProblem(grouped.error) }); return }
          observations = grouped.value.rowCount
          resampling = grouped.value.record
        }
        const leadingRows = transformWarmup(recipe.value.seriesTransforms)
        if (observations <= leadingRows) {
          dispatch({ type: 'preparation-failed', detail: 'First differencing needs at least two retained observations. Choose a longer interval or keep the series in levels.' })
          return
        }
        observations -= leadingRows
      }

      const artifact = preparedArtifact(recipe.value, profile, observations, resolved.value.resolution, resampling, panelStructure)
      if (!artifact.ok) { dispatch({ type: 'preparation-failed', detail: 'The unit and time columns were not saved. Select both panel keys and create the prepared dataset version again.' }); return }
      dispatch({ type: 'preparation-succeeded', artifact: artifact.value })
      onPrepared(artifact.value)
      setSavedRecipe(JSON.stringify(recipe.value))
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
                Source frequency
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
              <button type="button" className={button('quiet')} aria-label="Select all columns" title="Select all columns" onClick={() => numericColumns.forEach((column) => { if (column.id !== currentTime && column.id !== currentUnit && !selectedIds.includes(column.id)) dispatch({ type: 'variable-toggled', column: column.id }) })}>
                <Icon name="select_all" size={15} className="@md/card:hidden" />
                <span className="hidden @md/card:inline">Select all</span>
              </button>
              <button type="button" className={button('quiet')} aria-label="Clear selected columns" title="Clear selected columns" onClick={() => selectedIds.forEach((column) => dispatch({ type: 'variable-toggled', column }))}>
                <Icon name="deselect" size={15} className="@md/card:hidden" />
                <span className="hidden @md/card:inline">Clear</span>
              </button>
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
                <div className="mt-2 grid gap-3 pl-6 @md/card:grid-cols-[minmax(0,1fr)_auto] sm:items-start">
                  <div>
                    <span className={fieldLabel}>Method</span>
                    <RadioList
                      className="mt-1"
                      legend="Imputation method"
                      legendHidden
                      value={draft.missingness.kind === 'imputation' ? draft.missingness.method : null}
                      onChange={(method) => dispatch({ type: 'missingness-selected', resolution: { ...(draft.missingness as Extract<MissingnessDraft, { kind: 'imputation' }>), method } })}
                      options={[
                        { value: 'linearInterior', label: 'Linear inside the series', hint: 'Fill each gap on a straight line between its neighbouring values.' },
                        { value: 'forwardFill', label: 'Carry forward', hint: 'Repeat the last recorded value through the gap.' },
                        { value: 'structuralZero', label: 'Structural zero', hint: 'Treat each missing cell as a true zero; asks for confirmation.' },
                      ]}
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
              <p className="mb-0 mt-1 max-w-[75ch] text-body text-faint">A transformation changes the values used by later analyses. Save a separate prepared version so results on levels and transformed values remain comparable. Missingness is resolved before calendar resampling, seasonal adjustment, and per-column transformations.</p>
              <div className="mt-4 rounded-md border border-line bg-panel p-3">
                <div className="flex flex-wrap items-center justify-between gap-3">
                  <div>
                    <span className={fieldLabel}>Calendar resampling</span>
                    <span className="block text-label text-faint">Create UTC calendar weeks or months from a daily source before diagnostics and analysis.</span>
                  </div>
                  <SegmentedControl
                    size="sm"
                    ariaLabel="Calendar resampling"
                    value={draft.resampling.kind === 'none' ? 'none' : draft.resampling.targetFrequency}
                    onChange={(next) => {
                      if (next === 'none') { dispatch({ type: 'resampling-selected', resampling: { kind: 'none' } }); return }
                      if (next === 'weekly' || next === 'monthly') dispatch({ type: 'resampling-selected', resampling: { kind: 'daily-downsample', targetFrequency: next, incompleteBins: 'keep', aggregations: draft.resampling.kind === 'daily-downsample' ? draft.resampling.aggregations : [] } })
                    }}
                    options={[
                      { value: 'none', label: 'Keep source' },
                      { value: 'weekly', label: 'Weekly', disabled: draft.sampling.frequency !== 'daily' },
                      { value: 'monthly', label: 'Monthly', disabled: draft.sampling.frequency !== 'daily' },
                    ]}
                  />
                </div>
                {draft.sampling.frequency !== 'daily' && <p className="mb-0 mt-2 text-body text-faint">This increment resamples daily sources. Other source frequencies remain unchanged.</p>}
                {draft.resampling.kind === 'daily-downsample' && (
                  <div className="mt-3 border-t border-hair pt-3">
                    <label className="block text-body text-ink">
                      <span className={fieldLabel}>Incomplete calendar bins</span>
                      <Select
                        className={field('text', 'mt-1 max-w-64')}
                        value={draft.resampling.incompleteBins}
                        onChange={(event) => {
                          if (draft.resampling.kind === 'daily-downsample') dispatch({ type: 'resampling-selected', resampling: { ...draft.resampling, incompleteBins: event.target.value === 'drop' ? 'drop' : 'keep' } })
                        }}
                      >
                        <option value="keep">Keep and record</option>
                        <option value="drop">Drop</option>
                      </Select>
                    </label>
                    <div className="mt-3 rounded-md border border-hair" role="group" aria-label="Aggregation by column">
                      <div className="border-b border-hair px-3 py-1.5 text-label text-faint">Choose sum for interval totals; use mean for rates or measurements.</div>
                      <div className="divide-y divide-hair">
                        {selectedIds.map((column) => (
                          <label key={column} className="flex items-center justify-between gap-3 px-3 py-2 text-body text-ink">
                            <span>{columnName(column)}</span>
                            <Select
                              className={field('text', 'w-36')}
                              aria-label={`Aggregation for ${columnName(column)}`}
                              value={aggregationFor(draft.resampling.kind === 'daily-downsample' ? draft.resampling.aggregations : [], column) ?? ''}
                              onChange={(event) => {
                                const chosen = RESAMPLING_AGGREGATIONS.find((candidate) => candidate.value === event.target.value)
                                if (chosen) dispatch({ type: 'resampling-aggregation-selected', column, aggregation: chosen.value })
                              }}
                            >
                              <option value="">Choose aggregation</option>
                              {RESAMPLING_AGGREGATIONS.map((aggregation) => <option key={aggregation.value} value={aggregation.value}>{aggregation.label}</option>)}
                            </Select>
                          </label>
                        ))}
                      </div>
                    </div>
                    {!readiness.ok && readiness.error.kind === 'resampling-aggregations-required' && (
                      <p className="mb-0 mt-2 text-body text-danger" role="status">
                        {describeReadinessProblem(readiness.error)}
                      </p>
                    )}
                  </div>
                )}
              </div>
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
                    <span className="block text-faint">STL separates a fitted trend, a repeating seasonal component, and a remainder. Adjustment subtracts the seasonal component while retaining the trend; the decomposition does not assign causal meaning.</span>
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
                  <p className="mb-0 mt-3 text-body text-faint">First difference replaces xₜ with xₜ − xₜ₋₁. The first retained row is removed from every column so timestamps remain aligned. An effect on a differenced outcome is an effect on its period-to-period change, not directly on its level.</p>
                )}
              </div>
            </>
          ) : (
            <p className="m-0 text-body text-faint">This prepared panel keeps outcomes in levels. Any later lag, difference, or interpolation must operate separately within each unit.</p>
          )}
        </section>
        )}
      </div>

      {preparedCurrent !== null && (
        <p role="status" className="mb-0 mt-4 flex flex-wrap items-center gap-1.5 text-body text-muted">
          <Icon name="check_circle" size={14} className="text-ok" />
          {preparedCurrent.kind === 'prepared-time-series' ? 'Prepared time series' : preparedCurrent.kind === 'prepared-panel' ? 'Prepared panel' : 'Prepared cross-section'} · <span className={num()}>{preparedCurrent.observations.toLocaleString()} rows</span>
          {describeResolutionRecord(preparedCurrent.resolution) !== null && <> · {describeResolutionRecord(preparedCurrent.resolution)}</>}
          {preparedCurrent.kind === 'prepared-time-series' && describeResampling(preparedCurrent.resampling, columnName) !== null && <> · {describeResampling(preparedCurrent.resampling, columnName)}</>}
          {describeSeasonalAdjustment(preparedCurrent.seasonalAdjustment, columnName) !== null && <> · {describeSeasonalAdjustment(preparedCurrent.seasonalAdjustment, columnName)}</>}
          {preparedCurrent.kind === 'prepared-time-series' && preparedCurrent.seriesTransforms.some((record) => record.transform.kind !== 'levels') && <> · {preparedCurrent.seriesTransforms.filter((record) => record.transform.kind !== 'levels').map((record) => `${columnName(record.column)}: ${describeSeriesTransform(record.transform)}`).join(', ')}</>}
          {preparedCurrent.kind === 'prepared-panel' && <> · {preparedCurrent.panel.units.toLocaleString()} units × {preparedCurrent.panel.periods.toLocaleString()} periods</>}
        </p>
      )}
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
                { value: 'structure', label: <DiagnosticLabel text="Breaks" done={structureChecked} /> },
                { value: 'granger', label: <DiagnosticLabel text="Granger" done={grangerEvidence.length > 0} /> },
                { value: 'count-series', label: <DiagnosticLabel text="Count model" done={countSeriesModels.length > 0} /> },
              ]}
            />
          </div>
          {preparedTimeSeries === null && (
            <p role="status" className="mb-0 mt-3 text-body text-faint">Create a prepared dataset version to run these diagnostics.</p>
          )}
          <div hidden={diagnostic !== 'stationarity'} className="mt-4 border-t border-hair pt-4">
          <div>
            <h4 className="m-0 text-body font-medium text-ink">Stationarity tests</h4>
            <p className="mb-0 mt-1 max-w-[75ch] text-body text-faint">A stationary process has stable probabilistic behavior over time after accounting for the deterministic terms in the test. ADF tests a unit root as its null; KPSS tests stationarity as its null. Hirmos reads them together because either test alone can be inconclusive. Zivot–Andrews allows one structural break.</p>
            <p className="mb-0 mt-1 max-w-[75ch] text-body text-faint">Hirmos reports I(1) only when the values in levels support unit-root behavior and their first difference supports stationarity. The prepared data are not changed unless first differencing is saved as a transformation.</p>
          </div>
          <MethodCaveats methods={STATIONARITY_METHODS} />
          <div className="mt-3">
            <span className={fieldLabel}>Run the tests on</span>
            <p className="mb-0 mt-1 max-w-[75ch] text-body text-faint">This moves the reported statistics only. The verdict is read from the values in levels and their first difference either way, because integration order is a property of the series rather than of the scale it is inspected on.</p>
            <RadioList
              className="mt-2"
              legend="Run the stationarity tests on"
              legendHidden
              value={draft.diagnosticTransform.kind}
              onChange={(next) => {
                const selected = TRANSFORMS.find((candidate) => candidate.value.kind === next)
                if (selected) dispatch({ type: 'diagnostic-transform-selected', transform: selected.value })
              }}
              options={TRANSFORMS.map((transform) => ({
                value: transform.value.kind,
                label: transform.value.kind === 'levels' ? 'Values as prepared' : transform.value.kind === 'difference' ? 'Their first difference' : 'Their linear detrend',
                hint: transform.value.kind === 'levels'
                  ? 'Test the data exactly as saved.'
                  : transform.value.kind === 'difference'
                    ? 'Test period-to-period changes; shows whether one difference is enough.'
                    : 'Test deviations from a fitted straight-line trend.',
              }))}
            />
            <div className="mt-3">
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
          </div>
          {draft.stationarity.kind === 'failed' && <Alert tone="danger" className="mt-3"><p className="m-0">{draft.stationarity.detail}</p></Alert>}
          {stationarityEvidence !== null && (
          <div className="mt-4 rounded-md border border-line bg-panel">
            <button
              type="button"
              onClick={() => setStationarityOpen((value) => !value)}
              aria-expanded={stationarityOpen}
              className={'flex w-full items-center gap-1.5 px-2.5 py-1.5 text-label text-muted transition-colors hover:text-ink'}
            >
              <Icon name="expand_more" size={14} className={`shrink-0 transition-transform duration-150 ${stationarityOpen ? 'rotate-180' : ''}`} />
              <span className="flex flex-wrap items-center gap-2">
                <Icon name="check_circle" size={14} className="text-ok" />
                Stationarity tests · {stationarityEvidence.variables.length} variables · {stationarityEvidence.observations.toLocaleString()} rows · {
                  stationarityEvidence.diagnosticTransform.kind === 'levels'
                    ? 'prepared values'
                    : stationarityEvidence.diagnosticTransform.kind === 'difference'
                      ? 'first difference of prepared values'
                      : 'linear detrend of prepared values'
                }
              </span>
            </button>
            <div className="grid transition-[grid-template-rows] duration-200" style={{ gridTemplateRows: stationarityOpen ? '1fr' : '0fr' }}>
            <div className="overflow-hidden">
            <div className="border-t border-hair px-2.5 pb-3">
            <div className="figure-strip overflow-x-auto">
            <table className={table}>
              <thead>
                <tr>
                  <th className={th()}>Variable</th>
                  <th className={th('text-right')}>ADF p · constant</th>
                  <th className={th('text-right')}>KPSS p · constant</th>
                  <th className={th('text-right')}>Zivot–Andrews p · constant and trend</th>
                  <th className={th()} title="Integration order is a property of the series, so it is read from the values in levels and their first difference whichever scale the columns show.">Verdict · from levels and first difference</th>
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
            <div className="mt-4 space-y-2" role="region" aria-label="Stationarity raw evidence">
              {stationarityEvidence.variables.map((evidence) => {
                const column = profile.columns.find((candidate) => candidate.id === evidence.column)
                const result = evidence.result
                const rows = [
                  {
                    name: 'ADF · constant',
                    statistic: result.adf.constant.statistic,
                    p: result.adf.constant.pValue,
                    fit: `lag ${result.adf.constant.usedLag} · n ${result.adf.constant.observations}`,
                    critical: labelledCriticalValues('adf', result.adf.constant.criticalValues),
                  },
                  {
                    name: 'ADF · constant + trend',
                    statistic: result.adf.constantAndTrend.statistic,
                    p: result.adf.constantAndTrend.pValue,
                    fit: `lag ${result.adf.constantAndTrend.usedLag} · n ${result.adf.constantAndTrend.observations}`,
                    critical: labelledCriticalValues('adf', result.adf.constantAndTrend.criticalValues),
                  },
                  {
                    name: 'KPSS · constant',
                    statistic: result.kpss.constant.statistic,
                    p: result.kpss.constant.pValue,
                    fit: `lag ${result.kpss.constant.usedLag}`,
                    critical: labelledCriticalValues('kpss', result.kpss.constant.criticalValues),
                  },
                  {
                    name: 'KPSS · constant + trend',
                    statistic: result.kpss.constantAndTrend.statistic,
                    p: result.kpss.constantAndTrend.pValue,
                    fit: `lag ${result.kpss.constantAndTrend.usedLag}`,
                    critical: labelledCriticalValues('kpss', result.kpss.constantAndTrend.criticalValues),
                  },
                  {
                    name: 'Zivot–Andrews · level',
                    statistic: result.zivotAndrews.level.statistic,
                    p: result.zivotAndrews.level.pValue,
                    fit: `base lag ${result.zivotAndrews.level.baseLags} · break ${result.zivotAndrews.level.breakIndex}`,
                    critical: labelledCriticalValues('zivot-andrews', result.zivotAndrews.level.criticalValues),
                  },
                  {
                    name: 'Zivot–Andrews · trend',
                    statistic: result.zivotAndrews.trend.statistic,
                    p: result.zivotAndrews.trend.pValue,
                    fit: `base lag ${result.zivotAndrews.trend.baseLags} · break ${result.zivotAndrews.trend.breakIndex}`,
                    critical: labelledCriticalValues('zivot-andrews', result.zivotAndrews.trend.criticalValues),
                  },
                  {
                    name: 'Zivot–Andrews · level + trend',
                    statistic: result.zivotAndrews.levelAndTrend.statistic,
                    p: result.zivotAndrews.levelAndTrend.pValue,
                    fit: `base lag ${result.zivotAndrews.levelAndTrend.baseLags} · break ${result.zivotAndrews.levelAndTrend.breakIndex}`,
                    critical: labelledCriticalValues('zivot-andrews', result.zivotAndrews.levelAndTrend.criticalValues),
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
            </div>
            </div>
            </div>
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
          <div hidden={diagnostic !== 'count-series'} className="mt-4 border-t border-hair pt-4">
            {preparedTimeSeries !== null && (
              <CountSeriesCard source={source} profile={profile} prepared={preparedTimeSeries} artifacts={countSeriesModels} onArtifact={onCountSeriesModel} />
            )}
          </div>
        </section>
      )}

      {draft.preparation.kind === 'failed' && <Alert tone="danger" className="mt-4"><p className="m-0">{draft.preparation.detail}</p></Alert>}
      {recipeDirty && (
      <div className="pop sticky bottom-3 z-(--z-sticky) ml-auto mt-4 w-fit max-w-full">
        <button
          type="button"
          className={button('signal', 'float')}
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
