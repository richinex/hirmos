import { Orb } from '@/components/ui/Orb'
import { ConfirmDialog } from '@/components/ui/ConfirmDialog'
import { EmptyState } from '@/components/ui/EmptyState'
import { RunFold } from '@/components/ui/RunFold'
import { Select } from '@/components/ui/Select'
import { useMemo, useReducer, useState } from 'react'
import { EChart } from '@/charts/EChart'
import { matrixHeatmapOption } from '@/charts/discovery/matrixHeatmap'
import { useChartTheme } from '@/charts/theme'
import { MethodCaveats } from '@/components/MethodCaveats'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { Alert } from '@/components/ui/Alert'
import { MetricTile } from '@/components/ui/figures'
import { ResultInterpretation } from '@/components/ui/ResultInterpretation'
import { RadioList } from '@/components/ui/RadioList'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { button, field, fieldLabel, figureGrid, label, literal, num } from '@/components/ui/recipes'
import type { ColumnId, DatasetProfile } from '@/domain/dataset'
import { assertNever, type NonEmptyArray } from '@/domain/dop'
import { adjustmentLabels, contemporaneousAdjustmentVariables, describeEstimator, type EstimationRunArtifact, type EstimationRunId } from '@/domain/estimation'
import { DATA_SUBSET_REFUTER_METHOD_ID, LJUNG_BOX_METHOD_ID, PLACEBO_REFUTER_METHOD_ID, RANDOM_COMMON_CAUSE_REFUTER_METHOD_ID,
  DML_REFUTATION_METHOD_ID,
  DML_SENSITIVITY_METHODS, REFUTER_METHODS, SENSITIVITY_DIAGNOSTIC_METHODS, SHAPIRO_WILK_METHOD_ID, UNOBSERVED_COMMON_CAUSE_METHOD_ID } from '@/domain/methods'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import {
  DEFAULT_DML_REFUTATION,
  DEFAULT_REFUTATION,
  DEFAULT_UNOBSERVED,
  describeProbe,
  kappaValues,
  newSensitivityRunId,
  probeEligibility,
  type DiagnosticFact,
  type RefuterFact,
  type SensitivityConfiguration,
  type SensitivityProbe,
  type SensitivityRunArtifact,
} from '@/domain/sensitivity'
import { estimandSentence, type StudySpecification, type StudyVariable } from '@/domain/study'
import type { SelectedSource } from '@/domain/workflow'
import { formatCount, formatP, formatStatistic } from '@/lib/format/number'
import { lowerFirst } from '@/lib/text'
import { useRunActivity } from '@/lib/useRunActivity'
import type { RunActivity } from '@/domain/activity'
import { formatTime, formatTimestamp } from '@/lib/format/date'
import { interpretSensitivityResult } from '@/domain/resultInterpretation'

type Job = { readonly kind: 'idle' } | { readonly kind: 'running' } | { readonly kind: 'failed'; readonly detail: string }

interface State {
  readonly estimationRun: EstimationRunId | null
  readonly probe: SensitivityProbe
  readonly configurations: Readonly<Record<SensitivityProbe, SensitivityConfiguration>>
  readonly job: Job
}

type Event =
  | { readonly type: 'run-chosen'; readonly run: EstimationRunId | null }
  | { readonly type: 'probe-chosen'; readonly probe: SensitivityProbe }
  | { readonly type: 'configured'; readonly configuration: SensitivityConfiguration }
  | { readonly type: 'job-started' }
  | { readonly type: 'job-failed'; readonly detail: string }
  | { readonly type: 'job-finished' }

const step = (state: State, event: Event): State => {
  switch (event.type) {
    case 'run-chosen': return { ...state, estimationRun: event.run, job: { kind: 'idle' } }
    case 'probe-chosen': return { ...state, probe: event.probe, job: { kind: 'idle' } }
    case 'configured': return { ...state, configurations: { ...state.configurations, [event.configuration.kind]: event.configuration }, job: { kind: 'idle' } }
    case 'job-started': return { ...state, job: { kind: 'running' } }
    case 'job-failed': return { ...state, job: { kind: 'failed', detail: event.detail } }
    case 'job-finished': return { ...state, job: { kind: 'idle' } }
    default: return assertNever(event)
  }
}

const PROBES: NonEmptyArray<SensitivityProbe> = ['linear-refutation', 'unobserved-confounding', 'dml-refutation']

const probeHint = (probe: SensitivityProbe): string => {
  switch (probe) {
    case 'linear-refutation': return 'Refits the linear back-door estimate under placebo-treatment, data-subset and random-common-cause perturbations, with residual diagnostics.'
    case 'unobserved-confounding': return 'Adds a simulated confounder sized from the observed common causes and refits the linear back-door estimate.'
    case 'dml-refutation': return 'Repeats the double machine learning fit, then runs the placebo, random common cause and confounding bounds probes from one seed.'
    default: return assertNever(probe)
  }
}

const refuterInterpretation = (fact: RefuterFact): string => {
  switch (fact.interpretation.kind) {
    case 'reference-distance': {
      const reference = fact.interpretation.reference === 'zero' ? 0 : fact.original
      const distance = Math.abs(fact.refuted - reference)
      return `${fact.interpretation.reading} Absolute distance from the reference: ${formatStatistic('raw', distance).text}. No hypothesis-test p-value is reported.`
    }
    case 'mean-shift-test':
      return `${fact.interpretation.nullHypothesis} p ${formatP(fact.interpretation.pValue, { withLabel: false }).text}; ${fact.interpretation.pValue < fact.interpretation.alpha ? 'the simulated mean differs from zero at α = 0.05' : 'the procedure does not reject a zero simulated mean at α = 0.05'}.`
    default: return assertNever(fact.interpretation)
  }
}

function DmlRefutationCard({ run, estimation, study, current, onDelete }: { readonly run: Extract<SensitivityRunArtifact, { readonly kind: 'dml-refutation-run' }>; readonly estimation: EstimationRunArtifact; readonly study: StudySpecification; readonly current: boolean; readonly onDelete?: () => void }) {
  const { evidence } = run
  const stamp = `${describeEstimator(estimation.configuration.kind)} · seed ${evidence.seed} · order ${evidence.order.join(' → ')} · ${formatTime(run.createdAt)}`
  if (!current) {
    return (
      <RunFold title="Double machine learning probe batch" figure={`placebo ${formatStatistic('raw', evidence.placebo.refutedEffect).text} · robustness ${formatStatistic('score', evidence.sensitivity.robustnessValue).text}`} stamp={stamp} onDelete={onDelete} deleteLabel="Delete this probe">
        <DmlRefutationRecord run={run} study={study} />
      </RunFold>
    )
  }
  return (
    <article className="rounded-xl border border-edge bg-panel p-4" aria-label={`${estimandSentence(study)} DML refutation`}>
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <span className={label('text-signal')}>Current probe</span>
        <span className={num('text-micro text-faint')}>{stamp}</span>
      </div>
      <DmlRefutationRecord run={run} study={study} />
    </article>
  )
}

function DmlRefutationRecord({ run, study }: { readonly run: Extract<SensitivityRunArtifact, { readonly kind: 'dml-refutation-run' }>; readonly study: StudySpecification }) {
  const { evidence } = run
  return (
    <>
      <h3 className="mb-1 mt-2 text-title font-medium text-ink">Double machine learning probe batch on {lowerFirst(estimandSentence(study))}</h3>
      <p className="m-0 text-body text-muted">Main estimate <span className={num('text-ink')}>{formatStatistic('raw', evidence.mainEstimate).text}</span>. The placebo and random-common-cause probes use the same seeded stream in the recorded order.</p>
      <ResultInterpretation interpretation={interpretSensitivityResult(run)} className="mt-3" />
      <ul className="m-0 mt-3 list-none divide-y divide-hair border-y border-hair p-0" aria-label="Double machine learning probes">
        {run.refuters.map((fact) => (
          <li key={fact.id} className="py-2">
            <span className={label('text-faint')}>{fact.id === 'placebo' ? 'Placebo treatment' : 'Random common cause'}</span>
            <p className={num('mb-0 mt-1 text-body text-ink')}>{formatStatistic('raw', fact.original).text} → {formatStatistic('raw', fact.refuted).text}</p>
            <p className="mb-0 mt-1 text-body text-muted">{refuterInterpretation(fact)}</p>
          </li>
        ))}
      </ul>
      <div className="figure-strip mt-3 overflow-x-auto">
        <table className="w-full border-collapse text-left text-body" aria-label="Confounding scenarios">
          <thead className="text-faint">
            <tr><th className="border-b border-hair px-2 py-1.5 font-normal">Confounding share</th><th className="border-b border-hair px-2 py-1.5 text-right font-normal">Effect bounds</th><th className="border-b border-hair px-2 py-1.5 text-right font-normal">Interval bounds</th></tr>
          </thead>
          <tbody>
            {evidence.sensitivity.scenarios.map((scenario) => (
              <tr key={scenario.confounding} className="border-b border-hair last:border-0">
                <td className={num('px-2 py-1.5 text-ink')}>{Math.round(scenario.confounding * 100)}%</td>
                <td className={num('px-2 py-1.5 text-right text-muted')}>{formatStatistic('raw', scenario.effectLower).text} to {formatStatistic('raw', scenario.effectUpper).text}</td>
                <td className={num('px-2 py-1.5 text-right text-muted')}>{formatStatistic('raw', scenario.ciLower).text} to {formatStatistic('raw', scenario.ciUpper).text}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      <p className={num('mb-0 mt-2 text-body text-muted')}>Robustness value {formatStatistic('score', evidence.sensitivity.robustnessValue).text} (interval {formatStatistic('score', evidence.sensitivity.robustnessValueCi).text}): the equal confounding share that would move the effect, or its interval, to zero.</p>
    </>
  )
}

function RefutationCard({ run, estimation, study, current, onDelete }: { readonly run: Extract<SensitivityRunArtifact, { readonly kind: 'linear-refutation-run' }>; readonly estimation: EstimationRunArtifact; readonly study: StudySpecification; readonly current: boolean; readonly onDelete?: () => void }) {
  const { evidence } = run
  const stamp = `${describeEstimator(estimation.configuration.kind)} · seed ${evidence.seed} · ${formatCount(evidence.simulations).text} simulations · ${formatTime(run.createdAt)}`
  if (!current) {
    return (
      <RunFold title="Perturbation and residual probes" figure={`placebo ${formatStatistic('raw', evidence.placeboEffect).text} · subset ${formatStatistic('raw', evidence.subsetEffect).text}`} stamp={stamp} onDelete={onDelete} deleteLabel="Delete this probe">
        <RefutationRecord run={run} study={study} />
      </RunFold>
    )
  }
  return (
    <article className="rounded-xl border border-edge bg-panel p-4" aria-label={`${estimandSentence(study)} refutation`}>
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <span className={label('text-signal')}>Current probe</span>
        <span className={num('text-micro text-faint')}>{stamp}</span>
      </div>
      <RefutationRecord run={run} study={study} />
    </article>
  )
}

function RefutationRecord({ run, study }: { readonly run: Extract<SensitivityRunArtifact, { readonly kind: 'linear-refutation-run' }>; readonly study: StudySpecification }) {
  const { evidence } = run
  return (
    <>
      <h3 className="mb-1 mt-2 text-title font-medium text-ink">Perturbation probes on {lowerFirst(estimandSentence(study))}</h3>
      <p className="m-0 text-body text-muted">Original linear back-door estimate <span className={num('text-ink')}>{formatStatistic('raw', evidence.estimate).text}</span>. Interpret each diagnostic according to its stated perturbation and reference value.</p>
      <ResultInterpretation interpretation={interpretSensitivityResult(run)} className="mt-3" />
      <ul className={figureGrid('m-0 mt-3 list-none p-0 @2xl/panel:grid-cols-3')} aria-label="Refuters">
        {run.refuters.map((fact) => (
          <li key={fact.id}>
            <MetricTile label={fact.id === 'placebo' ? 'Placebo treatment' : fact.id === 'data-subset' ? `Data subset · ${Math.round(evidence.subsetFraction * 100)}%` : 'Random common cause'} size="compact" frame="cell" className="h-full" value={formatStatistic('raw', fact.refuted)} context={refuterInterpretation(fact)} />
          </li>
        ))}
      </ul>
      <span className={label('mt-4 block text-faint')}>Residual diagnostics</span>
      <ul className="m-0 mt-1 list-none divide-y divide-hair border-y border-hair p-0" aria-label="Residual diagnostics">
        {run.diagnostics.map((fact) => <li key={fact.id} className="py-2 text-body text-muted">{fact.reading}</li>)}
      </ul>
      <details className="mt-3 text-body">
        <summary className="cursor-pointer text-ink">Ljung–Box by lag</summary>
        <div className="figure-strip mt-2 overflow-x-auto">
        <table className="w-full border-collapse text-body" aria-label="Ljung-Box by lag">
          <thead><tr className="text-left"><th scope="col" className={label('px-2 py-1 font-normal text-muted')}>Lag</th><th scope="col" className={label('px-2 py-1 text-right font-normal text-muted')}>Q</th><th scope="col" className={label('px-2 py-1 text-right font-normal text-muted')}>p</th></tr></thead>
          <tbody>
            {evidence.ljungBoxLags.map((lag, index) => (
              <tr key={lag} className="border-t border-hair">
                <td className={num('px-2 py-1 text-ink')}>{lag}</td>
                <td className={num('px-2 py-1 text-right text-muted')}>{formatStatistic('raw', evidence.ljungBoxStatistics[index] ?? Number.NaN).text}</td>
                <td className={num('px-2 py-1 text-right text-muted')}>{formatP(evidence.ljungBoxPValues[index] ?? Number.NaN, { withLabel: false }).text}</td>
              </tr>
            ))}
          </tbody>
        </table>
        </div>
      </details>
      <details className="mt-3 rounded-lg border border-hair bg-well px-3 py-2 text-body">
        <summary className="cursor-pointer text-ink">Run details</summary>
        <dl className="mb-0 mt-2 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-micro text-faint">
          <dt>Probe</dt><dd className={literal('m-0 break-all')}>{run.id}</dd>
          <dt>Estimation run</dt><dd className={literal('m-0 break-all')}>{run.estimationRun}</dd>
          <dt>Configuration</dt><dd className={literal('m-0 break-all')}>{JSON.stringify(run.configuration)}</dd>
          <dt>Created</dt><dd className={literal('m-0')}>{formatTimestamp(run.createdAt)}</dd>
        </dl>
      </details>
    </>
  )
}

function UnobservedCard({ run, estimation, study, current, onDelete }: { readonly run: Extract<SensitivityRunArtifact, { readonly kind: 'unobserved-confounding-run' }>; readonly estimation: EstimationRunArtifact; readonly study: StudySpecification; readonly current: boolean; readonly onDelete?: () => void }) {
  const theme = useChartTheme()
  const { evidence } = run
  const flat = evidence.effects.flat()
  const least = Math.min(...flat)
  const most = Math.max(...flat)
  const flips = flat.filter((value) => Math.sign(value) !== Math.sign(evidence.originalEffect)).length
  const option = useMemo(() => matrixHeatmapOption({
    title: 'Simulated confounder grid',
    sources: evidence.kappaT.map((value) => `flip ${formatStatistic('score', value).text}`),
    targets: evidence.kappaY.map((value) => `shift ${formatStatistic('raw', value).text}`),
    values: evidence.effects,
    scale: 'signed',
    quantity: 'refitted effect',
  }, theme), [evidence, theme])
  const stamp = `${describeEstimator(estimation.configuration.kind)} · seed ${evidence.seed} · ${formatTime(run.createdAt)}`
  const record = (
    <>
      <h3 className="mb-1 mt-2 text-title font-medium text-ink">Simulated unmeasured confounder</h3>
      <p className="m-0 text-body text-muted">Rows vary the simulated effect on treatment assignment. Columns vary the simulated outcome shift. Each cell reports a refitted linear back-door estimate. Original estimate: <span className={num('text-ink')}>{formatStatistic('raw', evidence.originalEffect).text}</span>.</p>
      <ResultInterpretation interpretation={interpretSensitivityResult(run)} className="mt-3" />
      {flat.length === 1 && <Alert tone="info" live={false} className="mt-3"><p className="m-0">The inferred strengths collapsed to one point because a single observed common cause bounds them. Set explicit ranges to sweep a grid.</p></Alert>}
      <div className={figureGrid('mt-3 @2xl/panel:grid-cols-3')} aria-label="Grid facts">
        <MetricTile label="Smallest effect" size="compact" frame="cell" value={formatStatistic('raw', least)} context="over the grid" />
        <MetricTile label="Largest effect" size="compact" frame="cell" value={formatStatistic('raw', most)} context="over the grid" />
        <MetricTile label="Sign changes" size="compact" frame="cell" value={formatCount(flips)} context={`of ${formatCount(flat.length).text} cells`} />
      </div>
      <EChart option={option} label="Refitted effect over simulated confounder strengths" className="mt-3 h-[300px]" testId="unobserved-grid" />
      <details className="mt-3 rounded-lg border border-hair bg-well px-3 py-2 text-body">
        <summary className="cursor-pointer text-ink">Run details</summary>
        <dl className="mb-0 mt-2 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-micro text-faint">
          <dt>Probe</dt><dd className={literal('m-0 break-all')}>{run.id}</dd>
          <dt>Estimation run</dt><dd className={literal('m-0 break-all')}>{run.estimationRun}</dd>
          <dt>Treatment flip strength</dt><dd className={literal('m-0 break-all')}>{evidence.kappaT.map((value) => formatStatistic('score', value).text).join(', ')}</dd>
          <dt>Outcome shift strength</dt><dd className={literal('m-0 break-all')}>{evidence.kappaY.map((value) => formatStatistic('raw', value).text).join(', ')}</dd>
          <dt>Created</dt><dd className={literal('m-0')}>{formatTimestamp(run.createdAt)}</dd>
        </dl>
      </details>
    </>
  )
  if (!current) {
    return (
      <RunFold title="Unmeasured confounder" figure={`${evidence.kappaT.length}×${evidence.kappaY.length} grid`} stamp={stamp} onDelete={onDelete} deleteLabel="Delete this probe">
        {record}
      </RunFold>
    )
  }
  return (
    <article className="rounded-xl border border-edge bg-panel p-4" aria-label={`${estimandSentence(study)} unmeasured confounder`}>
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <span className={label('text-signal')}>Current probe</span>
        <span className={num('text-micro text-faint')}>{stamp}</span>
      </div>
      {record}
    </article>
  )
}

export function SensitivityPanel({ source, profile, prepared, studies, estimationRuns, runs, onRun, onDeleteRun, onActivity }: {
  readonly onActivity?: (activity: RunActivity | null) => void
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly studies: readonly StudySpecification[]
  readonly estimationRuns: readonly EstimationRunArtifact[]
  readonly runs: readonly SensitivityRunArtifact[]
  readonly onRun: (run: SensitivityRunArtifact) => void
  readonly onDeleteRun: (run: SensitivityRunArtifact['id']) => void
}) {
  const [state, dispatch] = useReducer(step, null, (): State => ({
    estimationRun: [...estimationRuns].reverse().find((run) => run.kind === 'backdoor-linear-run')?.id ?? estimationRuns.at(-1)?.id ?? null,
    probe: 'linear-refutation',
    configurations: { 'linear-refutation': DEFAULT_REFUTATION, 'unobserved-confounding': DEFAULT_UNOBSERVED, 'dml-refutation': DEFAULT_DML_REFUTATION },
    job: { kind: 'idle' },
  }))
  const estimation = estimationRuns.find((run) => run.id === state.estimationRun) ?? null
  const study = estimation === null ? null : studies.find((candidate) => candidate.id === estimation.study) ?? null
  const configuration = state.configurations[state.probe]
  const eligibility = probeEligibility(state.probe, estimation, null)
  const configure = (next: SensitivityConfiguration) => dispatch({ type: 'configured', configuration: next })

  useRunActivity(onActivity, state.job.kind === 'running' ? { label: describeProbe(state.probe), progress: null } : null)
  const execute = async () => {
    if (estimation === null || study === null || eligibility.kind === 'refused' || state.job.kind === 'running') return
    dispatch({ type: 'job-started' })
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, analysis] = await Promise.all([import('@/data/prepared'), import('@/analysis/client')])
      const adjustmentVariables = contemporaneousAdjustmentVariables(estimation.estimate.adjustment)
      if (adjustmentVariables === null) { dispatch({ type: 'job-failed', detail: 'This probe requires contemporaneous adjustment columns; the selected run used time-indexed adjustment rows.' }); return }
      const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, ...adjustmentVariables]
      const matrix = await materialisePrepared(source, profile, prepared, columns.map((column) => column.column) as unknown as NonEmptyArray<ColumnId>)
      if (!matrix.ok) { dispatch({ type: 'job-failed', detail: describePreparedMaterialisationProblem(matrix.error) }); return }
      const adjustment = adjustmentVariables.map((_, index) => index + 2)
      const identity = { id: newSensitivityRunId(), estimationRun: estimation.id, preparedDataset: prepared.id, createdAt: new Date().toISOString(), columns } as const
      switch (configuration.kind) {
        case 'linear-refutation': {
          const result = await analysis.runLinearRefutation(matrix.value.values, matrix.value.rowCount, columns.length, { treatment: 0, outcome: 1, adjustment, simulations: configuration.simulations, subsetFraction: configuration.subsetFraction, seed: configuration.seed, ljungBoxLags: configuration.ljungBoxLags })
          if (!result.ok) { dispatch({ type: 'job-failed', detail: result.error.detail }); return }
          const evidence = result.value
          const refuters: NonEmptyArray<RefuterFact> = [
            { id: 'placebo', method: PLACEBO_REFUTER_METHOD_ID, original: evidence.estimate, refuted: evidence.placeboEffect, interpretation: { kind: 'reference-distance', reference: 'zero', reading: 'A permuted treatment should produce an estimate near zero.' } },
            { id: 'data-subset', method: DATA_SUBSET_REFUTER_METHOD_ID, original: evidence.estimate, refuted: evidence.subsetEffect, interpretation: { kind: 'reference-distance', reference: 'original-estimate', reading: 'The estimate on the sampled rows should be close to the full-data estimate if it is not driven by particular rows.' } },
            { id: 'random-common-cause', method: RANDOM_COMMON_CAUSE_REFUTER_METHOD_ID, original: evidence.estimate, refuted: evidence.randomCommonCauseEffect, interpretation: { kind: 'reference-distance', reference: 'original-estimate', reading: 'Adding an independent random covariate should not materially move the estimate.' } },
          ]
          const lastLag = evidence.ljungBoxLags.at(-1) ?? 0
          const lastP = evidence.ljungBoxPValues.at(-1) ?? Number.NaN
          const diagnostics: NonEmptyArray<DiagnosticFact> = [
            { id: 'ljung-box', method: LJUNG_BOX_METHOD_ID, reading: `Ljung–Box to lag ${lastLag}: p ${formatP(lastP, { withLabel: false }).text}${lastP < 0.05 ? '; residuals are autocorrelated' : '; no autocorrelation detected'}.` },
            { id: 'shapiro-wilk', method: SHAPIRO_WILK_METHOD_ID, reading: evidence.shapiroW === null || evidence.shapiroP === null ? 'Shapiro–Wilk not run: outside 3 to 5000 rows.' : `Shapiro–Wilk W ${formatStatistic('score', evidence.shapiroW).text}, p ${formatP(evidence.shapiroP, { withLabel: false }).text}.` },
            { id: 'durbin-watson', method: LJUNG_BOX_METHOD_ID, reading: `Durbin–Watson ${formatStatistic('raw', evidence.durbinWatson).text}${evidence.durbinWatson < 1.5 ? '; positive serial correlation' : evidence.durbinWatson > 2.5 ? '; negative serial correlation' : '; near 2'}.` },
          ]
          onRun({ ...identity, kind: 'linear-refutation-run', configuration, evidence, refuters, diagnostics })
          dispatch({ type: 'job-finished' })
          return
        }
        case 'dml-refutation': {
          if (estimation.kind !== 'double-ml-run') { dispatch({ type: 'job-failed', detail: 'The DML batch needs a double machine learning run.' }); return }
          const result = await analysis.runDmlRefutationBatch(matrix.value.values, matrix.value.rowCount, columns.length, { treatment: 0, outcome: 1, adjustment, model: estimation.evidence.model, att: estimation.evidence.att, seed: configuration.seed })
          if (!result.ok) { dispatch({ type: 'job-failed', detail: result.error.detail }); return }
          const evidence = result.value
          const refuters: NonEmptyArray<RefuterFact> = [
            { id: 'placebo', method: DML_REFUTATION_METHOD_ID, original: evidence.placebo.originalEffect, refuted: evidence.placebo.refutedEffect, interpretation: { kind: 'mean-shift-test', nullHypothesis: 'Null: the mean estimate across permuted-treatment refits is zero.', pValue: evidence.placebo.pValue, alpha: 0.05 } },
            { id: 'random-common-cause', method: DML_REFUTATION_METHOD_ID, original: evidence.randomCommonCause.originalEffect, refuted: evidence.randomCommonCause.refutedEffect, interpretation: { kind: 'mean-shift-test', nullHypothesis: 'Null: the mean shift after adding an independent random covariate is zero.', pValue: evidence.randomCommonCause.pValue, alpha: 0.05 } },
          ]
          onRun({ ...identity, kind: 'dml-refutation-run', configuration, evidence, refuters })
          dispatch({ type: 'job-finished' })
          return
        }
        case 'unobserved-confounding': {
          const treatment = Array.from(matrix.value.values.subarray(0, matrix.value.rowCount))
          if (treatment.some((value) => value !== 0 && value !== 1)) { dispatch({ type: 'job-failed', detail: `${study.treatment.name} is not binary; the simulation flips a 0 or 1 treatment.` }); return }
          const result = await analysis.runUnobservedConfounding(matrix.value.values, matrix.value.rowCount, columns.length, { treatment: 0, outcome: 1, adjustment, seed: configuration.seed, kappaT: kappaValues(configuration.kappaT), kappaY: kappaValues(configuration.kappaY) })
          if (!result.ok) { dispatch({ type: 'job-failed', detail: result.error.detail }); return }
          onRun({ ...identity, kind: 'unobserved-confounding-run', configuration, evidence: result.value })
          dispatch({ type: 'job-finished' })
          return
        }
        default:
          return assertNever(configuration)
      }
    } catch (cause: unknown) {
      dispatch({ type: 'job-failed', detail: cause instanceof Error ? cause.message : String(cause) })
    }
  }

  const latest = runs.at(-1) ?? null
  const [pendingDelete, setPendingDelete] = useState<SensitivityRunArtifact | null>(null)
  const stage = (
    <section aria-labelledby="sensitivity-title" className="@container/panel flex flex-col gap-5">
      <div>
        <span className={label('text-faint')}>07 · Sensitivity</span>
        <h2 id="sensitivity-title" className="mb-2 mt-2 text-heading text-ink">Assess sensitivity to assumptions</h2>
        <p className="m-0 max-w-[65ch] text-body text-muted">A sensitivity analysis examines how an estimate changes when a specified part of the analysis is perturbed. In this chapter, apply procedures supported by the selected estimator and interpret each result against that procedure's reference value. Please note that stability under one perturbation does not assess the remaining assumptions.</p>
      </div>

      <section className="rounded-xl border border-hair bg-panel p-4" aria-labelledby="sensitivity-setup-title">
        <h3 id="sensitivity-setup-title" className="mb-3 mt-0 text-title font-medium text-ink">{describeProbe(state.probe)}</h3>
        {estimationRuns.length === 0 ? (
          <Alert tone="info" live={false}><p className="m-0">Run an estimate before choosing a sensitivity probe.</p></Alert>
        ) : (
          <>
            <label className="block">
              <span className={fieldLabel}>Estimation run</span>
              <Select className={field('text', 'mt-1')} value={state.estimationRun ?? ''} onChange={(event) => dispatch({ type: 'run-chosen', run: event.target.value === '' ? null : (event.target.value as EstimationRunId) })}>
                {[...estimationRuns].reverse().map((run) => {
                  const bound = studies.find((candidate) => candidate.id === run.study)
                  return <option key={run.id} value={run.id}>{bound === undefined ? run.id : `${estimandSentence(bound)} · ${describeEstimator(run.configuration.kind)} · ${formatTime(run.createdAt)}`}</option>
                })}
              </Select>
            </label>
            <div className="mt-4">
              <RadioList legend="Probe" value={state.probe} onChange={(probe) => dispatch({ type: 'probe-chosen', probe })} options={PROBES.map((probe) => ({ value: probe, label: describeProbe(probe), hint: probeHint(probe) }))} />
            </div>
            <div className="mt-4 grid gap-3 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4">
              {configuration.kind === 'linear-refutation' && (
                <>
                  <label className="block"><span className={fieldLabel}>Simulations</span><input type="number" min={1} max={2000} className={field('text', 'mt-1')} value={configuration.simulations} onChange={(event) => configure({ ...configuration, simulations: Math.max(1, Math.min(2000, Number(event.target.value) || 1)) })} /></label>
                  <label className="block"><span className={fieldLabel}>Subset fraction</span><input type="number" step="0.05" min={0.15} max={0.95} className={field('text', 'mt-1')} value={configuration.subsetFraction} onChange={(event) => configure({ ...configuration, subsetFraction: Math.max(0.15, Math.min(0.95, Number(event.target.value) || 0.8)) })} /></label>
                  <label className="block"><span className={fieldLabel}>Seed</span><input type="number" min={0} className={field('text', 'mt-1')} value={configuration.seed} onChange={(event) => configure({ ...configuration, seed: Math.max(0, Math.floor(Number(event.target.value) || 0)) })} /></label>
                  <label className="block"><span className={fieldLabel}>Ljung–Box lags</span><input type="number" min={1} max={200} className={field('text', 'mt-1')} value={configuration.ljungBoxLags} onChange={(event) => configure({ ...configuration, ljungBoxLags: Math.max(1, Math.min(200, Number(event.target.value) || 1)) })} /></label>
                </>
              )}
              {configuration.kind === 'dml-refutation' && (
                <>
                  <label className="block"><span className={fieldLabel}>Fold seed</span><input type="number" min={0} aria-label="Batch fold seed" className={field('text', 'mt-1')} value={configuration.seed} onChange={(event) => configure({ ...configuration, seed: Math.max(0, Math.floor(Number(event.target.value) || 0)) })} /></label>
                  <p className="m-0 self-end text-body text-faint @md/panel:col-span-2 @4xl/panel:col-span-3">Main fit, placebo, random common cause, then confounding bounds, all from this seed. Use the estimation run’s seed{estimation?.kind === 'double-ml-run' ? ` (${estimation.configuration.seed})` : ''} so the main fit repeats it.</p>
                </>
              )}
              {configuration.kind === 'unobserved-confounding' && (
                <>
                  <label className="block"><span className={fieldLabel}>Seed</span><input type="number" min={0} className={field('text', 'mt-1')} value={configuration.seed} onChange={(event) => configure({ ...configuration, seed: Math.max(0, Math.floor(Number(event.target.value) || 0)) })} /></label>
                  {(['kappaT', 'kappaY'] as const).map((axis) => {
                    const range = configuration[axis]
                    const title = axis === 'kappaT' ? 'Treatment flip strength' : 'Outcome shift strength'
                    return (
                      <div key={axis} className="@md/panel:col-span-2 @4xl/panel:col-span-1">
                        <span className={fieldLabel}>{title}</span>
                        <div className="mt-1 flex flex-wrap items-center gap-2">
                          <SegmentedControl ariaLabel={title} value={range.kind} onChange={(kind) => configure({ ...configuration, [axis]: kind === 'inferred' ? { kind: 'inferred' } : axis === 'kappaT' ? { kind: 'range', from: 0.05, to: 0.5, steps: 10 } : { kind: 'range', from: 0, to: Math.abs(estimation?.estimate.effect.kind === 'additive' ? estimation.estimate.effect.value : 1), steps: 10 } })} options={[{ value: 'inferred', label: 'Inferred' }, { value: 'range', label: 'Range' }]} />
                          {range.kind === 'range' && (
                            <>
                              <input type="number" step="any" aria-label={`${title} from`} className={field('text', 'w-20')} value={range.from} onChange={(event) => configure({ ...configuration, [axis]: { ...range, from: Number(event.target.value) || 0 } })} />
                              <span className="text-body text-faint">to</span>
                              <input type="number" step="any" aria-label={`${title} to`} className={field('text', 'w-20')} value={range.to} onChange={(event) => configure({ ...configuration, [axis]: { ...range, to: Number(event.target.value) || 0 } })} />
                              <span className="text-body text-faint">in</span>
                              <input type="number" min={1} max={200} aria-label={`${title} steps`} className={field('text', 'w-16')} value={range.steps} onChange={(event) => configure({ ...configuration, [axis]: { ...range, steps: Math.max(1, Math.min(200, Number(event.target.value) || 1)) } })} />
                              <span className="text-body text-faint">steps</span>
                            </>
                          )}
                        </div>
                      </div>
                    )
                  })}
                </>
              )}
            </div>
            {eligibility.kind === 'refused' && <Alert tone="warn" live={false} className="mt-4"><p className="m-0">{eligibility.reason}</p></Alert>}
            {state.job.kind === 'failed' && <Alert tone="danger" className="mt-3"><p className="m-0">The probe could not run: {state.job.detail}</p></Alert>}
            <div className="mt-4 flex items-center gap-3">
              <button type="button" className={button('signal')} disabled={eligibility.kind === 'refused'} aria-busy={state.job.kind === 'running'} onClick={state.job.kind === 'running' ? undefined : () => void execute()}>
                Run {lowerFirst(describeProbe(state.probe))}
              </button>
              {state.job.kind === 'running' && <Orb state="working" aria-label="Probe running" />}
            </div>
          </>
        )}
      </section>

      {latest !== null ? (
        <section aria-labelledby="sensitivity-results-title" className="grid gap-4">
          <div>
            <h2 id="sensitivity-results-title" className="m-0 text-title font-medium text-ink">Probes</h2>
          </div>
          {(() => {
            const target = estimationRuns.find((candidate) => candidate.id === latest.estimationRun)
            const bound = target === undefined ? undefined : studies.find((candidate) => candidate.id === target.study)
            if (target === undefined || bound === undefined) return null
            switch (latest.kind) {
              case 'linear-refutation-run': return <RefutationCard run={latest} estimation={target} study={bound} current />
              case 'unobserved-confounding-run': return <UnobservedCard run={latest} estimation={target} study={bound} current />
              case 'dml-refutation-run': return <DmlRefutationCard run={latest} estimation={target} study={bound} current />
              default: return assertNever(latest)
            }
          })()}
        </section>
      ) : estimationRuns.length > 0 ? (
        <EmptyState>Choose a probe and run it against the selected estimate.</EmptyState>
      ) : null}
    </section>
  )

  const inspector = (
    <div className="space-y-4">
      <section aria-labelledby="sensitivity-target-title">
        <h3 id="sensitivity-target-title" className="mb-2 mt-1 text-body font-medium text-ink">{study === null ? 'No run chosen' : estimandSentence(study)}</h3>
        {estimation !== null && (
          <dl className="m-0 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-body" aria-label="Estimate under test">
            <dt className="text-faint">Estimator</dt><dd className="m-0 text-ink">{describeEstimator(estimation.configuration.kind)}</dd>
            <dt className="text-faint">Adjustment set</dt><dd className="m-0 text-ink">{adjustmentLabels(estimation.estimate.adjustment).length === 0 ? 'None' : adjustmentLabels(estimation.estimate.adjustment).join(', ')}</dd>
            <dt className="text-faint">Rows</dt><dd className={num('m-0 text-ink')}>{formatCount(estimation.estimate.sample.observations).text}</dd>
          </dl>
        )}
      </section>
      <MethodCaveats methods={state.probe === 'linear-refutation' ? [REFUTER_METHODS[0], REFUTER_METHODS[1], REFUTER_METHODS[2], ...SENSITIVITY_DIAGNOSTIC_METHODS] : state.probe === 'dml-refutation' ? DML_SENSITIVITY_METHODS : [REFUTER_METHODS[3]]} />
    </div>
  )

  const ledger = (
    <ul className="m-0 list-none divide-y divide-hair p-0 text-body" aria-label="Probe ledger">
      {runs.length === 0 && <li className="px-3 py-2 text-faint">Choose a probe and run it against an estimate.</li>}
      {[...runs].reverse().map((run) => {
        const target = estimationRuns.find((candidate) => candidate.id === run.estimationRun)
        const bound = target === undefined ? undefined : studies.find((candidate) => candidate.id === target.study)
        if (target === undefined || bound === undefined) return null
        const remove = () => setPendingDelete(run)
        switch (run.kind) {
          case 'linear-refutation-run': return <RefutationCard key={run.id} run={run} estimation={target} study={bound} current={false} onDelete={remove} />
          case 'unobserved-confounding-run': return <UnobservedCard key={run.id} run={run} estimation={target} study={bound} current={false} onDelete={remove} />
          case 'dml-refutation-run': return <DmlRefutationCard key={run.id} run={run} estimation={target} study={bound} current={false} onDelete={remove} />
          default: return assertNever(run)
        }
      })}
    </ul>
  )

  const deleteDialog = (
    <ConfirmDialog
      open={pendingDelete !== null}
      title="Delete this probe?"
      danger
      confirmLabel="Delete probe"
      message="Removes this probe record. Recorded results cannot be restored."
      onConfirm={() => { if (pendingDelete !== null) onDeleteRun(pendingDelete.id) }}
      onClose={() => setPendingDelete(null)}
    />
  )

  return (
    <>
    <WorkbenchLayout
      id="sensitivity"
      stage={stage}
      inspector={{ title: 'Estimate and method requirements', body: inspector }}
      bottom={{ title: `Probes · ${runs.length}`, body: <>{ledger}{deleteDialog}</>, defaultSize: 150 }}
    />
    </>
  )
}
