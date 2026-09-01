import { Orb } from '@/components/ui/Orb'
import { Alert } from '@/components/ui/Alert'
import { ConfirmDialog } from '@/components/ui/ConfirmDialog'
import { EmptyState } from '@/components/ui/EmptyState'
import { RunFold } from '@/components/ui/RunFold'
import { Select } from '@/components/ui/Select'
import { useMemo, useReducer, useState } from 'react'
import { EChart } from '@/charts/EChart'
import { outcomePathsOption } from '@/charts/counterfactual/outcomePaths'
import { useChartTheme } from '@/charts/theme'
import { EligibilityView } from '@/components/EligibilityView'
import { MethodCaveats } from '@/components/MethodCaveats'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { MetricTile } from '@/components/ui/figures'
import { ResultInterpretation } from '@/components/ui/ResultInterpretation'
import { RadioList } from '@/components/ui/RadioList'
import { button, field, fieldLabel, figureGrid, label, literal, num } from '@/components/ui/recipes'
import { DEFAULT_LINEAR_SCM, evaluateCounterfactualEligibility, newCounterfactualRunId, type CounterfactualRunArtifact, type LinearScmConfiguration } from '@/domain/counterfactual'
import type { ColumnId, DatasetProfile } from '@/domain/dataset'
import { assertNever, type NonEmptyArray } from '@/domain/dop'
import { COUNTERFACTUAL_METHODS, LINEAR_SCM_METHOD_ID, methodDefinition } from '@/domain/methods'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import { estimandSentence, type IdentificationArtifact, type IdentificationId, type StudySpecification, type StudyVariable } from '@/domain/study'
import type { SelectedSource } from '@/domain/workflow'
import { formatCount, formatStatistic, formatWords } from '@/lib/format/number'
import { useRunActivity } from '@/lib/useRunActivity'
import { interpretCounterfactualResult } from '@/domain/resultInterpretation'
import type { RunActivity } from '@/domain/activity'
import { formatTime } from '@/lib/format/date'

type Job = { readonly kind: 'idle' } | { readonly kind: 'running' } | { readonly kind: 'failed'; readonly detail: string }

interface State {
  readonly identification: IdentificationId | null
  readonly configuration: LinearScmConfiguration
  readonly job: Job
}

type Event =
  | { readonly type: 'identification-chosen'; readonly identification: IdentificationId | null }
  | { readonly type: 'configured'; readonly configuration: LinearScmConfiguration }
  | { readonly type: 'run-started' }
  | { readonly type: 'run-failed'; readonly detail: string }
  | { readonly type: 'run-finished' }

const step = (state: State, event: Event): State => {
  switch (event.type) {
    case 'identification-chosen': return { ...state, identification: event.identification, job: { kind: 'idle' } }
    case 'configured': return { ...state, configuration: event.configuration, job: { kind: 'idle' } }
    case 'run-started': return { ...state, job: { kind: 'running' } }
    case 'run-failed': return { ...state, job: { kind: 'failed', detail: event.detail } }
    case 'run-finished': return { ...state, job: { kind: 'idle' } }
    default: return assertNever(event)
  }
}


function EquationsTable({ run }: { readonly run: CounterfactualRunArtifact }) {
  const name = (node: number) => run.nodes[node]?.name ?? String(node)
  return (
    <div className="figure-strip mt-3 overflow-x-auto">
      <table className="w-full border-collapse text-left text-body" aria-label="Structural equations">
        <thead className="text-faint">
          <tr>
            <th className="border-b border-hair px-2 py-1.5 font-normal">Node</th>
            <th className="border-b border-hair px-2 py-1.5 font-normal">Equation</th>
            <th className="border-b border-hair px-2 py-1.5 text-right font-normal">Residual SD</th>
            <th className="border-b border-hair px-2 py-1.5 text-right font-normal">R²</th>
          </tr>
        </thead>
        <tbody>
          {run.evidence.equations.map((equation) => (
            <tr key={equation.node} className="border-b border-hair last:border-0">
              <td className="px-2 py-1.5 text-ink">{name(equation.node)}</td>
              <td className={num('px-2 py-1.5 text-muted')}>
                {formatStatistic('raw', equation.intercept).text}
                {equation.parents.map(([parent, coefficient]) => ` ${coefficient < 0 ? '−' : '+'} ${formatStatistic('raw', Math.abs(coefficient)).text} × ${name(parent)}`).join('')}
                {' + noise'}
              </td>
              <td className={num('px-2 py-1.5 text-right text-muted')}>{formatStatistic('sd', equation.residualSd).text}</td>
              <td className={num('px-2 py-1.5 text-right text-muted')}>{formatStatistic('score', equation.rSquared).text}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}

function RunCard({ run, study, current, stepLabel, onDelete }: { readonly run: CounterfactualRunArtifact; readonly study: StudySpecification; readonly current: boolean; readonly stepLabel: string; readonly onDelete?: () => void }) {
  const theme = useChartTheme()
  const { evidence } = run
  const [row, setRow] = useState(1)
  const index = Math.max(1, Math.min(evidence.observations, row)) - 1
  const option = useMemo(() => outcomePathsOption({
    outcome: study.outcome.name,
    treatment: study.treatment.name,
    interventions: evidence.interventions,
    factual: evidence.factualOutcome,
    low: evidence.counterfactualLow,
    high: evidence.counterfactualHigh,
    stepLabel,
  }, theme), [evidence, stepLabel, study.outcome.name, study.treatment.name, theme])
  const sd = Math.sqrt(evidence.effects.reduce((sum, value) => sum + (value - evidence.averageEffect) ** 2, 0) / Math.max(1, evidence.effects.length - 1))
  // A linear model with exact abduction gives every row the same difference: the coefficient times the change in treatment.
  const constant = evidence.observationNoise === null && Math.max(...evidence.effects) - Math.min(...evidence.effects) < 1e-9 * Math.max(1, Math.abs(evidence.averageEffect))
  const stamp = `linear structural causal model · ${evidence.observationNoise === null ? 'exact disturbance terms' : `observation noise ${evidence.observationNoise}`} · ${formatTime(run.createdAt)}`
  const record = (
    <>
      <h3 className="mb-1 mt-2 text-title font-medium text-ink">What {study.outcome.name} would have been with {study.treatment.name} set to {evidence.interventions[1]} instead of {evidence.interventions[0]}</h3>
      <p className="m-0 text-body text-muted">For each {stepLabel}, the model infers disturbance terms from the observed values. It then sets {study.treatment.name} to each specified value and predicts {study.outcome.name}. The difference is the observation-specific effect implied by the fitted equations.</p>
      <div className={figureGrid('mt-3 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4')} aria-label="Counterfactual summary">
        <MetricTile label={constant ? 'Effect for every row' : 'Average individual effect'} size="compact" frame="cell" value={formatStatistic('raw', evidence.averageEffect)} context={constant ? 'the same for all rows: a linear model with exact abduction gives coefficient × change' : `SD across ${stepLabel}s ${formatStatistic('sd', sd).text}`} />
        {constant
          ? <MetricTile label="Rows" size="compact" frame="cell" value={formatCount(evidence.observations)} context="one model-implied outcome pair per observation" />
          : <MetricTile label="Share positive" size="compact" frame="cell" value={formatStatistic('score', evidence.sharePositive)} context={`of ${formatCount(evidence.observations).text} ${stepLabel}s`} />}
        <MetricTile label="Interventions" size="compact" frame="cell" value={formatWords(`${evidence.interventions[0]} → ${evidence.interventions[1]}`)} context={`${study.treatment.name} set for every ${stepLabel}`} />
        <MetricTile label="Equations" size="compact" frame="cell" value={formatCount(evidence.equations.length)} context={`order ${evidence.order.map((node) => run.nodes[node]?.name ?? node).join(' → ')}`} />
      </div>
      <ResultInterpretation interpretation={interpretCounterfactualResult(run, study, stepLabel)} className="mt-3" />
      <EChart option={option} label={`${study.outcome.name} observed and under both interventions`} className="mt-3 h-[260px]" testId="counterfactual-paths" />
      <div className="mt-3 grid items-start gap-3 rounded-lg border border-hair bg-well p-3 @md/panel:grid-cols-[auto_1fr]">
        <label className="block text-body text-ink"><span className={fieldLabel}>Inspect {stepLabel}</span><input type="number" min={1} max={evidence.observations} aria-label={`Inspect ${stepLabel}`} className={field('text', 'mt-1 w-28')} value={row} onChange={(event) => setRow(Math.max(1, Math.min(evidence.observations, Math.floor(Number(event.target.value) || 1))))} /></label>
        <dl className="m-0 grid w-fit grid-cols-[auto_auto] gap-x-6 gap-y-1 text-body" aria-label={`${stepLabel} ${index + 1} counterfactual`}>
          <dt className="text-faint">Observed {study.outcome.name}</dt><dd className={num('m-0 text-right text-ink')}>{formatStatistic('raw', evidence.factualOutcome[index] ?? Number.NaN).text}</dd>
          <dt className="text-faint">With {study.treatment.name} = {evidence.interventions[0]}</dt><dd className={num('m-0 text-right text-ink')}>{formatStatistic('raw', evidence.counterfactualLow[index] ?? Number.NaN).text}</dd>
          <dt className="text-faint">With {study.treatment.name} = {evidence.interventions[1]}</dt><dd className={num('m-0 text-right text-ink')}>{formatStatistic('raw', evidence.counterfactualHigh[index] ?? Number.NaN).text}</dd>
          <dt className="border-t border-hair pt-1 text-ink">Difference</dt><dd className={num('m-0 border-t border-hair pt-1 text-right font-medium text-ink')}>{formatStatistic('raw', evidence.effects[index] ?? Number.NaN).text}</dd>
        </dl>
      </div>
      <EquationsTable run={run} />
      <details className="mt-3 rounded-lg border border-hair bg-well px-3 py-2 text-body">
        <summary className="cursor-pointer text-muted">Run details</summary>
        <dl className="mb-0 mt-2 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-micro text-faint">
          <dt>Run</dt><dd className={literal('m-0 break-all')}>{run.id}</dd>
          <dt>Study</dt><dd className={literal('m-0 break-all')}>{run.study}</dd>
          <dt>Identification</dt><dd className={literal('m-0 break-all')}>{run.identification}</dd>
          <dt>Prepared dataset</dt><dd className={literal('m-0 break-all')}>{run.preparedDataset}</dd>
        </dl>
      </details>
    </>
  )
  if (!current) {
    return (
      <RunFold title={`${study.treatment.name} ${evidence.interventions[0]} → ${evidence.interventions[1]}`} figure={`average ${formatStatistic('raw', evidence.averageEffect).text}`} stamp={stamp} onDelete={onDelete} deleteLabel="Delete this run">
        {record}
      </RunFold>
    )
  }
  return (
    <article className="rounded-xl border border-edge bg-panel p-4" aria-label={`${estimandSentence(study)} counterfactual`}>
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <span className={label('text-signal')}>Current counterfactual</span>
        <span className={num('text-micro text-faint')}>{stamp}</span>
      </div>
      {record}
    </article>
  )
}

export function CounterfactualPanel({ source, profile, prepared, studies, identifications, runs, onRun, onDeleteRun, onActivity }: {
  readonly onActivity?: (activity: RunActivity | null) => void
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly studies: readonly StudySpecification[]
  readonly identifications: readonly IdentificationArtifact[]
  readonly runs: readonly CounterfactualRunArtifact[]
  readonly onRun: (run: CounterfactualRunArtifact) => void
  readonly onDeleteRun: (run: CounterfactualRunArtifact['id']) => void
}) {
  const identified = identifications.filter((identification) => identification.result.kind === 'identified')
  const [state, dispatch] = useReducer(step, null, (): State => ({ identification: identified.at(-1)?.id ?? null, configuration: DEFAULT_LINEAR_SCM, job: { kind: 'idle' } }))
  const identification = identified.find((candidate) => candidate.id === state.identification) ?? null
  const study = identification === null ? null : studies.find((candidate) => candidate.id === identification.study) ?? null
  const method = methodDefinition(LINEAR_SCM_METHOD_ID)
  const stepLabel = prepared.kind === 'prepared-time-series' ? 'observation' : 'row'
  const eligibility = useMemo(
    () => (identification === null || study === null || !method.ok ? null : evaluateCounterfactualEligibility(method.value, { study, identification: identification.result, prepared, configuration: state.configuration })),
    [identification, method, prepared, state.configuration, study],
  )
  const configure = (configuration: LinearScmConfiguration) => dispatch({ type: 'configured', configuration })
  void profile

  useRunActivity(onActivity, state.job.kind === 'running' ? { label: 'Counterfactual', progress: null } : null)
  const execute = async () => {
    if (identification === null || study === null || eligibility === null || eligibility.kind === 'refused' || state.job.kind === 'running') return
    dispatch({ type: 'run-started' })
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, analysis] = await Promise.all([import('@/data/prepared'), import('@/analysis/client')])
      const measured = study.graph.nodes.flatMap((node) => (node.column === null ? [] : [{ node: node.node, column: node.column, name: node.name }]))
      if (measured.length !== study.graph.nodes.length) { dispatch({ type: 'run-failed', detail: 'Counterfactual estimation requires measured values for every node. Replace or remove unmeasured nodes in the DAG workspace.' }); return }
      const nodes = measured as unknown as NonEmptyArray<StudyVariable>
      const matrix = await materialisePrepared(source, profile, prepared, nodes.map((variable) => variable.column) as unknown as NonEmptyArray<ColumnId>)
      if (!matrix.ok) { dispatch({ type: 'run-failed', detail: describePreparedMaterialisationProblem(matrix.error) }); return }
      const position = (node: StudyVariable['node']) => measured.findIndex((candidate) => candidate.node === node)
      const result = await analysis.runLinearScmCounterfactual(matrix.value.values, matrix.value.rowCount, nodes.length, {
        nodes: measured.map((_, index) => index),
        names: measured.map((variable) => variable.name),
        edges: study.graph.edges.map(([cause, effect]) => [position(study.graph.nodes[cause]?.node ?? study.treatment.node), position(study.graph.nodes[effect]?.node ?? study.outcome.node)] as const),
        treatment: position(study.treatment.node),
        outcome: position(study.outcome.node),
        interventions: state.configuration.interventions,
        observationNoise: state.configuration.observationNoise,
      })
      if (!result.ok) { dispatch({ type: 'run-failed', detail: result.error.detail }); return }
      onRun({
        kind: 'linear-scm-run',
        id: newCounterfactualRunId(),
        study: study.id,
        identification: identification.id,
        preparedDataset: prepared.id,
        createdAt: new Date().toISOString(),
        method: LINEAR_SCM_METHOD_ID,
        configuration: state.configuration,
        nodes,
        evidence: result.value,
        eligibility,
      })
      dispatch({ type: 'run-finished' })
    } catch (cause: unknown) {
      dispatch({ type: 'run-failed', detail: cause instanceof Error ? cause.message : String(cause) })
    }
  }

  const latest = runs.at(-1) ?? null
  const [pendingDelete, setPendingDelete] = useState<CounterfactualRunArtifact | null>(null)
  const stage = (
    <section aria-labelledby="counterfactual-title" className="@container/panel flex flex-col gap-5">
      <div>
        <span className={label('text-faint')}>08 · Counterfactuals</span>
        <h2 id="counterfactual-title" className="mb-2 mt-2 text-heading text-ink">Estimate individual counterfactual outcomes</h2>
        <p className="m-0 max-w-[65ch] text-body text-muted">A counterfactual compares the outcomes that the same unit would have under different interventions. In this chapter, fit a linear structural causal model, infer each row's disturbance terms, hold them fixed, and predict the outcome under two treatment values. These are model-implied counterfactuals and require stronger structural assumptions than an average intervention effect.</p>
      </div>
      <section className="rounded-xl border border-hair bg-panel p-4" aria-labelledby="counterfactual-setup-title">
        <h3 id="counterfactual-setup-title" className="mb-3 mt-0 text-title font-medium text-ink">Linear SCM counterfactual</h3>
        {identified.length === 0 ? (
          <p className="m-0 text-body text-faint">Identify a study first.</p>
        ) : (
          <>
            <label className="block">
              <span className={fieldLabel}>Identified study</span>
              <Select className={field('text', 'mt-1')} value={state.identification ?? ''} onChange={(event) => dispatch({ type: 'identification-chosen', identification: event.target.value === '' ? null : (event.target.value as IdentificationId) })}>
                {identified.map((candidate) => {
                  const bound = studies.find((item) => item.id === candidate.study)
                  return <option key={candidate.id} value={candidate.id}>{bound === undefined ? candidate.id : `${estimandSentence(bound)} · ${bound.dagName}`}</option>
                })}
              </Select>
            </label>
            <div className="mt-4 grid gap-3 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4">
              <label className="block"><span className={fieldLabel}>Set {study?.treatment.name ?? 'treatment'} to</span><input type="number" step="any" aria-label="First intervention value" className={field('text', 'mt-1')} value={state.configuration.interventions[0]} onChange={(event) => configure({ ...state.configuration, interventions: [Number(event.target.value) || 0, state.configuration.interventions[1]] })} /></label>
              <label className="block"><span className={fieldLabel}>and to</span><input type="number" step="any" aria-label="Second intervention value" className={field('text', 'mt-1')} value={state.configuration.interventions[1]} onChange={(event) => configure({ ...state.configuration, interventions: [state.configuration.interventions[0], Number(event.target.value) || 0] })} /></label>
              <div className="@md/panel:col-span-2">
                <RadioList
                  legend="Infer disturbance terms"
                  value={state.configuration.observationNoise === null ? 'exact' : 'noise'}
                  onChange={(mode) => configure({ ...state.configuration, observationNoise: mode === 'exact' ? null : 0.1 })}
                  options={[
                    { value: 'exact', label: 'Exactly', hint: 'Recovers each disturbance term exactly from the fitted equations.' },
                    { value: 'noise', label: 'Allow observation noise', hint: 'Infers disturbance terms allowing observation noise at the chosen scale.' },
                  ]}
                />
              </div>
              {state.configuration.observationNoise !== null && (
                <label className="block"><span className={fieldLabel}>Noise scale</span><input type="number" step="any" min={0.0001} aria-label="Observation noise scale" className={field('text', 'mt-1')} value={state.configuration.observationNoise} onChange={(event) => configure({ ...state.configuration, observationNoise: Math.max(0.0001, Number(event.target.value) || 0.0001) })} /></label>
              )}
            </div>
            {method.ok && <p className="mb-0 mt-3 max-w-[65ch] text-body text-faint">{method.value.summary}</p>}
            {eligibility !== null && <EligibilityView eligibility={eligibility} subject="this study" />}
            {state.job.kind === 'failed' && <Alert tone="danger" className="mt-3"><p className="m-0">The counterfactual could not run: {state.job.detail}</p></Alert>}
            <div className="mt-4 flex items-center gap-3">
              <button type="button" className={button('signal')} disabled={eligibility === null || eligibility.kind === 'refused'} aria-busy={state.job.kind === 'running'} onClick={state.job.kind === 'running' ? undefined : () => void execute()}>
                Run counterfactual
              </button>
              {state.job.kind === 'running' && <Orb state="shaping" aria-label="Counterfactual running" />}
            </div>
          </>
        )}
      </section>
      {latest !== null && (
        <section aria-labelledby="counterfactual-results-title" className="grid gap-4">
          <div>
            <h2 id="counterfactual-results-title" className="m-0 text-title font-medium text-ink">Counterfactuals</h2>
          </div>
          {(() => {
            const bound = studies.find((candidate) => candidate.id === latest.study)
            return bound === undefined ? null : <RunCard run={latest} study={bound} current stepLabel={stepLabel} />
          })()}
        </section>
      )}
      {runs.length === 0 && identified.length > 0 && (
        <EmptyState>Set two values for the treatment and run the counterfactual.</EmptyState>
      )}
    </section>
  )

  const inspector = (
    <div className="space-y-4">
      <section aria-labelledby="counterfactual-study-title">
        <span className={label('text-faint')}>Study</span>
        <h3 id="counterfactual-study-title" className="mb-2 mt-1 text-body font-medium text-ink">{study === null ? 'No study chosen' : estimandSentence(study)}</h3>
        {study !== null && (
          <dl className="m-0 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-body" aria-label="Study">
            <dt className="text-faint">Graph</dt><dd className="m-0 text-ink">{study.dagName} · {study.graph.nodes.length} nodes, {study.graph.edges.length} arrows</dd>
            <dt className="text-faint">Treatment</dt><dd className="m-0 text-ink">{study.treatment.name}</dd>
            <dt className="text-faint">Outcome</dt><dd className="m-0 text-ink">{study.outcome.name}</dd>
            <dt className="text-faint">Rows</dt><dd className={num('m-0 text-ink')}>{formatCount(prepared.observations).text}</dd>
          </dl>
        )}
      </section>
      <MethodCaveats
        methods={COUNTERFACTUAL_METHODS}
        eligibility={eligibility}
        identification={identification?.result ?? null}
      />
    </div>
  )

  const ledger = (
    <ul className="m-0 list-none divide-y divide-hair p-0 text-body" aria-label="Counterfactual ledger">
      {runs.length === 0 && <li className="px-3 py-2 text-faint">Choose 2 treatment values and run the counterfactual.</li>}
      {[...runs].reverse().map((run) => {
        const bound = studies.find((candidate) => candidate.id === run.study)
        return bound === undefined ? null : (
          <RunCard key={run.id} run={run} study={bound} current={false} stepLabel={stepLabel} onDelete={() => setPendingDelete(run)} />
        )
      })}
    </ul>
  )

  const deleteDialog = (
    <ConfirmDialog
      open={pendingDelete !== null}
      title="Delete this counterfactual?"
      danger
      confirmLabel="Delete run"
      message="Removes this counterfactual record. Recorded results cannot be restored."
      onConfirm={() => { if (pendingDelete !== null) onDeleteRun(pendingDelete.id) }}
      onClose={() => setPendingDelete(null)}
    />
  )

  return (
    <>
    <WorkbenchLayout
      id="counterfactual"
      stage={stage}
      inspector={{ title: 'Study and method requirements', body: inspector }}
      bottom={{ title: `Runs · ${runs.length}`, body: <>{ledger}{deleteDialog}</>, defaultSize: 150 }}
    />
    </>
  )
}
