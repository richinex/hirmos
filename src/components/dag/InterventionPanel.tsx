import { RunActions } from '@/components/ui/RunActions'
import { Metadata } from '@/components/ui/Metadata'
import { RunFold } from '@/components/ui/RunFold'
import { ConfirmDialog } from '@/components/ui/ConfirmDialog'
import { formatTime } from '@/lib/format/date'
import { DEFAULT_BDEU_EQUIVALENT_SAMPLE_SIZE } from '@/domain/discreteDefaults'
import { NetworkQueryPanel } from './NetworkQueryPanel'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import type { ContrastQueryArtifact } from '@/domain/intervention'
import type { NetworkQueryArtifact } from '@/domain/networkQuery'
import type { ConditionalGaussianArtifact } from '@/domain/conditionalGaussianQuery'
import { ConditionalGaussianPanel } from './ConditionalGaussianPanel'
import { useMemo, useState, type ReactNode } from 'react'
import { Alert } from '@/components/ui/Alert'
import { EChart } from '@/charts/EChart'
import { interventionBarsOption } from '@/charts/dag/interventionBars'
import { useChartTheme } from '@/charts/theme'
import { Icon } from '@/components/Icon'
import { Formula } from '@/components/ui/Formula'
import { MetricTile } from '@/components/ui/figures'
import { Select } from '@/components/ui/Select'
import {
  button,
  field,
  fieldHint,
  fieldLabel,
  figureGrid,
  num,
  well,
} from '@/components/ui/recipes'
import { cn } from '@/lib/utils'
import type { ColumnId, DatasetProfile } from '@/domain/dataset'
import type { DagDocument, DagNodeId } from '@/domain/dag'
import { assertNever, type NonEmptyArray } from '@/domain/dop'
import {
  describeInterventionReadiness,
  describeInterventionVerdict,
  newInterventionQueryId,
  readyInterventionQuery,
  type IdentifiedDiscreteQueryEvidence,
  type InterventionOverlay,
  type InterventionQueryArtifact,
} from '@/domain/intervention'
import { describeDiscreteStatePreparations, type DiscreteBnEvidence } from '@/domain/estimation'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'
import { formatStatistic } from '@/lib/format/number'
import {
  describeAnalysisWorkerProblem,
  type DiscreteConditionState,
} from '@/workers/analysisProtocol'
import { useJob } from '@/analysis/JobsProvider'
import { JobNotice } from '@/components/ui/JobNotice'

type ConditionDraft =
  | { readonly kind: 'none' }
  | { readonly kind: 'selected'; readonly node: DagNodeId; readonly state: DiscreteConditionState }

const BIN_OPTIONS = [2, 3, 4, 5] as const

/** One recorded query as a fold: the question, its headline figure, the time, and a delete control. */
function QueryFrame({
  query,
  open,
  summary,
  onDelete,
  children,
}: {
  readonly query: ContrastQueryArtifact
  readonly open: boolean
  readonly summary: ReactNode
  readonly onDelete: (query: InterventionQueryArtifact) => void
  readonly children: ReactNode
}) {
  return (
    <RunFold
      title={`do(${query.set.name}) → ${query.read.name}`}
      figure={summary}
      stamp={formatTime(query.createdAt)}
      defaultOpen={open}
      onDelete={() => onDelete(query)}
      deleteLabel="Delete this query"
    >
      {children}
    </RunFold>
  )
}

function DistributionRecord({
  query,
  result,
  context,
  methodNote,
  formula,
}: {
  readonly query: ContrastQueryArtifact
  readonly result: Pick<
    DiscreteBnEvidence,
    | 'observations'
    | 'bins'
    | 'statePreparations'
    | 'treatmentStates'
    | 'expectations'
    | 'effect'
    | 'distributionLow'
    | 'distributionHigh'
  >
  readonly context: React.ReactNode
  readonly methodNote: string
  readonly formula?: { readonly plain: string; readonly tex: string }
}) {
  const theme = useChartTheme()
  const option = useMemo(
    () =>
      interventionBarsOption(
        {
          set: query.set.name,
          read: query.read.name,
          states: result.distributionLow.map(([state]) => state),
          low: result.distributionLow.map(([, probability]) => probability),
          high: result.distributionHigh.map(([, probability]) => probability),
        },
        theme,
      ),
    [query.read.name, query.set.name, result.distributionHigh, result.distributionLow, theme],
  )
  return (
    <>
      <p className="m-0 text-body text-muted">{describeInterventionVerdict(query)}</p>
      <p className="mb-0 mt-1 text-label text-faint">{methodNote}</p>
      <p className="mb-0 mt-1 text-label text-faint">
        State preparation: {describeDiscreteStatePreparations(result.statePreparations)}.
      </p>
      {formula !== undefined && (
        <div className="mt-3 rounded-lg border border-hair bg-raised p-3">
          <Formula plain={formula.plain} tex={formula.tex} />
        </div>
      )}
      <div className={figureGrid('mt-3 @sm/inspector:grid-cols-3')}>
        <MetricTile
          label={`${query.read.name} if ${query.set.name} set low`}
          size="compact"
          frame="cell"
          value={formatStatistic('raw', result.expectations[0])}
          context={`bin ${result.treatmentStates[0]}`}
        />
        <MetricTile
          label={`${query.read.name} if ${query.set.name} set high`}
          size="compact"
          frame="cell"
          value={formatStatistic('raw', result.expectations[1])}
          context={`bin ${result.treatmentStates[1]}`}
        />
        <MetricTile
          label="Difference"
          size="compact"
          frame="cell"
          value={formatStatistic('raw', result.effect)}
          context={context}
        />
      </div>
      <div className={well('mt-3 p-2')}>
        <EChart
          option={option}
          label={`${query.read.name} distribution under do(${query.set.name})`}
          className="h-[200px]"
        />
      </div>
      <p className="mb-0 mt-2 text-label text-faint">
        <Metadata>
          <span>Graph revision {query.dagRevision.slice(0, 8)}</span>
          <span>{result.observations} rows</span>
        </Metadata>
      </p>
    </>
  )
}

function BayesianNetworkRecord({
  query,
  result,
  open,
  equivalentSampleSize,
  onDelete,
}: {
  readonly query: ContrastQueryArtifact
  readonly result: DiscreteBnEvidence
  readonly open: boolean
  readonly equivalentSampleSize: number
  readonly onDelete: (query: InterventionQueryArtifact) => void
}) {
  return (
    <QueryFrame
      query={query}
      open={open}
      onDelete={onDelete}
      summary={
        <span className={num('text-body text-ink')}>
          {formatStatistic('raw', result.effect).text}
        </span>
      }
    >
      <DistributionRecord
        query={query}
        result={result}
        context={
          <Metadata>
            <span>{result.bins} bins</span>
            <span>equivalent sample size {equivalentSampleSize}</span>
          </Metadata>
        }
        methodNote="The contrast is based on BDeu conditional probability tables fitted to the fully observed DAG. No uncertainty interval is reported."
      />
    </QueryFrame>
  )
}

function IdentifiedExpressionRecord({
  query,
  evidence,
  document,
  open,
  onDelete,
}: {
  readonly query: ContrastQueryArtifact
  readonly evidence: IdentifiedDiscreteQueryEvidence
  readonly document: DagDocument
  readonly open: boolean
  readonly onDelete: (query: InterventionQueryArtifact) => void
}) {
  const recordedRevision =
    document.audit.find((revision) => revision.id === query.dagRevision) ?? document.current
  if (evidence.result.kind === 'unidentifiable') {
    const nodeName = (position: number): string =>
      recordedRevision.graph.nodes[position]?.name ?? `node ${position}`
    return (
      <QueryFrame
        query={query}
        open={open}
        onDelete={onDelete}
        summary={<span className="text-body text-danger">Not identified</span>}
      >
        <p className="m-0 text-body text-muted">{describeInterventionVerdict(query)}</p>
        <p className="mb-0 mt-2 text-label text-faint">
          <Metadata>
            <span>Hedge: {evidence.result.hedgeGraph.map(nodeName).join(', ') || 'none'}</span>
            <span>
              treatment-removed subgraph:{' '}
              {evidence.result.hedgeSubgraph.map(nodeName).join(', ') || 'none'}.
            </span>
          </Metadata>
        </p>
      </QueryFrame>
    )
  }
  const result = evidence.result
  const conditionName =
    evidence.query.kind === 'conditional'
      ? (recordedRevision.graph.nodes[evidence.query.variable]?.name ??
        `node ${evidence.query.variable}`)
      : null
  const context =
    evidence.query.kind === 'conditional'
      ? `${result.algorithm}, ${conditionName} bin ${evidence.query.state}`
      : result.algorithm
  const conditionNote =
    evidence.query.kind === 'conditional'
      ? ` The query conditions on ${conditionName} in bin ${evidence.query.state}, represented by ${formatStatistic('raw', evidence.query.representativeValue).text}.`
      : ''
  return (
    <QueryFrame
      query={query}
      open={open}
      onDelete={onDelete}
      summary={
        <span className={num('text-body text-ink')}>
          {formatStatistic('raw', result.effect).text}
        </span>
      }
    >
      <DistributionRecord
        query={query}
        result={{ ...evidence, ...result }}
        context={context}
        methodNote={`The ${result.algorithm} algorithm identified this distribution from the observed joint distribution.${conditionNote} Each variable had a budget of ${evidence.bins} states; observed low-cardinality states were preserved and higher-cardinality values were divided at quantiles. No uncertainty interval is reported. Normalization: ${result.normalizationLow.toFixed(6)} / ${result.normalizationHigh.toFixed(6)}.`}
        formula={{ plain: result.expression, tex: result.latex }}
      />
    </QueryFrame>
  )
}

function QueryRecord({
  query,
  document,
  open,
  onDelete,
}: {
  readonly query: ContrastQueryArtifact
  readonly document: DagDocument
  readonly open: boolean
  readonly onDelete: (query: InterventionQueryArtifact) => void
}) {
  switch (query.route.kind) {
    case 'bayesian-network':
      return (
        <BayesianNetworkRecord
          query={query}
          result={query.route.result}
          equivalentSampleSize={query.route.equivalentSampleSize}
          open={open}
          onDelete={onDelete}
        />
      )
    case 'identified-expression':
      return (
        <IdentifiedExpressionRecord
          query={query}
          evidence={query.route.result}
          document={document}
          open={open}
          onDelete={onDelete}
        />
      )
    default:
      return assertNever(query.route)
  }
}

export function InterventionPanel({
  document,
  source,
  profile,
  prepared,
  queries,
  onQuery,
  onDeleteQuery,
  onOverlay,
}: {
  readonly document: DagDocument
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly queries: readonly InterventionQueryArtifact[]
  readonly onQuery: (query: InterventionQueryArtifact) => void
  readonly onDeleteQuery: (query: InterventionQueryArtifact['id']) => void
  readonly onOverlay: (overlay: InterventionOverlay | null) => void
}) {
  const [mode, setMode] = useState<'contrast' | 'distribution'>('contrast')
  const [pendingDelete, setPendingDelete] = useState<InterventionQueryArtifact | null>(null)
  const deleteDialog = (
    <ConfirmDialog
      open={pendingDelete !== null}
      title="Delete this query?"
      danger
      confirmLabel="Delete query"
      message="Removes the recorded query and its result from the project. Recorded results cannot be restored."
      onConfirm={() => {
        if (pendingDelete !== null) onDeleteQuery(pendingDelete.id)
      }}
      onClose={() => setPendingDelete(null)}
    />
  )
  const [distributionModel, setDistributionModel] = useState<'discrete' | 'gaussian'>('discrete')
  const [set, setSet] = useState<DagNodeId | null>(null)
  const [read, setRead] = useState<DagNodeId | null>(null)
  const [condition, setCondition] = useState<ConditionDraft>({ kind: 'none' })
  const [bins, setBins] = useState<number>(3)
  const [equivalentSampleSize, setEquivalentSampleSize] = useState(
    DEFAULT_BDEU_EQUIVALENT_SAMPLE_SIZE,
  )
  const session = useJob(`intervention:${document.current.id}`)
  const { job } = session
  const graphNodes = document.current.graph.nodes
  const observed = graphNodes.filter((node) => node.kind === 'observed')
  const hasLatent = graphNodes.some((node) => node.kind === 'latent')
  const identifiedRoute = hasLatent || condition.kind === 'selected'
  const readiness = readyInterventionQuery(document, set, read)

  const choose = (which: 'set' | 'read', value: string) => {
    const next = value === '' ? null : (value as DagNodeId)
    const nextSet = which === 'set' ? next : set
    const nextRead = which === 'read' ? next : read
    if (which === 'set') setSet(next)
    else setRead(next)
    setCondition((current) =>
      current.kind === 'selected' && (current.node === nextSet || current.node === nextRead)
        ? { kind: 'none' }
        : current,
    )
    onOverlay(nextSet === null ? null : { set: nextSet, read: nextRead })
  }

  const run = async () => {
    if (!readiness.ok) return
    const current = session.start('analysis', 'Intervention query')
    if (current === null) return
    const fail = (detail: string) => session.fail(current, detail)
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, analysis] =
        await Promise.all([import('@/data/prepared'), import('@/analysis/client')])
      if (!session.current(current)) return
      const columns = observed.map((node) => node.column) as unknown as NonEmptyArray<ColumnId>
      const matrix = await materialisePrepared(source, profile, prepared, columns)
      if (!session.current(current)) return
      if (!matrix.ok) {
        fail(describePreparedMaterialisationProblem(matrix.error))
        return
      }
      const fullPosition = new Map(graphNodes.map((node, index) => [node.id, index] as const))
      const position = (id: DagNodeId): number => fullPosition.get(id) ?? -1
      const fullEdges = document.current.graph.edges.flatMap((edge) =>
        edge.cause === edge.effect ? [] : [[position(edge.cause), position(edge.effect)] as const],
      )
      const base = {
        kind: 'intervention-query' as const,
        id: newInterventionQueryId(),
        dagDocument: document.id,
        dagRevision: document.current.id,
        preparedDataset: prepared.id,
        createdAt: new Date().toISOString(),
        set: { node: readiness.value.set.id, name: readiness.value.set.name },
        read: { node: readiness.value.read.id, name: readiness.value.read.name },
      }
      if (!identifiedRoute) {
        const evidence = await analysis.runDiscreteBnQuery(
          matrix.value.values,
          matrix.value.rowCount,
          observed.length,
          {
            nodes: observed.map((_, index) => index),
            names: observed.map((node) => node.name),
            edges: fullEdges,
            treatment: position(readiness.value.set.id),
            outcome: position(readiness.value.read.id),
            bins,
            equivalentSampleSize,
          },
        )
        if (!session.current(current)) return
        if (!evidence.ok) {
          fail(describeAnalysisWorkerProblem(evidence.error))
          return
        }
        onQuery({
          ...base,
          route: { kind: 'bayesian-network', bins, equivalentSampleSize, result: evidence.value },
        })
      } else {
        const evidence = await analysis.runIdentifiedDiscreteQuery(
          matrix.value.values,
          matrix.value.rowCount,
          observed.length,
          {
            observedNodes: observed.map((_, index) => index),
            names: graphNodes.map((node) => node.name),
            edges: fullEdges,
            treatment: position(readiness.value.set.id),
            outcome: position(readiness.value.read.id),
            unobserved: graphNodes.flatMap((node, index) =>
              node.kind === 'latent' ? [index] : [],
            ),
            bins,
            condition:
              condition.kind === 'none'
                ? null
                : { variable: position(condition.node), state: condition.state },
          },
        )
        if (!session.current(current)) return
        if (!evidence.ok) {
          fail(describeAnalysisWorkerProblem(evidence.error))
          return
        }
        onQuery({ ...base, route: { kind: 'identified-expression', result: evidence.value } })
      }
      session.finish(current)
    } catch (cause: unknown) {
      fail(cause instanceof Error ? cause.message : String(cause))
    }
  }

  const recorded = [...queries]
    .filter(
      (query): query is ContrastQueryArtifact =>
        query.kind === 'intervention-query' && query.dagDocument === document.id,
    )
    .reverse()
  const switcher = (
    <SegmentedControl
      ariaLabel="Query view"
      value={mode}
      onChange={(value) => {
        setMode(value)
        onOverlay(null)
      }}
      options={[
        { value: 'contrast', label: 'Contrast' },
        { value: 'distribution', label: 'Distribution' },
      ]}
      fill
    />
  )
  if (mode === 'distribution')
    return (
      <section aria-label="Network probabilities">
        {switcher}
        <div className="mt-4">
          <SegmentedControl
            ariaLabel="Distribution model"
            value={distributionModel}
            onChange={setDistributionModel}
            options={[
              { value: 'discrete', label: 'Discrete' },
              { value: 'gaussian', label: 'Conditional Gaussian' },
            ]}
            fill
          />
        </div>
        {distributionModel === 'discrete' ? (
          <NetworkQueryPanel
            document={document}
            source={source}
            profile={profile}
            prepared={prepared}
            records={queries.filter(
              (q): q is NetworkQueryArtifact =>
                q.kind === 'network-query' && q.dagDocument === document.id,
            )}
            onQuery={onQuery}
            onDelete={setPendingDelete}
          />
        ) : (
          <ConditionalGaussianPanel
            key={document.current.id}
            document={document}
            source={source}
            profile={profile}
            prepared={prepared}
            records={queries.filter(
              (q): q is ConditionalGaussianArtifact =>
                q.kind === 'conditional-gaussian-query' && q.dagDocument === document.id,
            )}
            onQuery={onQuery}
            onDelete={setPendingDelete}
          />
        )}
        {deleteDialog}
      </section>
    )
  return (
    <section className="border-t border-hair pt-4" aria-labelledby="intervene-title">
      {switcher}
      <h3 id="intervene-title" className="mb-1 mt-0 text-body font-medium text-ink">
        Intervene
      </h3>
      <p className={cn(fieldHint, 'm-0')}>
        Set one measured variable and read another. If there are unmeasured variables, Hirmos first
        checks whether the interventional distribution can be identified from the observed data.
      </p>
      <div className="mt-4 grid gap-4 @sm/inspector:grid-cols-2">
        <label className="min-w-0 text-body text-ink">
          <span className={fieldLabel}>Set</span>
          <Select
            aria-label="Variable to set"
            className={field('text', 'mt-1')}
            value={set ?? ''}
            onChange={(event) => choose('set', event.target.value)}
          >
            <option value="">Choose variable</option>
            {observed.map((node) => (
              <option key={node.id} value={node.id} disabled={node.id === read}>
                {node.name}
              </option>
            ))}
          </Select>
        </label>
        <label className="min-w-0 text-body text-ink">
          <span className={fieldLabel}>Read</span>
          <Select
            aria-label="Variable to read"
            className={field('text', 'mt-1')}
            value={read ?? ''}
            onChange={(event) => choose('read', event.target.value)}
          >
            <option value="">Choose variable</option>
            {observed.map((node) => (
              <option key={node.id} value={node.id} disabled={node.id === set}>
                {node.name}
              </option>
            ))}
          </Select>
        </label>
        <label className="min-w-0 text-body text-ink">
          <span className={fieldLabel}>Condition on</span>
          <Select
            aria-label="Conditioning variable"
            className={field('text', 'mt-1')}
            value={condition.kind === 'none' ? '' : condition.node}
            onChange={(event) =>
              setCondition(
                event.target.value === ''
                  ? { kind: 'none' }
                  : {
                      kind: 'selected',
                      node: event.target.value as DagNodeId,
                      state: { kind: 'lowest' },
                    },
              )
            }
          >
            <option value="">No condition</option>
            {observed.map((node) => (
              <option key={node.id} value={node.id} disabled={node.id === set || node.id === read}>
                {node.name}
              </option>
            ))}
          </Select>
        </label>
        <label className="min-w-0 text-body text-ink">
          <span className={fieldLabel}>State budget</span>
          <Select
            aria-label="State budget"
            className={field('text', 'mt-1')}
            value={bins}
            onChange={(event) => {
              const value = Number(event.target.value)
              if (BIN_OPTIONS.some((option) => option === value)) setBins(value)
            }}
          >
            {BIN_OPTIONS.map((value) => (
              <option key={value} value={value}>
                {value}
              </option>
            ))}
          </Select>
        </label>
        {condition.kind === 'selected' && (
          <label className="min-w-0 text-body text-ink">
            <span className={fieldLabel}>Condition state</span>
            <Select
              aria-label="Condition state"
              className={field('text', 'mt-1')}
              value={condition.state.kind}
              onChange={(event) => {
                const value = event.target.value
                if (value !== 'lowest' && value !== 'highest' && value !== 'index') return
                setCondition((current) =>
                  current.kind === 'selected'
                    ? {
                        ...current,
                        state: value === 'index' ? { kind: 'index', state: 1 } : { kind: value },
                      }
                    : current,
                )
              }}
            >
              <option value="lowest">Lowest</option>
              <option value="highest">Highest</option>
              <option value="index">Specify state index</option>
            </Select>
          </label>
        )}
        {condition.kind === 'selected' && condition.state.kind === 'index' && (
          <label className="min-w-0 text-body text-ink">
            <span className={fieldLabel}>State index</span>
            <input
              aria-label="Condition state index"
              type="number"
              min={0}
              step={1}
              className={field('text', 'mt-1')}
              value={condition.state.state}
              onChange={(event) => {
                const state = event.target.valueAsNumber
                if (Number.isSafeInteger(state) && state >= 0)
                  setCondition((current) =>
                    current.kind === 'selected'
                      ? { ...current, state: { kind: 'index', state } }
                      : current,
                  )
              }}
            />
            <span className={fieldHint}>
              States are numbered from 0. The selected index must exist after discretisation; the
              state budget does not guarantee that many states.
            </span>
          </label>
        )}
        {!identifiedRoute && (
          <label className="min-w-0 text-body text-ink">
            <span className={fieldLabel}>Equivalent sample size</span>
            <input
              type="number"
              min={1}
              step={1}
              aria-label="Equivalent sample size"
              className={field('text', 'mt-1')}
              value={equivalentSampleSize}
              onChange={(event) => {
                const value = Number(event.target.value)
                if (Number.isInteger(value) && value >= 1) setEquivalentSampleSize(value)
              }}
            />
          </label>
        )}
      </div>
      <p className={cn(fieldHint, 'mb-0 mt-4')}>
        Method:{' '}
        {identifiedRoute
          ? condition.kind === 'selected'
            ? 'IDC expression'
            : 'ID expression'
          : 'fully observed Bayesian network'}
      </p>
      {!readiness.ok && (
        <Alert tone="danger" className="mt-4">
          {describeInterventionReadiness(readiness.error)}
        </Alert>
      )}
      <JobNotice job={job} />
      <RunActions
        className="mt-6"
        running={job.kind === 'running'}
        onCancel={session.cancel}
        orbLabel="Intervention running"
      >
        <button
          type="button"
          className={button('signal')}
          disabled={!readiness.ok || job.kind === 'running' || session.blocked}
          aria-busy={job.kind === 'running'}
          onClick={() => void run()}
        >
          Evaluate intervention
        </button>
      </RunActions>
      {recorded.length > 0 && (
        <ul
          className="m-0 mt-4 list-none divide-y divide-hair p-0 text-body"
          aria-label="Intervention queries"
        >
          {recorded.map((query, index) => (
            <QueryRecord
              key={query.id}
              query={query}
              document={document}
              open={index === 0}
              onDelete={setPendingDelete}
            />
          ))}
        </ul>
      )}
      {deleteDialog}
    </section>
  )
}
