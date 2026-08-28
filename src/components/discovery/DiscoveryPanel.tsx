import { useReducer } from 'react'
import { Icon } from '@/components/Icon'
import { MethodCaveats } from '@/components/MethodCaveats'
import { button, field, label, literal, num, segment } from '@/components/ui/recipes'
import type { ColumnId, DatasetProfile, NumericColumnSelection } from '@/domain/dataset'
import {
  DISCOVERY_LAG_OPTIONS,
  INITIAL_DISCOVERY_DRAFT,
  PCMCI_ALPHA_OPTIONS,
  describeDiscoveryReadiness,
  describeDiscoveryRunProblem,
  evaluateDiscoveryEligibility,
  newDiscoveryRunId,
  readyDiscoverySpecification,
  stepDiscovery,
  type ColumnChoice,
  type DiscoveryConfiguration,
  type DiscoveryLag,
  type DiscoveryRunArtifact,
  type PcmciAlpha,
} from '@/domain/discovery'
import { assertNever, type NonEmptyArray } from '@/domain/dop'
import {
  GRANGER_SSR_F_METHOD_ID,
  PCMCI_PLUS_PAR_CORR_METHOD_ID,
  methodDefinition,
  type MethodDefinition,
  type MethodEligibility,
  type MethodSource,
} from '@/domain/methods'
import type { PreparedDatasetArtifact, StationarityEvidenceArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'

interface DiscoveryPanelProps {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly stationarity: StationarityEvidenceArtifact | null
  readonly runs: readonly DiscoveryRunArtifact[]
  readonly onRun: (artifact: DiscoveryRunArtifact) => void
}

const selectedColumn = (value: string, columns: readonly ColumnId[]): ColumnChoice => {
  const column = columns.find((candidate) => candidate === value)
  return column === undefined ? { kind: 'unselected' } : { kind: 'selected', column }
}

const lagFromValue = (value: string): DiscoveryLag | null =>
  DISCOVERY_LAG_OPTIONS.find((candidate) => String(candidate) === value) ?? null

const alphaFromValue = (value: string): PcmciAlpha | null =>
  PCMCI_ALPHA_OPTIONS.find((candidate) => String(candidate) === value) ?? null

const methodIdOf = (configuration: DiscoveryConfiguration) => {
  switch (configuration.kind) {
    case 'pcmci-plus': return PCMCI_PLUS_PAR_CORR_METHOD_ID
    case 'granger-ssr-f': return GRANGER_SSR_F_METHOD_ID
    default: return assertNever(configuration)
  }
}

const pValue = (value: number): string => value < 0.0001 ? '<0.0001' : value.toFixed(4)
const statistic = (value: number): string => Math.abs(value) >= 10_000 ? value.toExponential(4) : value.toFixed(6)

function EligibilityView({ eligibility }: { readonly eligibility: MethodEligibility }) {
  switch (eligibility.kind) {
    case 'eligible':
      return (
        <div className="mt-4 rounded-lg border border-hair bg-well p-3 text-body text-muted">
          <p className="m-0 flex items-center gap-2 text-ink"><Icon name="check_circle" size={16} className="text-ok" /> Eligible on recorded structural checks</p>
          <p className="mb-0 mt-1 text-faint">{eligibility.satisfied.length} requirements have project evidence.</p>
        </div>
      )
    case 'caution':
      return (
        <div className="mt-4 rounded-lg border border-edge bg-well p-3 text-body text-muted">
          <p className="m-0 flex items-center gap-2 text-ink"><Icon name="warning" size={16} className="text-warn" /> Available with unresolved assumptions</p>
          <details className="mt-2">
            <summary className="cursor-pointer text-faint">Review {eligibility.unresolved.length} unresolved requirements</summary>
            <ul className="mb-0 mt-2 space-y-2 pl-4">
              {eligibility.unresolved.map((evaluation) => (
                <li key={evaluation.caveat.id}>
                  <span className="text-ink">{evaluation.caveat.requirement}</span>
                  <span className="mt-0.5 block text-faint">{evaluation.missingEvidence}</span>
                </li>
              ))}
            </ul>
          </details>
        </div>
      )
    case 'refused':
      return (
        <div className="mt-4 rounded-lg border border-danger/30 bg-well p-3 text-body">
          <p className="m-0 flex items-center gap-2 text-danger"><Icon name="block" size={16} /> Method refused for this prepared dataset</p>
          <ul className="mb-0 mt-2 space-y-2 pl-4 text-muted">
            {eligibility.violations.map((evaluation) => (
              <li key={evaluation.caveat.id}>{evaluation.evidence}</li>
            ))}
          </ul>
        </div>
      )
    default: return assertNever(eligibility)
  }
}

function ResultEligibility({ eligibility }: { readonly eligibility: MethodEligibility }) {
  switch (eligibility.kind) {
    case 'eligible': return <span className="text-ok">Eligible · {eligibility.satisfied.length} checks recorded</span>
    case 'caution': return <span className="text-warn">Caution · {eligibility.unresolved.length} unresolved assumptions retained</span>
    case 'refused': return <span className="text-danger">Refused · {eligibility.violations.length} violations</span>
    default: return assertNever(eligibility)
  }
}

const sourceLabel = (source: MethodSource): string => {
  switch (source.kind) {
    case 'reference-implementation': return `${source.repository}@${source.revision.slice(0, 8)} · ${source.locator}`
    case 'paper': return `${source.title} · ${source.locator}`
    case 'hirmos-constraint': return `Hirmos boundary · ${source.locator}`
    default: return assertNever(source)
  }
}

function ResultAssumptions({ eligibility }: { readonly eligibility: MethodEligibility }) {
  const evaluations = eligibility.kind === 'eligible'
    ? eligibility.satisfied
    : eligibility.kind === 'caution'
      ? eligibility.unresolved
      : eligibility.violations
  return (
    <details className="mt-3 rounded-lg border border-hair bg-well px-3 py-2 text-body">
      <summary className="cursor-pointer text-ink">Assumption snapshot retained with this run</summary>
      <ul className="mb-0 mt-2 space-y-3 pl-4 text-muted">
        {evaluations.map((evaluation) => (
          <li key={evaluation.caveat.id}>
            <p className="m-0 text-ink">{evaluation.caveat.requirement}</p>
            <p className="mb-1 mt-0.5 text-faint">If unmet: {evaluation.caveat.consequenceIfUnmet}</p>
            <p className="m-0 text-micro text-faint">
              {evaluation.kind === 'satisfied' ? evaluation.evidence : evaluation.kind === 'unresolved' ? evaluation.missingEvidence : evaluation.evidence}
            </p>
            <p className="mb-0 mt-1 text-micro text-faint">Source: {evaluation.caveat.sources.map(sourceLabel).join('; ')}</p>
          </li>
        ))}
      </ul>
    </details>
  )
}

function RunProvenance({ run }: { readonly run: DiscoveryRunArtifact }) {
  return (
    <details className="mt-3 rounded-lg border border-hair bg-well px-3 py-2 text-body">
      <summary className="cursor-pointer text-ink">Run provenance</summary>
      <dl className="mb-0 mt-2 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-micro text-faint">
        <dt>Run</dt><dd className={literal('m-0 break-all')}>{run.id}</dd>
        <dt>Prepared data</dt><dd className={literal('m-0 break-all')}>{run.preparedDataset}</dd>
        <dt>Created</dt><dd className={literal('m-0')}>{run.createdAt}</dd>
        <dt>Method</dt><dd className={literal('m-0')}>{run.method}</dd>
      </dl>
    </details>
  )
}

function PcmciResult({ run }: { readonly run: Extract<DiscoveryRunArtifact, { readonly kind: 'pcmci-plus-run' }> }) {
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
  const reported = cells.filter((cell) => cell.mark.length > 0).length
  return (
    <article className="rounded-xl border border-edge bg-panel p-4" aria-labelledby={`run-${run.id}`}>
      <div className="flex flex-wrap items-start justify-between gap-3">
        <div>
          <span className={label('text-signal')}>PCMCI+ · ParCorr</span>
          <h3 id={`run-${run.id}`} className="mb-1 mt-1 text-title font-medium text-ink">Stationary lag-graph evidence</h3>
          <p className="m-0 text-body text-faint">{run.result.observations} observations · {run.result.variables} variables · τ max {run.result.tauMax} · α {run.result.pcAlpha}</p>
        </div>
        <span className={num('text-body text-muted')}>{reported} marked cells</span>
      </div>
      <p className="mb-0 mt-3 text-body"><ResultEligibility eligibility={run.eligibility} /></p>
      <ResultAssumptions eligibility={run.eligibility} />
      <RunProvenance run={run} />
      <p className="mb-3 mt-3 text-body text-muted">Every graph mark, p-value, and ParCorr value is retained below. Empty marks remain visible as “—”; contemporaneous unresolved endpoints are not converted into arrows.</p>
      <div className="max-h-96 overflow-auto rounded-lg border border-hair" aria-label="PCMCI+ raw evidence">
        <table className="w-full border-collapse text-left text-body">
          <thead className="sticky top-0 bg-panel text-faint">
            <tr>
              <th className="border-b border-hair px-2 py-2 font-normal">Source</th>
              <th className="border-b border-hair px-2 py-2 font-normal">Target</th>
              <th className="border-b border-hair px-2 py-2 text-right font-normal">Lag</th>
              <th className="border-b border-hair px-2 py-2 font-normal">Mark</th>
              <th className="border-b border-hair px-2 py-2 text-right font-normal">p</th>
              <th className="border-b border-hair px-2 py-2 text-right font-normal">ParCorr</th>
            </tr>
          </thead>
          <tbody>
            {cells.map((cell) => (
              <tr key={`${cell.sourceIndex}:${cell.targetIndex}:${cell.lag}`} className="border-b border-hair last:border-0">
                <td className="px-2 py-2 text-ink">{run.variables[cell.sourceIndex].name}</td>
                <td className="px-2 py-2 text-ink">{run.variables[cell.targetIndex].name}</td>
                <td className={num('px-2 py-2 text-right text-muted')}>{cell.lag}</td>
                <td className={literal('px-2 py-2 text-muted')}>{cell.mark || '—'}</td>
                <td className={num('px-2 py-2 text-right text-muted')}>{pValue(cell.p)}</td>
                <td className={num('px-2 py-2 text-right text-muted')}>{statistic(cell.value)}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </article>
  )
}

function GrangerResult({ run }: { readonly run: Extract<DiscoveryRunArtifact, { readonly kind: 'granger-ssr-f-run' }> }) {
  return (
    <article className="rounded-xl border border-edge bg-panel p-4" aria-labelledby={`run-${run.id}`}>
      <span className={label('text-signal')}>Granger SSR F</span>
      <h3 id={`run-${run.id}`} className="mb-1 mt-1 text-title font-medium text-ink">{run.candidateCause.name} → {run.target.name} predictive evidence</h3>
      <p className="m-0 text-body text-faint">{run.result.observations} paired observations · lags 1–{run.result.maxLag}</p>
      <p className="mb-0 mt-3 text-body"><ResultEligibility eligibility={run.eligibility} /></p>
      <ResultAssumptions eligibility={run.eligibility} />
      <RunProvenance run={run} />
      <p className="mb-3 mt-3 text-body text-muted">The null at each lag is that past {run.candidateCause.name} values add no predictive information beyond past {run.target.name}. This is not intervention causality.</p>
      <div className="overflow-x-auto rounded-lg border border-hair" aria-label="Granger raw evidence">
        <table className="w-full border-collapse text-left text-body">
          <thead className="text-faint">
            <tr>
              <th className="border-b border-hair px-2 py-2 text-right font-normal">Lag</th>
              <th className="border-b border-hair px-2 py-2 text-right font-normal">SSR F statistic</th>
              <th className="border-b border-hair px-2 py-2 text-right font-normal">p-value</th>
            </tr>
          </thead>
          <tbody>
            {run.result.tests.map((test) => (
              <tr key={test.lag} className="border-b border-hair last:border-0">
                <td className={num('px-2 py-2 text-right text-ink')}>{test.lag}</td>
                <td className={num('px-2 py-2 text-right text-muted')}>{statistic(test.statistic)}</td>
                <td className={num('px-2 py-2 text-right text-muted')}>{pValue(test.pValue)}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </article>
  )
}

function DiscoveryResult({ run }: { readonly run: DiscoveryRunArtifact }) {
  switch (run.kind) {
    case 'pcmci-plus-run': return <PcmciResult run={run} />
    case 'granger-ssr-f-run': return <GrangerResult run={run} />
    default: return assertNever(run)
  }
}

export function DiscoveryPanel({ source, profile, prepared, stationarity, runs, onRun }: DiscoveryPanelProps) {
  const [draft, dispatch] = useReducer(stepDiscovery, INITIAL_DISCOVERY_DRAFT)
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

  const execute = async () => {
    const specification = readyDiscoverySpecification(configuration, prepared)
    if (!specification.ok || eligibility.kind === 'refused') return
    dispatch({ type: 'run-started' })
    try {
      const [{ materializeNumericColumnsInWorker }, { runGrangerSsrF, runPcmciPlus }] = await Promise.all([
        import('@/data/client'),
        import('@/analysis/client'),
      ])
      const requestedColumns: NonEmptyArray<ColumnId> = specification.value.kind === 'pcmci-plus'
        ? prepared.columns
        : [specification.value.target, specification.value.candidateCause]
      const matrix = await materializeNumericColumnsInWorker(source.file, profile, requestedColumns)
      if (!matrix.ok) {
        dispatch({
          type: 'run-failed',
          problem: { kind: 'materialization-refused', detail: `Numeric materialization refused: ${matrix.error.kind}.` },
        })
        return
      }
      if (matrix.value.missingCells > 0) {
        dispatch({ type: 'run-failed', problem: { kind: 'missing-values-remain', cells: matrix.value.missingCells } })
        return
      }

      switch (specification.value.kind) {
      case 'pcmci-plus': {
        const result = await runPcmciPlus(
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
      case 'granger-ssr-f': {
        const granger = specification.value
        const target = matrix.value.columns.find((column) => column.id === granger.target)
        const candidateCause = matrix.value.columns.find((column) => column.id === granger.candidateCause)
        if (target === undefined || candidateCause === undefined) {
          dispatch({
            type: 'run-failed',
            problem: { kind: 'materialization-refused', detail: 'The returned matrix omitted the selected Granger pair.' },
          })
          return
        }
        const result = await runGrangerSsrF(matrix.value.values, matrix.value.rowCount, granger.maxLag)
        if (!result.ok) {
          dispatch({ type: 'run-failed', problem: { kind: 'analysis-refused', detail: result.error.detail } })
          return
        }
        const artifact: DiscoveryRunArtifact = {
          kind: 'granger-ssr-f-run',
          id: newDiscoveryRunId(),
          preparedDataset: prepared.id,
          createdAt: new Date().toISOString(),
          method: GRANGER_SSR_F_METHOD_ID,
          target,
          candidateCause,
          eligibility,
          result: result.value,
        }
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

  return (
    <section aria-labelledby="discovery-title">
      <div className="mb-5">
        <span className={label('text-signal')}>03 · Discovery lab</span>
        <h2 id="discovery-title" className="mb-2 mt-2 text-heading text-ink">Explore temporal structure</h2>
        <p className="m-0 max-w-3xl text-body text-muted">Discovery produces evidence for a later graph draft. It does not silently create or validate causal arrows.</p>
      </div>

      <div className="grid gap-4 lg:grid-cols-[minmax(0,1fr)_minmax(18rem,0.72fr)]">
        <section className="rounded-xl border border-line bg-panel p-4" aria-labelledby="discovery-method-title">
          <span className={label('text-faint')}>Method and run</span>
          <h3 id="discovery-method-title" className="mb-3 mt-1 text-title font-medium text-ink">Temporal evidence</h3>
          <div className="flex flex-wrap gap-1 rounded-lg border border-hair bg-well p-1">
            <button
              type="button"
              className={segment(configuration.kind === 'pcmci-plus')}
              aria-pressed={configuration.kind === 'pcmci-plus'}
              onClick={() => dispatch({ type: 'method-selected', method: 'pcmci-plus' })}
            >
              PCMCI+
            </button>
            <button
              type="button"
              className={segment(configuration.kind === 'granger-ssr-f')}
              aria-pressed={configuration.kind === 'granger-ssr-f'}
              onClick={() => dispatch({ type: 'method-selected', method: 'granger-ssr-f' })}
            >
              Granger SSR F
            </button>
          </div>

          {configuration.kind === 'pcmci-plus' && (
            <div className="mt-4 grid gap-3 sm:grid-cols-2">
              <label className="text-body text-ink">
                Maximum lag
                <select
                  className={field('text', 'mt-1')}
                  value={configuration.tauMax}
                  onChange={(event) => {
                    const value = lagFromValue(event.target.value)
                    if (value !== null) dispatch({ type: 'tau-max-selected', value })
                  }}
                >
                  {DISCOVERY_LAG_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
                </select>
              </label>
              <label className="text-body text-ink">
                PC alpha
                <select
                  className={field('text', 'mt-1')}
                  value={configuration.pcAlpha}
                  onChange={(event) => {
                    const value = alphaFromValue(event.target.value)
                    if (value !== null) dispatch({ type: 'pc-alpha-selected', value })
                  }}
                >
                  {PCMCI_ALPHA_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
                </select>
              </label>
            </div>
          )}

          {configuration.kind === 'granger-ssr-f' && (
            <div className="mt-4 grid gap-3 sm:grid-cols-3">
              <label className="text-body text-ink">
                Candidate cause
                <select
                  className={field('text', 'mt-1')}
                  value={configuration.candidateCause.kind === 'selected' ? configuration.candidateCause.column : ''}
                  onChange={(event) => dispatch({
                    type: 'candidate-cause-selected',
                    column: selectedColumn(event.target.value, prepared.columns),
                  })}
                >
                  <option value="">Choose variable</option>
                  {preparedColumns.map((column) => <option key={column.id} value={column.id}>{column.name}</option>)}
                </select>
              </label>
              <label className="text-body text-ink">
                Target
                <select
                  className={field('text', 'mt-1')}
                  value={configuration.target.kind === 'selected' ? configuration.target.column : ''}
                  onChange={(event) => dispatch({
                    type: 'target-selected',
                    column: selectedColumn(event.target.value, prepared.columns),
                  })}
                >
                  <option value="">Choose variable</option>
                  {preparedColumns.map((column) => <option key={column.id} value={column.id}>{column.name}</option>)}
                </select>
              </label>
              <label className="text-body text-ink">
                Maximum lag
                <select
                  className={field('text', 'mt-1')}
                  value={configuration.maxLag}
                  onChange={(event) => {
                    const value = lagFromValue(event.target.value)
                    if (value !== null) dispatch({ type: 'max-lag-selected', value })
                  }}
                >
                  {DISCOVERY_LAG_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
                </select>
              </label>
            </div>
          )}

          <EligibilityView eligibility={eligibility} />
          {!readiness.ok && <p role="status" className="mb-0 mt-3 text-body text-faint">{describeDiscoveryReadiness(readiness.error)}</p>}
          {draft.job.kind === 'failed' && <p role="alert" className="mb-0 mt-3 text-body text-danger">{describeDiscoveryRunProblem(draft.job.problem)}</p>}
          <button
            type="button"
            className={button('signal', 'mt-4')}
            disabled={!readiness.ok || eligibility.kind === 'refused' || draft.job.kind === 'running'}
            onClick={() => void execute()}
          >
            {draft.job.kind === 'running' ? 'Running…' : `Run ${method.name}`}
          </button>
        </section>

        <section className="rounded-xl border border-line bg-panel p-4" aria-labelledby="prepared-input-title">
          <span className={label('text-faint')}>Prepared input</span>
          <h3 id="prepared-input-title" className="mb-3 mt-1 text-title font-medium text-ink">Bound dataset version</h3>
          <dl className="grid grid-cols-[auto_1fr] gap-x-4 gap-y-2 text-body">
            <dt className="text-faint">Structure</dt>
            <dd className="m-0 text-ink">{prepared.kind === 'prepared-time-series' ? `Regular ${prepared.sampling.frequency} series` : 'Independent observations'}</dd>
            <dt className="text-faint">Observations</dt>
            <dd className={num('m-0 text-ink')}>{prepared.observations.toLocaleString()}</dd>
            <dt className="text-faint">Variables</dt>
            <dd className={num('m-0 text-ink')}>{prepared.columns.length}</dd>
            <dt className="text-faint">Stationarity</dt>
            <dd className="m-0 text-ink">{stationarity === null ? 'No evidence attached' : `${stationarity.observations} observations tested`}</dd>
          </dl>
          <p className={literal('mb-0 mt-4 break-all text-micro text-faint')}>Prepared {prepared.id}</p>
        </section>
      </div>

      <MethodCaveats methods={[method]} />

      <section className="mt-6" aria-labelledby="discovery-runs-title">
        <div className="mb-3 flex items-end justify-between gap-3">
          <div>
            <span className={label('text-faint')}>Immutable evidence</span>
            <h2 id="discovery-runs-title" className="mb-0 mt-1 text-title font-medium text-ink">Run results</h2>
          </div>
          <span className={num('text-body text-faint')}>{runs.length} runs</span>
        </div>
        {runs.length === 0 ? (
          <p className="rounded-xl border border-dashed border-line bg-panel p-4 text-body text-faint">No discovery run has been accepted for this prepared version.</p>
        ) : (
          <div className="space-y-4">
            {[...runs].reverse().map((run) => <DiscoveryResult key={run.id} run={run} />)}
          </div>
        )}
      </section>
    </section>
  )
}
