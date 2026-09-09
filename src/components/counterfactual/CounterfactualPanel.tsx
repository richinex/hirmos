import { Alert } from '@/components/ui/Alert'
import { ConfirmDialog } from '@/components/ui/ConfirmDialog'
import { EmptyState } from '@/components/ui/EmptyState'
import { RunFold } from '@/components/ui/RunFold'
import { Select } from '@/components/ui/Select'
import { useEffect, useMemo, useReducer, useState } from 'react'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { outcomePathsOption } from '@/charts/counterfactual/outcomePaths'
import { counterfactualEffectPathOption } from '@/charts/counterfactual/effectPath'
import { useChartTheme } from '@/charts/theme'
import type { VisibleWindow } from '@/charts/window'
import { EligibilityView } from '@/components/EligibilityView'
import { MethodCaveats } from '@/components/MethodCaveats'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { MetricTile } from '@/components/ui/figures'
import { Formula } from '@/components/ui/Formula'
import { ResultInterpretation } from '@/components/ui/ResultInterpretation'
import { RadioList } from '@/components/ui/RadioList'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { button, chapterIntro, field, fieldLabel, figureGrid, label, literal, num, panel, prose, sectionTitle, well } from '@/components/ui/recipes'
import { DEFAULT_DYNAMIC_LINEAR_SCM, DEFAULT_LINEAR_SCM, evaluateCounterfactualEligibility, newCounterfactualRunId, type CounterfactualConfiguration, type CounterfactualRunArtifact, type DynamicCounterfactualUncertainty } from '@/domain/counterfactual'
import type { DagDocument } from '@/domain/dag'
import type { ColumnId, DatasetProfile } from '@/domain/dataset'
import { assertNever, type NonEmptyArray } from '@/domain/dop'
import { chapterLabel } from '@/domain/navigation'
import { COUNTERFACTUAL_METHODS, DYNAMIC_LINEAR_SCM_METHOD_ID, LINEAR_SCM_METHOD_ID, methodDefinition } from '@/domain/methods'
import { stationaryMarksFromGraph } from '@/domain/estimation'
import { frequencyUnit, type PreparedDatasetArtifact } from '@/domain/preprocessing'
import { estimandSentence, type IdentificationArtifact, type IdentificationId, type StudySpecification, type StudyVariable } from '@/domain/study'
import type { SelectedSource } from '@/domain/workflow'
import { formatCount, formatStatistic, formatWords } from '@/lib/format/number'
import { useRunActivity } from '@/lib/useRunActivity'
import { interpretCounterfactualResult } from '@/domain/resultInterpretation'
import type { RunActivity } from '@/domain/activity'
import { formatTime } from '@/lib/format/date'
import { describeAnalysisWorkerProblem, type AnalysisProgress } from '@/workers/analysisProtocol'
import { cn } from '@/lib/utils'

type Job = { readonly kind: 'idle' } | { readonly kind: 'running'; readonly progress: AnalysisProgress | null } | { readonly kind: 'failed'; readonly detail: string }

interface State {
  readonly identification: IdentificationId | null
  readonly configuration: CounterfactualConfiguration
  readonly job: Job
}

type Event =
  | { readonly type: 'identification-chosen'; readonly identification: IdentificationId | null }
  | { readonly type: 'configured'; readonly configuration: CounterfactualConfiguration }
  | { readonly type: 'run-started' }
  | { readonly type: 'run-progressed'; readonly progress: AnalysisProgress }
  | { readonly type: 'run-failed'; readonly detail: string }
  | { readonly type: 'run-finished' }

const step = (state: State, event: Event): State => {
  switch (event.type) {
    case 'identification-chosen': return { ...state, identification: event.identification, job: { kind: 'idle' } }
    case 'configured': return { ...state, configuration: event.configuration, job: { kind: 'idle' } }
    case 'run-started': return { ...state, job: { kind: 'running', progress: null } }
    case 'run-progressed': return state.job.kind === 'running' ? { ...state, job: { kind: 'running', progress: event.progress } } : state
    case 'run-failed': return { ...state, job: { kind: 'failed', detail: event.detail } }
    case 'run-finished': return { ...state, job: { kind: 'idle' } }
    default: return assertNever(event)
  }
}

const texName = (name: string): string =>
  `\\text{${name.replace(/[\\{}$&#_%^~]/g, (character) => (character === '\\' ? '\\textbackslash{}' : character === '^' || character === '~' ? `\\${character}{}` : `\\${character}`))}}`

const displayedNumber = (value: number): string => formatStatistic('raw', value).text
const texNumber = (value: number): string => displayedNumber(value).replaceAll(',', '').replaceAll('−', '-')


function EquationsTable({ run }: { readonly run: CounterfactualRunArtifact }) {
  const name = (node: number) => run.nodes[node]?.name ?? String(node)
  const equations = (() => {
    switch (run.kind) {
      case 'linear-scm-run': return run.evidence.equations.map((equation) => ({
        variable: equation.node,
        intercept: equation.intercept,
        parents: equation.parents.map(([variable, coefficient]) => ({ variable, lag: 0, coefficient })),
        residualScale: equation.residualSd,
        rSquared: equation.rSquared,
      }))
      case 'dynamic-linear-scm-run': return run.evidence.equations.map((equation) => ({ ...equation, rSquared: null }))
      default: return assertNever(run)
    }
  })()
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
          {equations.map((equation) => {
            const plain = `${name(equation.variable)}(t) = ${displayedNumber(equation.intercept)}${equation.parents.map((parent) => ` ${parent.coefficient < 0 ? '−' : '+'} ${displayedNumber(Math.abs(parent.coefficient))} × ${name(parent.variable)}${parent.lag === 0 ? '(t)' : `(t−${parent.lag})`}`).join('')} + ε(t)`
            const tex = `${texName(name(equation.variable))}_{t} = ${texNumber(equation.intercept)}${equation.parents.map((parent) => ` ${parent.coefficient < 0 ? '-' : '+'} ${texNumber(Math.abs(parent.coefficient))}\\,${texName(name(parent.variable))}_{${parent.lag === 0 ? 't' : `t-${parent.lag}`}}`).join('')} + \\varepsilon_{t}`
            return (
              <tr key={equation.variable} className="border-b border-hair last:border-0">
                <td className="px-2 py-1.5 text-ink">{name(equation.variable)}</td>
                <td className="px-2 py-1.5 text-muted"><Formula tex={tex} plain={plain} /></td>
                <td className={num('px-2 py-1.5 text-right text-muted')}>{formatStatistic('sd', equation.residualScale).text}</td>
                <td className={num('px-2 py-1.5 text-right text-muted')}>{equation.rSquared === null ? '—' : formatStatistic('score', equation.rSquared).text}</td>
              </tr>
            )
          })}
        </tbody>
      </table>
    </div>
  )
}

function RunCard({ run, study, current, stepLabel, onDelete }: { readonly run: CounterfactualRunArtifact; readonly study: StudySpecification; readonly current: boolean; readonly stepLabel: string; readonly onDelete?: () => void }) {
  const theme = useChartTheme()
  const view = (() => {
    switch (run.kind) {
      case 'linear-scm-run': return {
        observations: run.evidence.observations,
        plottedObservations: run.evidence.observations,
        interventions: run.evidence.interventions,
        factual: run.evidence.factualOutcome,
        low: run.evidence.counterfactualLow,
        high: run.evidence.counterfactualHigh,
        effects: run.evidence.effects,
        averageEffect: run.evidence.averageEffect,
        cumulativeEffect: null,
        sharePositive: run.evidence.sharePositive,
        firstStep: 1,
        model: 'linear structural causal model',
        detail: run.evidence.observationNoise === null ? 'exact disturbance terms' : `observation noise ${run.evidence.observationNoise}`,
      }
      case 'dynamic-linear-scm-run': return {
        observations: run.evidence.observations,
        plottedObservations: run.evidence.effects.length,
        interventions: run.evidence.interventions,
        factual: run.evidence.factualOutcome,
        low: run.evidence.counterfactualLow,
        high: run.evidence.counterfactualHigh,
        effects: run.evidence.effects,
        averageEffect: run.evidence.averageEffect,
        cumulativeEffect: run.evidence.cumulativeEffect,
        sharePositive: run.evidence.effects.filter((effect) => effect > 0).length / run.evidence.effects.length,
        firstStep: run.evidence.start + 1,
        model: 'dynamic linear structural causal model',
        detail: run.evidence.timing.kind === 'point' ? `one-time intervention at row ${run.evidence.timing.time + 1}` : `persistent intervention from row ${run.evidence.timing.start + 1}`,
      }
      default: return assertNever(run)
    }
  })()
  const uncertainty = run.kind === 'dynamic-linear-scm-run' && run.evidence.uncertainty.kind === 'blockBootstrap' ? run.evidence.uncertainty : null
  const [row, setRow] = useState(1)
  // The two paths share a time axis, so a zoom on one is a zoom on both.
  const [window, setWindow] = useState<VisibleWindow | null>(null)
  const index = Math.max(1, Math.min(view.plottedObservations, row)) - 1
  const option = useMemo(() => outcomePathsOption({
    outcome: study.outcome.name,
    treatment: study.treatment.name,
    interventions: view.interventions,
    factual: view.factual,
    low: view.low,
    high: view.high,
    stepLabel,
    firstStep: view.firstStep,
  }, theme), [stepLabel, study.outcome.name, study.treatment.name, theme, view.factual, view.firstStep, view.high, view.interventions, view.low])
  const effectOption = useMemo(() => uncertainty === null ? null : counterfactualEffectPathOption({
    outcome: study.outcome.name,
    effects: view.effects,
    lower: uncertainty.pointwiseEffectInterval[0],
    upper: uncertainty.pointwiseEffectInterval[1],
    confidenceLevel: uncertainty.confidenceLevel,
    stepLabel,
    firstStep: view.firstStep,
  }, theme), [stepLabel, study.outcome.name, theme, uncertainty, view.effects, view.firstStep])
  const sd = Math.sqrt(view.effects.reduce((sum, value) => sum + (value - view.averageEffect) ** 2, 0) / Math.max(1, view.effects.length - 1))
  // A linear model with exact abduction gives every row the same difference: the coefficient times the change in treatment.
  const constant = run.kind === 'linear-scm-run' && run.evidence.observationNoise === null && Math.max(...view.effects) - Math.min(...view.effects) < 1e-9 * Math.max(1, Math.abs(view.averageEffect))
  const stamp = `${view.model} · ${view.detail} · ${formatTime(run.createdAt)}`
  const record = (
    <>
      <h3 className="mb-1 mt-2 text-title font-medium text-ink">What {study.outcome.name} would have been with {study.treatment.name} set to {view.interventions[1]} instead of {view.interventions[0]}</h3>
      <p className="m-0 text-body text-muted">{run.kind === 'linear-scm-run' ? `For each ${stepLabel}, the model infers disturbance terms from the observed values and predicts both treatment worlds.` : `The model recovers the observed innovation at each time point, preserves the factual history through row ${view.firstStep - 1}, and replays both treatment worlds through the recorded lagged graph.`} The difference is conditional on the fitted structural equations.</p>
      <div className={figureGrid('mt-3 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4')} aria-label="Counterfactual summary">
        <MetricTile label={constant ? 'Effect for every row' : run.kind === 'dynamic-linear-scm-run' ? 'Average horizon effect' : 'Average individual effect'} size="compact" frame="cell" value={formatStatistic('raw', view.averageEffect)} context={constant ? 'the same for all rows: a linear model with exact abduction gives coefficient × change' : uncertainty === null ? `SD across ${stepLabel}s ${formatStatistic('sd', sd).text}` : `${Math.round(uncertainty.confidenceLevel * 100)}% block-bootstrap CI [${formatStatistic('raw', uncertainty.averageInterval[0]).text}, ${formatStatistic('raw', uncertainty.averageInterval[1]).text}]`} />
        {constant
          ? <MetricTile label="Rows" size="compact" frame="cell" value={formatCount(view.observations)} context="one model-implied outcome pair per observation" />
          : run.kind === 'dynamic-linear-scm-run'
            ? <MetricTile label="Cumulative horizon effect" size="compact" frame="cell" value={formatStatistic('raw', view.cumulativeEffect ?? Number.NaN)} context={uncertainty === null ? `${view.plottedObservations} time points` : `${Math.round(uncertainty.confidenceLevel * 100)}% block-bootstrap CI [${formatStatistic('raw', uncertainty.cumulativeInterval[0]).text}, ${formatStatistic('raw', uncertainty.cumulativeInterval[1]).text}]`} />
            : <MetricTile label="Share positive" size="compact" frame="cell" value={formatStatistic('score', view.sharePositive)} context={`of ${formatCount(view.observations).text} ${stepLabel}s`} />}
        <MetricTile label="Interventions" size="compact" frame="cell" value={formatWords(`${view.interventions[0]} → ${view.interventions[1]}`)} context={view.detail} />
        <MetricTile label="Equations" size="compact" frame="cell" value={formatCount(run.evidence.equations.length)} context={`order ${run.evidence.order.map((node) => run.nodes[node]?.name ?? node).join(' → ')}`} />
      </div>
      <ResultInterpretation interpretation={interpretCounterfactualResult(run, study, stepLabel)} className="mt-3" />
      <ExpandableChart option={option} label={`${study.outcome.name} observed and under both interventions`} className="mt-3 h-[260px]" testId="counterfactual-paths" window={window} onWindow={setWindow} />
      {effectOption !== null && <ExpandableChart window={window} onWindow={setWindow} option={effectOption} label={`${study.outcome.name} counterfactual contrast with pointwise block-bootstrap interval`} className="mt-3 h-[240px]" testId="counterfactual-effect-interval" />}
      <div className={well('mt-3 grid items-start gap-3 p-3 @md/panel:grid-cols-[auto_1fr]')}>
        <label className="block text-body text-ink"><span className={fieldLabel}>Inspect plotted point</span><input type="number" min={1} max={view.plottedObservations} aria-label="Inspect plotted point" className={field('text', 'mt-1 w-28')} value={row} onChange={(event) => setRow(Math.max(1, Math.min(view.plottedObservations, Math.floor(Number(event.target.value) || 1))))} /></label>
        <dl className="m-0 grid w-fit grid-cols-[auto_auto] gap-x-6 gap-y-1 text-body" aria-label={`${stepLabel} ${view.firstStep + index} counterfactual`}>
          <dt className="text-faint">Observed {study.outcome.name}</dt><dd className={num('m-0 text-right text-ink')}>{formatStatistic('raw', view.factual[index] ?? Number.NaN).text}</dd>
          <dt className="text-faint">With {study.treatment.name} = {view.interventions[0]}</dt><dd className={num('m-0 text-right text-ink')}>{formatStatistic('raw', view.low[index] ?? Number.NaN).text}</dd>
          <dt className="text-faint">With {study.treatment.name} = {view.interventions[1]}</dt><dd className={num('m-0 text-right text-ink')}>{formatStatistic('raw', view.high[index] ?? Number.NaN).text}</dd>
          <dt className="border-t border-hair pt-1 text-ink">Difference</dt><dd className={num('m-0 border-t border-hair pt-1 text-right font-medium text-ink')}>{formatStatistic('raw', view.effects[index] ?? Number.NaN).text}</dd>
          {uncertainty !== null && <><dt className="text-faint">Pointwise {Math.round(uncertainty.confidenceLevel * 100)}% CI</dt><dd className={num('m-0 text-right text-muted')}>[{formatStatistic('raw', uncertainty.pointwiseEffectInterval[0][index] ?? Number.NaN).text}, {formatStatistic('raw', uncertainty.pointwiseEffectInterval[1][index] ?? Number.NaN).text}]</dd></>}
        </dl>
      </div>
      <EquationsTable run={run} />
      <details className={well('mt-3 px-3 py-2 text-body')}>
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
      <RunFold title={`${study.treatment.name} ${view.interventions[0]} → ${view.interventions[1]}`} figure={`average ${formatStatistic('raw', view.averageEffect).text}`} stamp={stamp} onDelete={onDelete} deleteLabel="Delete this run">
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

export function CounterfactualPanel({ source, profile, prepared, documents, studies, identifications, runs, onRun, onDeleteRun, onActivity }: {
  readonly onActivity?: (activity: RunActivity | null) => void
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly documents: readonly DagDocument[]
  readonly studies: readonly StudySpecification[]
  readonly identifications: readonly IdentificationArtifact[]
  readonly runs: readonly CounterfactualRunArtifact[]
  readonly onRun: (run: CounterfactualRunArtifact) => void
  readonly onDeleteRun: (run: CounterfactualRunArtifact['id']) => void
}) {
  const identified = identifications.filter((identification) => identification.result.kind === 'identified')
  const [state, dispatch] = useReducer(step, null, (): State => {
    // The controls open as the latest recorded run set them, so a reopened project shows the query it holds.
    const latest = runs.at(-1) ?? null
    const recorded = latest === null ? null : identified.find((candidate) => candidate.id === latest.identification) ?? null
    return {
      identification: recorded?.id ?? identified.at(-1)?.id ?? null,
      configuration: latest !== null && recorded !== null ? latest.configuration : prepared.kind === 'prepared-time-series' ? DEFAULT_DYNAMIC_LINEAR_SCM : DEFAULT_LINEAR_SCM,
      job: { kind: 'idle' },
    }
  })
  const identification = identified.find((candidate) => candidate.id === state.identification) ?? null
  const study = identification === null ? null : studies.find((candidate) => candidate.id === identification.study) ?? null
  const selectedRevision = study === null ? null : documents.find((document) => document.id === study.dagDocument)?.audit.find((revision) => revision.id === study.dagRevision) ?? null
  const selectedMaxLag = selectedRevision === null ? 0 : Math.max(0, ...selectedRevision.graph.edges.map((edge) => edge.timing.kind === 'lagged' ? edge.timing.lag : 0))
  const dynamicStartRow = state.configuration.kind === 'dynamic-linear-scm' ? (state.configuration.schedule.kind === 'point' ? state.configuration.schedule.row : state.configuration.schedule.startRow) : 1
  const dynamicMaxHorizon = Math.max(1, prepared.observations - dynamicStartRow + 1)
  const dynamicConfiguration = state.configuration.kind === 'dynamic-linear-scm' ? state.configuration : null
  const dynamicBootstrap = dynamicConfiguration?.uncertainty.kind === 'blockBootstrap' ? dynamicConfiguration.uncertainty : null
  const methodId = state.configuration.kind === 'linear-scm' ? LINEAR_SCM_METHOD_ID : DYNAMIC_LINEAR_SCM_METHOD_ID
  const method = methodDefinition(methodId)
  const stepLabel = prepared.kind === 'prepared-time-series' ? frequencyUnit(prepared.sampling.frequency) : prepared.kind === 'prepared-panel' ? 'panel row' : 'row'
  const eligibility = useMemo(
    () => (identification === null || study === null || !method.ok ? null : evaluateCounterfactualEligibility(method.value, { study, identification: identification.result, prepared, configuration: state.configuration })),
    [identification, method, prepared, state.configuration, study],
  )
  const configure = (configuration: CounterfactualConfiguration) => dispatch({ type: 'configured', configuration })
  useEffect(() => {
    if (state.configuration.kind !== 'dynamic-linear-scm') return
    const minimum = selectedMaxLag + 1
    const current = state.configuration.schedule.kind === 'point' ? state.configuration.schedule.row : state.configuration.schedule.startRow
    const row = Math.max(minimum, Math.min(prepared.observations, current))
    const steps = Math.max(1, Math.min(state.configuration.steps, prepared.observations - row + 1))
    if (row === current && steps === state.configuration.steps) return
    const schedule = state.configuration.schedule.kind === 'point' ? { kind: 'point' as const, row } : { kind: 'persistent' as const, startRow: row }
    dispatch({ type: 'configured', configuration: { ...state.configuration, schedule, steps } })
  }, [prepared.observations, selectedMaxLag, state.configuration])
  const configureObservationNoise = (observationNoise: number | null) => {
    switch (state.configuration.kind) {
      case 'linear-scm': configure({ ...state.configuration, observationNoise }); return
      case 'dynamic-linear-scm': return
      default: return assertNever(state.configuration)
    }
  }
  const configureDynamicSchedule = (kind: 'point' | 'persistent') => {
    switch (state.configuration.kind) {
      case 'linear-scm': return
      case 'dynamic-linear-scm': {
        const row = state.configuration.schedule.kind === 'point' ? state.configuration.schedule.row : state.configuration.schedule.startRow
        configure({ ...state.configuration, schedule: kind === 'point' ? { kind: 'point', row } : { kind: 'persistent', startRow: row } })
        return
      }
      default: return assertNever(state.configuration)
    }
  }
  const configureDynamicStartRow = (row: number) => {
    switch (state.configuration.kind) {
      case 'linear-scm': return
      case 'dynamic-linear-scm': configure({ ...state.configuration, schedule: state.configuration.schedule.kind === 'point' ? { kind: 'point', row } : { kind: 'persistent', startRow: row } }); return
      default: return assertNever(state.configuration)
    }
  }
  const configureDynamicSteps = (steps: number) => {
    switch (state.configuration.kind) {
      case 'linear-scm': return
      case 'dynamic-linear-scm': configure({ ...state.configuration, steps }); return
      default: return assertNever(state.configuration)
    }
  }
  const configureDynamicUncertainty = (uncertainty: DynamicCounterfactualUncertainty) => {
    switch (state.configuration.kind) {
      case 'linear-scm': return
      case 'dynamic-linear-scm': configure({ ...state.configuration, uncertainty }); return
      default: return assertNever(state.configuration)
    }
  }
  const configureDynamicUncertaintyMode = (kind: DynamicCounterfactualUncertainty['kind']) => {
    switch (kind) {
      case 'none': configureDynamicUncertainty({ kind: 'none' }); return
      case 'blockBootstrap': configureDynamicUncertainty({ kind: 'blockBootstrap', samples: 200, blockLength: { kind: 'cubeRoot' }, confidenceLevel: 0.9, seed: 0 }); return
      default: return assertNever(kind)
    }
  }
  void profile

  useRunActivity(onActivity, state.job.kind === 'running' ? { label: 'Counterfactual', progress: state.job.progress === null ? null : state.job.progress.completed / Math.max(1, state.job.progress.total) } : null)
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
      switch (state.configuration.kind) {
        case 'linear-scm': {
          const result = await analysis.runLinearScmCounterfactual(matrix.value.values, matrix.value.rowCount, nodes.length, {
            nodes: measured.map((_, index) => index),
            names: measured.map((variable) => variable.name),
            edges: study.graph.edges.map(([cause, effect]) => [position(study.graph.nodes[cause]?.node ?? study.treatment.node), position(study.graph.nodes[effect]?.node ?? study.outcome.node)] as const),
            treatment: position(study.treatment.node),
            outcome: position(study.outcome.node),
            interventions: state.configuration.interventions,
            observationNoise: state.configuration.observationNoise,
          })
          if (!result.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(result.error) }); return }
          onRun({ kind: 'linear-scm-run', id: newCounterfactualRunId(), study: study.id, identification: identification.id, preparedDataset: prepared.id, createdAt: new Date().toISOString(), method: LINEAR_SCM_METHOD_ID, configuration: state.configuration, nodes, evidence: result.value, eligibility })
          break
        }
        case 'dynamic-linear-scm': {
          const document = documents.find((candidate) => candidate.id === study.dagDocument)
          const revision = document?.audit.find((candidate) => candidate.id === study.dagRevision)
          if (revision === undefined) { dispatch({ type: 'run-failed', detail: 'The DAG revision recorded by this study is not available.' }); return }
          const stationary = stationaryMarksFromGraph(revision.graph)
          const timing = state.configuration.schedule.kind === 'point'
            ? { kind: 'point' as const, time: state.configuration.schedule.row - 1 }
            : { kind: 'persistent' as const, start: state.configuration.schedule.startRow - 1 }
          const result = await analysis.runDynamicLinearScmCounterfactual(matrix.value.values, matrix.value.rowCount, nodes.length, {
            nodes: measured.map((_, index) => index),
            statLag: stationary.statLag,
            graph: stationary.marks,
            treatment: position(study.treatment.node),
            outcome: position(study.outcome.node),
            timing,
            steps: state.configuration.steps,
            interventions: state.configuration.interventions,
            uncertainty: state.configuration.uncertainty,
          }, (progress) => dispatch({ type: 'run-progressed', progress }))
          if (!result.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(result.error) }); return }
          onRun({ kind: 'dynamic-linear-scm-run', id: newCounterfactualRunId(), study: study.id, identification: identification.id, preparedDataset: prepared.id, createdAt: new Date().toISOString(), method: DYNAMIC_LINEAR_SCM_METHOD_ID, configuration: state.configuration, nodes, evidence: result.value, eligibility })
          break
        }
        default:
          assertNever(state.configuration)
      }
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
        <span className={label('text-faint')}>{chapterLabel('counterfactual')}</span>
        <h2 id="counterfactual-title" className="mb-2 mt-2 text-heading text-ink">Estimate counterfactual outcomes</h2>
        <p className={chapterIntro}>A counterfactual compares outcomes for the same unit or evolving system under alternative interventions. The row-wise model treats observations independently. The dynamic model preserves the recorded lags, infers the innovation at each time point, and propagates an intervention through the later series.</p>
      </div>
      <section className={panel('p-(--panel-space)')} aria-labelledby="counterfactual-setup-title">
        <h3 id="counterfactual-setup-title" className={cn(sectionTitle, 'mb-3 mt-0')}>Structural counterfactual</h3>
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
            <div className="mt-4">
              <span className={fieldLabel}>Structural model</span>
              <SegmentedControl
                className="mt-1"
                ariaLabel="Counterfactual structural model"
                value={state.configuration.kind}
                onChange={(kind) => {
                  switch (kind) {
                    case 'linear-scm': configure(DEFAULT_LINEAR_SCM); break
                    case 'dynamic-linear-scm': {
                      const firstRow = Math.min(prepared.observations, selectedMaxLag + 1)
                      configure({ ...DEFAULT_DYNAMIC_LINEAR_SCM, schedule: { kind: 'point', row: firstRow }, steps: Math.max(1, Math.min(12, prepared.observations - firstRow + 1)) })
                      break
                    }
                    default: assertNever(kind)
                  }
                }}
                options={[
                  { value: 'linear-scm', label: 'Row-wise SCM' },
                  { value: 'dynamic-linear-scm', label: 'Dynamic SCM', disabled: prepared.kind !== 'prepared-time-series', title: prepared.kind === 'prepared-time-series' ? 'Uses contemporaneous and lagged arrows.' : 'Requires a regular time series.' },
                ]}
              />
            </div>
            <div className="mt-4 grid gap-3 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4">
              <label className="block"><span className={fieldLabel}>Set {study?.treatment.name ?? 'treatment'} to</span><input type="number" step="any" aria-label="First intervention value" className={field('text', 'mt-1')} value={state.configuration.interventions[0]} onChange={(event) => configure({ ...state.configuration, interventions: [Number(event.target.value) || 0, state.configuration.interventions[1]] })} /></label>
              <label className="block"><span className={fieldLabel}>and to</span><input type="number" step="any" aria-label="Second intervention value" className={field('text', 'mt-1')} value={state.configuration.interventions[1]} onChange={(event) => configure({ ...state.configuration, interventions: [state.configuration.interventions[0], Number(event.target.value) || 0] })} /></label>
              {state.configuration.kind === 'linear-scm' && <div className="@md/panel:col-span-2">
                <RadioList
                  legend="Infer disturbance terms"
                  value={state.configuration.observationNoise === null ? 'exact' : 'noise'}
                  onChange={(mode) => configureObservationNoise(mode === 'exact' ? null : 0.1)}
                  options={[
                    { value: 'exact', label: 'Exactly', hint: 'Recovers each disturbance term exactly from the fitted equations.' },
                    { value: 'noise', label: 'Allow observation noise', hint: 'Infers disturbance terms allowing observation noise at the chosen scale.' },
                  ]}
                />
              </div>}
              {state.configuration.kind === 'linear-scm' && state.configuration.observationNoise !== null && (
                <label className="block"><span className={fieldLabel}>Noise scale</span><input type="number" step="any" min={0.0001} aria-label="Observation noise scale" className={field('text', 'mt-1')} value={state.configuration.observationNoise} onChange={(event) => configureObservationNoise(Math.max(0.0001, Number(event.target.value) || 0.0001))} /></label>
              )}
              {state.configuration.kind === 'dynamic-linear-scm' && (
                <>
                  <div>
                    <span className={fieldLabel}>Intervention schedule</span>
                    <SegmentedControl
                      className="mt-1"
                      ariaLabel="Dynamic intervention schedule"
                      value={state.configuration.schedule.kind}
                      onChange={configureDynamicSchedule}
                      options={[{ value: 'point', label: 'One time' }, { value: 'persistent', label: 'Persistent' }]}
                    />
                  </div>
                  {state.configuration.schedule.kind === 'point'
                    ? <label className="block"><span className={fieldLabel}>Intervention row</span><input type="number" min={selectedMaxLag + 1} max={prepared.observations} className={field('text', 'mt-1')} value={state.configuration.schedule.row} onChange={(event) => configureDynamicStartRow(Math.max(selectedMaxLag + 1, Math.min(prepared.observations, Math.floor(Number(event.target.value) || selectedMaxLag + 1))))} /></label>
                    : <label className="block"><span className={fieldLabel}>First intervention row</span><input type="number" min={selectedMaxLag + 1} max={prepared.observations} className={field('text', 'mt-1')} value={state.configuration.schedule.startRow} onChange={(event) => configureDynamicStartRow(Math.max(selectedMaxLag + 1, Math.min(prepared.observations, Math.floor(Number(event.target.value) || selectedMaxLag + 1))))} /></label>}
                  <label className="block"><span className={fieldLabel}>Horizon points</span><input type="number" min={1} max={dynamicMaxHorizon} className={field('text', 'mt-1')} value={state.configuration.steps} onChange={(event) => configureDynamicSteps(Math.max(1, Math.min(dynamicMaxHorizon, Math.floor(Number(event.target.value) || 1))))} /></label>
                  <p className="m-0 self-end text-body text-faint">The horizon includes the intervention row. Point mode replaces one treatment value; persistent mode replaces the treatment equation throughout the horizon.</p>
                </>
              )}
            </div>
            {dynamicConfiguration !== null && (
              <div className="mt-4 border-t border-hair pt-4">
                <RadioList frame="none"
                  legend="Sampling uncertainty"
                  value={dynamicConfiguration.uncertainty.kind}
                  onChange={configureDynamicUncertaintyMode}
                  options={[
                    { value: 'blockBootstrap', label: 'Block-bootstrap interval', hint: 'Refits the dynamic equations to circular blocks and reports pointwise and aggregate confidence intervals.' },
                    { value: 'none', label: 'Point estimate only', hint: 'Reports the fitted counterfactual path without a sampling interval.' },
                  ]}
                />
                {dynamicBootstrap !== null && (
                  <div className="mt-3 grid gap-3 @md/panel:grid-cols-2 @4xl/panel:grid-cols-5">
                    <label className="block"><span className={fieldLabel}>Bootstrap refits</span><input type="number" min={20} max={5000} className={field('text', 'mt-1')} value={dynamicBootstrap.samples} onChange={(event) => configureDynamicUncertainty({ ...dynamicBootstrap, samples: Math.max(20, Math.min(5000, Math.floor(Number(event.target.value) || 20))) })} /></label>
                    <div>
                      <span className={fieldLabel}>Block length</span>
                      <SegmentedControl
                        className="mt-1"
                        ariaLabel="Bootstrap block length"
                        value={dynamicBootstrap.blockLength.kind}
                        onChange={(kind) => configureDynamicUncertainty({ ...dynamicBootstrap, blockLength: kind === 'cubeRoot' ? { kind: 'cubeRoot' } : { kind: 'fixed', length: Math.max(1, Math.floor(Math.cbrt(Math.max(1, prepared.observations - selectedMaxLag)))) } })}
                        options={[{ value: 'cubeRoot', label: 'Automatic' }, { value: 'fixed', label: 'Fixed' }]}
                      />
                    </div>
                    {dynamicBootstrap.blockLength.kind === 'fixed' && <label className="block"><span className={fieldLabel}>Points per block</span><input type="number" min={1} max={Math.max(1, prepared.observations - selectedMaxLag - 1)} className={field('text', 'mt-1')} value={dynamicBootstrap.blockLength.length} onChange={(event) => configureDynamicUncertainty({ ...dynamicBootstrap, blockLength: { kind: 'fixed', length: Math.max(1, Math.min(Math.max(1, prepared.observations - selectedMaxLag - 1), Math.floor(Number(event.target.value) || 1))) } })} /></label>}
                    <label className="block"><span className={fieldLabel}>Confidence level</span><input type="number" min={50} max={99} step={1} className={field('text', 'mt-1')} value={Math.round(dynamicBootstrap.confidenceLevel * 100)} onChange={(event) => configureDynamicUncertainty({ ...dynamicBootstrap, confidenceLevel: Math.max(0.5, Math.min(0.99, (Number(event.target.value) || 90) / 100)) })} /></label>
                    <label className="block"><span className={fieldLabel}>Seed</span><input type="number" min={0} max={0xffff_ffff} step={1} className={field('text', 'mt-1')} value={dynamicBootstrap.seed} onChange={(event) => configureDynamicUncertainty({ ...dynamicBootstrap, seed: Math.max(0, Math.min(0xffff_ffff, Math.floor(Number(event.target.value) || 0))) })} /></label>
                  </div>
                )}
              </div>
            )}
            {method.ok && <p className={prose('mb-0 mt-3 text-faint')}>{method.value.summary}</p>}
            {eligibility !== null && <EligibilityView eligibility={eligibility} subject="this study" />}
            {state.job.kind === 'failed' && <Alert tone="danger" className="mt-3"><p className="m-0">The counterfactual could not run: {state.job.detail}</p></Alert>}
            <div className="mt-4 flex items-center gap-3">
              <button type="button" className={button('signal')} disabled={eligibility === null || eligibility.kind === 'refused'} aria-busy={state.job.kind === 'running'} onClick={state.job.kind === 'running' ? undefined : () => void execute()}>
                Run counterfactual
              </button>
            </div>
            {state.job.kind === 'running' && state.job.progress !== null && (
              <div className="mt-2 max-w-sm text-label text-faint">
                <div className="mb-1 flex justify-between gap-3"><span>Bootstrap refits</span><span className={num()}>{state.job.progress.completed} / {state.job.progress.total}</span></div>
                <div className="bar-live h-1.5 w-full overflow-hidden rounded-full bg-line" role="progressbar" aria-valuemin={0} aria-valuemax={state.job.progress.total} aria-valuenow={state.job.progress.completed}>
                <span className="bar-live__fill block rounded-full bg-signal" style={{ width: `${Math.round((state.job.progress.completed / Math.max(1, state.job.progress.total)) * 100)}%` }} />
              </div>
              </div>
            )}
          </>
        )}
      </section>
      {latest !== null && (
        <section aria-labelledby="counterfactual-results-title" className="grid gap-4">
          <div>
            <h2 id="counterfactual-results-title" className={cn(sectionTitle, 'm-0')}>Counterfactuals</h2>
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
            <dt className="text-faint">Graph</dt><dd className="m-0 text-ink">{study.dagName} · {state.configuration.kind === 'dynamic-linear-scm' && selectedRevision !== null
              ? `${selectedRevision.graph.nodes.length} nodes, ${selectedRevision.graph.edges.length} arrows · ${selectedRevision.graph.edges.filter((edge) => edge.timing.kind === 'lagged').length} lagged`
              : `${study.graph.nodes.length} nodes, ${study.graph.edges.length} arrows`}</dd>
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
