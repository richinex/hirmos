import { Orb } from '@/components/ui/Orb'
import { EmptyState } from '@/components/ui/EmptyState'
import { Select } from '@/components/ui/Select'
import { useMemo, useReducer, useState, type ReactNode } from 'react'
import { Icon } from '@/components/Icon'
import { Alert } from '@/components/ui/Alert'
import { ResultInterpretation } from '@/components/ui/ResultInterpretation'
import { MethodCaveats } from '@/components/MethodCaveats'
import { EligibilityView } from '@/components/EligibilityView'
import { EvidenceTable, type EvidenceColumn } from '@/components/table/EvidenceTable'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { ParameterHelp, ParameterLabel } from '@/components/ui/ParameterLabel'
import {
  EVIDENCE_SCOPES,
  explainEvidenceScope,
  describeEvidenceScope,
  matchesEvidenceSelection,
  NO_EVIDENCE_SELECTION,
  selectsEverything,
  type EvidenceScope,
  type EvidenceSelection,
} from '@/domain/evidenceScope'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { OcsePlot, RpcmciMembershipPlot, RpcmciTimeGraphPlot, StructurePlot, TimeGraphPlot, WeightPlot } from './DiscoveryPlots'
import { RadioList } from '@/components/ui/RadioList'
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
  RPCMCI_PAR_CORR_METHOD_ID,
  OCSE_METHOD_ID,
  PCMCI_PLUS_PAR_CORR_METHOD_ID,
  VAR_LINGAM_METHOD_ID,
  methodDefinition,
  type MethodDefinition,
  type MethodEligibility,
} from '@/domain/methods'
import { describeSeriesTransform, type PreparedDatasetArtifact, type StationarityEvidenceArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'
import { useRunActivity } from '@/lib/useRunActivity'
import { interpretDiscoveryResult } from '@/domain/resultInterpretation'
import type { RunActivity } from '@/domain/activity'
import { formatTimestamp } from '@/lib/format/date'
import { formatCount } from '@/lib/format/number'
import { DISCOVERY_PARAMETER_HELP } from '@/domain/parameterHelp'

const CROSS_SECTIONAL_DISCOVERY_METHODS: readonly (readonly [DiscoveryMethodChoice, string])[] = [['direct-lingam', 'DirectLiNGAM']]
const TEMPORAL_DISCOVERY_METHODS: readonly (readonly [DiscoveryMethodChoice, string])[] = [['pcmci-plus', 'PCMCI+'], ['lpcmci', 'LPCMCI'], ['rpcmci', 'RPCMCI'], ['dynotears', 'DYNOTEARS'], ['var-lingam', 'VAR-LiNGAM'], ['ocse', 'oCSE']]
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
    case 'rpcmci': return RPCMCI_PAR_CORR_METHOD_ID
    case 'dynotears': return DYNOTEARS_METHOD_ID
    case 'var-lingam': return VAR_LINGAM_METHOD_ID
    case 'ocse': return OCSE_METHOD_ID
    default: return assertNever(method)
  }
}

const methodIdOf = (configuration: DiscoveryConfiguration) => methodIdForChoice(configuration.kind)

const eligibilityHint = (eligibility: MethodEligibility): string => {
  switch (eligibility.kind) {
    case 'eligible': return 'Available: pre-run checks completed.'
    case 'caution': return 'Review: runnable, with conditions to assess.'
    case 'refused': return `Unavailable: ${eligibility.violations[0]?.evidence ?? 'a requirement is not met.'}`
    default: return assertNever(eligibility)
  }
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

type RpcmciConfiguration = Extract<DiscoveryConfiguration, { readonly kind: 'rpcmci' }>

function RpcmciControls({ configuration, onChange }: {
  readonly configuration: RpcmciConfiguration
  readonly onChange: (configuration: RpcmciConfiguration) => void
}) {
  const changeNumber = (fieldName: Exclude<keyof RpcmciConfiguration, 'kind'>, value: number) => {
    if (Number.isFinite(value)) onChange({ ...configuration, [fieldName]: value })
  }
  return (
    <div className="mt-4 grid gap-3 @md/panel:grid-cols-2 @2xl/panel:grid-cols-3">
      <div className="text-body text-ink">
        <ParameterLabel label="Regimes" help={DISCOVERY_PARAMETER_HELP.rpcmci.regimes} htmlFor="rpcmci-regimes" />
        <Select id="rpcmci-regimes" className={field('text', 'mt-1')} value={configuration.numRegimes} onChange={(event) => changeNumber('numRegimes', Number(event.target.value))}>
          {[2, 3, 4, 5, 6].map((value) => <option key={value} value={value}>{value}</option>)}
        </Select>
      </div>
      <div className="text-body text-ink">
        <ParameterLabel label="Maximum transitions" help={DISCOVERY_PARAMETER_HELP.rpcmci.maximumTransitions} htmlFor="rpcmci-maximum-transitions" />
        <input id="rpcmci-maximum-transitions" className={field('text', 'mt-1')} type="number" min={0} step={1} value={configuration.maxTransitions} onChange={(event) => changeNumber('maxTransitions', event.currentTarget.valueAsNumber)} />
      </div>
      <div className="text-body text-ink">
        <ParameterLabel label="Maximum lag" help={DISCOVERY_PARAMETER_HELP.rpcmci.maximumLag} htmlFor="rpcmci-maximum-lag" />
        <Select id="rpcmci-maximum-lag" className={field('text', 'mt-1')} value={configuration.tauMax} onChange={(event) => changeNumber('tauMax', Number(event.target.value))}>
          {DISCOVERY_LAG_OPTIONS.filter((value) => value <= 6).map((value) => <option key={value} value={value}>{value}</option>)}
        </Select>
      </div>
      <div className="text-body text-ink">
        <ParameterLabel label="Graph alpha" help={DISCOVERY_PARAMETER_HELP.rpcmci.graphAlpha} htmlFor="rpcmci-graph-alpha" />
        <Select id="rpcmci-graph-alpha" className={field('text', 'mt-1')} value={configuration.alphaLevel} onChange={(event) => changeNumber('alphaLevel', Number(event.target.value))}>
          {PCMCI_ALPHA_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
        </Select>
      </div>
      <details className="@md/panel:col-span-2 @2xl/panel:col-span-3 rounded-lg border border-hair bg-well px-3 py-2">
        <summary className="cursor-pointer text-body text-ink">Annealing and conditional-independence settings</summary>
        <div className="mt-3 grid gap-3 @md/panel:grid-cols-2 @2xl/panel:grid-cols-3">
          <div className="text-body text-ink">
            <ParameterLabel label="Minimum lag" help={DISCOVERY_PARAMETER_HELP.rpcmci.minimumLag} htmlFor="rpcmci-minimum-lag" />
            <Select id="rpcmci-minimum-lag" className={field('text', 'mt-1')} value={configuration.tauMin} onChange={(event) => changeNumber('tauMin', Number(event.target.value))}>
              {Array.from({ length: configuration.tauMax + 1 }, (_, value) => value).map((value) => <option key={value} value={value}>{value}</option>)}
            </Select>
          </div>
          <div className="text-body text-ink">
            <ParameterLabel label="PC alpha" help={DISCOVERY_PARAMETER_HELP.rpcmci.pcAlpha} htmlFor="rpcmci-pc-alpha" />
            <Select id="rpcmci-pc-alpha" className={field('text', 'mt-1')} value={configuration.pcAlpha} onChange={(event) => changeNumber('pcAlpha', Number(event.target.value))}>
              {PCMCI_ALPHA_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
            </Select>
          </div>
          <div className="text-body text-ink">
            <ParameterLabel label="Switching threshold" help={DISCOVERY_PARAMETER_HELP.rpcmci.switchingThreshold} htmlFor="rpcmci-switching-threshold" />
            <input id="rpcmci-switching-threshold" className={field('text', 'mt-1')} type="number" min={0} max={1} step={0.01} value={configuration.switchThres} onChange={(event) => changeNumber('switchThres', event.currentTarget.valueAsNumber)} />
          </div>
          <div className="text-body text-ink">
            <ParameterLabel label="Iterations per annealing" help={DISCOVERY_PARAMETER_HELP.rpcmci.iterationsPerAnnealing} htmlFor="rpcmci-iterations" />
            <input id="rpcmci-iterations" className={field('text', 'mt-1')} type="number" min={1} max={100} step={1} value={configuration.numIterations} onChange={(event) => changeNumber('numIterations', event.currentTarget.valueAsNumber)} />
          </div>
          <div className="text-body text-ink">
            <ParameterLabel label="Annealing runs" help={DISCOVERY_PARAMETER_HELP.rpcmci.annealingRuns} htmlFor="rpcmci-annealing-runs" />
            <input id="rpcmci-annealing-runs" className={field('text', 'mt-1')} type="number" min={1} max={50} step={1} value={configuration.maxAnneal} onChange={(event) => changeNumber('maxAnneal', event.currentTarget.valueAsNumber)} />
          </div>
          <div className="text-body text-ink">
            <ParameterLabel label="Seed" help={DISCOVERY_PARAMETER_HELP.rpcmci.seed} htmlFor="rpcmci-seed" />
            <input id="rpcmci-seed" className={field('text', 'mt-1')} type="number" min={0} step={1} value={configuration.seed} onChange={(event) => changeNumber('seed', event.currentTarget.valueAsNumber)} />
          </div>
        </div>
      </details>
    </div>
  )
}


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
            <span className={label(current ? 'text-signal' : 'text-faint')}>{method}</span>
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

/**
 * The scope, source, target and lag controls for a lag-graph evidence table.
 *
 * Every ordered pair appears at every lag, so the unfiltered table is mostly absences. These
 * controls answer the questions a reader actually brings to it.
 */
function EvidenceScopeControls({ selection, onChange, variables, tauMax, alpha }: {
  readonly selection: EvidenceSelection
  readonly onChange: (next: EvidenceSelection) => void
  readonly variables: readonly string[]
  readonly tauMax: number
  readonly alpha: number
}) {
  const lags = Array.from({ length: tauMax + 1 }, (_, lag) => lag)
  return (
    <>
      <SegmentedControl<EvidenceScope['kind']>
        size="sm"
        ariaLabel="Which cells to show"
        value={selection.scope.kind}
        onChange={(kind) => onChange({ ...selection, scope: EVIDENCE_SCOPES.find((scope) => scope.kind === kind) ?? { kind: 'all' } })}
        options={EVIDENCE_SCOPES.map((scope) => ({ value: scope.kind, label: describeEvidenceScope(scope), title: explainEvidenceScope(scope, alpha) }))}
      />
      <Select className={field('text', 'w-28')} aria-label="Filter by source" value={selection.source ?? ''} onChange={(event) => onChange({ ...selection, source: event.target.value === '' ? null : event.target.value })}>
        <option value="">Any source</option>
        {variables.map((name) => <option key={name} value={name}>{name}</option>)}
      </Select>
      <Select className={field('text', 'w-28')} aria-label="Filter by target" value={selection.target ?? ''} onChange={(event) => onChange({ ...selection, target: event.target.value === '' ? null : event.target.value })}>
        <option value="">Any target</option>
        {variables.map((name) => <option key={name} value={name}>{name}</option>)}
      </Select>
      <Select className={field('text', 'w-24')} aria-label="Filter by lag" value={selection.lag === null ? '' : String(selection.lag)} onChange={(event) => onChange({ ...selection, lag: event.target.value === '' ? null : Number(event.target.value) })}>
        <option value="">Any lag</option>
        {lags.map((lag) => <option key={lag} value={lag}>lag {lag}</option>)}
      </Select>
    </>
  )
}

function TimeGraphResult({ run, open, current }: { readonly open: boolean; readonly current: boolean; readonly run: TimeGraphRun }) {
  const [selection, setSelection] = useState<EvidenceSelection>(NO_EVIDENCE_SELECTION)
  const rows = useMemo(
    () => run.result.graph.flatMap((targets, sourceIndex) =>
      targets.flatMap((lags, targetIndex) =>
        lags.map((mark, lag) => ({
          key: `${sourceIndex}:${targetIndex}:${lag}`,
          source: run.variables[sourceIndex].name,
          target: run.variables[targetIndex].name,
          lag,
          mark,
          p: run.result.pMatrix[sourceIndex][targetIndex][lag],
          value: run.result.valMatrix[sourceIndex][targetIndex][lag],
        })),
      ),
    ),
    [run],
  )
  const selected = useMemo(
    () => (selectsEverything(selection) ? rows : rows.filter((row) => matchesEvidenceSelection(row, selection, run.result.pcAlpha))),
    [rows, selection, run.result.pcAlpha],
  )
  const variableNames = useMemo(() => run.variables.map((variable) => variable.name), [run])
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
        rows={selected}
        total={rows.length}
        rowKey={(row) => row.key}
        noun="cell"
        empty="The run reported no cell."
        exportName={`${isLpcmci ? 'lpcmci' : 'pcmci-plus'}-evidence`}
        filters={<EvidenceScopeControls selection={selection} onChange={setSelection} variables={variableNames} tauMax={run.result.tauMax} alpha={run.result.pcAlpha} />}
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

function RpcmciResult({ run, open, current }: { readonly open: boolean; readonly current: boolean; readonly run: Extract<DiscoveryRunArtifact, { readonly kind: 'rpcmci-run' }> }) {
  const [regime, setRegime] = useState(0)
  const [selection, setSelection] = useState<EvidenceSelection>(NO_EVIDENCE_SELECTION)
  const graph = run.result.graphs[regime]
  const pMatrix = run.result.pMatrices[regime]
  const valMatrix = run.result.valMatrices[regime]
  const rows = useMemo(
    () => graph.flatMap((targets, sourceIndex) => targets.flatMap((lags, targetIndex) => lags.map((mark, lag) => ({
      key: `${regime}:${sourceIndex}:${targetIndex}:${lag}`,
      source: run.variables[sourceIndex].name,
      target: run.variables[targetIndex].name,
      lag,
      mark,
      p: pMatrix[sourceIndex][targetIndex][lag],
      value: valMatrix[sourceIndex][targetIndex][lag],
    })))),
    [graph, pMatrix, regime, run.variables, valMatrix],
  )
  const selected = useMemo(
    () => (selectsEverything(selection) ? rows : rows.filter((row) => matchesEvidenceSelection(row, selection, run.result.alphaLevel))),
    [rows, run.result.alphaLevel, selection],
  )
  const variableNames = useMemo(() => run.variables.map((variable) => variable.name), [run.variables])
  return (
    <ResultCard
      run={run}
      open={open}
      current={current}
      method="RPCMCI · ParCorr"
      title={<>Regime-dependent lag-graph evidence</>}
      meta={<>{formatCount(run.result.observations).text} rows · {run.result.variables} variables · {run.result.numRegimes} regimes · lags {run.result.tauMin}–{run.result.tauMax} · graph alpha {run.result.alphaLevel}</>}
    >
      <p className="mb-0 mt-3 text-body"><ResultEligibility eligibility={run.eligibility} /></p>
      <RunRecord run={run} />
      <RpcmciMembershipPlot run={run} />
      <div className="mt-3 flex items-center gap-2">
        <label className="text-body text-ink">
          Regime shown
          <Select className={field('text', 'ml-2 w-auto')} value={regime} onChange={(event) => setRegime(Number(event.target.value))}>
            {run.result.graphs.map((_, index) => <option key={index} value={index}>Regime {index + 1}</option>)}
          </Select>
        </label>
      </div>
      <StructurePlot run={run} regime={regime} label={`RPCMCI regime ${regime + 1} structure`} />
      <RpcmciTimeGraphPlot run={run} regime={regime} />
      <p className="mb-3 mt-3 text-body text-muted">The selected regime changes both the graph and its partial-correlation matrix. Regime numbers are labels and may be exchanged without changing the fitted model.</p>
      <EvidenceTable<typeof rows[number]>
        title={`RPCMCI regime ${regime + 1} raw evidence`}
        rows={selected}
        total={rows.length}
        rowKey={(row) => row.key}
        noun="cell"
        empty="The run reported no cell."
        exportName={`rpcmci-regime-${regime + 1}-evidence`}
        filters={<EvidenceScopeControls selection={selection} onChange={setSelection} variables={variableNames} tauMax={run.result.tauMax} alpha={run.result.alphaLevel} />}
        columns={[
          ...linkColumns<typeof rows[number]>(),
          { id: 'mark', header: 'Mark', mono: true, value: (row) => row.mark, format: (value) => (value === '' ? '—' : value) },
          figureColumn<typeof rows[number]>('p', 'p', (row) => row.p, pValue),
          figureColumn<typeof rows[number]>('value', 'ParCorr', (row) => row.value),
        ]}
      />
      <p className="mb-0 mt-3 text-micro text-faint">{run.result.errorFreeAnnealings} of {run.result.maxAnneal} annealing runs completed without an optimisation error · switch threshold {run.result.switchThres} · transition budget {run.result.maxTransitions} · seed {run.result.seed}</p>
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
    case 'rpcmci-run': return <RpcmciResult run={run} open={open} current={current} />
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
      label: name,
      hint: eligibilityHint(candidateEligibility),
      disabled: candidateEligibility.kind === 'refused',
      title: candidateEligibility.kind === 'refused'
        ? `${name}: ${candidateEligibility.violations[0]?.evidence ?? 'a requirement is not met'}`
        : undefined,
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
      case 'rpcmci': {
        const result = await analysis.runRpcmci(
          matrix.value.values,
          matrix.value.rowCount,
          matrix.value.columns.length,
          specification.value,
          (progress) => dispatch({ type: 'run-progressed', progress }),
        )
        if (!result.ok) {
          dispatch({ type: 'run-failed', problem: { kind: 'analysis-refused', detail: result.error.detail } })
          return
        }
        const artifact: DiscoveryRunArtifact = {
          kind: 'rpcmci-run',
          id: newDiscoveryRunId(),
          preparedDataset: prepared.id,
          createdAt: new Date().toISOString(),
          method: RPCMCI_PAR_CORR_METHOD_ID,
          variables: matrix.value.columns,
          eligibility,
          result: result.value,
        }
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
        {prepared.kind === 'prepared-time-series' && prepared.seriesTransforms.some((record) => record.transform.kind !== 'levels') && (
          <p className="mb-0 mt-3 text-body text-muted"><span className={label('text-faint')}>Prepared scale</span><br />{prepared.seriesTransforms.filter((record) => record.transform.kind !== 'levels').map((record) => `${profile.columns.find((column) => column.id === record.column)?.name ?? record.column}: ${describeSeriesTransform(record.transform)}`).join(' · ')}</p>
        )}
        <p className={literal('mb-0 mt-3 break-all text-micro text-faint')}>Dataset version {prepared.id.slice(0, 8)}</p>
      </section>
      <MethodCaveats methods={[method]} eligibility={eligibility} />
    </div>
  )

  const stage = (
    <section aria-labelledby="discovery-title" className="@container/panel flex flex-col gap-5">
      <div>
        <span className={label('text-faint')}>03 · Discovery lab</span>
        <h2 id="discovery-title" className="mb-2 mt-2 text-heading text-ink">Examine candidate relationships</h2>
        <p className="m-0 max-w-[65ch] text-body text-muted">Causal discovery uses patterns in data to propose relations between variables, including same-period and lagged relations when time is part of the study. In this chapter, choose a method suited to the observation structure and compare the candidate relations it produces. The result depends on the method's assumptions and does not establish a causal graph on its own.</p>
      </div>

      <div className="grid gap-4">
        <section className="rounded-xl border border-hair bg-panel p-4" aria-labelledby="discovery-method-title">
            <h3 id="discovery-method-title" className="mb-3 mt-0 text-title font-medium text-ink">Discovery method</h3>
          <RadioList
            className="mt-1"
            legend="Discovery method"
            legendHidden
            value={configuration.kind}
            onChange={(method) => dispatch({ type: 'method-selected', method })}
            options={methodOptions}
          />

          {(configuration.kind === 'pcmci-plus' || configuration.kind === 'lpcmci') && (
            <div className="mt-4 grid gap-3 @md/panel:grid-cols-2">
              <div className="text-body text-ink">
                <ParameterLabel label="Maximum lag" help={DISCOVERY_PARAMETER_HELP.pcmci.maximumLag} htmlFor="pcmci-maximum-lag" />
                <Select
                  id="pcmci-maximum-lag"
                  className={field('text', 'mt-1')}
                  value={configuration.tauMax}
                  onChange={(event) => {
                    const value = lagFromValue(event.target.value)
                    if (value !== null) dispatch({ type: 'tau-max-selected', value })
                  }}
                >
                  {DISCOVERY_LAG_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </div>
              <div className="text-body text-ink">
                <ParameterLabel label="PC alpha" help={DISCOVERY_PARAMETER_HELP.pcmci.pcAlpha} htmlFor="pcmci-pc-alpha" />
                <Select
                  id="pcmci-pc-alpha"
                  className={field('text', 'mt-1')}
                  value={configuration.pcAlpha}
                  onChange={(event) => {
                    const value = alphaFromValue(event.target.value)
                    if (value !== null) dispatch({ type: 'pc-alpha-selected', value })
                  }}
                >
                  {PCMCI_ALPHA_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </div>
            </div>
          )}

          {configuration.kind === 'rpcmci' && (
            <RpcmciControls
              configuration={configuration}
              onChange={(next) => dispatch({ type: 'rpcmci-configured', configuration: next })}
            />
          )}

          {configuration.kind === 'dynotears' && (
            <div className="mt-4 grid gap-3 @2xl/panel:grid-cols-3">
              <div className="text-body text-ink">
                <ParameterLabel label="Maximum lag" help={DISCOVERY_PARAMETER_HELP.dynotears.maximumLag} htmlFor="dynotears-maximum-lag" />
                <Select id="dynotears-maximum-lag" className={field('text', 'mt-1')} value={configuration.maxLag} onChange={(event) => {
                  const value = lagFromValue(event.target.value)
                  if (value !== null) dispatch({ type: 'max-lag-selected', value })
                }}>
                  {DISCOVERY_LAG_OPTIONS.filter((value) => value <= 6).map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </div>
              <div className="text-body text-ink">
                <ParameterLabel label="Contemporaneous λ" help={DISCOVERY_PARAMETER_HELP.dynotears.contemporaneousPenalty} htmlFor="dynotears-contemporaneous-penalty" />
                <Select id="dynotears-contemporaneous-penalty" className={field('text', 'mt-1')} value={configuration.lambdaW} onChange={(event) => {
                  const value = penaltyFromValue(event.target.value)
                  if (value !== null) dispatch({ type: 'dynotears-lambda-w-selected', value })
                }}>
                  {DYNOTEARS_PENALTY_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </div>
              <div className="text-body text-ink">
                <ParameterLabel label="Lagged λ" help={DISCOVERY_PARAMETER_HELP.dynotears.laggedPenalty} htmlFor="dynotears-lagged-penalty" />
                <Select id="dynotears-lagged-penalty" className={field('text', 'mt-1')} value={configuration.lambdaA} onChange={(event) => {
                  const value = penaltyFromValue(event.target.value)
                  if (value !== null) dispatch({ type: 'dynotears-lambda-a-selected', value })
                }}>
                  {DYNOTEARS_PENALTY_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </div>
            </div>
          )}

          {configuration.kind === 'var-lingam' && (
            <div className="mt-4 grid gap-3 @md/panel:grid-cols-2">
              <div className="text-body text-ink">
                <ParameterLabel label="Maximum lag" help={DISCOVERY_PARAMETER_HELP.varLingam.maximumLag} htmlFor="var-lingam-maximum-lag" />
                <Select id="var-lingam-maximum-lag" className={field('text', 'mt-1')} value={configuration.maxLag} onChange={(event) => {
                  const value = lagFromValue(event.target.value)
                  if (value !== null) dispatch({ type: 'max-lag-selected', value })
                }}>
                  {DISCOVERY_LAG_OPTIONS.filter((value) => value <= 6).map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </div>
              <div className="self-end pb-2 text-body text-ink">
                <div className="flex items-center gap-1">
                  <label htmlFor="var-lingam-prune" className="flex items-center gap-2">
                    <input id="var-lingam-prune" type="checkbox" checked={configuration.prune} onChange={(event) => dispatch({ type: 'var-lingam-prune-selected', value: event.target.checked })} />
                    Adaptive-lasso pruning
                  </label>
                  <ParameterHelp label="Adaptive-lasso pruning" help={DISCOVERY_PARAMETER_HELP.varLingam.prune} />
                </div>
                <span className="block pl-6 text-faint">BIC selects the lag order up to the maximum.</span>
              </div>
            </div>
          )}

          {configuration.kind === 'ocse' && (
            <div className="mt-4 grid gap-3 @md/panel:grid-cols-2">
              <div className="text-body text-ink">
                <ParameterLabel label="Maximum lag" help={DISCOVERY_PARAMETER_HELP.ocse.maximumLag} htmlFor="ocse-maximum-lag" />
                <Select id="ocse-maximum-lag" className={field('text', 'mt-1')} value={configuration.maxLag} onChange={(event) => {
                  const value = lagFromValue(event.target.value)
                  if (value !== null) dispatch({ type: 'max-lag-selected', value })
                }}>
                  {DISCOVERY_LAG_OPTIONS.filter((value) => value <= 8).map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </div>
              <div className="text-body text-ink">
                <ParameterLabel label="Information estimator" help={DISCOVERY_PARAMETER_HELP.ocse.informationEstimator} htmlFor="ocse-information-estimator" />
                <Select id="ocse-information-estimator" className={field('text', 'mt-1')} value={configuration.method} onChange={(event) => {
                  if (event.target.value === 'gaussian' || event.target.value === 'knn') dispatch({ type: 'ocse-method-selected', value: event.target.value })
                }}>
                  <option value="gaussian">Gaussian conditional mutual information</option>
                  <option value="knn">k-nearest neighbours (k = 5)</option>
                </Select>
              </div>
              <div className="text-body text-ink">
                <ParameterLabel label="Test alpha" help={DISCOVERY_PARAMETER_HELP.ocse.testAlpha} htmlFor="ocse-test-alpha" />
                <Select id="ocse-test-alpha" className={field('text', 'mt-1')} value={configuration.alpha} onChange={(event) => {
                  const value = alphaFromValue(event.target.value)
                  if (value !== null) dispatch({ type: 'ocse-alpha-selected', value })
                }}>
                  {PCMCI_ALPHA_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </div>
              <div className="text-body text-ink">
                <ParameterLabel label="Permutation shuffles" help={DISCOVERY_PARAMETER_HELP.ocse.permutationShuffles} htmlFor="ocse-permutation-shuffles" />
                <Select id="ocse-permutation-shuffles" className={field('text', 'mt-1')} value={configuration.nShuffles} onChange={(event) => {
                  const value = shufflesFromValue(event.target.value)
                  if (value !== null) dispatch({ type: 'ocse-shuffles-selected', value })
                }}>
                  {OCSE_SHUFFLE_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </div>
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
            <h2 id="discovery-runs-title" className="m-0 text-title font-medium text-ink">Discovery runs</h2>
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
