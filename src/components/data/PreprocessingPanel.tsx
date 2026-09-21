import { MetricGrid, MetricTile } from '@/components/ui/figures'
import { TIME_INTERPRETATIONS } from '@/domain/timeInterpretation'
import { TimePreview } from './TimePreview'
import { CalendarReport } from './CalendarReport'
import { ParameterLabel, ParameterHelp } from '@/components/ui/ParameterLabel'
import { DisclosureSummary } from '@/components/ui/DisclosureSummary'
import { resultSurface } from '@/components/ui/recipes'
import { Metadata } from '@/components/ui/Metadata'
import { Select } from '@/components/ui/Select'
import { describePanelDataProblem } from '@/domain/panel'
import { RadioList } from '@/components/ui/RadioList'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { useState, type ReactNode } from 'react'
import { useJob } from '@/analysis/JobsProvider'
import { useWorkflow } from '@/components/WorkflowProvider'
import { JobNotice } from '@/components/ui/JobNotice'
import { Orb } from '@/components/ui/Orb'
import { Icon } from '@/components/Icon'
import { SelectionActions } from '@/components/ui/SelectionActions'
import { Alert } from '@/components/ui/Alert'
import { MethodCaveats } from '@/components/MethodCaveats'
import { SeriesStructureCard } from './SeriesStructureCard'
import { GrangerCard } from './GrangerCard'
import { PreparedSeriesPreview } from './PreparedSeriesPreview'
import { MulticollinearityCard } from './MulticollinearityCard'
import type { GrangerEvidenceArtifact } from '@/domain/granger'
import type { MissingnessResolutionRecord } from '@/domain/missingness'
import { button, field, fieldLabel, label, num, panel, prose, sectionTitle, table, td, th, tr, well } from '@/components/ui/recipes'
import { cellPadding, SortHeader, useTableDensity } from '@/components/table/primitives'
import { createColumnHelper, flexRender, getCoreRowModel, getSortedRowModel, useReactTable, type SortingState } from '@tanstack/react-table'
import { cn } from '@/lib/utils'
import { isNumericDuckDbType, type ColumnId, type DatasetProfile } from '@/domain/dataset'
import { assertNever, err, isNonEmpty, ok, type Result } from '@/domain/dop'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import { STATIONARITY_METHODS } from '@/domain/methods'
import {
  describeReadinessProblem,
  preparationAction,
  describeSeriesTransform,
  newPreparedDatasetVersionId,
  newStationarityEvidenceId,
  newTransformRecipeId,
  readyPreprocessingRecipe,
  seriesTransformFor,
  type PreprocessingEvent,
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
import { seasonalPeriodOf } from '@/domain/seasonal'
import type { SelectedSource } from '@/domain/workflow'
import type { PreparedMatrix } from '@/data/prepared'
import { formatP, formatStatistic } from '@/lib/format/number'
import type { PanelStructureEvidence } from '@/domain/panel'
import type { MulticollinearitySelection } from '@/domain/multicollinearity'
import {
  aggregationFor,
  aggregationsForColumns,
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
  readonly onClearStationarityEvidence: () => void
  /** The recorded battery, so a reopened project shows the diagnostic as done before the panel runs one. */
  readonly stationarity: StationarityEvidenceArtifact | null
  /** The workflow's prepared version, so a reopened project shows its diagnostics before the form is touched. */
  readonly preparedVersion: PreparedDatasetArtifact | null
  readonly grangerEvidence: readonly GrangerEvidenceArtifact[]
  readonly onGrangerEvidence: (evidence: GrangerEvidenceArtifact) => void
}

type Diagnostic = 'multicollinearity' | 'stationarity' | 'structure' | 'granger'

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

const TRANSFORMS: readonly { readonly value: SeriesTransform; readonly label: string }[] = [
  { value: { kind: 'levels' }, label: 'Keep levels' },
  { value: { kind: 'difference', order: 1 }, label: 'First difference' },
  { value: { kind: 'linear-detrend' }, label: 'Linear detrend' },
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
type AnalysisMaskRole = Extract<LagAwareExclusionDraft['analysisExclusions'], { readonly kind: 'roles' }>['roles'][number]

const MISSINGNESS_CHOICES: readonly MissingnessChoiceKind[] = [
  'unresolved',
  'lag-aware-exclusion',
  'complete-interval',
  'imputation',
]

const MISSINGNESS_LABELS: Record<MissingnessChoiceKind, string> = {
  unresolved: 'Leave unresolved',
  'lag-aware-exclusion': 'Lag-aware sample exclusion',
  'complete-interval': 'Complete contiguous interval',
  imputation: 'Explicit imputation',
}
const MISSINGNESS_HELP: Record<MissingnessChoiceKind, string> = {
  unresolved: 'Leave missing cells unchanged; choose a resolution before creating the prepared dataset.',
  'lag-aware-exclusion': 'Keep the time grid and missing cells; compatible methods exclude constructed lagged samples affected by missing values.',
  'complete-interval': 'Keep the longest consecutive run where every selected column is observed, dropping rows before and after it.',
  imputation: 'Replace missing cells using the chosen method; this does not create rows for absent time points.',
}

const ANALYSIS_MASK_ROLES: readonly { readonly value: AnalysisMaskRole; readonly label: string }[] = [
  { value: 'candidate-cause', label: 'Candidate cause (X)' },
  { value: 'tested-outcome', label: 'Tested outcome (Y)' },
  { value: 'conditioner', label: 'Conditioning variable (Z)' },
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
  return `${test}, ${specificationName(ref.specification)}, ${scale}: p ${pValue(ref.pValue)}`
}

/** The interpreted route for one series with the tests that decided it. */
function StationarityVerdict({ assessment, transform }: { readonly assessment: StationarityAssessment; readonly transform: SeriesTransform }) {
  const described = describeStationarityAssessment(assessment)
  const stationaryLabel = {
    levels: 'The saved series appears stationary.',
    difference: 'The first-differenced series appears stationary.',
    'linear-detrend': 'The detrended series appears stationary.',
  } satisfies Record<SeriesTransform['kind'], string>
  const verdict = assessment.kind === 'levelStationary' ? stationaryLabel[transform.kind] : described.verdict
  // An inconclusive verdict is only actionable with the disagreement that produced it: which
  // specification the two tests fell out over, and which way each of them went.
  const conflicts = assessment.kind === 'inconclusive' ? assessment.conflicts.map(describeStationarityConflict) : []
  return (
    <div className="min-w-0" aria-label={`Verdict: ${verdict}`}>
      <span className={`text-body font-medium ${TONE_CLASS[described.tone]}`}>{verdict}</span>
      {conflicts.map((conflict) => <span key={conflict} className="block text-label text-warn">{conflict}</span>)}
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
      cutOff: 'method-default',
      propagateThroughMaxLag: false,
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

const toggleAnalysisMaskRole = (
  current: LagAwareExclusionDraft['analysisExclusions'],
  role: AnalysisMaskRole,
): LagAwareExclusionDraft['analysisExclusions'] => {
  const selected: readonly AnalysisMaskRole[] = current.kind === 'roles' ? current.roles : []
  const next = selected.includes(role)
    ? selected.filter((candidate) => candidate !== role)
    : [...selected, role]
  return isNonEmpty(next) ? { kind: 'roles', roles: next } : { kind: 'ignore' }
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
  values.map(({ level, value }) => `${level}: ${rawNumber(value)}`).join('; ')

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
    resolution,
  }
  switch (recipe.kind) {
    case 'regular-series': return ok({
      ...identity,
      kind: 'prepared-time-series',
      missingness: recipe.missingness,
      sampling: { ...recipe.sampling, frequency: effectiveFrequency(recipe.sampling.frequency, recipe.resampling) },
      resampling,
      seasonalAdjustment: recipe.seasonalAdjustment,
      seriesTransforms: recipe.seriesTransforms,
    })
    case 'cross-sectional': return ok({ ...identity, kind: 'prepared-cross-section', sampling: recipe.sampling, missingness: recipe.missingness, seasonalAdjustment: { kind: 'none' } })
    case 'regular-panel': {
      if (panel === null) return err({ kind: 'panel-evidence-missing' })
      return ok({ ...identity, kind: 'prepared-panel', sampling: recipe.sampling, missingness: recipe.missingness, panel, seasonalAdjustment: { kind: 'none' } })
    }
    default: return assertNever(recipe)
  }
}

export function PreprocessingPanel({ source, profile, onPrepared, onStationarityEvidence, onClearStationarityEvidence, stationarity, preparedVersion, grangerEvidence, onGrangerEvidence }: PreprocessingPanelProps) {
  const preparation = useJob('preparation')
  const preprocessing = useWorkflow(state => state.preprocessing)
  const changePreprocessing = useWorkflow(state => state.changePreprocessing)
  const saveRecipe = useWorkflow(state => state.saveRecipe)
  const session = useJob('stationarity')
  const { job } = session
  const diagnostics = useWorkflow(state => state.diagnosticDraft)
  const changeDiagnostic = useWorkflow(state => state.changeDiagnostic)
  const multicollinearityChecked = useWorkflow(state => state.redundancy !== null && state.redundancy.prepared === preparedVersion?.id)
  const structureChecked = useWorkflow(state => state.temporalStructure !== null && state.temporalStructure.prepared === preparedVersion?.id)
  const diagnostic = diagnostics?.view ?? 'multicollinearity'
  const setDiagnostic = (view: Diagnostic) => {
    if (preparedVersion !== null) changeDiagnostic(preparedVersion.id, { type: 'view', view })
  }
  if (preprocessing === null || preprocessing.profile !== profile.id) throw new Error('Preprocessing requires the current source profile.')
  const { draft, savedRecipe } = preprocessing
  const dispatch = (event: PreprocessingEvent) => changePreprocessing(profile.id, event)
  const numericColumns = profile.columns.filter((column) => isNumericDuckDbType(column.duckdbType))
  const [density] = useTableDensity()
  const selectedIds: readonly ColumnId[] = draft.variables.kind === 'selected' ? draft.variables.columns : []
  // The transform every selected column shares, or null when they differ: what the "all columns" control shows.
  const sharedTransformKind = ((): SeriesTransform['kind'] | null => {
    const kinds = new Set(selectedIds.map((column) => seriesTransformFor(draft.seriesTransforms, column).kind))
    return kinds.size === 1 ? [...kinds][0] ?? null : null
  })()
  const readiness = readyPreprocessingRecipe(draft)
  const action = preparationAction(readiness, savedRecipe, preparation.job.kind === 'running' ? 'running' : preparation.blocked ? 'busy' : 'idle')
  const timeSeriesSelected = draft.sampling.kind === 'regular-series' || draft.sampling.kind === 'regular-series-awaiting-time'
  const preparedCurrent = preparedVersion
  const preparedTimeSeries = preparedCurrent !== null && preparedCurrent.kind === 'prepared-time-series' ? preparedCurrent : null
  const testColumns = diagnostics?.kind === 'series' && diagnostics.prepared === preparedTimeSeries?.id ? diagnostics.stationarity : []
  const selectTestColumns = (columns: readonly ColumnId[]) => {
    if (preparedTimeSeries !== null) changeDiagnostic(preparedTimeSeries.id, { type: 'stationarity', columns })
  }
  const stationarityEvidence = stationarity !== null && preparedCurrent !== null && stationarity.preparedDataset === preparedCurrent.id ? stationarity : null
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

  const applyMulticollinearitySelection = (selection: MulticollinearitySelection) => {
    dispatch({ type: 'column-selection-replaced', columns: selection.columns })
  }

  const createPreparedVersion = async () => {
    const recipe = readyPreprocessingRecipe(draft)
    if (!recipe.ok) return
    const id = preparation.start('analysis', 'Preparing dataset')
    if (id === null) return
    const dispatch = (event: { readonly type: 'preparation-failed'; readonly detail: string }) => preparation.fail(id, event.detail)
    try {
      let panelStructure: PanelStructureEvidence | null = null
      if (recipe.value.kind === 'regular-panel') {
        const { inspectPanelInWorker } = await import('@/data/client')
        if (!preparation.current(id)) return
        const inspected = await inspectPanelInWorker(source.file, profile, recipe.value.sampling.unitColumn, recipe.value.sampling.timeColumn)
        if (!preparation.current(id)) return
        if (!inspected.ok) { dispatch({ type: 'preparation-failed', detail: describePanelDataProblem(inspected.error) }); return }
        panelStructure = inspected.value
        if (panelStructure.missingUnitKeys > 0 || panelStructure.missingTimeKeys > 0) {
          dispatch({ type: 'preparation-failed', detail: `Panel keys have ${panelStructure.missingUnitKeys} missing unit values and ${panelStructure.missingTimeKeys} missing time values. Fill or drop those rows.` }); return
        }
        if (panelStructure.duplicateKeys > 0) { dispatch({ type: 'preparation-failed', detail: `${panelStructure.duplicateKeys} unit–time keys repeat. Remove the duplicate rows.` }); return }
        if (!panelStructure.balanced) { dispatch({ type: 'preparation-failed', detail: `The panel is unbalanced: ${panelStructure.observations} rows for ${panelStructure.units} units × ${panelStructure.periods} periods. This preparation route requires a balanced panel. Review each unit's observation window before changing the data.` }); return }
      }
      const { materializeNumericColumnsInWorker, materializeTimeSeriesColumnsInWorker } = await import('@/data/client')
      if (!preparation.current(id)) return
      const matrix = recipe.value.kind === 'regular-series'
        ? await materializeTimeSeriesColumnsInWorker(source.file, profile, recipe.value.sampling.timeColumn, recipe.value.columns, recipe.value.sampling.interpretation)
        : await materializeNumericColumnsInWorker(source.file, profile, recipe.value.columns)
      if (!preparation.current(id)) return
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
      if (!preparation.current(id)) return
      let observations: number
      let resolution: MissingnessResolutionRecord
      let resolvedMatrix: PreparedMatrix | null = null
      if (recipe.value.missingness.kind === 'lag-aware-exclusion') {
        if (matrix.value.missingCells !== recipe.value.missingness.cells) {
          dispatch({ type: 'preparation-failed', detail: 'The source missing-value count changed. Profile the source again before saving this version.' })
          return
        }
        observations = matrix.value.rowCount
        resolution = { kind: 'lag-aware-exclusion', cells: matrix.value.missingCells }
      } else {
        const resolved = await resolveNullableInput(matrix.value, recipe.value.missingness)
        if (!preparation.current(id)) return
        if (!resolved.ok) { dispatch({ type: 'preparation-failed', detail: describePreparedMaterialisationProblem(resolved.error) }); return }
        observations = resolved.value.matrix.rowCount
        resolution = resolved.value.resolution
        resolvedMatrix = resolved.value.matrix
      }
      let resampling: ResamplingRecord = { kind: 'none' }

      if (recipe.value.kind === 'regular-series') {
        if (recipe.value.resampling.kind === 'daily-downsample') {
          if (resolvedMatrix === null) {
            dispatch({ type: 'preparation-failed', detail: 'Calendar resampling requires a dense prepared matrix.' })
            return
          }
          const timeAxis = resolvedMatrix.timeAxis
          if (timeAxis?.kind !== 'calendar') { dispatch({ type: 'preparation-failed', detail: 'Weekly and monthly resampling require a date or timestamp column. An ordinal time key can order rows but cannot define calendar bins.' }); return }
          const input = { ...resolvedMatrix, timestamps: timeAxis.timestamps }
          const aggregations = aggregationsForColumns(input.columns, recipe.value.resampling)
          if (!aggregations.ok) { dispatch({ type: 'preparation-failed', detail: describeResamplingProblem(aggregations.error) }); return }
          const { runPandasResampling } = await import('@/analysis/client')
          if (!preparation.current(id)) return
          const evidence = await runPandasResampling(input.timestamps, input.values, input.rowCount, input.columns.length, recipe.value.resampling.targetFrequency, recipe.value.resampling.incompleteBins, aggregations.value, input.imputedCells)
          if (!preparation.current(id)) return
          if (!evidence.ok) { dispatch({ type: 'preparation-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
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

      const artifact = preparedArtifact(recipe.value, profile, observations, resolution, resampling, panelStructure)
      if (!artifact.ok) { dispatch({ type: 'preparation-failed', detail: 'The unit and time columns were not saved. Select both panel keys and create the prepared dataset version again.' }); return }
      preparation.finish(id)
      saveRecipe(profile.id, JSON.stringify(recipe.value))
      onPrepared(artifact.value)
    } catch (cause: unknown) {
      dispatch({
        type: 'preparation-failed',
        detail: cause instanceof Error ? cause.message : String(cause),
      })
    }
  }

  const runDiagnostics = async () => {
    if (preparedTimeSeries === null || !isNonEmpty(testColumns)) return
    const current = session.start('analysis', 'Stationarity tests')
    if (current === null) return
    const fail = (detail: string) => session.fail(current, detail)
    const prepared = preparedTimeSeries
    session.progress(current, 'Preparing selected variables', { completed: 0, total: testColumns.length })
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, { runStationarityBattery }] = await Promise.all([
        import('@/data/prepared'),
        import('@/analysis/client'),
      ])
      if (!session.current(current)) return
      const matrix = await materialisePrepared(source, profile, prepared, testColumns)
      if (!session.current(current)) return
      if (!matrix.ok) {
        fail(describePreparedMaterialisationProblem(matrix.error))
        return
      }

      const evidence: VariableStationarityEvidence[] = []
      for (const [columnIndex, column] of matrix.value.columns.entries()) {
        const start = columnIndex * matrix.value.rowCount
        const savedValues = matrix.value.values.slice(start, start + matrix.value.rowCount)
        const saved = await runStationarityBattery(savedValues)
        if (!session.current(current)) return
        if (!saved.ok) {
          fail(`${column.name}: ${describeAnalysisWorkerProblem(saved.error)}`)
          return
        }
        evidence.push({ column: column.id, result: saved.value, levels: saved.value, differenced: null, assessment: assessStationarity(saved.value, null) })
        session.progress(current, 'Stationarity tests', { completed: evidence.length, total: testColumns.length })
      }
      if (!isNonEmpty(evidence)) {
        fail('The stationarity tests returned no results. Check the selected numeric columns and run the tests again.')
        return
      }
      const [firstEvidence] = evidence
      const completedEvidence: StationarityEvidenceArtifact = {
        kind: 'stationarity-evidence',
        id: newStationarityEvidenceId(),
        preparedDataset: prepared.id,
        observations: firstEvidence.result.observations,
        diagnosticTransform: { kind: 'levels' },
        variables: [firstEvidence, ...evidence.slice(1), ...(stationarityEvidence?.variables.filter(previous => !evidence.some(next => next.column === previous.column)) ?? [])],
      }
      onStationarityEvidence(completedEvidence)
      session.finish(current)
    } catch (cause: unknown) {
      fail(cause instanceof Error ? cause.message : String(cause))
    }
  }

  const cancelDiagnostics = session.cancel

  const clearDiagnostics = () => {
    onClearStationarityEvidence()
  }

  const deleteDiagnostic = (column: ColumnId) => {
    if (stationarityEvidence === null) return
    const variables = stationarityEvidence.variables.filter(value => value.column !== column)
    if (!isNonEmpty(variables)) { clearDiagnostics(); return }
    const evidence = { ...stationarityEvidence, id: newStationarityEvidenceId(), variables }
    onStationarityEvidence(evidence)
  }

  const diagnosticOptions: readonly { readonly value: Diagnostic; readonly label: ReactNode }[] = [
    { value: 'multicollinearity', label: <DiagnosticLabel text="Redundancy" done={multicollinearityChecked} /> },
    ...(timeSeriesSelected || preparedTimeSeries !== null ? [
      { value: 'stationarity' as const, label: <DiagnosticLabel text="Stationarity" done={stationarityEvidence !== null} /> },
      { value: 'structure' as const, label: <DiagnosticLabel text="Breaks" done={structureChecked} /> },
      { value: 'granger' as const, label: <DiagnosticLabel text="Granger" done={grangerEvidence.length > 0} /> },
    ] : []),
  ]

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
        <section className={panel('@container/card p-(--panel-space)')} aria-labelledby="sampling-title">
          <span className={label('text-faint')}>How rows are organised</span>
          <h3 id="sampling-title" className={cn(sectionTitle, 'mb-3 mt-1')}>Choose the observation structure</h3>
          <RadioList frame="none"
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
                  <span className={fieldLabel}>Unit column</span>
                  <Select className={field('text', 'mt-1')} value={currentUnit} onChange={(event) => { const column = profile.columns.find((candidate) => candidate.id === event.target.value); if (column) dispatch({ type: 'unit-column-selected', unitColumn: column.id }) }}>
                    <option value="">Choose column</option>
                    {profile.columns.filter((column) => column.id !== currentTime).map((column) => <option key={column.id} value={column.id}>{column.name}</option>)}
                  </Select>
                </label>
              )}
              <label className="block text-body text-ink">
                <span className={fieldLabel}>Time column</span>
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
              {draft.sampling.kind === 'regular-series' ? <div className="block text-body text-ink">
                {draft.sampling.interpretation?.kind === 'iso-week'
                  ? <ParameterLabel className={fieldLabel} htmlFor="time-interpretation" label="Time interpretation" help="Weeks start on Monday. The ISO week-year can differ from the calendar year." />
                  : <label className={fieldLabel} htmlFor="time-interpretation">Time interpretation</label>}
                <Select id="time-interpretation" className={field('text', 'mt-1')}
                  value={draft.sampling.interpretation?.kind === 'date-format' ? draft.sampling.interpretation.format : draft.sampling.interpretation?.kind ?? 'source-type'}
                  onChange={(event) => {
                    const choice = TIME_INTERPRETATIONS.find((candidate) => candidate.value === event.target.value)
                    if (choice) dispatch({ type: 'time-interpretation-selected', interpretation: choice.interpretation })
                  }}>
                  {TIME_INTERPRETATIONS.map((choice) => <option key={choice.value} value={choice.value}>{choice.label}</option>)}
                </Select>
              </div> : null}
              <label className="block text-body text-ink">
                <span className={fieldLabel}>Source frequency</span>
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
          {draft.sampling.kind === 'regular-series' ? <details className="mt-3">
            <DisclosureSummary>Time preview</DisclosureSummary>
            <TimePreview file={source.file} profile={profile} column={draft.sampling.timeColumn} interpretation={draft.sampling.interpretation} />
          </details> : null}
          {draft.sampling.kind === 'regular-series' && <CalendarReport file={source.file} profile={profile} column={draft.sampling.timeColumn} interpretation={draft.sampling.interpretation} frequency={draft.sampling.frequency} />}
          {draft.sampling.kind === 'regular-panel' && <CalendarReport file={source.file} profile={profile} column={draft.sampling.timeColumn} unitColumn={draft.sampling.unitColumn} frequency={draft.sampling.frequency} />}
          {crossSectionSelected && (
            <p className="m-0 text-body text-muted">Rows are independent units. Their order does not represent time.</p>
          )}
          {draft.sampling.kind === 'unconfigured' && (
            <p className="m-0 text-body text-faint">Select the structure that describes one row.</p>
          )}
        </section>

        <section className={panel('@container/card flex min-h-0 flex-col p-(--panel-space)')} aria-labelledby="variables-title">
          <div className="mb-3 flex shrink-0 flex-wrap items-end justify-between gap-3">
            <div>
              <span className={label('text-faint')}>Variables</span>
              <h3 id="variables-title" className={cn(sectionTitle, 'mb-0 mt-1')}>Select analysis columns</h3>
            </div>
            <SelectionActions
              selectLabel="Select all columns"
              clearLabel="Clear selected columns"
              onSelectAll={() => numericColumns.forEach((column) => { if (column.id !== currentTime && column.id !== currentUnit && !selectedIds.includes(column.id)) dispatch({ type: 'variable-toggled', column: column.id }) })}
              onClear={() => selectedIds.forEach((column) => dispatch({ type: 'variable-toggled', column }))}
            />
          </div>
          <div className="panel-scroll grid max-h-64 content-start gap-1 overflow-y-auto @md/card:grid-cols-2 @3xl/panel:min-h-40 @3xl/panel:max-h-none @3xl/panel:flex-1 @3xl/panel:[contain:size]" data-testid="analysis-columns-list">
            {numericColumns.map((column) => {
              const isKey = column.id === currentTime || column.id === currentUnit
              return (
                <label key={column.id} className="flex min-w-0 items-center gap-2 rounded-md px-2 py-1.5 text-body text-muted hover:bg-well">
                  <input
                    type="checkbox"
                    className="shrink-0"
                    checked={selectedIds.includes(column.id)}
                    disabled={isKey}
                    onChange={() => dispatch({ type: 'variable-toggled', column: column.id })}
                  />
                  <span className={cn('min-w-0 [overflow-wrap:anywhere]', isKey ? 'text-faint' : 'text-ink')}>{column.name}</span>
                </label>
              )
            })}
          </div>
        </section>

        <section className={panel('@container/card p-(--panel-space) @3xl/panel:col-span-2')} aria-labelledby="missingness-title">
          <span className={label('text-faint')}>Missing values</span>
          <h3 id="missingness-title" className={cn(sectionTitle, 'mb-3 mt-1')}>{draft.missingness.kind === 'not-present' ? 'Missing-data status' : 'Choose how to handle missing data'}</h3>
          {draft.missingness.kind === 'not-present' ? (
            <p className="m-0 flex items-center gap-2 text-body text-muted">
              <Icon name="check_circle" size={16} className="text-ok" /> No missing values detected.
            </p>
          ) : panelSelected ? (
            <Alert tone="danger" live={false}>
              <p className="m-0">Fill the missing values separately within each unit before preparing this panel.</p>
              <p className="mb-0 mt-1 text-muted">This form does not yet support missing-value handling within panel units. Prepare those values in the pipeline or SQL editor.</p>
            </Alert>
          ) : (
            <div className="space-y-2">
              <RadioList frame="none"
                legend="Missing-value policy"
                legendHidden
                value={draft.missingness.kind}
                options={missingnessChoices.map((value) => ({ value, label: MISSINGNESS_LABELS[value], help: MISSINGNESS_HELP[value] }))}
                onChange={(kind) => dispatch({
                  type: 'missingness-selected',
                  resolution: missingnessChoice(kind, draft.missingness.kind === 'not-present' ? 0 : draft.missingness.cells),
                })}
              />
              {lagExclusion !== null && (
                <div className="mt-2 grid gap-3 pl-6 @md/card:grid-cols-2">
                  <label className="block text-body text-ink">
                    <span className={fieldLabel}>Sample cutoff</span>
                    <Select
                      className={field('text', 'mt-1')}
                      value={lagExclusion.cutOff}
                      onChange={(event) => dispatch({
                        type: 'missingness-selected',
                        resolution: { ...lagExclusion, cutOff: event.target.value as LagAwareExclusionDraft['cutOff'] },
                      })}
                    >
                      <option value="method-default">Method default</option>
                      <option value="2xtau-max">2 × maximum lag</option>
                      <option value="tau-max">Maximum lag</option>
                      <option value="max-lag">Largest selected lag</option>
                      <option value="max-lag-or-tau-max">Larger of selected and maximum lag</option>
                      <option value="2xtau-max-future">Future-aligned 2 × maximum lag</option>
                    </Select>
                  </label>
                  <label className="flex items-center gap-2 self-end py-2 text-body text-ink">
                    <input
                      type="checkbox"
                      checked={lagExclusion.propagateThroughMaxLag}
                      onChange={(event) => dispatch({
                        type: 'missingness-selected',
                        resolution: { ...lagExclusion, propagateThroughMaxLag: event.target.checked },
                      })}
                    />
                    <span>Exclude following samples through the cutoff window.</span>
                  </label>
                  <fieldset className="@md/card:col-span-2">
                    <legend className={fieldLabel}>Apply the analysis mask when a cell is used as</legend>
                    <div className="mt-2 flex flex-wrap gap-x-5 gap-y-2">
                      {ANALYSIS_MASK_ROLES.map((role) => (
                        <label key={role.value} className="flex items-center gap-2 text-body text-ink">
                          <input
                            type="checkbox"
                            checked={lagExclusion.analysisExclusions.kind === 'roles' && lagExclusion.analysisExclusions.roles.includes(role.value)}
                            onChange={() => dispatch({
                              type: 'missingness-selected',
                              resolution: {
                                ...lagExclusion,
                                analysisExclusions: toggleAnalysisMaskRole(lagExclusion.analysisExclusions, role.value),
                              },
                            })}
                          />
                          <span>{role.label}</span>
                        </label>
                      ))}
                    </div>
                  </fieldset>
                  <p className="mb-0 text-body text-faint @md/card:col-span-2">Missing cells are always excluded. These roles apply only to separately marked analysis-mask cells. The original time grid is retained.</p>
                </div>
              )}
              {draft.missingness.kind === 'imputation' && (
                <div className="mt-2 grid gap-3 pl-6 @md/card:grid-cols-[minmax(0,1fr)_auto] sm:items-start">
                  <div>
                    <span className={`${fieldLabel} min-h-5 pointer-coarse:min-h-11 flex items-center`}>Method</span>
                    <RadioList frame="none"
                      className="mt-1"
                      legend="Imputation method"
                      legendHidden
                      value={draft.missingness.kind === 'imputation' ? draft.missingness.method : null}
                      onChange={(method) => dispatch({ type: 'missingness-selected', resolution: { ...(draft.missingness as Extract<MissingnessDraft, { kind: 'imputation' }>), method } })}
                      options={[
                        { value: 'linearInterior', label: 'Linear inside the series', help: 'Interpolate between the observed values on both sides of a gap; edge gaps and gaps exceeding the limit remain missing.' },
                        { value: 'forwardFill', label: 'Carry forward', help: 'Repeat the preceding observed value through a gap; leading gaps and gaps exceeding the limit remain missing.' },
                        { value: 'structuralZero', label: 'Structural zero', help: 'Replace every missing cell with zero only when you confirm it represents a true zero, not an unknown value.' },
                      ]}
                    />
                  </div>
                  {draft.missingness.method !== 'structuralZero' ? (
                    <div className="text-body text-ink"><ParameterLabel className={fieldLabel} label="Longest gap to fill" help="Maximum consecutive missing cells per column; a longer gap is left entirely unfilled, not partially filled." /><input type="number" min={1} max={1000} aria-label="Longest gap to fill" className={field('text', 'mt-1 w-24')} value={draft.missingness.maxGap} onChange={(event) => dispatch({ type: 'missingness-selected', resolution: { ...(draft.missingness as Extract<MissingnessDraft, { kind: 'imputation' }>), maxGap: Math.max(1, Math.min(1000, Number(event.target.value) || 1)) } })} /></div>
                  ) : (
                    <label className="flex items-start gap-2 text-body text-ink"><input type="checkbox" checked={draft.missingness.confirmedStructuralZero} onChange={(event) => dispatch({ type: 'missingness-selected', resolution: { ...(draft.missingness as Extract<MissingnessDraft, { kind: 'imputation' }>), confirmedStructuralZero: event.target.checked } })} /><span>Confirm that each missing value represents a true zero.</span></label>
                  )}
                </div>
              )}
            </div>
          )}
        </section>

        {(timeSeriesSelected || panelSelected) && (
        <section className={panel('@container/card p-(--panel-space) @3xl/panel:col-span-2')} aria-labelledby="transform-title">
          <span className={label('text-faint')}>Time-series values</span>
          <h3 id="transform-title" className={cn(sectionTitle, 'mb-1 mt-1')}>Prepare the analysis scale</h3>
          {timeSeriesSelected ? (
            <>
              <p className={prose('mb-0 mt-1 text-faint')}>A transformation changes the values used by later analyses. Save a separate prepared version so results on levels and transformed values remain comparable. Missingness is resolved before calendar resampling, seasonal adjustment, and per-column transformations.</p>
              <div className="mt-4 rounded-md border border-line p-3">
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
                      <div className="space-y-2">
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
              <div className="mt-4 pt-3">
                <div role="group" aria-label="Transformations by column">
                  {/* One layout for the whole list, decided by the panel's width rather than per row: a name beside
                      its control when the panel is at least md wide, every name above its control below that. */}
                  <div className="grid gap-x-3 gap-y-1 px-3 py-1.5 @md/panel:grid-cols-[minmax(0,1fr)_auto] @md/panel:items-center">
                    <span className="text-body text-muted">All columns</span>
                    {/* The same control as each row: it shows the transform the columns share, nothing when they differ, and sets every column when chosen. */}
                    <SegmentedControl
                      size="sm"
                      wrap
                      frame="none"
                      ariaLabel="Transformation for all selected columns"
                      value={sharedTransformKind}
                      onChange={(next) => {
                        const chosen = TRANSFORMS.find((candidate) => candidate.value.kind === next)
                        if (chosen) dispatch({ type: 'all-series-transforms-selected', transform: chosen.value })
                      }}
                      options={TRANSFORMS.map((transform) => ({ value: transform.value.kind, label: transform.label }))}
                    />
                  </div>
                  <div>
                    {selectedIds.map((column) => {
                      const name = columnName(column)
                      const selected = seriesTransformFor(draft.seriesTransforms, column)
                      // The row fills under the pointer, as a table row does (`tr('action')`), so a name and its control read as one row across the width without a rule between rows.
                      return (
                        <div key={column} className="grid gap-x-3 gap-y-1 rounded-md px-3 py-1 transition-colors hover:bg-well has-[button:focus-visible]:bg-well @md/panel:grid-cols-[minmax(0,1fr)_auto] @md/panel:items-center">
                          <span className="min-w-0 text-body text-ink">{name}</span>
                          <SegmentedControl
                            size="sm"
                            wrap
                            frame="none"
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

      {preparedTimeSeries !== null && <PreparedSeriesPreview key={preparedTimeSeries.id} source={source} profile={profile} prepared={preparedTimeSeries} />}

      {(preparedCurrent !== null || timeSeriesSelected) && (
        <section className={panel('mt-4 p-(--panel-space)')} aria-labelledby="diagnostics-title">
          <div className="flex flex-wrap items-center justify-between gap-3">
            <div className="flex items-center gap-1.5">
              <h3 id="diagnostics-title" className={cn(sectionTitle, 'm-0')}>Diagnostics</h3>
              <ParameterHelp label="diagnostics" help="These tests do not change the prepared dataset." />
            </div>
            {/* A cross-section has one diagnostic, and one option is not a choice, so the switch appears only when there are several. */}
            {diagnosticOptions.length > 1 && (
              <SegmentedControl variant="line" size="sm" ariaLabel="Diagnostic" value={diagnostic} onChange={setDiagnostic} options={diagnosticOptions} />
            )}
          </div>
          {preparedCurrent === null && (
            <p role="status" className="mb-0 mt-3 text-body text-faint">Create a prepared dataset version to run these diagnostics.</p>
          )}
          <div hidden={diagnostic !== 'multicollinearity'} className="mt-4">
            {preparedCurrent !== null
              ? <MulticollinearityCard source={source} profile={profile} prepared={preparedCurrent} onSelection={applyMulticollinearitySelection} />
              : null}
          </div>
          <div hidden={diagnostic !== 'stationarity'} className="mt-4">
          <div>
            <h4 className="m-0 text-body font-medium text-ink">Stationarity tests</h4>
            <div className="mt-2 grid gap-3 @3xl/panel:grid-cols-2 @3xl/panel:gap-6">
              <p className={prose('m-0 text-faint')}>A stationary process has stable probabilistic behavior over time after accounting for the deterministic terms in the test. ADF tests a unit root as its null; KPSS tests stationarity as its null. Hirmos reads them together because either test alone can be inconclusive. Zivot–Andrews allows one structural break.</p>
              <p className={prose('m-0 text-faint')}>These tests assess the saved values. To test a different transformation, change it above and create a new prepared dataset version.</p>
            </div>
          </div>
          <MethodCaveats methods={STATIONARITY_METHODS} />
          <fieldset disabled={job.kind === 'running'} className="mt-4 min-w-0 border-0 p-0" aria-label="Stationarity variables">
            <legend className={fieldLabel}>Variables to test</legend>
            <div className="mb-2 flex items-center justify-between gap-3">
              <p className="m-0 text-body text-faint">Only selected variables are tested. The prepared dataset is unchanged.</p>
              <SelectionActions selectLabel="Select all stationarity variables" clearLabel="Clear stationarity variables"
                onSelectAll={() => selectTestColumns(preparedTimeSeries?.columns ?? [])} onClear={() => selectTestColumns([])} />
            </div>
            <div className="grid gap-1 @md/panel:grid-cols-2 @3xl/panel:grid-cols-3">
              {(preparedTimeSeries?.columns ?? []).map(column => <label key={column} className="flex items-center gap-2 rounded-md px-2 py-1.5 text-body text-ink hover:bg-well">
                <input type="checkbox" checked={testColumns.includes(column)} onChange={() => selectTestColumns(testColumns.includes(column) ? testColumns.filter(id => id !== column) : [...testColumns, column])} />
                <span className="min-w-0 break-words">{columnName(column)}</span>
              </label>)}
            </div>
          </fieldset>
          <div className="mt-3">
            <div className="mt-3 flex flex-wrap items-center gap-3">
              <button
                type="button"
                className={button(testColumns.length > 0 ? 'signal' : 'quiet')}
                disabled={preparedTimeSeries === null || testColumns.length === 0 || job.kind === 'running' || session.blocked}
                aria-busy={job.kind === 'running'}
                onClick={() => void runDiagnostics()}
              >
                Run stationarity tests
              </button>
              {job.kind === 'running' && <>
                <Orb state="solving" aria-label="Stationarity tests running" />
                <button type="button" className={button('quiet')} onClick={cancelDiagnostics}>Cancel tests</button>
                <span role="status" className="sr-only">{job.progress?.completed ?? 0} of {job.progress?.total ?? 0} variables completed</span>
              </>}
            </div>
          </div>
          {job.kind === 'failed' && <Alert tone="danger" className="mt-3"><p className="m-0">{job.detail}</p></Alert>}
          {job.kind === 'cancelled' && <Alert tone="info" className="mt-3"><p className="m-0">Stationarity tests cancelled. No new results were saved.</p></Alert>}
          {stationarityEvidence !== null && <section className="mt-5 space-y-4" aria-label="Stationarity results">
            <div className="flex items-center justify-between gap-3">
              <h4 className="m-0 text-title font-medium">Results</h4>
              <button type="button" className={button('quiet')} disabled={job.kind === 'running'} onClick={clearDiagnostics}>Clear results</button>
            </div>
            {stationarityEvidence.variables.map(evidence => {
              const column = profile.columns.find(candidate => candidate.id === evidence.column)
              const name = column?.name ?? String(evidence.column)
              const result = evidence.result
                const rows = [
                  {
                    name: 'ADF (constant)',
                    statistic: result.adf.constant.statistic,
                    p: result.adf.constant.pValue,
                    fit: `lag ${result.adf.constant.usedLag}, n ${result.adf.constant.observations}`,
                    critical: labelledCriticalValues('adf', result.adf.constant.criticalValues),
                  },
                  {
                    name: 'ADF (constant + trend)',
                    statistic: result.adf.constantAndTrend.statistic,
                    p: result.adf.constantAndTrend.pValue,
                    fit: `lag ${result.adf.constantAndTrend.usedLag}, n ${result.adf.constantAndTrend.observations}`,
                    critical: labelledCriticalValues('adf', result.adf.constantAndTrend.criticalValues),
                  },
                  {
                    name: 'KPSS (constant)',
                    statistic: result.kpss.constant.statistic,
                    p: result.kpss.constant.pValue,
                    fit: `lag ${result.kpss.constant.usedLag}`,
                    critical: labelledCriticalValues('kpss', result.kpss.constant.criticalValues),
                  },
                  {
                    name: 'KPSS (constant + trend)',
                    statistic: result.kpss.constantAndTrend.statistic,
                    p: result.kpss.constantAndTrend.pValue,
                    fit: `lag ${result.kpss.constantAndTrend.usedLag}`,
                    critical: labelledCriticalValues('kpss', result.kpss.constantAndTrend.criticalValues),
                  },
                  {
                    name: 'Zivot–Andrews (level)',
                    statistic: result.zivotAndrews.level.statistic,
                    p: result.zivotAndrews.level.pValue,
                    fit: `base lag ${result.zivotAndrews.level.baseLags}, break row ${result.zivotAndrews.level.breakIndex + 1}`,
                    critical: labelledCriticalValues('zivot-andrews', result.zivotAndrews.level.criticalValues),
                  },
                  {
                    name: 'Zivot–Andrews (trend)',
                    statistic: result.zivotAndrews.trend.statistic,
                    p: result.zivotAndrews.trend.pValue,
                    fit: `base lag ${result.zivotAndrews.trend.baseLags}, break row ${result.zivotAndrews.trend.breakIndex + 1}`,
                    critical: labelledCriticalValues('zivot-andrews', result.zivotAndrews.trend.criticalValues),
                  },
                  {
                    name: 'Zivot–Andrews (level + trend)',
                    statistic: result.zivotAndrews.levelAndTrend.statistic,
                    p: result.zivotAndrews.levelAndTrend.pValue,
                    fit: `base lag ${result.zivotAndrews.levelAndTrend.baseLags}, break row ${result.zivotAndrews.levelAndTrend.breakIndex + 1}`,
                    critical: labelledCriticalValues('zivot-andrews', result.zivotAndrews.levelAndTrend.criticalValues),
                  },
                ] as const
              return <article key={evidence.column} className={resultSurface('space-y-2 p-3')} aria-label={`Stationarity result for ${name}`}>
                <header className="flex flex-wrap items-center justify-between gap-x-3 gap-y-1">
                  <div className="min-w-0">
                    <h5 className="m-0 break-words text-body font-medium text-ink">{name}</h5>
                    <p className="m-0 text-label text-faint">{result.observations.toLocaleString()} rows{preparedTimeSeries === null ? '' : `, ${describeSeriesTransform(seriesTransformFor(preparedTimeSeries.seriesTransforms, evidence.column))}`}</p>
                  </div>
                  <StationarityVerdict assessment={evidence.assessment} transform={preparedTimeSeries === null ? { kind: 'levels' } : seriesTransformFor(preparedTimeSeries.seriesTransforms, evidence.column)} />
                  <button type="button" className={button('quiet')} aria-label={`Delete stationarity result for ${name}`} disabled={job.kind === 'running'} onClick={() => deleteDiagnostic(evidence.column)}><Icon name="delete" size={15} />Delete</button>
                </header>
                <MetricGrid className="stationarity-metrics" label={`Stationarity p-values for ${name}`}>
                  <MetricTile label="ADF p-value" value={formatP(result.adf.constant.pValue)} context="Constant" size="compact" />
                  <MetricTile label="KPSS p-value" value={formatP(result.kpss.constant.pValue)} context="Constant" size="compact" />
                  <MetricTile label="Zivot–Andrews p-value" value={formatP(result.zivotAndrews.levelAndTrend.pValue)} context="Constant and trend" size="compact" />
                </MetricGrid>
                <details className="group">
                  <summary className="flex cursor-pointer list-none items-center gap-1.5 text-body text-muted [&::-webkit-details-marker]:hidden"><Icon name="chevron_right" size={16} className="transition-transform group-open:rotate-90" />Test details for {name}</summary>
                  <div className="mt-3 overflow-x-auto"><TestStatisticsTable rows={rows} density={density} /></div>
                </details>
              </article>
            })}
          </section>}
          </div>
          <div hidden={diagnostic !== 'structure'} className="mt-4">
            {preparedTimeSeries !== null
              ? <SeriesStructureCard embedded source={source} profile={profile} prepared={preparedTimeSeries} />
              : null}
          </div>
          <div hidden={diagnostic !== 'granger'} className="mt-4">
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

      <JobNotice job={preparation.job} />
      {action.kind === 'blocked' ? <div id="preparation-requirement" className="mt-3"><Alert tone="info" live={false}>{action.reason}</Alert></div> : null}
      <div className="sticky bottom-3 z-(--z-sticky) ml-auto mt-4 flex w-fit max-w-full flex-wrap items-center justify-end gap-2">
        {preparation.job.kind === 'running' && <button type="button" className={button('quiet', 'bg-panel')} onClick={preparation.cancel}>Cancel preparation</button>}
        <button
          type="button"
          // It floats over the panel as the reader scrolls, so it carries the double shadow whatever its state.
          className={button('signal', 'float disabled:shadow-(--shadow-float)')}
          disabled={action.kind !== 'ready'}
          aria-describedby={action.kind === 'blocked' ? 'preparation-requirement' : undefined}
          aria-busy={preparation.job.kind === 'running'}
          onClick={() => void createPreparedVersion()}
        >
          Create prepared dataset version
        </button>
      </div>
    </section>
  )
}
