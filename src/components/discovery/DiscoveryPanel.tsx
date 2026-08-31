import { Orb } from '@/components/ui/Orb'
import { EmptyState } from '@/components/ui/EmptyState'
import { Select } from '@/components/ui/Select'
import { useReducer, useState, type ReactNode } from 'react'
import { Icon } from '@/components/Icon'
import { Alert } from '@/components/ui/Alert'
import { ResultInterpretation } from '@/components/ui/ResultInterpretation'
import { MethodCaveats } from '@/components/MethodCaveats'
import { EligibilityView } from '@/components/EligibilityView'
import { EvidenceTable, type EvidenceColumn } from '@/components/table/EvidenceTable'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { OcsePlot, StructurePlot, TimeGraphPlot, WeightPlot } from './DiscoveryPlots'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { button, field, figureGrid, label, literal, num } from '@/components/ui/recipes'
import type { DatasetProfile } from '@/domain/dataset'
import {
  DISCOVERY_LAG_OPTIONS,
  DYNOTEARS_PENALTY_OPTIONS,
  OCSE_SHUFFLE_OPTIONS,
  PCMCI_ALPHA_OPTIONS,
  describeDiscoveryReadiness,
  describeDiscoveryRunProblem,
  evaluateDiscoveryEligibility,
  initialDiscoveryDraftFor,
  newDiscoveryRunId,
  readyDiscoverySpecification,
  stepDiscovery,
  type DiscoveryConfiguration,
  type DiscoveryLag,
  type DiscoveryRunArtifact,
  type DynotearsPenalty,
  type OcseShuffles,
  type PcmciAlpha,
  type DiscoveryMethodChoice,
} from '@/domain/discovery'
import { assertNever } from '@/domain/dop'
import {
  DYNOTEARS_METHOD_ID,
  DIRECT_LINGAM_METHOD_ID,
  LPCMCI_PAR_CORR_METHOD_ID,
  OCSE_METHOD_ID,
  PCMCI_PLUS_PAR_CORR_METHOD_ID,
  VAR_LINGAM_METHOD_ID,
  methodDefinition,
  type MethodDefinition,
  type MethodEligibility,
} from '@/domain/methods'
import type { PreparedDatasetArtifact, StationarityEvidenceArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'
import { useRunActivity } from '@/lib/useRunActivity'
import { interpretDiscoveryResult } from '@/domain/resultInterpretation'
import type { RunActivity } from '@/domain/activity'
import { formatTimestamp } from '@/lib/format/date'
import { formatCount } from '@/lib/format/number'
import { cn } from '@/lib/utils'

const CROSS_SECTIONAL_DISCOVERY_METHODS: readonly (readonly [DiscoveryMethodChoice, string])[] = [['direct-lingam', 'DirectLiNGAM']]
const TEMPORAL_DISCOVERY_METHODS: readonly (readonly [DiscoveryMethodChoice, string])[] = [['pcmci-plus', 'PCMCI+'], ['lpcmci', 'LPCMCI'], ['dynotears', 'DYNOTEARS'], ['var-lingam', 'VAR-LiNGAM'], ['ocse', 'oCSE']]
const DISCOVERY_METHODS = [...CROSS_SECTIONAL_DISCOVERY_METHODS, ...TEMPORAL_DISCOVERY_METHODS] as const

interface DiscoveryPanelProps {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly stationarity: StationarityEvidenceArtifact | null
  readonly runs: readonly DiscoveryRunArtifact[]
  readonly onRun: (artifact: DiscoveryRunArtifact) => void
  readonly onActivity?: (activity: RunActivity | null) => void
}

const lagFromValue = (value: string): DiscoveryLag | null =>
  DISCOVERY_LAG_OPTIONS.find((candidate) => String(candidate) === value) ?? null

const alphaFromValue = (value: string): PcmciAlpha | null =>
  PCMCI_ALPHA_OPTIONS.find((candidate) => String(candidate) === value) ?? null

const penaltyFromValue = (value: string): DynotearsPenalty | null =>
  DYNOTEARS_PENALTY_OPTIONS.find((candidate) => String(candidate) === value) ?? null

const shufflesFromValue = (value: string): OcseShuffles | null =>
  OCSE_SHUFFLE_OPTIONS.find((candidate) => String(candidate) === value) ?? null

const methodIdForChoice = (method: DiscoveryMethodChoice) => {
  switch (method) {
    case 'direct-lingam': return DIRECT_LINGAM_METHOD_ID
    case 'pcmci-plus': return PCMCI_PLUS_PAR_CORR_METHOD_ID
    case 'lpcmci': return LPCMCI_PAR_CORR_METHOD_ID
    case 'dynotears': return DYNOTEARS_METHOD_ID
    case 'var-lingam': return VAR_LINGAM_METHOD_ID
    case 'ocse': return OCSE_METHOD_ID
    default: return assertNever(method)
  }
}

const methodIdOf = (configuration: DiscoveryConfiguration) => methodIdForChoice(configuration.kind)

const eligibilityLabel = (eligibility: MethodEligibility): string => {
  switch (eligibility.kind) {
    case 'eligible': return 'available'
    case 'caution': return 'review'
    case 'refused': return 'unavailable'
    default: return assertNever(eligibility)
  }
}

function DiscoveryMethodOptionLabel({ name, eligibility }: { readonly name: string; readonly eligibility: MethodEligibility }) {
  const tone = eligibility.kind === 'eligible' ? 'text-ok' : eligibility.kind === 'caution' ? 'text-warn' : 'text-danger/60'
  return <span>{name} <span className={cn('ml-1 text-label', tone)}>· {eligibilityLabel(eligibility)}</span></span>
}

const pValue = (value: number): string => value < 0.0001 ? '<0.0001' : value.toFixed(4)
const statistic = (value: number): string => Math.abs(value) >= 10_000 ? value.toExponential(4) : value.toFixed(6)

const asNumber = (value: string | number): number => (typeof value === 'number' ? value : Number(value))

interface LinkRow { readonly source: string; readonly target: string; readonly lag: number }

const linkColumns = <Row extends LinkRow>(): readonly EvidenceColumn<Row>[] => [
  { id: 'source', header: 'Source', value: (row) => row.source },
  { id: 'target', header: 'Target', value: (row) => row.target },
  { id: 'lag', header: 'Lag', align: 'right', value: (row) => row.lag },
]

const figureColumn = <Row,>(id: string, header: string, value: (row: Row) => number, print: (value: number) => string = statistic): EvidenceColumn<Row> =>
  ({ id, header, align: 'right', value, format: (value) => print(asNumber(value)) })


function ResultEligibility({ eligibility }: { readonly eligibility: MethodEligibility }) {
  switch (eligibility.kind) {
    case 'eligible': return <span className="text-ok">Available · {eligibility.satisfied.length} requirements checked</span>
    case 'caution': return <span className="text-warn">Available · {eligibility.unresolved.length} requirements to review</span>
    case 'refused': return <span className="text-danger">Unavailable · {eligibility.violations.length} requirements fail</span>
    default: return assertNever(eligibility)
  }
}

function RunRecord({ run }: { readonly run: DiscoveryRunArtifact }) {
  return (
    <details className="mt-3 rounded-lg border border-hair bg-well px-3 py-2 text-body">
      <summary className="cursor-pointer text-ink">Run details</summary>
      <dl className="mb-0 mt-2 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-micro text-faint">
        <dt>Run</dt><dd className={literal('m-0 break-all')}>{run.id}</dd>
        <dt>Prepared dataset</dt><dd className={literal('m-0 break-all')}>{run.preparedDataset}</dd>
        <dt>Created</dt><dd className={literal('m-0')}>{formatTimestamp(run.createdAt)}</dd>
        <dt>Method</dt><dd className={literal('m-0')}>{run.method}</dd>
      </dl>
    </details>
  )
}

/** One run's card: a disclosure whose summary carries the method, title and figures, so a reader can keep one run open and fold the rest. */
function ResultCard({ run, method, title, meta, open, current, children }: {
  readonly run: DiscoveryRunArtifact
  readonly method: string
  readonly title: ReactNode
  readonly meta: ReactNode
  readonly open: boolean
  /** The newest run: the one card that carries the emphasised border. */
  readonly current: boolean
  readonly children: ReactNode
}) {
  return (
    <article aria-labelledby={`run-${run.id}`}>
      <details className={`group rounded-xl border bg-panel ${current ? 'border-edge' : 'border-hair'}`} open={open}>
        <summary className="flex cursor-pointer list-none items-start gap-3 rounded-xl p-4 transition-colors hover:bg-well [&::-webkit-details-marker]:hidden">
          <Icon name="expand_more" size={16} className="mt-1 shrink-0 text-faint transition-transform duration-150 group-open:rotate-180" />
          <div className="min-w-0 flex-1">
            <span className={label('text-signal')}>{method}</span>
            <h3 id={`run-${run.id}`} className="mb-1 mt-1 text-title font-medium text-ink">{title}</h3>
            <p className="m-0 text-body text-faint">{meta}</p>
          </div>
        </summary>
        <div className="px-4 pb-4">
          <ResultInterpretation interpretation={interpretDiscoveryResult(run)} className="mb-3" />
          {children}
        </div>
      </details>
    </article>
  )
}

type TimeGraphRun = Extract<DiscoveryRunArtifact, { readonly kind: 'pcmci-plus-run' | 'lpcmci-run' }>

function TimeGraphResult({ run, open, current }: { readonly open: boolean; readonly current: boolean; readonly run: TimeGraphRun }) {
  const cells = run.result.graph.flatMap((targets, sourceIndex) =>
    targets.flatMap((lags, targetIndex) =>
      lags.map((mark, lag) => ({
        sourceIndex,
        targetIndex,
        lag,
        mark,
        p: run.result.pMatrix[sourceIndex][targetIndex][lag],
        value: run.result.valMatrix[sourceIndex][targetIndex][lag],
      })),
    ),
  )
  const rows = cells.map((cell) => ({ key: `${cell.sourceIndex}:${cell.targetIndex}:${cell.lag}`, source: run.variables[cell.sourceIndex].name, target: run.variables[cell.targetIndex].name, lag: cell.lag, mark: cell.mark, p: cell.p, value: cell.value }))
  const isLpcmci = run.kind === 'lpcmci-run'
  const methodLabel = isLpcmci ? 'LPCMCI · ParCorr' : 'PCMCI+ · ParCorr'
  const resultTitle = isLpcmci ? 'Latent-aware partial ancestral graph evidence' : 'Stationary lag-graph evidence'
  const tableLabel = isLpcmci ? 'LPCMCI raw evidence' : 'PCMCI+ raw evidence'
  return (
    <ResultCard run={run} open={open} current={current} method={methodLabel} title={resultTitle} meta={<>{formatCount(run.result.observations).text} rows · {run.result.variables} variables · maximum lag {run.result.tauMax} · alpha {run.result.pcAlpha}</>}>
      <p className="mb-0 mt-3 text-body"><ResultEligibility eligibility={run.eligibility} /></p>
      <RunRecord run={run} />
      <StructurePlot run={run} label={`${methodLabel} structure`} />
      <TimeGraphPlot run={run} />
      <p className="mb-3 mt-3 text-body text-muted">Empty marks appear as “—”. {isLpcmci ? 'The legend under the structure view defines each mark.' : 'Unoriented same-period links keep the o-o mark.'}</p>
      <EvidenceTable<typeof rows[number]>
        title={tableLabel}
        rows={rows}
        rowKey={(row) => row.key}
        noun="cell"
        empty="The run reported no cell."
        columns={[
          ...linkColumns<typeof rows[number]>(),
          { id: 'mark', header: 'Mark', mono: true, value: (row) => row.mark, format: (value) => (value === '' ? '—' : value) },
          figureColumn<typeof rows[number]>('p', 'p', (row) => row.p, pValue),
          figureColumn<typeof rows[number]>('value', 'ParCorr', (row) => row.value),
        ]}
      />
    </ResultCard>
  )
}

function DynotearsResult({ run, open, current }: { readonly open: boolean; readonly current: boolean; readonly run: Extract<DiscoveryRunArtifact, { readonly kind: 'dynotears-run' }> }) {
  const weights = [run.result.contemporaneousWeights, ...run.result.laggedWeights].flatMap((matrix, lag) =>
    matrix.flatMap((targets, source) => targets.map((weight, target) => ({ lag, source, target, weight }))),
  )
  const rows = weights.map((cell) => ({ key: `${cell.lag}:${cell.source}:${cell.target}`, source: run.variables[cell.source].name, target: run.variables[cell.target].name, lag: cell.lag, weight: cell.weight }))
  return (
    <ResultCard run={run} open={open} current={current} method="DYNOTEARS" title={<>Sparse dynamic structural equation model weights</>} meta={<>{formatCount(run.result.observations).text} rows · {run.result.variables} variables · maximum lag {run.result.maxLag} · λW {run.result.lambdaW} · λA {run.result.lambdaA}</>}>
      <p className="mb-0 mt-3 text-body"><ResultEligibility eligibility={run.eligibility} /></p>
      <RunRecord run={run} />
      <StructurePlot run={run} label="DYNOTEARS structure" />
      <WeightPlot run={run} />
      <p className="mb-3 mt-3 text-body text-muted">All fitted weights are retained without a display threshold. Lag 0 is contemporaneous; a row denotes source(t−lag) → target(t).</p>
      <EvidenceTable<typeof rows[number]>
        title="DYNOTEARS raw weights"
        rows={rows}
        rowKey={(row) => row.key}
        noun="weight"
        empty="The run reported no weight."
        columns={[...linkColumns<typeof rows[number]>(), figureColumn<typeof rows[number]>('weight', 'Weight', (row) => row.weight)]}
      />
    </ResultCard>
  )
}

function VarLingamResult({ run, open, current }: { readonly open: boolean; readonly current: boolean; readonly run: Extract<DiscoveryRunArtifact, { readonly kind: 'var-lingam-run' }> }) {
  const weights = [run.result.contemporaneousWeights, ...run.result.laggedWeights].flatMap((matrix, lag) =>
    matrix.flatMap((targets, source) => targets.map((weight, target) => ({ lag, source, target, weight }))),
  )
  const rows = weights.map((cell) => ({ key: `${cell.lag}:${cell.source}:${cell.target}`, source: run.variables[cell.source].name, target: run.variables[cell.target].name, lag: cell.lag, weight: cell.weight }))
  const order = run.result.causalOrder.map((index) => run.variables[index]?.name ?? String(index))
  return (
    <ResultCard run={run} open={open} current={current} method="VAR-LiNGAM" title={<>Non-Gaussian structural vector autoregression weights</>} meta={<>{formatCount(run.result.observations).text} rows · {run.result.variables} variables · Bayesian information criterion lag {run.result.selectedLag} of at most {run.result.lags} · {run.result.prune ? 'adaptive-lasso pruned' : 'unpruned'}</>}>
      <p className="mb-0 mt-3 text-body"><ResultEligibility eligibility={run.eligibility} /></p>
      <RunRecord run={run} />
      <p className="mb-1 mt-3 text-body text-muted">Contemporaneous causal order from residual non-Gaussianity:</p>
      <p className={num('mb-3 mt-0 text-body text-ink')} aria-label="VAR-LiNGAM causal order">{order.join(' → ')}</p>
      <StructurePlot run={run} label="VAR-LiNGAM structure" />
      <WeightPlot run={run} />
      <p className="mb-3 mt-3 text-body text-muted">Lag 0 is contemporaneous; a row denotes source(t−lag) → target(t).</p>
      <EvidenceTable<typeof rows[number]>
        title="VAR-LiNGAM raw weights"
        rows={rows}
        rowKey={(row) => row.key}
        noun="weight"
        empty="The run reported no weight."
        columns={[...linkColumns<typeof rows[number]>(), figureColumn<typeof rows[number]>('weight', 'Weight', (row) => row.weight)]}
      />
    </ResultCard>
  )
}

function DirectLingamResult({ run, open, current }: { readonly open: boolean; readonly current: boolean; readonly run: Extract<DiscoveryRunArtifact, { readonly kind: 'direct-lingam-run' }> }) {
  const rows = run.result.weights.flatMap((targets, source) => targets.map((weight, target) => ({
    key: `${source}:${target}`,
    source: run.variables[source].name,
    target: run.variables[target].name,
    weight,
  })))
  const order = run.result.causalOrder.map((index) => run.variables[index]?.name ?? String(index))
  return (
    <ResultCard run={run} open={open} current={current} method="DirectLiNGAM" title={<>Linear non-Gaussian directed structure</>} meta={<>{formatCount(run.result.observations).text} independent observations · {run.result.variables} variables · adaptive-lasso adjacency</>}>
      <p className="mb-0 mt-3 text-body"><ResultEligibility eligibility={run.eligibility} /></p>
      <RunRecord run={run} />
      <p className="mb-1 mt-3 text-body text-muted">Causal order inferred from non-Gaussianity:</p>
      <p className={num('mb-3 mt-0 text-body text-ink')} aria-label="DirectLiNGAM causal order">{order.join(' → ')}</p>
      <StructurePlot run={run} label="DirectLiNGAM structure" />
      <WeightPlot run={run} />
      <p className="mb-3 mt-3 text-body text-muted">A nonzero row denotes source → target. The coefficient is a fitted structural weight on the variables' observed scales, not an intervention-effect estimate.</p>
      <EvidenceTable<typeof rows[number]>
        title="DirectLiNGAM raw weights"
        rows={rows}
        rowKey={(row) => row.key}
        noun="weight"
        empty="The run reported no weight."
        columns={[
          { id: 'source', header: 'Source', value: (row) => row.source },
          { id: 'target', header: 'Target', value: (row) => row.target },
          figureColumn<typeof rows[number]>('weight', 'Weight', (row) => row.weight),
        ]}
      />
    </ResultCard>
  )
}

function OcseResult({ run, open, current }: { readonly open: boolean; readonly current: boolean; readonly run: Extract<DiscoveryRunArtifact, { readonly kind: 'ocse-run' }> }) {
  const rows = run.result.edges.map((edge, index) => ({ key: `${edge.source}:${edge.target}:${edge.lag}:${index}`, source: run.variables[edge.source].name, target: run.variables[edge.target].name, lag: edge.lag, cmi: edge.cmi, pValue: edge.pValue }))
  return (
    <ResultCard run={run} open={open} current={current} method="Optimal causation entropy" title={<>Conditional-information network evidence</>} meta={<>{formatCount(run.result.observations).text} rows · maximum lag {run.result.maxLag} · {run.result.method} conditional mutual information · {run.result.nShuffles} shuffles · seed {run.result.seed}</>}>
      <p className="mb-0 mt-3 text-body"><ResultEligibility eligibility={run.eligibility} /></p>
      <RunRecord run={run} />
      <StructurePlot run={run} label="oCSE structure" />
      <OcsePlot run={run} />
      <p className="mb-3 mt-3 text-body text-muted">Selected lagged relations can be reviewed in the DAG workspace. They are not estimates of intervention effects.</p>
      <EvidenceTable<typeof rows[number]>
        title="oCSE raw evidence"
        rows={rows}
        rowKey={(row) => row.key}
        noun="edge"
        empty="No edge survived forward and backward selection."
        columns={[...linkColumns<typeof rows[number]>(), figureColumn<typeof rows[number]>('cmi', 'CMI', (row) => row.cmi), figureColumn<typeof rows[number]>('p', 'p-value', (row) => row.pValue, pValue)]}
      />
    </ResultCard>
  )
}

function DiscoveryResult({ run, open, current }: { readonly open: boolean; readonly current: boolean; readonly run: DiscoveryRunArtifact }) {
  switch (run.kind) {
    case 'direct-lingam-run': return <DirectLingamResult run={run} open={open} current={current} />
    case 'pcmci-plus-run': return <TimeGraphResult run={run} open={open} current={current} />
    case 'lpcmci-run': return <TimeGraphResult run={run} open={open} current={current} />
    case 'dynotears-run': return <DynotearsResult run={run} open={open} current={current} />
    case 'var-lingam-run': return <VarLingamResult run={run} open={open} current={current} />
    case 'ocse-run': return <OcseResult run={run} open={open} current={current} />
    default: return assertNever(run)
  }
}

export function DiscoveryPanel({ source, profile, prepared, stationarity, runs, onRun, onActivity }: DiscoveryPanelProps) {
  const [draft, dispatch] = useReducer(stepDiscovery, prepared, initialDiscoveryDraftFor)
  const [expanded, setExpanded] = useState<'latest' | 'all' | 'none'>('latest')
  useRunActivity(onActivity, draft.job.kind === 'running' ? { label: DISCOVERY_METHODS.find(([value]) => value === draft.configuration.kind)?.[1] ?? 'Discovery', progress: draft.job.progress === null ? null : draft.job.progress.completed / Math.max(1, draft.job.progress.total) } : null)
  const preparedColumns = profile.columns.filter((column) => prepared.columns.includes(column.id))
  const configuration = draft.configuration
  const methodId = methodIdOf(configuration)
  const selectedMethod = methodDefinition(methodId)
  if (!selectedMethod.ok) {
    return <p role="alert" className="text-body text-danger">The selected discovery method is not registered.</p>
  }
  const method: MethodDefinition = selectedMethod.value
  const eligibility = evaluateDiscoveryEligibility(method, prepared, stationarity)
  const readiness = readyDiscoverySpecification(configuration, prepared)
  const methodOptions = DISCOVERY_METHODS.flatMap(([value, name]) => {
    const definition = methodDefinition(methodIdForChoice(value))
    if (!definition.ok) return []
    const candidateEligibility = evaluateDiscoveryEligibility(definition.value, prepared, stationarity)
    return [{
      value,
      label: <DiscoveryMethodOptionLabel name={name} eligibility={candidateEligibility} />,
      disabled: candidateEligibility.kind === 'refused',
      title: candidateEligibility.kind === 'refused'
        ? `${name}: ${candidateEligibility.violations[0]?.evidence ?? 'a requirement is not met'}`
        : `${name}: ${eligibilityLabel(candidateEligibility)}`,
    }]
  })

  const execute = async () => {
    const specification = readyDiscoverySpecification(configuration, prepared)
    if (!specification.ok || eligibility.kind === 'refused') return
    dispatch({ type: 'run-started' })
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, analysis] = await Promise.all([
        import('@/data/prepared'),
        import('@/analysis/client'),
      ])
      const matrix = await materialisePrepared(source, profile, prepared, prepared.columns)
      if (!matrix.ok) {
        dispatch(matrix.error.kind === 'missing-values-remain'
          ? { type: 'run-failed', problem: { kind: 'missing-values-remain', cells: matrix.error.cells } }
          : { type: 'run-failed', problem: { kind: 'materialization-refused', detail: describePreparedMaterialisationProblem(matrix.error) } })
        return
      }

      switch (specification.value.kind) {
      case 'direct-lingam': {
        const result = await analysis.runDirectLingam(matrix.value.values, matrix.value.rowCount, matrix.value.columns.length, (progress) => dispatch({ type: 'run-progressed', progress }))
        if (!result.ok) {
          dispatch({ type: 'run-failed', problem: { kind: 'analysis-refused', detail: result.error.detail } })
          return
        }
        const artifact: DiscoveryRunArtifact = { kind: 'direct-lingam-run', id: newDiscoveryRunId(), preparedDataset: prepared.id, createdAt: new Date().toISOString(), method: DIRECT_LINGAM_METHOD_ID, variables: matrix.value.columns, eligibility, result: result.value }
        dispatch({ type: 'run-succeeded', artifact })
        onRun(artifact)
        return
      }
      case 'pcmci-plus': {
        const result = await analysis.runPcmciPlus(
          matrix.value.values,
          matrix.value.rowCount,
          matrix.value.columns.length,
          specification.value.tauMax,
          specification.value.pcAlpha,
        )
        if (!result.ok) {
          dispatch({ type: 'run-failed', problem: { kind: 'analysis-refused', detail: result.error.detail } })
          return
        }
        const artifact: DiscoveryRunArtifact = {
          kind: 'pcmci-plus-run',
          id: newDiscoveryRunId(),
          preparedDataset: prepared.id,
          createdAt: new Date().toISOString(),
          method: PCMCI_PLUS_PAR_CORR_METHOD_ID,
          variables: matrix.value.columns,
          eligibility,
          result: result.value,
        }
        dispatch({ type: 'run-succeeded', artifact })
        onRun(artifact)
        return
      }
      case 'lpcmci': {
        const result = await analysis.runLpcmci(matrix.value.values, matrix.value.rowCount, matrix.value.columns.length, specification.value.tauMax, specification.value.pcAlpha, (progress) => dispatch({ type: 'run-progressed', progress }))
        if (!result.ok) {
          dispatch({ type: 'run-failed', problem: { kind: 'analysis-refused', detail: result.error.detail } })
          return
        }
        const artifact: DiscoveryRunArtifact = { kind: 'lpcmci-run', id: newDiscoveryRunId(), preparedDataset: prepared.id, createdAt: new Date().toISOString(), method: LPCMCI_PAR_CORR_METHOD_ID, variables: matrix.value.columns, eligibility, result: result.value }
        dispatch({ type: 'run-succeeded', artifact })
        onRun(artifact)
        return
      }
      case 'dynotears': {
        const result = await analysis.runDynotears(matrix.value.values, matrix.value.rowCount, matrix.value.columns.length, specification.value.maxLag, specification.value.lambdaW, specification.value.lambdaA, (progress) => dispatch({ type: 'run-progressed', progress }))
        if (!result.ok) {
          dispatch({ type: 'run-failed', problem: { kind: 'analysis-refused', detail: result.error.detail } })
          return
        }
        const artifact: DiscoveryRunArtifact = { kind: 'dynotears-run', id: newDiscoveryRunId(), preparedDataset: prepared.id, createdAt: new Date().toISOString(), method: DYNOTEARS_METHOD_ID, variables: matrix.value.columns, eligibility, result: result.value }
        dispatch({ type: 'run-succeeded', artifact })
        onRun(artifact)
        return
      }
      case 'var-lingam': {
        const result = await analysis.runVarLingam(matrix.value.values, matrix.value.rowCount, matrix.value.columns.length, specification.value.maxLag, specification.value.prune, (progress) => dispatch({ type: 'run-progressed', progress }))
        if (!result.ok) {
          dispatch({ type: 'run-failed', problem: { kind: 'analysis-refused', detail: result.error.detail } })
          return
        }
        const artifact: DiscoveryRunArtifact = { kind: 'var-lingam-run', id: newDiscoveryRunId(), preparedDataset: prepared.id, createdAt: new Date().toISOString(), method: VAR_LINGAM_METHOD_ID, variables: matrix.value.columns, eligibility, result: result.value }
        dispatch({ type: 'run-succeeded', artifact })
        onRun(artifact)
        return
      }
      case 'ocse': {
        const result = await analysis.runOcse(matrix.value.values, matrix.value.rowCount, matrix.value.columns.length, specification.value.maxLag, specification.value.alpha, specification.value.nShuffles, specification.value.method, specification.value.k, (progress) => dispatch({ type: 'run-progressed', progress }))
        if (!result.ok) {
          dispatch({ type: 'run-failed', problem: { kind: 'analysis-refused', detail: result.error.detail } })
          return
        }
        const artifact: DiscoveryRunArtifact = { kind: 'ocse-run', id: newDiscoveryRunId(), preparedDataset: prepared.id, createdAt: new Date().toISOString(), method: OCSE_METHOD_ID, variables: matrix.value.columns, eligibility, result: result.value }
        dispatch({ type: 'run-succeeded', artifact })
        onRun(artifact)
        return
      }
        default: return assertNever(specification.value)
      }
    } catch (cause: unknown) {
      dispatch({
        type: 'run-failed',
        problem: {
          kind: 'execution-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        },
      })
    }
  }

  const inspector = (
    <div className="flex flex-col gap-4">
      <section aria-labelledby="prepared-input-title">
        <h3 id="prepared-input-title" className="mb-3 mt-0 text-body font-medium text-ink">Prepared dataset</h3>
        <dl className={figureGrid('m-0 grid-cols-2')}>
          <div className="bg-panel px-3 py-2"><dt className={label('text-faint')}>Structure</dt><dd className="m-0 mt-1 text-body text-ink">{prepared.kind === 'prepared-time-series' ? `Regular ${prepared.sampling.frequency} series` : prepared.kind === 'prepared-panel' ? `Panel · ${prepared.panel.units} units × ${prepared.panel.periods} periods` : 'Independent observations'}</dd></div>
          <div className="bg-panel px-3 py-2"><dt className={label('text-faint')}>Rows</dt><dd className={num('m-0 mt-1 text-title text-ink')}>{formatCount(prepared.observations).text}</dd></div>
          <div className="bg-panel px-3 py-2"><dt className={label('text-faint')}>Variables</dt><dd className={num('m-0 mt-1 text-title text-ink')}>{prepared.columns.length}</dd></div>
          <div className="bg-panel px-3 py-2"><dt className={label('text-faint')}>Stationarity</dt><dd className="m-0 mt-1 text-body text-ink">{prepared.kind !== 'prepared-time-series' ? 'Not applicable' : stationarity === null ? 'Tests not run' : `${formatCount(stationarity.observations).text} rows tested`}</dd></div>
        </dl>
        <p className={literal('mb-0 mt-3 break-all text-micro text-faint')}>Dataset version {prepared.id.slice(0, 8)}</p>
      </section>
      <MethodCaveats methods={[method]} eligibility={eligibility} />
    </div>
  )

  const stage = (
    <section aria-labelledby="discovery-title" className="@container/panel flex flex-col gap-5">
      <div>
        <span className={label('text-signal')}>03 · Discovery lab</span>
        <h2 id="discovery-title" className="mb-2 mt-2 text-heading text-ink">Examine candidate relationships</h2>
        <p className="m-0 max-w-[65ch] text-body text-muted">Causal discovery uses patterns in data to propose relations between variables, including same-period and lagged relations when time is part of the study. In this chapter, choose a method suited to the observation structure and compare the candidate relations it produces. The result depends on the method's assumptions and does not establish a causal graph on its own.</p>
      </div>

      <div className="grid gap-4">
        <section className="rounded-xl border border-hair bg-panel p-4" aria-labelledby="discovery-method-title">
            <h3 id="discovery-method-title" className="mb-3 mt-0 text-title font-medium text-ink">Discovery method</h3>
          <SegmentedControl
            className="mt-1"
            ariaLabel="Discovery method"
            wrap
            value={configuration.kind}
            onChange={(method) => dispatch({ type: 'method-selected', method })}
            options={methodOptions}
          />
          <p className="mb-0 mt-2 max-w-[65ch] text-body text-faint">Available: pre-run checks completed. Review: runnable, with conditions to assess. Unavailable: the prepared observation structure does not meet a method requirement.</p>

          {(configuration.kind === 'pcmci-plus' || configuration.kind === 'lpcmci') && (
            <div className="mt-4 grid gap-3 @md/panel:grid-cols-2">
              <label className="text-body text-ink">
                Maximum lag
                <Select
                  className={field('text', 'mt-1')}
                  value={configuration.tauMax}
                  onChange={(event) => {
                    const value = lagFromValue(event.target.value)
                    if (value !== null) dispatch({ type: 'tau-max-selected', value })
                  }}
                >
                  {DISCOVERY_LAG_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </label>
              <label className="text-body text-ink">
                PC alpha
                <Select
                  className={field('text', 'mt-1')}
                  value={configuration.pcAlpha}
                  onChange={(event) => {
                    const value = alphaFromValue(event.target.value)
                    if (value !== null) dispatch({ type: 'pc-alpha-selected', value })
                  }}
                >
                  {PCMCI_ALPHA_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </label>
            </div>
          )}

          {configuration.kind === 'dynotears' && (
            <div className="mt-4 grid gap-3 @2xl/panel:grid-cols-3">
              <label className="text-body text-ink">
                Maximum lag
                <Select className={field('text', 'mt-1')} value={configuration.maxLag} onChange={(event) => {
                  const value = lagFromValue(event.target.value)
                  if (value !== null) dispatch({ type: 'max-lag-selected', value })
                }}>
                  {DISCOVERY_LAG_OPTIONS.filter((value) => value <= 6).map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </label>
              <label className="text-body text-ink">
                Contemporaneous λ
                <Select className={field('text', 'mt-1')} value={configuration.lambdaW} onChange={(event) => {
                  const value = penaltyFromValue(event.target.value)
                  if (value !== null) dispatch({ type: 'dynotears-lambda-w-selected', value })
                }}>
                  {DYNOTEARS_PENALTY_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </label>
              <label className="text-body text-ink">
                Lagged λ
                <Select className={field('text', 'mt-1')} value={configuration.lambdaA} onChange={(event) => {
                  const value = penaltyFromValue(event.target.value)
                  if (value !== null) dispatch({ type: 'dynotears-lambda-a-selected', value })
                }}>
                  {DYNOTEARS_PENALTY_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </label>
            </div>
          )}

          {configuration.kind === 'var-lingam' && (
            <div className="mt-4 grid gap-3 @md/panel:grid-cols-2">
              <label className="text-body text-ink">
                Maximum lag
                <Select className={field('text', 'mt-1')} value={configuration.maxLag} onChange={(event) => {
                  const value = lagFromValue(event.target.value)
                  if (value !== null) dispatch({ type: 'max-lag-selected', value })
                }}>
                  {DISCOVERY_LAG_OPTIONS.filter((value) => value <= 6).map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </label>
              <label className="flex items-start gap-2 self-end pb-2 text-body text-ink">
                <input type="checkbox" className="mt-1" checked={configuration.prune} onChange={(event) => dispatch({ type: 'var-lingam-prune-selected', value: event.target.checked })} />
                <span>Adaptive-lasso pruning<span className="block text-faint">BIC selects the lag order up to the maximum.</span></span>
              </label>
            </div>
          )}

          {configuration.kind === 'ocse' && (
            <div className="mt-4 grid gap-3 @md/panel:grid-cols-2">
              <label className="text-body text-ink">
                Maximum lag
                <Select className={field('text', 'mt-1')} value={configuration.maxLag} onChange={(event) => {
                  const value = lagFromValue(event.target.value)
                  if (value !== null) dispatch({ type: 'max-lag-selected', value })
                }}>
                  {DISCOVERY_LAG_OPTIONS.filter((value) => value <= 8).map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </label>
              <label className="text-body text-ink">
                Information estimator
                <Select className={field('text', 'mt-1')} value={configuration.method} onChange={(event) => {
                  if (event.target.value === 'gaussian' || event.target.value === 'knn') dispatch({ type: 'ocse-method-selected', value: event.target.value })
                }}>
                  <option value="gaussian">Gaussian conditional mutual information</option>
                  <option value="knn">k-nearest neighbours (k = 5)</option>
                </Select>
              </label>
              <label className="text-body text-ink">
                Test alpha
                <Select className={field('text', 'mt-1')} value={configuration.alpha} onChange={(event) => {
                  const value = alphaFromValue(event.target.value)
                  if (value !== null) dispatch({ type: 'ocse-alpha-selected', value })
                }}>
                  {PCMCI_ALPHA_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </label>
              <label className="text-body text-ink">
                Permutation shuffles
                <Select className={field('text', 'mt-1')} value={configuration.nShuffles} onChange={(event) => {
                  const value = shufflesFromValue(event.target.value)
                  if (value !== null) dispatch({ type: 'ocse-shuffles-selected', value })
                }}>
                  {OCSE_SHUFFLE_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </label>
            </div>
          )}


          <EligibilityView eligibility={eligibility} />
          {!readiness.ok && <p role="status" className="mb-0 mt-3 text-body text-faint">{describeDiscoveryReadiness(readiness.error)}</p>}
          {draft.job.kind === 'failed' && <Alert tone="danger" className="mt-3"><p className="m-0">{describeDiscoveryRunProblem(draft.job.problem)}</p></Alert>}
          {draft.job.kind === 'running' && draft.job.progress !== null && (
            <div className="mt-3" role="status" aria-live="polite">
              <div className="mb-1 flex items-center justify-between gap-3 text-micro text-faint">
                <span>{draft.job.progress.stage}</span>
                <span className={num()}>{draft.job.progress.completed} / {draft.job.progress.total}</span>
              </div>
              <progress className="block h-1.5 w-full accent-signal" max={draft.job.progress.total} value={draft.job.progress.completed} />
            </div>
          )}
          <div className="mt-4 flex items-center gap-3">
            <button
              type="button"
              className={button('signal')}
              disabled={!readiness.ok || eligibility.kind === 'refused'}
              aria-busy={draft.job.kind === 'running'}
              onClick={draft.job.kind === 'running' ? undefined : () => void execute()}
            >
              Run {method.name}
            </button>
            {draft.job.kind === 'running' && <Orb state="searching" aria-label="Discovery method running" />}
          </div>
        </section>
      </div>

      <section aria-labelledby="discovery-runs-title">
        <div className="mb-3 flex items-end justify-between gap-3">
          <div>
            <span className={label('text-faint')}>Discovery runs</span>
            <h2 id="discovery-runs-title" className="mb-0 mt-1 text-title font-medium text-ink">Results</h2>
          </div>
          <div className="flex items-center gap-2">
            {runs.length > 1 && (
              <>
                <button type="button" className={button('quiet', 'h-7 px-2 text-label')} onClick={() => setExpanded('all')}>Expand all</button>
                <button type="button" className={button('quiet', 'h-7 px-2 text-label')} onClick={() => setExpanded('none')}>Collapse all</button>
              </>
            )}
            <span className={num('text-body text-faint')}>{runs.length} run{runs.length === 1 ? '' : 's'}</span>
          </div>
        </div>
        {runs.length === 0 ? (
          <EmptyState>Choose a method and run discovery.</EmptyState>
        ) : (
          <div className="space-y-4">
            {[...runs].reverse().map((run, index) => (
              <DiscoveryResult key={`${run.id}:${expanded}`} run={run} open={expanded === 'all' || (expanded === 'latest' && index === 0)} current={index === 0} />
            ))}
          </div>
        )}
      </section>
    </section>
  )

  return <WorkbenchLayout id="discovery" stage={stage} inspector={{ title: 'Prepared dataset and method requirements', body: inspector }} />
}
