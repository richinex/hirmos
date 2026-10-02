import { RunActions } from '@/components/ui/RunActions'
import { RunDetails } from '@/components/ui/RunDetails'
import { DisclosureSummary } from '@/components/ui/DisclosureSummary'
import { ChapterHeading } from '@/components/ui/ChapterHeading'
import { Orb } from '@/components/ui/Orb'
import { ConfirmDialog } from '@/components/ui/ConfirmDialog'
import { EmptyState } from '@/components/ui/EmptyState'
import { RunFold } from '@/components/ui/RunFold'
import { RunMeta } from '@/components/ui/RunMeta'
import { Select } from '@/components/ui/Select'
import { useMemo, useState } from 'react'
import { useWorkflow } from '@/components/WorkflowProvider'
import { emptySensitivityDraft, type SensitivityEvent } from '@/domain/sensitivityDraft'
import { useJob } from '@/analysis/JobsProvider'
import { JobNotice } from '@/components/ui/JobNotice'
import { EChart } from '@/charts/EChart'
import { matrixHeatmapOption } from '@/charts/discovery/matrixHeatmap'
import { useChartTheme } from '@/charts/theme'
import { MethodCaveats } from '@/components/MethodCaveats'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { Alert } from '@/components/ui/Alert'
import { MetricGrid, MetricTile } from '@/components/ui/figures'
import { ResultInterpretation } from '@/components/ui/ResultInterpretation'
import { RadioList } from '@/components/ui/RadioList'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { actionGap, button, chapterIntro, field, fieldHint, fieldLabel, label, literal, num, panel, sectionTitle, stepsStack, well } from '@/components/ui/recipes'
import { SettingsStep } from '@/components/ui/SettingsStep'
import type { ColumnId, DatasetProfile } from '@/domain/dataset'
import { assertNever, type NonEmptyArray } from '@/domain/dop'
import { adjustmentLabels, contemporaneousAdjustmentVariables, describeEstimator, type EstimationRunArtifact, type EstimationRunId } from '@/domain/estimation'
import { DATA_SUBSET_REFUTER_METHOD_ID, LJUNG_BOX_METHOD_ID, PLACEBO_REFUTER_METHOD_ID, RANDOM_COMMON_CAUSE_REFUTER_METHOD_ID,
  DML_REFUTATION_METHOD_ID,
  DML_SENSITIVITY_METHODS, REFUTER_METHODS, SENSITIVITY_DIAGNOSTIC_METHODS, SHAPIRO_WILK_METHOD_ID, UNOBSERVED_COMMON_CAUSE_METHOD_ID } from '@/domain/methods'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import {
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
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import type { SelectedSource } from '@/domain/workflow'
import { formatCount, formatP, formatStatistic } from '@/lib/format/number'
import { lowerFirst } from '@/lib/text'
import { SENSITIVITY_PARAMETER_HELP } from '@/domain/parameterHelp'
import { useRunActivity } from '@/lib/useRunActivity'
import type { RunActivity } from '@/domain/activity'
import { formatTime, formatTimestamp } from '@/lib/format/date'
import { interpretSensitivityResult } from '@/domain/resultInterpretation'
import { cn } from '@/lib/utils'
import {HonestDidPanel} from './HonestDidPanel'
import type {TimeSeriesRun} from '@/domain/timeSeries'
import type {LegacySensitivityRunArtifact} from '@/domain/sensitivity'

const PROBES: NonEmptyArray<SensitivityProbe> = ['linear-refutation', 'unobserved-confounding', 'dml-refutation']

const probeHint = (probe: SensitivityProbe): string => {
  switch (probe) {
    case 'linear-refutation': return 'Hirmos refits the linear back-door estimate under placebo-treatment, data-subset and random-common-cause perturbations, and reports residual diagnostics.'
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
  const stamp = <RunMeta>{[describeEstimator(estimation.configuration.kind), `Seed ${evidence.seed}`, `Order ${evidence.order.join(' → ')}`, formatTime(run.createdAt)]}</RunMeta>
  if (!current) {
    return (
      <RunFold title="Double machine learning probe batch" figure={<RunMeta>{[`Placebo ${formatStatistic('raw', evidence.placebo.refutedEffect).text}`, `Robustness ${formatStatistic('score', evidence.sensitivity.robustnessValue).text}`]}</RunMeta>} stamp={stamp} onDelete={onDelete} deleteLabel="Delete this probe">
        <RunDetails label="Probe run details"><RunRecord run={run} /></RunDetails>
        <DmlRefutationRecord run={run} study={study} />
      </RunFold>
    )
  }
  return (
    <article className="rounded-xl bg-panel lift p-4" aria-label={`${estimandSentence(study)} DML refutation`}>
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <span className={label('text-signal-text')}>Current probe</span>
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
      <ResultInterpretation interpretation={interpretSensitivityResult(run)} context="sensitivity-check" className="mt-3" />
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
        <table className="w-full border-collapse text-left text-table" aria-label="Confounding scenarios">
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
  const stamp = <RunMeta>{[describeEstimator(estimation.configuration.kind), `Seed ${evidence.seed}`, `${formatCount(evidence.simulations).text} simulations`, formatTime(run.createdAt)]}</RunMeta>
  if (!current) {
    return (
      <RunFold title="Perturbation and residual probes" figure={<RunMeta>{[`Placebo ${formatStatistic('raw', evidence.placeboEffect).text}`, `Subset ${formatStatistic('raw', evidence.subsetEffect).text}`]}</RunMeta>} stamp={stamp} onDelete={onDelete} deleteLabel="Delete this probe">
        <RunDetails label="Probe run details"><RunRecord run={run} /></RunDetails>
        <RefutationRecord run={run} study={study} />
      </RunFold>
    )
  }
  return (
    <article className="rounded-xl bg-panel lift p-4" aria-label={`${estimandSentence(study)} refutation`}>
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <span className={label('text-signal-text')}>Current probe</span>
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
      <ResultInterpretation interpretation={interpretSensitivityResult(run)} context="sensitivity-check" className="mt-3" />
      <MetricGrid as="ul" className="m-0 mt-3" label="Refuters">
        {run.refuters.map((fact) => (
          <li key={fact.id}>
            <MetricTile label={fact.id === 'placebo' ? 'Placebo treatment' : fact.id === 'data-subset' ? `Data subset, ${Math.round(evidence.subsetFraction * 100)}%` : 'Random common cause'} size="compact" frame="cell" className="h-full" value={formatStatistic('raw', fact.refuted)} context={refuterInterpretation(fact)} />
          </li>
        ))}
      </MetricGrid>
      <span className={label('mt-4 block text-faint')}>Residual diagnostics</span>
      <ul className="m-0 mt-1 list-none divide-y divide-hair border-y border-hair p-0" aria-label="Residual diagnostics">
        {run.diagnostics.map((fact) => <li key={fact.id} className="py-2 text-body text-muted">{fact.reading}</li>)}
      </ul>
      <details className="mt-3 text-body">
        <DisclosureSummary className="cursor-pointer text-ink">Ljung–Box by lag</DisclosureSummary>
        <div className="figure-strip mt-2 overflow-x-auto">
        <table className="w-full border-collapse text-table" aria-label="Ljung-Box by lag">
          <thead><tr className="text-left"><th scope="col" className="px-2 py-1 text-label font-medium text-muted">Lag</th><th scope="col" className="px-2 py-1 text-right text-label font-medium text-muted">Q</th><th scope="col" className="px-2 py-1 text-right text-label font-medium text-muted">p</th></tr></thead>
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
  const stamp = <RunMeta>{[describeEstimator(estimation.configuration.kind), `Seed ${evidence.seed}`, formatTime(run.createdAt)]}</RunMeta>
  const record = (
    <>
      <h3 className={cn(sectionTitle, 'mb-1 mt-2')}>Simulated unmeasured confounder</h3>
      <p className="m-0 text-body text-muted">Rows vary the simulated effect on treatment assignment. Columns vary the simulated outcome shift. Each cell reports a refitted linear back-door estimate. Original estimate: <span className={num('text-ink')}>{formatStatistic('raw', evidence.originalEffect).text}</span>.</p>
      <ResultInterpretation interpretation={interpretSensitivityResult(run)} context="sensitivity-check" className="mt-3" />
      {flat.length === 1 && <Alert tone="info" live={false} className="mt-3"><p className="m-0">The inferred strengths collapsed to one point because a single observed common cause bounds them. Set explicit ranges to sweep a grid.</p></Alert>}
      <MetricGrid className="mt-3" label="Grid facts">
        <MetricTile label="Smallest effect" size="compact" frame="cell" value={formatStatistic('raw', least)} context="over the grid" />
        <MetricTile label="Largest effect" size="compact" frame="cell" value={formatStatistic('raw', most)} context="over the grid" />
        <MetricTile label="Sign changes" size="compact" frame="cell" value={formatCount(flips)} context={`of ${formatCount(flat.length).text} cells`} />
      </MetricGrid>
      <EChart option={option} label="Refitted effect over simulated confounder strengths" className="mt-3 h-[300px]" testId="unobserved-grid" />

    </>
  )
  if (!current) {
    return (
      <RunFold title="Unmeasured confounder" figure={`${evidence.kappaT.length}×${evidence.kappaY.length} grid`} stamp={stamp} onDelete={onDelete} deleteLabel="Delete this probe">
        <RunDetails label="Probe run details"><RunRecord run={run} /></RunDetails>
        {record}
      </RunFold>
    )
  }
  return (
    <article className="rounded-xl bg-panel lift p-4" aria-label={`${estimandSentence(study)} unmeasured confounder`}>
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <span className={label('text-signal-text')}>Current probe</span>
        <span className={num('text-micro text-faint')}>{stamp}</span>
      </div>
      {record}
    </article>
  )
}

function LegacySensitivityPanel({ source, profile, prepared, studies, estimationRuns, runs, onRun: recordRun, onDeleteRun, onActivity, selector }: {
  readonly onActivity?: (activity: RunActivity | null) => void
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly studies: readonly StudySpecification[]
  readonly estimationRuns: readonly EstimationRunArtifact[]
  readonly runs: readonly LegacySensitivityRunArtifact[]
  readonly selector: React.ReactNode
  readonly onRun: (run: SensitivityRunArtifact) => void
  readonly onDeleteRun: (run: SensitivityRunArtifact['id']) => void
}) {
  const state = useWorkflow(store => store.sensitivityDraft?.prepared === prepared.id ? store.sensitivityDraft.draft : emptySensitivityDraft)
  const change = useWorkflow(store => store.changeSensitivity)
  const dispatch = (event: SensitivityEvent) => change(prepared.id, event)
  const estimation = estimationRuns.find((run) => run.id === state.estimationRun) ?? null
  const study = estimation === null ? null : studies.find((candidate) => candidate.id === estimation.study) ?? null
  const configuration = state.configurations[state.probe]
  const eligibility = probeEligibility(state.probe, estimation, null)
  const configure = (next: SensitivityConfiguration) => dispatch({ type: 'configured', configuration: next })

  const session = useJob('sensitivity')
  const { job } = session
  useRunActivity(onActivity, job.kind === 'running' ? { label: describeProbe(state.probe), progress: null } : null)
  const execute = async () => {
    if (estimation === null || study === null || eligibility.kind === 'refused') return
    const current = session.start('analysis', describeProbe(state.probe))
    if (current === null) return
    const fail = (detail: string) => session.fail(current, detail)
    const onRun = (run: SensitivityRunArtifact) => {
      if (!session.current(current)) return
      recordRun(run)
      session.finish(current)
    }
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, analysis] = await Promise.all([import('@/data/prepared'), import('@/analysis/client')])
      if (!session.current(current)) return
      const adjustmentVariables = contemporaneousAdjustmentVariables(estimation.estimate.adjustment)
      if (adjustmentVariables === null) { fail('This probe requires contemporaneous adjustment columns; the selected run used time-indexed adjustment rows.'); return }
      const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, ...adjustmentVariables]
      const matrix = await materialisePrepared(source, profile, prepared, columns.map((column) => column.column) as unknown as NonEmptyArray<ColumnId>)
      if (!session.current(current)) return
      if (!matrix.ok) { fail(describePreparedMaterialisationProblem(matrix.error)); return }
      const adjustment = adjustmentVariables.map((_, index) => index + 2)
      const identity = { id: newSensitivityRunId(), estimationRun: estimation.id, preparedDataset: prepared.id, createdAt: new Date().toISOString(), columns } as const
      switch (configuration.kind) {
        case 'linear-refutation': {
          const result = await analysis.runLinearRefutation(matrix.value.values, matrix.value.rowCount, columns.length, { treatment: 0, outcome: 1, adjustment, simulations: configuration.simulations, subsetFraction: configuration.subsetFraction, seed: configuration.seed, ljungBoxLags: configuration.ljungBoxLags })
          if (!session.current(current)) return
          if (!result.ok) { fail(describeAnalysisWorkerProblem(result.error)); return }
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
          return
        }
        case 'dml-refutation': {
          if (estimation.kind !== 'double-ml-run') { fail('The DML batch needs a double machine learning run.'); return }
          const result = await analysis.runDmlRefutationBatch(matrix.value.values, matrix.value.rowCount, columns.length, { treatment: 0, outcome: 1, adjustment, model: estimation.evidence.model, att: estimation.evidence.att, seed: configuration.seed })
          if (!session.current(current)) return
          if (!result.ok) { fail(describeAnalysisWorkerProblem(result.error)); return }
          const evidence = result.value
          const refuters: NonEmptyArray<RefuterFact> = [
            { id: 'placebo', method: DML_REFUTATION_METHOD_ID, original: evidence.placebo.originalEffect, refuted: evidence.placebo.refutedEffect, interpretation: { kind: 'mean-shift-test', nullHypothesis: 'Null: the mean estimate across permuted-treatment refits is zero.', pValue: evidence.placebo.pValue, alpha: 0.05 } },
            { id: 'random-common-cause', method: DML_REFUTATION_METHOD_ID, original: evidence.randomCommonCause.originalEffect, refuted: evidence.randomCommonCause.refutedEffect, interpretation: { kind: 'mean-shift-test', nullHypothesis: 'Null: the mean shift after adding an independent random covariate is zero.', pValue: evidence.randomCommonCause.pValue, alpha: 0.05 } },
          ]
          onRun({ ...identity, kind: 'dml-refutation-run', configuration, evidence, refuters })
          return
        }
        case 'unobserved-confounding': {
          const treatment = Array.from(matrix.value.values.subarray(0, matrix.value.rowCount))
          if (treatment.some((value) => value !== 0 && value !== 1)) { fail(`The simulation flips a 0 or 1 treatment, so it cannot run because ${study.treatment.name} is not binary.`); return }
          const result = await analysis.runUnobservedConfounding(matrix.value.values, matrix.value.rowCount, columns.length, { treatment: 0, outcome: 1, adjustment, seed: configuration.seed, kappaT: kappaValues(configuration.kappaT), kappaY: kappaValues(configuration.kappaY) })
          if (!session.current(current)) return
          if (!result.ok) { fail(describeAnalysisWorkerProblem(result.error)); return }
          onRun({ ...identity, kind: 'unobserved-confounding-run', configuration, evidence: result.value })
          return
        }
        default:
          return assertNever(configuration)
      }
    } catch (cause: unknown) {
      fail(cause instanceof Error ? cause.message : String(cause))
    }
  }

  const latest = runs.at(-1) ?? null
  const [pendingDelete, setPendingDelete] = useState<SensitivityRunArtifact | null>(null)
  const stage = (
    <section aria-labelledby="sensitivity-title" className="@container/panel flex flex-col gap-5">
      <div>
        <ChapterHeading id="sensitivity-title" className="mb-2">Sensitivity</ChapterHeading>
        <p className={chapterIntro}>A sensitivity analysis examines how an estimate changes when a specified part of the analysis is perturbed. In this section, apply procedures supported by the selected estimator and interpret each result against that procedure's reference value. Please note that stability under one perturbation does not assess the remaining assumptions.</p>
      </div>

      <section className={panel('p-(--panel-space)')} aria-label="Sensitivity setup">
        {selector}
        {estimationRuns.length === 0 ? (
          <Alert tone="info" live={false}><p className="m-0">Run an estimate before choosing a sensitivity probe.</p></Alert>
        ) : (
          <>
            <div className={stepsStack}>
            <SettingsStep number={1} title="Choose the estimate">
            <label className="block max-w-2xl">
              <span className={fieldLabel}>Estimation run</span>
              <Select className={field('text', 'mt-1')} value={state.estimationRun ?? ''} onChange={(event) => dispatch({ type: 'run-chosen', run: event.target.value === '' ? null : (event.target.value as EstimationRunId) })}>
                {[...estimationRuns].reverse().map((run) => {
                  const bound = studies.find((candidate) => candidate.id === run.study)
                  return <option key={run.id} value={run.id}>{bound === undefined ? run.id : `${estimandSentence(bound)}, ${describeEstimator(run.configuration.kind)}, ${formatTime(run.createdAt)}`}</option>
                })}
              </Select>
            </label>
            </SettingsStep>
            <SettingsStep number={2} title="Choose the probe">
              <RadioList frame="none" legend="Probe" legendHidden className="max-w-3xl" value={state.probe} onChange={(probe) => dispatch({ type: 'probe-chosen', probe })} options={PROBES.map((probe) => ({ value: probe, label: describeProbe(probe), hint: probeHint(probe) }))} />
            </SettingsStep>
            <SettingsStep number={3} title="Set the probe">
            <div className="grid max-w-4xl items-start gap-4 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4">
              {configuration.kind === 'linear-refutation' && (
                <>
                  <label className="block"><ParameterLabel className={fieldLabel} label="Simulations" help={SENSITIVITY_PARAMETER_HELP.linearRefutation.simulations} /><input aria-label="Simulations" type="number" min={1} max={2000} className={field('text', 'mt-1')} value={configuration.simulations} onChange={(event) => configure({ ...configuration, simulations: Math.max(1, Math.min(2000, Number(event.target.value) || 1)) })} /></label>
                  <label className="block"><ParameterLabel className={fieldLabel} label="Subset fraction" help={SENSITIVITY_PARAMETER_HELP.linearRefutation.subsetFraction} /><input aria-label="Subset fraction" type="number" step="0.05" min={0.15} max={0.95} className={field('text', 'mt-1')} value={configuration.subsetFraction} onChange={(event) => configure({ ...configuration, subsetFraction: Math.max(0.15, Math.min(0.95, Number(event.target.value) || 0.8)) })} /></label>
                  <label className="block"><ParameterLabel className={fieldLabel} label="Seed" help={SENSITIVITY_PARAMETER_HELP.linearRefutation.seed} /><input aria-label="Seed" type="number" min={0} className={field('text', 'mt-1')} value={configuration.seed} onChange={(event) => configure({ ...configuration, seed: Math.max(0, Math.floor(Number(event.target.value) || 0)) })} /></label>
                  <label className="block"><ParameterLabel className={fieldLabel} label="Ljung–Box lags" help={SENSITIVITY_PARAMETER_HELP.linearRefutation.ljungBoxLags} /><input aria-label="Ljung–Box lags" type="number" min={1} max={200} className={field('text', 'mt-1')} value={configuration.ljungBoxLags} onChange={(event) => configure({ ...configuration, ljungBoxLags: Math.max(1, Math.min(200, Number(event.target.value) || 1)) })} /></label>
                </>
              )}
              {configuration.kind === 'dml-refutation' && (
                <>
                  <label className="block"><ParameterLabel className={fieldLabel} label="Fold seed" help={SENSITIVITY_PARAMETER_HELP.dmlRefutation.foldSeed} /><input type="number" min={0} aria-label="Batch fold seed" className={field('text', 'mt-1')} value={configuration.seed} onChange={(event) => configure({ ...configuration, seed: Math.max(0, Math.floor(Number(event.target.value) || 0)) })} /></label>
                  <p className={cn(fieldHint, 'm-0 self-end @md/panel:col-span-2 @4xl/panel:col-span-3')}>Main fit, placebo, random common cause, then confounding bounds, all from this seed. Use the estimation run’s seed{estimation?.kind === 'double-ml-run' ? ` (${estimation.configuration.seed})` : ''} so the main fit repeats it.</p>
                </>
              )}
              {configuration.kind === 'unobserved-confounding' && (
                <>
                  <label className="block max-w-xs @md/panel:col-span-2 @4xl/panel:col-span-4"><ParameterLabel className={fieldLabel} label="Seed" help={SENSITIVITY_PARAMETER_HELP.unobservedConfounding.seed} /><input aria-label="Seed" type="number" min={0} className={field('text', 'mt-1')} value={configuration.seed} onChange={(event) => configure({ ...configuration, seed: Math.max(0, Math.floor(Number(event.target.value) || 0)) })} /></label>
                  {(['kappaT', 'kappaY'] as const).map((axis) => {
                    const range = configuration[axis]
                    const title = axis === 'kappaT' ? 'Treatment flip strength' : 'Outcome shift strength'
                    return (
                      <div key={axis} className="col-span-full">
                        <ParameterLabel className={fieldLabel} label={title} help={axis === 'kappaT' ? SENSITIVITY_PARAMETER_HELP.unobservedConfounding.treatmentFlipStrength : SENSITIVITY_PARAMETER_HELP.unobservedConfounding.outcomeShiftStrength} />
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
            </SettingsStep>
            </div>
            <div className={cn(actionGap, 'grid gap-3')}>
            {eligibility.kind === 'refused' && <Alert tone="warn" live={false}><p className="m-0">{eligibility.reason}</p></Alert>}
            <JobNotice job={job} />
            <RunActions running={job.kind === 'running'} onCancel={session.cancel} orb="working" orbLabel="Probe running">
              <button type="button" className={button('signal')} disabled={eligibility.kind === 'refused' || job.kind === 'running' || session.blocked} aria-busy={job.kind === 'running'} onClick={() => void execute()}>
                Run {lowerFirst(describeProbe(state.probe))}
              </button>
            </RunActions>
            </div>
          </>
        )}
      </section>

      {latest !== null ? (
        <section aria-labelledby="sensitivity-results-title" className="grid gap-4">
          <div>
            <h2 id="sensitivity-results-title" className={cn(sectionTitle, 'm-0')}>Probes</h2>
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
      inspector={{ trigger: { label: 'Requirements', icon: 'contract' }, title: 'Estimate and method requirements', body: inspector }}
      bottom={{ trigger: { label: 'History', icon: 'history' }, title: `Probes (${runs.length})`, body: <>{ledger}{deleteDialog}</>, defaultSize: 150 }}
    />
    </>
  )
}

function RunRecord({ run }: { readonly run: LegacySensitivityRunArtifact }) {
  if (run.kind === 'unobserved-confounding-run') { const { evidence } = run; return (<dl className="mb-0 mt-2 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-micro text-faint">
          <dt>Probe</dt><dd className={literal('m-0 break-all')}>{run.id}</dd>
          <dt>Estimation run</dt><dd className={literal('m-0 break-all')}>{run.estimationRun}</dd>
          <dt>Treatment flip strength</dt><dd className={literal('m-0 break-all')}>{evidence.kappaT.map((value) => formatStatistic('score', value).text).join(', ')}</dd>
          <dt>Outcome shift strength</dt><dd className={literal('m-0 break-all')}>{evidence.kappaY.map((value) => formatStatistic('raw', value).text).join(', ')}</dd>
          <dt>Created</dt><dd className={literal('m-0')}>{formatTimestamp(run.createdAt)}</dd>
        </dl>) }
  if (run.kind !== 'linear-refutation-run') return <dl><dt>Probe</dt><dd className="break-all">{run.id}</dd><dt>Estimation run</dt><dd className="break-all">{run.estimationRun}</dd></dl>
  return (<dl className="mb-0 mt-2 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-micro text-faint">
          <dt>Probe</dt><dd className={literal('m-0 break-all')}>{run.id}</dd>
          <dt>Estimation run</dt><dd className={literal('m-0 break-all')}>{run.estimationRun}</dd>
          <dt>Configuration</dt><dd className={literal('m-0 break-all')}>{JSON.stringify(run.configuration)}</dd>
          <dt>Created</dt><dd className={literal('m-0')}>{formatTimestamp(run.createdAt)}</dd>
        </dl>)
}

export function SensitivityPanel(props:Omit<React.ComponentProps<typeof LegacySensitivityPanel>,'runs'|'selector'>&{readonly runs:readonly SensitivityRunArtifact[];readonly designRuns:readonly TimeSeriesRun[]}){
  const [track,setTrack]=useState<'estimation'|'parallelTrends'>(props.runs.at(-1)?.kind==='honest-did-run'||props.estimationRuns.length===0?'parallelTrends':'estimation')
  const selector=<SegmentedControl variant="line" size="sm" ariaLabel="Sensitivity analysis" value={track} onChange={setTrack} options={[{value:'estimation',label:'Estimator probes'},{value:'parallelTrends',label:'Parallel trends'}]} />
  return track==='parallelTrends'?<HonestDidPanel prepared={props.prepared} estimates={props.estimationRuns} designs={props.designRuns} runs={props.runs.filter(run=>run.kind==='honest-did-run')} onRun={props.onRun} onDelete={props.onDeleteRun} onActivity={props.onActivity} selector={selector} />:<LegacySensitivityPanel {...props} runs={props.runs.filter(run=>run.kind!=='honest-did-run')} selector={selector} />
}
