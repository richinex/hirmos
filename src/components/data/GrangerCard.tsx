import { RunActions } from '@/components/ui/RunActions'
import { Metadata } from '@/components/ui/Metadata'
import { useMemo } from 'react'
import { useWorkflow } from '@/components/WorkflowProvider'
import { EChart } from '@/charts/EChart'
import { pValueBarsOption } from '@/charts/discovery/pValueBars'
import { useChartTheme } from '@/charts/theme'
import { Icon } from '@/components/Icon'
import { EvidenceTable, type EvidenceColumn } from '@/components/table/EvidenceTable'
import { Select } from '@/components/ui/Select'
import { button, caption, field, label, num, panel, prose, sectionTitle, well } from '@/components/ui/recipes'
import type { ColumnId, DatasetProfile } from '@/domain/dataset'
import { Alert } from '@/components/ui/Alert'
import {
  GRANGER_LAG_OPTIONS,
  describeGrangerReadiness,
  describeGrangerVerdict,
  evaluateGrangerEligibility,
  newGrangerEvidenceId,
  readyGrangerSpecification,
  type GrangerEvidenceArtifact,
  type GrangerLag,
} from '@/domain/granger'
import { GRANGER_SSR_F_METHOD_ID, methodDefinition } from '@/domain/methods'
import type { PreparedDatasetArtifact, StationarityEvidenceArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'
import { formatTime } from '@/lib/format/date'
import { formatCount } from '@/lib/format/number'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import { cn } from '@/lib/utils'
import { useJob } from '@/analysis/JobsProvider'
import { JobNotice } from '@/components/ui/JobNotice'

const pValue = (value: number): string => (value < 0.0001 ? '<0.0001' : value.toFixed(4))
const statistic = (value: number): string => (Math.abs(value) >= 10_000 ? value.toExponential(4) : value.toFixed(6))
const asNumber = (value: string | number): number => (typeof value === 'number' ? value : Number(value))

function GrangerPlot({ artifact }: { readonly artifact: GrangerEvidenceArtifact }) {
  const theme = useChartTheme()
  const option = useMemo(() => pValueBarsOption({
    title: `Granger sum-of-squared-residuals F test: ${artifact.candidateCause.name} → ${artifact.target.name}`,
    categories: artifact.result.tests.map((test) => `lag ${test.lag}`),
    pValues: artifact.result.tests.map((test) => test.pValue),
    statistics: artifact.result.tests.map((test) => test.statistic),
    statisticName: 'F',
    alpha: 0.05,
  }, theme), [artifact, theme])
  return (
    <div className={well('mt-3 p-(--panel-space)')}>
      <p className={caption('m-0')}><Metadata><span>p-value by lag order</span><span>alpha 0.05 reference, log scale</span></Metadata></p>
      <EChart option={option} label="Granger p-values by lag order" className="h-[clamp(160px,26cqb,240px)]" />
    </div>
  )
}

type TestRow = GrangerEvidenceArtifact['result']['tests'][number]

const TEST_COLUMNS: readonly EvidenceColumn<TestRow>[] = [
  { id: 'lag', header: 'Lag', align: 'right', value: (test) => test.lag },
  { id: 'statistic', header: 'Sum-of-squared-residuals F statistic', align: 'right', value: (test) => test.statistic, format: (value) => statistic(asNumber(value)) },
  { id: 'p', header: 'p-value', align: 'right', value: (test) => test.pValue, format: (value) => pValue(asNumber(value)) },
]

function GrangerRecord({ artifact, open }: { readonly artifact: GrangerEvidenceArtifact; readonly open: boolean }) {
  return (
    <li>
      <details className={well('group')} open={open}>
        <summary className="flex cursor-pointer list-none flex-wrap items-baseline gap-x-3 gap-y-1 rounded-lg px-3 py-2 transition-colors hover:bg-raised [&::-webkit-details-marker]:hidden">
          <Icon name="expand_more" size={14} className="shrink-0 self-center text-faint transition-transform duration-(--motion-fast) group-open:rotate-180" />
          <span className="text-body font-medium text-ink">{artifact.candidateCause.name} → {artifact.target.name}</span>
          <span className={num('text-label text-faint')}><Metadata><span>lags 1 to {artifact.maxLag}</span><span>{formatCount(artifact.result.observations).text} rows</span></Metadata></span>
          <span className={num('ml-auto text-micro text-faint')}>{formatTime(artifact.createdAt)}</span>
        </summary>
        <div className="px-3 pb-3">
          <p className="m-0 text-body text-muted">{describeGrangerVerdict(artifact)}</p>
          <GrangerPlot artifact={artifact} />
          <div className="mt-3">
            <EvidenceTable<TestRow> frame="none" title="Granger raw evidence" rows={artifact.result.tests} rowKey={(test) => String(test.lag)} noun="lag" empty="The test reported no lag." columns={TEST_COLUMNS} maxHeight="max-h-72" />
          </div>
        </div>
      </details>
    </li>
  )
}

/**
 * The Granger SSR F test as a data diagnostic: does the past of one series add predictive information
 * about another beyond its own past. Recorded with the prepared dataset like the stationarity tests;
 * never offered to the DAG as evidence, because prediction is not intervention.
 */
export function GrangerCard({ source, profile, prepared, stationarity, evidence, onEvidence, embedded = false }: {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: Extract<PreparedDatasetArtifact, { readonly kind: 'prepared-time-series' }>
  readonly stationarity: StationarityEvidenceArtifact | null
  readonly evidence: readonly GrangerEvidenceArtifact[]
  readonly onEvidence: (artifact: GrangerEvidenceArtifact) => void
  /** Inside the Diagnostics card: no border of its own, body-weight heading. */
  readonly embedded?: boolean
}) {
  const draft = useWorkflow(state => state.diagnosticDraft?.kind === 'series' && state.diagnosticDraft.prepared === prepared.id ? state.diagnosticDraft.granger : null)
  const change = useWorkflow(state => state.changeDiagnostic)
  const candidateCause = draft?.cause ?? null
  const target = draft?.target ?? null
  const maxLag = draft?.maxLag ?? 4
  const setCandidateCause = (column: ColumnId | null) => change(prepared.id, { type: 'granger-column', role: 'cause', column })
  const setTarget = (column: ColumnId | null) => change(prepared.id, { type: 'granger-column', role: 'target', column })
  const setMaxLag = (value: GrangerLag) => change(prepared.id, { type: 'granger-lag', value })
  const session = useJob('granger')
  const { job } = session
  const method = methodDefinition(GRANGER_SSR_F_METHOD_ID)
  if (!method.ok) return <p role="alert" className="mt-4 text-body text-danger">The Granger test is not registered in the method catalogue.</p>
  const readiness = readyGrangerSpecification(target, candidateCause, maxLag, prepared)
  const columns = profile.columns.filter((column) => prepared.columns.includes(column.id))
  const chosen = (id: ColumnId | null) => columns.find((column) => column.id === id)
  const pair = chosen(target) !== undefined && chosen(candidateCause) !== undefined && target !== candidateCause
    ? { target: { id: target as ColumnId, name: chosen(target)?.name ?? '' }, candidateCause: { id: candidateCause as ColumnId, name: chosen(candidateCause)?.name ?? '' } }
    : null
  const eligibility = evaluateGrangerEligibility(method.value, prepared, stationarity, pair)
  const warning = eligibility.kind === 'caution'
    ? eligibility.unresolved.find((entry) => entry.caveat.category === 'stationarity-and-dynamics')?.missingEvidence ?? null
    : null
  const columnOf = (id: ColumnId | null): ColumnId | null => (id !== null && prepared.columns.includes(id) ? id : null)

  const run = async () => {
    if (!readiness.ok || eligibility.kind === 'refused') return
    const current = session.start('analysis', 'Granger test')
    if (current === null) return
    const fail = (detail: string) => session.fail(current, detail)
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, analysis] = await Promise.all([import('@/data/prepared'), import('@/analysis/client')])
      if (!session.current(current)) return
      const matrix = await materialisePrepared(source, profile, prepared, [readiness.value.target, readiness.value.candidateCause])
      if (!session.current(current)) return
      if (!matrix.ok) { fail(describePreparedMaterialisationProblem(matrix.error)); return }
      const targetColumn = matrix.value.columns.find((column) => column.id === readiness.value.target)
      const causeColumn = matrix.value.columns.find((column) => column.id === readiness.value.candidateCause)
      if (targetColumn === undefined || causeColumn === undefined) { fail('The result does not include the selected pair of variables.'); return }
      const result = await analysis.runGrangerSsrF(matrix.value.values, matrix.value.rowCount, readiness.value.maxLag)
      if (!session.current(current)) return
      if (!result.ok) { fail(describeAnalysisWorkerProblem(result.error)); return }
      onEvidence({
        kind: 'granger-evidence',
        id: newGrangerEvidenceId(),
        preparedDataset: prepared.id,
        createdAt: new Date().toISOString(),
        method: GRANGER_SSR_F_METHOD_ID,
        target: targetColumn,
        candidateCause: causeColumn,
        maxLag: readiness.value.maxLag,
        eligibility,
        result: result.value,
      })
      session.finish(current)
    } catch (cause: unknown) {
      fail(cause instanceof Error ? cause.message : String(cause))
    }
  }

  const recorded = [...evidence].reverse()
  return (
    <section className={embedded ? undefined : panel('mt-4 p-(--panel-space)')} aria-labelledby="granger-title">
      <h3 id="granger-title" className={embedded ? 'm-0 text-body font-medium text-ink' : cn(sectionTitle, 'm-0')}>Granger predictive test</h3>
      <p className={prose('mb-0 mt-1 text-faint')}>The test assesses whether past values of one series improve prediction of another beyond that series’ own past, at each lag order up to the maximum. It assesses predictive precedence, not an intervention effect. Its result is not added to the DAG as causal evidence.</p>
      <div className="mt-3 flex flex-wrap items-end gap-2">
        <label className="text-body text-ink"><span className={label('block text-faint')}>Candidate cause</span>
          <Select aria-label="Candidate cause" className={field('text', 'mt-1 w-44')} value={columnOf(candidateCause) ?? ''} onChange={(event) => setCandidateCause(event.target.value === '' ? null : (event.target.value as ColumnId))}>
            <option value="">Choose variable</option>
            {columns.map((column) => <option key={column.id} value={column.id}>{column.name}</option>)}
          </Select>
        </label>
        <label className="text-body text-ink"><span className={label('block text-faint')}>Target</span>
          <Select aria-label="Target" className={field('text', 'mt-1 w-44')} value={columnOf(target) ?? ''} onChange={(event) => setTarget(event.target.value === '' ? null : (event.target.value as ColumnId))}>
            <option value="">Choose variable</option>
            {columns.map((column) => <option key={column.id} value={column.id}>{column.name}</option>)}
          </Select>
        </label>
        <label className="text-body text-ink"><span className={label('block text-faint')}>Maximum lag</span>
          <Select aria-label="Maximum lag" className={field('text', 'mt-1 w-24')} value={maxLag} onChange={(event) => { const value = GRANGER_LAG_OPTIONS.find((candidate) => String(candidate) === event.target.value); if (value !== undefined) setMaxLag(value) }}>
            {GRANGER_LAG_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
          </Select>
        </label>
        <RunActions running={job.kind === 'running'} onCancel={session.cancel} orbLabel="Granger test running">
          <button type="button" className={button('quiet')} disabled={!readiness.ok || eligibility.kind === 'refused' || job.kind === 'running' || session.blocked} aria-busy={job.kind === 'running'} onClick={() => void run()}>
            Run Granger test
          </button>
        </RunActions>
      </div>
      {!readiness.ok && <Alert tone="danger" className="mt-2">{describeGrangerReadiness(readiness.error)}</Alert>}
      <JobNotice job={job} />
      {pair !== null && warning !== null && (
        <Alert tone="warn" live={false} className="mt-3">
          <p className="m-0">Stationarity not confirmed</p>
          <p className="mb-0 mt-1 text-muted">{warning}</p>
        </Alert>
      )}
      {recorded.length > 0 && (
        <ul className="m-0 mt-4 list-none space-y-2 p-0" aria-label="Granger tests">
          {recorded.map((artifact, index) => <GrangerRecord key={artifact.id} artifact={artifact} open={index === 0} />)}
        </ul>
      )}
    </section>
  )
}
