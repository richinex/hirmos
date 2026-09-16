import { Metadata } from '@/components/ui/Metadata'
import { useMemo, useState, type ReactNode } from 'react'
import { Alert } from '@/components/ui/Alert'
import { EChart } from '@/charts/EChart'
import { interventionBarsOption } from '@/charts/dag/interventionBars'
import { useChartTheme } from '@/charts/theme'
import { Icon } from '@/components/Icon'
import { Formula } from '@/components/ui/Formula'
import { MetricTile } from '@/components/ui/figures'
import { Select } from '@/components/ui/Select'
import { button, field, fieldLabel, figureGrid, num, prose, well } from '@/components/ui/recipes'
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
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import { useJob } from '@/analysis/JobsProvider'
import { JobNotice } from '@/components/ui/JobNotice'

type ConditionDraft =
  | { readonly kind: 'none' }
  | { readonly kind: 'selected'; readonly node: DagNodeId; readonly state: number }

const BIN_OPTIONS = [2, 3, 4, 5] as const

function QueryFrame({ query, open, summary, children }: { readonly query: InterventionQueryArtifact; readonly open: boolean; readonly summary: ReactNode; readonly children: ReactNode }) {
  return (
    <li>
      <details className={well('group')} open={open}>
        <summary className="flex cursor-pointer list-none flex-wrap items-baseline gap-x-3 gap-y-1 rounded-lg px-3 py-2 transition-colors hover:bg-raised [&::-webkit-details-marker]:hidden">
          <Icon name="expand_more" size={14} className="shrink-0 self-center text-faint transition-transform duration-(--motion-fast) group-open:rotate-180" />
          <span className="text-body font-medium text-ink">do({query.set.name}) → {query.read.name}</span>
          {summary}
          <span className={num('ml-auto text-micro text-faint')}>{query.createdAt.slice(11, 19)}</span>
        </summary>
        <div className="px-3 pb-3">{children}</div>
      </details>
    </li>
  )
}

function DistributionRecord({ query, result, context, methodNote, formula }: { readonly query: InterventionQueryArtifact; readonly result: Pick<DiscreteBnEvidence, 'observations' | 'bins' | 'statePreparations' | 'treatmentStates' | 'expectations' | 'effect' | 'distributionLow' | 'distributionHigh'>; readonly context: React.ReactNode; readonly methodNote: string; readonly formula?: { readonly plain: string; readonly tex: string } }) {
  const theme = useChartTheme()
  const option = useMemo(() => interventionBarsOption({
    set: query.set.name,
    read: query.read.name,
    states: result.distributionLow.map(([state]) => state),
    low: result.distributionLow.map(([, probability]) => probability),
    high: result.distributionHigh.map(([, probability]) => probability),
  }, theme), [query.read.name, query.set.name, result.distributionHigh, result.distributionLow, theme])
  return (
    <>
      <p className="m-0 text-body text-muted">{describeInterventionVerdict(query)}</p>
      <p className="mb-0 mt-1 text-label text-faint">{methodNote}</p>
      <p className="mb-0 mt-1 text-label text-faint">State preparation: {describeDiscreteStatePreparations(result.statePreparations)}.</p>
      {formula !== undefined && <div className="mt-3 rounded-lg border border-hair bg-raised p-3"><Formula plain={formula.plain} tex={formula.tex} /></div>}
      <div className={figureGrid('mt-3 @sm/inspector:grid-cols-3')}>
        <MetricTile label={`${query.read.name} if ${query.set.name} set low`} size="compact" frame="cell" value={formatStatistic('raw', result.expectations[0])} context={`bin ${result.treatmentStates[0]}`} />
        <MetricTile label={`${query.read.name} if ${query.set.name} set high`} size="compact" frame="cell" value={formatStatistic('raw', result.expectations[1])} context={`bin ${result.treatmentStates[1]}`} />
        <MetricTile label="Difference" size="compact" frame="cell" value={formatStatistic('raw', result.effect)} context={context} />
      </div>
      <div className={well('mt-3 p-2')}>
        <EChart option={option} label={`${query.read.name} distribution under do(${query.set.name})`} className="h-[200px]" />
      </div>
      <p className="mb-0 mt-2 text-label text-faint"><Metadata><span>Graph revision {query.dagRevision.slice(0, 8)}</span><span>{result.observations} rows</span></Metadata></p>
    </>
  )
}

function BayesianNetworkRecord({ query, result, open, equivalentSampleSize }: { readonly query: InterventionQueryArtifact; readonly result: DiscreteBnEvidence; readonly open: boolean; readonly equivalentSampleSize: number }) {
  return (
    <QueryFrame query={query} open={open} summary={<span className={num('text-body text-ink')}>{formatStatistic('raw', result.effect).text}</span>}>
      <DistributionRecord query={query} result={result} context={<Metadata><span>{result.bins} bins</span><span>equivalent sample size {equivalentSampleSize}</span></Metadata>} methodNote="The contrast comes from BDeu conditional probability tables fitted to the fully observed DAG. No uncertainty interval is reported." />
    </QueryFrame>
  )
}

function IdentifiedExpressionRecord({ query, evidence, document, open }: { readonly query: InterventionQueryArtifact; readonly evidence: IdentifiedDiscreteQueryEvidence; readonly document: DagDocument; readonly open: boolean }) {
  const recordedRevision = document.audit.find((revision) => revision.id === query.dagRevision) ?? document.current
  if (evidence.result.kind === 'unidentifiable') {
    const nodeName = (position: number): string => recordedRevision.graph.nodes[position]?.name ?? `node ${position}`
    return (
      <QueryFrame query={query} open={open} summary={<span className="text-body text-danger">Not identified</span>}>
        <p className="m-0 text-body text-muted">{describeInterventionVerdict(query)}</p>
        <p className="mb-0 mt-2 text-label text-faint"><Metadata><span>Hedge: {evidence.result.hedgeGraph.map(nodeName).join(', ') || 'none'}</span><span>treatment-removed subgraph: {evidence.result.hedgeSubgraph.map(nodeName).join(', ') || 'none'}.</span></Metadata></p>
      </QueryFrame>
    )
  }
  const result = evidence.result
  const conditionName = evidence.query.kind === 'conditional'
    ? recordedRevision.graph.nodes[evidence.query.variable]?.name ?? `node ${evidence.query.variable}`
    : null
  const context = evidence.query.kind === 'conditional'
    ? `${result.algorithm}, ${conditionName} bin ${evidence.query.state}`
    : result.algorithm
  const conditionNote = evidence.query.kind === 'conditional'
    ? ` The query conditions on ${conditionName} in bin ${evidence.query.state}, represented by ${formatStatistic('raw', evidence.query.representativeValue).text}.`
    : ''
  return (
    <QueryFrame query={query} open={open} summary={<span className={num('text-body text-ink')}>{formatStatistic('raw', result.effect).text}</span>}>
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

function QueryRecord({ query, document, open }: { readonly query: InterventionQueryArtifact; readonly document: DagDocument; readonly open: boolean }) {
  switch (query.route.kind) {
    case 'bayesian-network': return <BayesianNetworkRecord query={query} result={query.route.result} equivalentSampleSize={query.route.equivalentSampleSize} open={open} />
    case 'identified-expression': return <IdentifiedExpressionRecord query={query} evidence={query.route.result} document={document} open={open} />
    default: return assertNever(query.route)
  }
}

export function InterventionPanel({ document, source, profile, prepared, queries, onQuery, onOverlay }: {
  readonly document: DagDocument
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly queries: readonly InterventionQueryArtifact[]
  readonly onQuery: (query: InterventionQueryArtifact) => void
  readonly onOverlay: (overlay: InterventionOverlay | null) => void
}) {
  const [set, setSet] = useState<DagNodeId | null>(null)
  const [read, setRead] = useState<DagNodeId | null>(null)
  const [condition, setCondition] = useState<ConditionDraft>({ kind: 'none' })
  const [bins, setBins] = useState<number>(3)
  const [equivalentSampleSize, setEquivalentSampleSize] = useState(5)
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
    if (which === 'set') setSet(next); else setRead(next)
    setCondition((current) => current.kind === 'selected' && (current.node === nextSet || current.node === nextRead) ? { kind: 'none' } : current)
    onOverlay(nextSet === null ? null : { set: nextSet, read: nextRead })
  }

  const run = async () => {
    if (!readiness.ok) return
    const current = session.start('analysis', 'Intervention query')
    if (current === null) return
    const fail = (detail: string) => session.fail(current, detail)
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, analysis] = await Promise.all([import('@/data/prepared'), import('@/analysis/client')])
      if (!session.current(current)) return
      const columns = observed.map((node) => node.column) as unknown as NonEmptyArray<ColumnId>
      const matrix = await materialisePrepared(source, profile, prepared, columns)
      if (!session.current(current)) return
      if (!matrix.ok) { fail(describePreparedMaterialisationProblem(matrix.error)); return }
      const fullPosition = new Map(graphNodes.map((node, index) => [node.id, index] as const))
      const position = (id: DagNodeId): number => fullPosition.get(id) ?? -1
      const fullEdges = document.current.graph.edges.flatMap((edge) => edge.cause === edge.effect ? [] : [[position(edge.cause), position(edge.effect)] as const])
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
        const evidence = await analysis.runDiscreteBnQuery(matrix.value.values, matrix.value.rowCount, observed.length, {
          nodes: observed.map((_, index) => index), names: observed.map((node) => node.name), edges: fullEdges,
          treatment: position(readiness.value.set.id), outcome: position(readiness.value.read.id), bins, equivalentSampleSize,
        })
        if (!session.current(current)) return
        if (!evidence.ok) { fail(describeAnalysisWorkerProblem(evidence.error)); return }
        onQuery({ ...base, route: { kind: 'bayesian-network', bins, equivalentSampleSize, result: evidence.value } })
      } else {
        const evidence = await analysis.runIdentifiedDiscreteQuery(matrix.value.values, matrix.value.rowCount, observed.length, {
          observedNodes: observed.map((_, index) => index), names: graphNodes.map((node) => node.name), edges: fullEdges,
          treatment: position(readiness.value.set.id), outcome: position(readiness.value.read.id),
          unobserved: graphNodes.flatMap((node, index) => node.kind === 'latent' ? [index] : []), bins,
          condition: condition.kind === 'none' ? null : { variable: position(condition.node), state: condition.state },
        })
        if (!session.current(current)) return
        if (!evidence.ok) { fail(describeAnalysisWorkerProblem(evidence.error)); return }
        onQuery({ ...base, route: { kind: 'identified-expression', result: evidence.value } })
      }
      session.finish(current)
    } catch (cause: unknown) {
      fail(cause instanceof Error ? cause.message : String(cause))
    }
  }

  const recorded = [...queries].filter((query) => query.dagDocument === document.id).reverse()
  return (
    <section className="border-t border-hair pt-4" aria-labelledby="intervene-title">
      <h3 id="intervene-title" className="mb-1 mt-0 text-body font-medium text-ink">Intervene</h3>
      <p className={prose('mb-3 mt-0 text-faint')}>Set one measured variable and read another. With unmeasured variables, Hirmos first determines whether the interventional distribution is identifiable from the observed data.</p>
      <div className="grid gap-2 @sm/inspector:grid-cols-2">
        <label className="min-w-0 text-body text-ink"><span className={fieldLabel}>Set</span>
          <Select aria-label="Variable to set" className={field('text', 'mt-1')} value={set ?? ''} onChange={(event) => choose('set', event.target.value)}>
            <option value="">Choose variable</option>{observed.map((node) => <option key={node.id} value={node.id} disabled={node.id === read}>{node.name}</option>)}
          </Select>
        </label>
        <label className="min-w-0 text-body text-ink"><span className={fieldLabel}>Read</span>
          <Select aria-label="Variable to read" className={field('text', 'mt-1')} value={read ?? ''} onChange={(event) => choose('read', event.target.value)}>
            <option value="">Choose variable</option>{observed.map((node) => <option key={node.id} value={node.id} disabled={node.id === set}>{node.name}</option>)}
          </Select>
        </label>
        <label className="min-w-0 text-body text-ink"><span className={fieldLabel}>Condition on</span>
          <Select aria-label="Conditioning variable" className={field('text', 'mt-1')} value={condition.kind === 'none' ? '' : condition.node} onChange={(event) => setCondition(event.target.value === '' ? { kind: 'none' } : { kind: 'selected', node: event.target.value as DagNodeId, state: 0 })}>
            <option value="">No condition</option>{observed.map((node) => <option key={node.id} value={node.id} disabled={node.id === set || node.id === read}>{node.name}</option>)}
          </Select>
        </label>
        <label className="min-w-0 text-body text-ink"><span className={fieldLabel}>State budget</span>
          <Select aria-label="State budget" className={field('text', 'mt-1')} value={bins} onChange={(event) => { const value = Number(event.target.value); if (BIN_OPTIONS.some((option) => option === value)) { setBins(value); setCondition((current) => current.kind === 'selected' ? { ...current, state: Math.min(current.state, value - 1) } : current) } }}>
            {BIN_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
          </Select>
        </label>
        {condition.kind === 'selected' && <label className="min-w-0 text-body text-ink"><span className={fieldLabel}>Condition state</span>
          <Select aria-label="Condition state" className={field('text', 'mt-1')} value={condition.state} onChange={(event) => setCondition((current) => current.kind === 'selected' ? { ...current, state: Number(event.target.value) } : current)}>
            {Array.from({ length: bins }, (_, state) => <option key={state} value={state}>{state === 0 ? 'Lowest' : state === bins - 1 ? 'Highest' : `State ${state}`}</option>)}
          </Select>
        </label>}
        {!identifiedRoute && <label className="min-w-0 text-body text-ink"><span className={fieldLabel}>Equivalent sample size</span>
          <input type="number" min={1} max={100} step={1} aria-label="Equivalent sample size" className={field('text', 'mt-1')} value={equivalentSampleSize} onChange={(event) => { const value = Number(event.target.value); if (Number.isInteger(value) && value >= 1) setEquivalentSampleSize(value) }} />
        </label>}
      </div>
      <p className="mb-0 mt-2 text-label text-faint">Method: {identifiedRoute ? (condition.kind === 'selected' ? 'IDC expression' : 'ID expression') : 'fully observed Bayesian network'}</p>
      {!readiness.ok && <Alert tone="danger" className="mt-2">{describeInterventionReadiness(readiness.error)}</Alert>}
      <JobNotice job={job} />
      <div className="mt-3 flex items-center gap-3">
        <button type="button" className={button('signal')} disabled={!readiness.ok || job.kind === 'running' || session.blocked} aria-busy={job.kind === 'running'} onClick={() => void run()}>Evaluate intervention</button>
        {job.kind === 'running' && <button type="button" className={button('quiet')} onClick={session.cancel}>Cancel run</button>}
      </div>
      {recorded.length > 0 && <ul className="m-0 mt-4 list-none space-y-2 p-0" aria-label="Intervention queries">{recorded.map((query, index) => <QueryRecord key={query.id} query={query} document={document} open={index === 0} />)}</ul>}
    </section>
  )
}
