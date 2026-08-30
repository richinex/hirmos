import { useMemo, useState } from 'react'
import { EChart } from '@/charts/EChart'
import { interventionBarsOption } from '@/charts/dag/interventionBars'
import { useChartTheme } from '@/charts/theme'
import { Icon } from '@/components/Icon'
import { MetricTile } from '@/components/ui/figures'
import { Select } from '@/components/ui/Select'
import { button, field, fieldLabel, num } from '@/components/ui/recipes'
import type { ColumnId, DatasetProfile } from '@/domain/dataset'
import type { DagDocument, DagNodeId } from '@/domain/dag'
import type { NonEmptyArray } from '@/domain/dop'
import {
  describeInterventionReadiness,
  describeInterventionVerdict,
  newInterventionQueryId,
  readyInterventionQuery,
  type InterventionOverlay,
  type InterventionQueryArtifact,
} from '@/domain/intervention'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'
import { formatStatistic } from '@/lib/format/number'

type Job =
  | { readonly kind: 'idle' }
  | { readonly kind: 'running' }
  | { readonly kind: 'failed'; readonly detail: string }

const BIN_OPTIONS = [2, 3, 4, 5] as const

function QueryRecord({ query, open }: { readonly query: InterventionQueryArtifact; readonly open: boolean }) {
  const theme = useChartTheme()
  const { result } = query
  const option = useMemo(() => interventionBarsOption({
    set: query.set.name,
    read: query.read.name,
    states: result.distributionLow.map(([state]) => state),
    low: result.distributionLow.map(([, probability]) => probability),
    high: result.distributionHigh.map(([, probability]) => probability),
  }, theme), [query, result, theme])
  return (
    <li>
      <details className="group rounded-lg border border-hair bg-well" open={open}>
        <summary className="flex cursor-pointer list-none flex-wrap items-baseline gap-x-3 gap-y-1 rounded-lg px-3 py-2 transition-colors hover:bg-raised [&::-webkit-details-marker]:hidden">
          <Icon name="expand_more" size={14} className="shrink-0 self-center text-faint transition-transform duration-150 group-open:rotate-180" />
          <span className="text-body font-medium text-ink">do({query.set.name}) → {query.read.name}</span>
          <span className={num('text-body text-ink')}>{formatStatistic('raw', result.effect).text}</span>
          <span className={num('ml-auto text-micro text-faint')}>{query.createdAt.slice(11, 19)}</span>
        </summary>
        <div className="px-3 pb-3">
          <p className="m-0 text-body text-muted">{describeInterventionVerdict(query)}</p>
          <div className="mt-3 grid gap-2 @sm/inspector:grid-cols-3">
            <MetricTile label={`${query.read.name} if ${query.set.name} set low`} size="compact" value={formatStatistic('raw', result.expectations[0])} context={`bin ${result.treatmentStates[0]}`} />
            <MetricTile label={`${query.read.name} if ${query.set.name} set high`} size="compact" value={formatStatistic('raw', result.expectations[1])} context={`bin ${result.treatmentStates[1]}`} />
            <MetricTile label="Difference" size="compact" value={formatStatistic('raw', result.effect)} context={`${result.bins} bins · equivalent sample size ${result.equivalentSampleSize}`} />
          </div>
          <div className="mt-3 rounded-lg border border-hair bg-well p-2">
            <EChart option={option} label={`${query.read.name} distribution under do(${query.set.name})`} className="h-[200px]" />
          </div>
          <p className="mb-0 mt-2 text-label text-faint">Graph revision {query.dagRevision.slice(0, 8)} · {result.observations} rows</p>
        </div>
      </details>
    </li>
  )
}

/**
 * The Intervene tab: set one node, read another, and ask the discrete network fitted to the DAG
 * what the read node would be. The canvas shows the surgery while the question is framed; the
 * answer is recorded against the graph revision.
 */
export function InterventionPanel({ document, source, profile, prepared, queries, onQuery, onOverlay }: {
  readonly document: DagDocument
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly queries: readonly InterventionQueryArtifact[]
  readonly onQuery: (query: InterventionQueryArtifact) => void
  /** Mirrors the framed question onto the canvas; null clears it. */
  readonly onOverlay: (overlay: InterventionOverlay | null) => void
}) {
  const [set, setSet] = useState<DagNodeId | null>(null)
  const [read, setRead] = useState<DagNodeId | null>(null)
  const [bins, setBins] = useState<number>(3)
  const [equivalentSampleSize, setEquivalentSampleSize] = useState(5)
  const [job, setJob] = useState<Job>({ kind: 'idle' })
  const observed = document.current.graph.nodes.filter((node) => node.kind === 'observed')
  const readiness = readyInterventionQuery(document, set, read)
  const choose = (which: 'set' | 'read', value: string) => {
    const next = value === '' ? null : (value as DagNodeId)
    const nextSet = which === 'set' ? next : set
    const nextRead = which === 'read' ? next : read
    if (which === 'set') setSet(next); else setRead(next)
    setJob({ kind: 'idle' })
    onOverlay(nextSet === null ? null : { set: nextSet, read: nextRead })
  }

  const run = async () => {
    if (!readiness.ok) return
    setJob({ kind: 'running' })
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, analysis] = await Promise.all([import('@/data/prepared'), import('@/analysis/client')])
      const nodes = document.current.graph.nodes.flatMap((node) => (node.kind === 'observed' ? [node] : []))
      const columns = nodes.map((node) => node.column) as unknown as NonEmptyArray<ColumnId>
      const matrix = await materialisePrepared(source, profile, prepared, columns)
      if (!matrix.ok) { setJob({ kind: 'failed', detail: describePreparedMaterialisationProblem(matrix.error) }); return }
      const position = (id: DagNodeId) => nodes.findIndex((node) => node.id === id)
      const edges = document.current.graph.edges.flatMap((edge) => (edge.cause === edge.effect ? [] : [[position(edge.cause), position(edge.effect)] as const]))
      const evidence = await analysis.runDiscreteBnQuery(matrix.value.values, matrix.value.rowCount, nodes.length, {
        nodes: nodes.map((_, index) => index),
        names: nodes.map((node) => node.name),
        edges,
        treatment: position(readiness.value.set.id),
        outcome: position(readiness.value.read.id),
        bins,
        equivalentSampleSize,
      })
      if (!evidence.ok) { setJob({ kind: 'failed', detail: evidence.error.detail }); return }
      onQuery({
        kind: 'intervention-query',
        id: newInterventionQueryId(),
        dagDocument: document.id,
        dagRevision: document.current.id,
        preparedDataset: prepared.id,
        createdAt: new Date().toISOString(),
        set: { node: readiness.value.set.id, name: readiness.value.set.name },
        read: { node: readiness.value.read.id, name: readiness.value.read.name },
        bins,
        equivalentSampleSize,
        result: evidence.value,
      })
      setJob({ kind: 'idle' })
    } catch (cause: unknown) {
      setJob({ kind: 'failed', detail: cause instanceof Error ? cause.message : String(cause) })
    }
  }

  const recorded = [...queries].filter((query) => query.dagDocument === document.id).reverse()
  return (
    <section className="border-t border-hair pt-4" aria-labelledby="intervene-title">
      <h3 id="intervene-title" className="mb-1 mt-0 text-body font-medium text-ink">Intervene</h3>
      <p className="mb-3 mt-0 max-w-[65ch] text-body text-faint">Set one variable and read another. Setting cuts the arrows into the variable; the answer is what the read variable would be if the set variable were set, not what it is when the set variable is observed. A discrete network fitted to the graph answers, adjusting for the set variable's parents.</p>
      <div className="grid gap-2 @sm/inspector:grid-cols-2">
        <label className="min-w-0 text-body text-ink"><span className={fieldLabel}>Set</span>
          <Select aria-label="Variable to set" className={field('text', 'mt-1')} value={set ?? ''} onChange={(event) => choose('set', event.target.value)}>
            <option value="">Choose variable</option>
            {observed.map((node) => <option key={node.id} value={node.id} disabled={node.id === read}>{node.name}</option>)}
          </Select>
        </label>
        <label className="min-w-0 text-body text-ink"><span className={fieldLabel}>Read</span>
          <Select aria-label="Variable to read" className={field('text', 'mt-1')} value={read ?? ''} onChange={(event) => choose('read', event.target.value)}>
            <option value="">Choose variable</option>
            {observed.map((node) => <option key={node.id} value={node.id} disabled={node.id === set}>{node.name}</option>)}
          </Select>
        </label>
        <label className="min-w-0 text-body text-ink"><span className={fieldLabel}>Quantile bins</span>
          <Select aria-label="Quantile bins" className={field('text', 'mt-1')} value={bins} onChange={(event) => { const value = Number(event.target.value); if (BIN_OPTIONS.some((option) => option === value)) setBins(value) }}>
            {BIN_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
          </Select>
        </label>
        <label className="min-w-0 text-body text-ink"><span className={fieldLabel}>Equivalent sample size</span>
          <input type="number" min={1} max={100} step={1} aria-label="Equivalent sample size" className={field('text', 'mt-1')} value={equivalentSampleSize} onChange={(event) => { const value = Number(event.target.value); if (Number.isInteger(value) && value >= 1) setEquivalentSampleSize(value) }} />
        </label>
      </div>
      {!readiness.ok && <p role="status" className="mb-0 mt-2 text-body text-faint">{describeInterventionReadiness(readiness.error)}</p>}
      {job.kind === 'failed' && <p role="alert" className="mb-0 mt-2 text-body text-danger">{job.detail}</p>}
      <button type="button" className={button('signal', 'mt-3')} disabled={!readiness.ok} aria-busy={job.kind === 'running'} onClick={job.kind === 'running' ? undefined : () => void run()}>
        Ask the network
      </button>
      {recorded.length > 0 && (
        <ul className="m-0 mt-4 list-none space-y-2 p-0" aria-label="Intervention queries">
          {recorded.map((query, index) => <QueryRecord key={query.id} query={query} open={index === 0} />)}
        </ul>
      )}
    </section>
  )
}
