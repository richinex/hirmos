import { SurrogateResult } from '@/components/estimation/SurrogateResult'
import { DidSensitivityResult } from '@/components/sensitivity/DidSensitivityPanel'
import { Manifest } from '@/components/results/ResultRecord'
import { ResultsDesk } from '@/components/results/desk/ResultsDesk'
import { useIsMobile } from '@/lib/useMediaQuery'
import type { SurrogateRun } from '@/domain/surrogateRun'
import { ChapterHeading } from '@/components/ui/ChapterHeading'
import { useMemo, useState } from 'react'
import { TimeSeriesRunResult } from '@/components/time-series/TimeSeriesRunResult'
import { HonestDidResult } from '@/components/sensitivity/HonestDidPanel'
import { PretestCoverageResult } from '@/components/sensitivity/PretestCoveragePanel'
import { CountSeriesRecord } from '@/components/time-series/CountSeriesCard'
import { isRegressionDesignRun, type TimeSeriesRun } from '@/domain/timeSeries'
import type { CountSeriesModelArtifact } from '@/domain/countSeries'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { Metadata } from '@/components/ui/Metadata'
import { SurvivalRunResult, survivalRunLabel } from '@/components/survival/SurvivalRunResult'
import { RootCauseRunResult } from '@/components/root-cause/RootCauseRunResult'
import { GcmEffectResult } from '@/components/root-cause/GcmEffectsPanel'
import type { GcmEffectsRun } from '@/domain/gcmEffects'
import type { GcmInfluenceRun } from '@/domain/gcmInfluence'
import { GcmInfluenceResult } from '@/components/root-cause/GcmInfluencePanel'
import type { RootCauseRun } from '@/domain/rootCauseAnalysis'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import {
  chapterIntroFor,
  label,
  num,
  panel,
  table,
  td,
  th,
  tr,
  well,
} from '@/components/ui/recipes'
import { Select } from '@/components/ui/Select'
import type { CounterfactualRunArtifact } from '@/domain/counterfactual'
import { type DagDocument } from '@/domain/dag'
import type { DatasetProfile } from '@/domain/dataset'
import {
  describeRun,
  headlineValue,
  intervalTypeOf,
  type EstimationRunArtifact,
  type EstimationRunId,
} from '@/domain/estimation'
import {
  frequencyUnit,
  type PreparedDatasetArtifact,
  type StationarityEvidenceArtifact,
} from '@/domain/preprocessing'
import { buildResultManifest, compareResults } from '@/domain/results'
import type { SensitivityRunArtifact } from '@/domain/sensitivity'
import type { SurvivalRunArtifact, SurvivalRunId } from '@/domain/survival'
import {
  estimandSentence,
  type IdentificationArtifact,
  type StudySpecification,
} from '@/domain/study'
import { assertNever } from '@/domain/dop'
import type { SelectedSource } from '@/domain/workflow'
import { formatCount, formatStatistic } from '@/lib/format/number'
import { formatTime } from '@/lib/format/date'

type ResultView =
  | { readonly kind: 'surrogates' }
  | { readonly kind: 'root-cause' }
  | { readonly kind: 'time-series' }
  | { readonly kind: 'regression-designs' }
  | {
      readonly kind: 'estimation'
      readonly selected: EstimationRunId | null
      readonly compareWith: EstimationRunId | null
    }
  | { readonly kind: 'survival'; readonly selected: SurvivalRunId | null }

const initialResultView = (
  estimationRuns: readonly EstimationRunArtifact[],
  survivalRuns: readonly SurvivalRunArtifact[],
  hasTimeSeries: boolean,
  hasRootCause: boolean,
  hasDesigns: boolean,
  hasSurrogates: boolean,
): ResultView =>
  estimationRuns.length > 0
    ? { kind: 'estimation', selected: estimationRuns.at(-1)?.id ?? null, compareWith: null }
    : survivalRuns.length > 0
      ? { kind: 'survival', selected: survivalRuns.at(-1)?.id ?? null }
      : hasDesigns
        ? { kind: 'regression-designs' }
        : hasTimeSeries
          ? { kind: 'time-series' }
          : hasSurrogates
            ? { kind: 'surrogates' }
            : hasRootCause
              ? { kind: 'root-cause' }
              : { kind: 'survival', selected: null }

const availableResultView = (
  view: ResultView,
  estimationRuns: readonly EstimationRunArtifact[],
  survivalRuns: readonly SurvivalRunArtifact[],
  hasTimeSeries: boolean,
  hasRootCause: boolean,
  hasDesigns: boolean,
  hasSurrogates: boolean,
): ResultView => {
  const available = initialResultView(
    estimationRuns,
    survivalRuns,
    hasTimeSeries,
    hasRootCause,
    hasDesigns,
    hasSurrogates,
  )
  switch (view.kind) {
    case 'surrogates':
      return hasSurrogates ? view : available
    case 'root-cause':
      return hasRootCause ? view : available
    case 'time-series':
      return hasTimeSeries ? view : available
    case 'regression-designs':
      return hasDesigns ? view : available
    case 'estimation': {
      if (estimationRuns.length === 0) return available
      const selected = estimationRuns.some((run) => run.id === view.selected)
        ? view.selected
        : (estimationRuns.at(-1)?.id ?? null)
      const compareWith = estimationRuns.some(
        (run) => run.id === view.compareWith && run.id !== selected,
      )
        ? view.compareWith
        : null
      return { kind: view.kind, selected, compareWith }
    }
    case 'survival':
      return survivalRuns.length === 0
        ? available
        : {
            kind: view.kind,
            selected: survivalRuns.some((run) => run.id === view.selected)
              ? view.selected
              : (survivalRuns.at(-1)?.id ?? null),
          }
    default:
      return assertNever(view)
  }
}

const resultIntroduction = (view: ResultView): string => {
  switch (view.kind) {
    case 'surrogates':
      return 'Review long-term effect estimates, sample definitions and recorded identifying assumptions.'
    case 'regression-designs':
      return 'Review regression event-time coefficients, interaction contrasts and the comparisons underlying a two-way fixed-effects coefficient. Their causal interpretation depends on the study design and identifying assumptions.'
    case 'root-cause':
      return 'Review the model’s attributions, their uncertainty and the assumptions behind them.'
    case 'time-series':
      return 'Review count-model scans and long-run relationships fitted to the prepared series. These results do not by themselves estimate the effect of an intervention.'
    case 'estimation':
      return 'A causal result must be interpreted with its causal question, identification strategy, estimate, uncertainty, diagnostics, and assumptions. In this section, examine those parts together, compare runs when the data or analysis choices differ, and export the analysis record.'
    case 'survival':
      return 'Review the event definition, fitted curve or group comparison, uncertainty, and assumptions together. Survival results describe time until an event or movement between states; they are not causal effects unless a separate study design supports that interpretation.'
    default:
      return assertNever(view)
  }
}

export function ResultsPanel({
  project,
  onExportProject,
  source,
  profile,
  prepared,
  stationarity,
  documents,
  studies,
  identifications,
  estimationRuns,
  sensitivityRuns,
  counterfactualRuns,
  survivalRuns,
  surrogateRuns,
  timeSeriesRuns,
  countSeriesModels,
  rootCauseRuns,
  gcmEffects,
  gcmInfluences,
}: {
  readonly project: string
  /** Saves the open project as a bundle file. */
  readonly onExportProject: () => void
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly stationarity: StationarityEvidenceArtifact | null
  readonly documents: readonly DagDocument[]
  readonly studies: readonly StudySpecification[]
  readonly identifications: readonly IdentificationArtifact[]
  readonly estimationRuns: readonly EstimationRunArtifact[]
  readonly sensitivityRuns: readonly SensitivityRunArtifact[]
  readonly counterfactualRuns: readonly CounterfactualRunArtifact[]
  readonly survivalRuns: readonly SurvivalRunArtifact[]
  readonly surrogateRuns: readonly SurrogateRun[]
  readonly timeSeriesRuns: readonly TimeSeriesRun[]
  readonly countSeriesModels: readonly CountSeriesModelArtifact[]
  readonly rootCauseRuns: readonly RootCauseRun[]
  readonly gcmEffects: readonly GcmEffectsRun[]
  readonly gcmInfluences: readonly GcmInfluenceRun[]
}) {
  const mobile = useIsMobile()
  const designRuns = timeSeriesRuns.filter(isRegressionDesignRun)
  const temporalRuns = timeSeriesRuns.filter((run) => !isRegressionDesignRun(run))
  const hasSurrogates = surrogateRuns.length > 0
  const hasDesigns = designRuns.length > 0
  const hasTimeSeries = temporalRuns.length + countSeriesModels.length > 0
  const hasRootCause = rootCauseRuns.length + gcmEffects.length + gcmInfluences.length > 0
  const [view, setView] = useState<ResultView>(() =>
    initialResultView(
      estimationRuns,
      survivalRuns,
      hasTimeSeries,
      hasRootCause,
      hasDesigns,
      hasSurrogates,
    ),
  )
  const activeView = availableResultView(
    view,
    estimationRuns,
    survivalRuns,
    hasTimeSeries,
    hasRootCause,
    hasDesigns,
    hasSurrogates,
  )
  const stepLabel =
    prepared.kind === 'prepared-time-series'
      ? frequencyUnit(prepared.sampling.frequency)
      : prepared.kind === 'prepared-panel'
        ? 'panel row'
        : 'row'
  const inputs = useMemo(
    () => ({
      source,
      profile,
      prepared,
      stationarity,
      documents,
      studies,
      identifications,
      sensitivityRuns,
      counterfactualRuns,
    }),
    [
      counterfactualRuns,
      documents,
      identifications,
      prepared,
      profile,
      sensitivityRuns,
      source,
      stationarity,
      studies,
    ],
  )
  const studyOf = (candidate: EstimationRunArtifact) =>
    studies.find((study) => study.id === candidate.study)

  const selectFamily = (kind: ResultView['kind']) => {
    switch (kind) {
      case 'surrogates':
        setView({ kind })
        return
      case 'root-cause':
        setView({ kind })
        return
      case 'time-series':
        setView({ kind })
        return
      case 'regression-designs':
        setView({ kind })
        return
      case 'estimation':
        setView({ kind, selected: estimationRuns.at(-1)?.id ?? null, compareWith: null })
        return
      case 'survival':
        setView({ kind, selected: survivalRuns.at(-1)?.id ?? null })
        return
      default:
        return assertNever(kind)
    }
  }

  const body = (() => {
    switch (activeView.kind) {
      case 'surrogates':
        return (
          <div className="space-y-4">
            {surrogateRuns.map((run) => (
              <SurrogateResult key={run.id} run={run} profile={profile} />
            ))}
          </div>
        )
      case 'root-cause':
        return (
          <div className="space-y-6">
            {[...rootCauseRuns].reverse().map((run) => (
              <article key={run.id}>
                <div className="mb-2 flex flex-wrap items-baseline justify-between gap-2 text-label text-muted">
                  <span>{run.comparison.name}</span>
                  <time>{formatTime(run.createdAt)}</time>
                </div>
                <RootCauseRunResult run={run} />
              </article>
            ))}
            {[...gcmEffects].reverse().map((run) => (
              <GcmEffectResult key={run.id} run={run} />
            ))}
            {[...gcmInfluences].reverse().map((run) => (
              <GcmInfluenceResult key={run.id} run={run} />
            ))}
          </div>
        )
      case 'time-series':
        return (
          <>
            <div className="space-y-4">
              {[...temporalRuns].reverse().map((run) => (
                <TimeSeriesRunResult key={run.id} run={run} />
              ))}
            </div>
            <ul className="list-none space-y-4 p-0">
              {[...countSeriesModels].reverse().map((artifact) => (
                <CountSeriesRecord key={artifact.id} artifact={artifact} open />
              ))}
            </ul>
          </>
        )
      case 'regression-designs':
        return (
          <div className="space-y-4">
            {[...designRuns].reverse().map((run) => (
              <div key={run.id} className="space-y-4">
                <TimeSeriesRunResult run={run} />
                {sensitivityRuns.map((probe) =>
                  (probe.kind === 'honest-did-run' || probe.kind === 'pretest-coverage-run') &&
                  probe.source.kind === 'regressionDesign' &&
                  probe.source.run === run.id ? (
                    probe.kind === 'pretest-coverage-run' ? (
                      <PretestCoverageResult key={probe.id} run={probe} />
                    ) : (
                      <HonestDidResult key={probe.id} run={probe} />
                    )
                  ) : null,
                )}
              </div>
            ))}
          </div>
        )
      case 'estimation': {
        const run = estimationRuns.find((candidate) => candidate.id === activeView.selected) ?? null
        const other =
          estimationRuns.find((candidate) => candidate.id === activeView.compareWith) ?? null
        const manifest =
          run === null ? null : buildResultManifest(inputs, run, new Date().toISOString())
        const otherManifest =
          other === null ? null : buildResultManifest(inputs, other, new Date().toISOString())
        const differences =
          manifest !== null && otherManifest !== null ? compareResults(manifest, otherManifest) : []
        return (
          <>
            <div className="grid grid-cols-1 gap-3 @lg/panel:grid-cols-2">
              <label className="block">
                <span className="block text-body font-bold text-ink">Estimate</span>
                <Select
                  className="mt-1 w-full rounded-md border border-control bg-well px-2 py-1.5 text-body text-ink"
                  value={activeView.selected ?? ''}
                  onChange={(event) =>
                    setView({
                      ...activeView,
                      selected:
                        event.target.value === '' ? null : (event.target.value as EstimationRunId),
                    })
                  }
                >
                  <option value="" disabled>
                    Choose a run
                  </option>
                  {[...estimationRuns].reverse().map((candidate) => (
                    <option key={candidate.id} value={candidate.id}>
                      {studyOf(candidate) === undefined
                        ? candidate.method
                        : estimandSentence(studyOf(candidate) as StudySpecification)}
                      , {describeRun(candidate.configuration)}, {formatTime(candidate.createdAt)}
                    </option>
                  ))}
                </Select>
              </label>
              <label className="block">
                <span className="block text-body font-bold text-ink">Compare with</span>
                <Select
                  className="mt-1 w-full rounded-md border border-control bg-well px-2 py-1.5 text-body text-ink"
                  value={activeView.compareWith ?? ''}
                  onChange={(event) =>
                    setView({
                      ...activeView,
                      compareWith:
                        event.target.value === '' ? null : (event.target.value as EstimationRunId),
                    })
                  }
                >
                  <option value="">None</option>
                  {[...estimationRuns]
                    .reverse()
                    .filter((candidate) => candidate.id !== activeView.selected)
                    .map((candidate) => (
                      <option key={candidate.id} value={candidate.id}>
                        {studyOf(candidate) === undefined
                          ? candidate.method
                          : estimandSentence(studyOf(candidate) as StudySpecification)}
                        , {describeRun(candidate.configuration)}, {formatTime(candidate.createdAt)}
                      </option>
                    ))}
                </Select>
              </label>
            </div>
            {otherManifest !== null && manifest !== null && (
              <section className={panel('p-(--panel-space)')} aria-label="Differences">
                <h3 className="mb-2 mt-0 text-faint text-label font-medium">
                  <Metadata>
                    <span>Differences</span>
                    <span>{differences.length}</span>
                  </Metadata>
                </h3>
                {differences.length === 0 ? (
                  <p className="m-0 text-body text-muted">
                    The two runs share every recorded field.
                  </p>
                ) : (
                  <div className="figure-strip overflow-x-auto">
                    <table className={table}>
                      <thead>
                        <tr>
                          <th className={th()}>Field</th>
                          <th className={th()}>Selected</th>
                          <th className={th()}>Compared</th>
                        </tr>
                      </thead>
                      <tbody>
                        {differences.map((difference) => (
                          <tr key={difference.field} className={tr()}>
                            <td className={td('text-ink')}>{difference.field}</td>
                            <td className={td('whitespace-normal text-muted')}>
                              {difference.left}
                            </td>
                            <td className={td('whitespace-normal text-muted')}>
                              {difference.right}
                            </td>
                          </tr>
                        ))}
                      </tbody>
                    </table>
                  </div>
                )}
              </section>
            )}
            {manifest !== null && run !== null && (
              <Manifest
                manifest={manifest}
                stepLabel={stepLabel}
                bite={studies
                  .filter((check) => check.biteOf === run.study)
                  .map((check) => ({
                    study: check,
                    run:
                      estimationRuns.filter((candidate) => candidate.study === check.id).at(-1) ??
                      null,
                  }))}
                checked={studies.find((check) => check.id === studyOf(run)?.biteOf) ?? null}
              />
            )}
            {manifest?.sensitivity.map((probe) =>
              probe.kind === 'pretest-coverage-run' ? (
                <PretestCoverageResult key={probe.id} run={probe} />
              ) : probe.kind === 'honest-did-run' ? (
                <HonestDidResult key={probe.id} run={probe} />
              ) : probe.kind === 'did-sensitivity-run' ? (
                <DidSensitivityResult
                  key={probe.id}
                  run={probe}
                  source={estimationRuns.find((r) => r.id === probe.estimationRun)}
                />
              ) : null,
            )}
          </>
        )
      }
      case 'survival': {
        const run = survivalRuns.find((candidate) => candidate.id === activeView.selected) ?? null
        return (
          <>
            <label className="block max-w-xl">
              <span className="block text-body font-bold text-ink">Survival run</span>
              <Select
                className="mt-1 w-full rounded-md border border-control bg-well px-2 py-1.5 text-body text-ink"
                value={activeView.selected ?? ''}
                onChange={(event) => {
                  const selected =
                    survivalRuns.find((candidate) => candidate.id === event.target.value)?.id ??
                    null
                  setView({ ...activeView, selected })
                }}
              >
                <option value="" disabled>
                  Choose a run
                </option>
                {[...survivalRuns].reverse().map((candidate) => (
                  <option key={candidate.id} value={candidate.id}>
                    {survivalRunLabel(candidate)}, {formatTime(candidate.createdAt)}
                  </option>
                ))}
              </Select>
            </label>
            {run === null ? (
              <div className={well('px-4 py-6 text-center text-body text-faint')}>
                Choose a survival run to review.
              </div>
            ) : (
              <SurvivalRunResult run={run} />
            )}
          </>
        )
      }
      default:
        return assertNever(activeView)
    }
  })()

  const bottom = (() => {
    switch (activeView.kind) {
      case 'surrogates':
        return undefined
      case 'root-cause':
        return undefined
      case 'time-series':
        return undefined
      case 'regression-designs':
        return undefined
      case 'estimation':
        return {
          title: `Estimates (${estimationRuns.length})`,
          body: (
            <div className="figure-strip overflow-x-auto">
              <table className={table} aria-label="Estimates">
                <thead>
                  <tr>
                    <th className={th()}>Target</th>
                    <th className={th()}>Estimator</th>
                    <th className={th('text-right')}>Estimate</th>
                    <th className={th()}>Graph</th>
                    <th className={th()}>Created</th>
                  </tr>
                </thead>
                <tbody>
                  {[...estimationRuns].reverse().map((candidate) => {
                    const study = studyOf(candidate)
                    const effect = candidate.estimate.effect
                    return (
                      <tr
                        key={candidate.id}
                        className={tr(candidate.id === activeView.selected ? 'selected' : 'action')}
                        onClick={() => setView({ ...activeView, selected: candidate.id })}
                      >
                        <td className={td('text-ink')}>
                          {study === undefined ? candidate.method : estimandSentence(study)}
                        </td>
                        <td className={td('text-muted')}>{describeRun(candidate.configuration)}</td>
                        <td className={td(num('whitespace-nowrap text-right text-ink'))}>
                          {
                            formatStatistic(
                              'raw',
                              effect.kind === 'path'
                                ? effect.aggregate.average
                                : headlineValue(effect),
                            ).text
                          }
                        </td>
                        <td className={td('text-muted')}>{study?.dagName ?? '—'}</td>
                        <td className={td(num('whitespace-nowrap text-muted'))}>
                          {formatTime(candidate.createdAt)}
                        </td>
                      </tr>
                    )
                  })}
                </tbody>
              </table>
            </div>
          ),
          defaultSize: 180,
        }
      case 'survival':
        return {
          title: `Survival runs (${survivalRuns.length})`,
          body: (
            <div className="figure-strip overflow-x-auto">
              <table className={table} aria-label="Survival runs">
                <thead>
                  <tr>
                    <th className={th()}>Analysis</th>
                    <th className={th()}>Records</th>
                    <th className={th()}>Created</th>
                  </tr>
                </thead>
                <tbody>
                  {[...survivalRuns].reverse().map((candidate) => (
                    <tr
                      key={candidate.id}
                      className={tr(candidate.id === activeView.selected ? 'selected' : 'action')}
                      onClick={() => setView({ ...activeView, selected: candidate.id })}
                    >
                      <td className={td('text-ink')}>{survivalRunLabel(candidate)}</td>
                      <td className={td(num('text-muted'))}>
                        {formatCount(candidate.evidence.observations).text}
                      </td>
                      <td className={td(num('whitespace-nowrap text-muted'))}>
                        {formatTime(candidate.createdAt)}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          ),
          defaultSize: 180,
        }
      default:
        return assertNever(activeView)
    }
  })()

  const families: { value: ResultView['kind']; label: string }[] = [
    ...(hasSurrogates ? [{ value: 'surrogates' as const, label: 'Two-sample surrogates' }] : []),
    ...(hasRootCause ? [{ value: 'root-cause' as const, label: 'Causal model analysis' }] : []),
    ...(estimationRuns.length > 0
      ? [{ value: 'estimation' as const, label: 'Causal estimates' }]
      : []),
    ...(hasDesigns ? [{ value: 'regression-designs' as const, label: 'Regression designs' }] : []),
    ...(hasTimeSeries ? [{ value: 'time-series' as const, label: 'Time series' }] : []),
    ...(survivalRuns.length > 0 ? [{ value: 'survival' as const, label: 'Survival' }] : []),
  ]
  const familyControl =
    families.length > 1 ? (
      <SegmentedControl
        variant="line"
        size="sm"
        ariaLabel="Result family"
        value={activeView.kind}
        onChange={selectFamily}
        options={families}
      />
    ) : null

  if (!mobile)
    return (
      <WorkbenchLayout
        id="results"
        stagePadding={false}
        stageScroll={false}
        stage={
          <ResultsDesk
            project={project}
            onExportProject={onExportProject}
            records={{
              inputs,
              stepLabel,
              studies,
              estimationRuns,
              sensitivityRuns,
              survivalRuns,
              surrogateRuns,
              timeSeriesRuns,
              countSeriesModels,
              rootCauseRuns,
              gcmEffects,
              gcmInfluences,
            }}
          />
        }
      />
    )

  const intro = resultIntroduction(activeView)
  return (
    <WorkbenchLayout
      id="results"
      stage={
        <section aria-labelledby="results-title" className="@container/panel flex flex-col gap-5">
          <div>
            <ChapterHeading id="results-title" className="mb-2">
              Results
            </ChapterHeading>
            <p className={chapterIntroFor(intro)}>{intro}</p>
          </div>
          {familyControl}
          {body}
        </section>
      }
      bottom={bottom}
    />
  )
}
