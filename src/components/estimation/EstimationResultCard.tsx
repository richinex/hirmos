import { ExpandableChart } from '@/charts/ExpandableChart'
import { histogramOption } from '@/charts/data/histogram'
import { counterfactualCurvesOption } from '@/charts/estimation/counterfactualCurves'
import { impactPathOption } from '@/charts/estimation/impactPath'
import { posteriorDensityOption } from '@/charts/estimation/posteriorDensity'
import { propensityDistributionOption } from '@/charts/estimation/propensityOverlap'
import { propensityWeightOption } from '@/charts/estimation/propensityWeights'
import { runComparisonOption, type RunComparisonRow } from '@/charts/estimation/runComparison'
import { useChartTheme } from '@/charts/theme'
import type { VisibleWindow } from '@/charts/window'
import { EstimateHeadline, headlineFigure, scaleOf } from '@/components/results/EstimateHeadline'
import { DagRevisionNotice } from '@/components/study/DagRevisionNotice'
import { Alert } from '@/components/ui/Alert'
import { ResultInterpretation } from '@/components/ui/ResultInterpretation'
import { RunDetails } from '@/components/ui/RunDetails'
import { RunFold } from '@/components/ui/RunFold'
import { RunMeta } from '@/components/ui/RunMeta'
import { Select } from '@/components/ui/Select'
import { field, label, literal, num } from '@/components/ui/recipes'
import { conventionalDidInferenceLabels } from '@/domain/conventionalDid'
import {
  armaReading,
  configurationSettings,
  contemporaneousAdjustmentVariables,
  describeCovariance,
  describeFixedEffects,
  describeRun,
  headlineValue,
  intervalTypeOf,
  preInterventionPoints,
  summariseRowEffects,
  tLearnerRowEffects,
  type CausalEstimate,
  type EstimationRunArtifact,
} from '@/domain/estimation'
import {
  interpretEstimationResult,
  resultHeadline,
  resultSampleLine,
  resultScaleLine,
} from '@/domain/resultInterpretation'
import { type StudySpecification } from '@/domain/study'
import { formatTime, formatTimestamp } from '@/lib/format/date'
import { formatInterval, formatStatistic } from '@/lib/format/number'
import { Fragment, memo, useMemo, useState } from 'react'
import { AdjustedDidConvergenceWarning } from './AdjustedDidResult'
import { ArmaUncertaintyAlert } from './ArmaUncertaintyAlert'
import { BayesianImpactResult } from './BayesianImpactResult'
import { CausalForestResults } from './CausalForestResults'
import { CovariateBalanceResult } from './CovariateBalanceResult'
import { Diagnostics } from './EstimationDiagnostics'
import { FeWeightDiagnosticResult } from './FeWeightDiagnosticResult'
import { ImpactEffectPanel } from './ImpactEffectPanel'
import { PanelEvidenceDetails, SyntheticControlEvidenceDetails } from './PanelEvidenceDetails'
import { PredictorSyntheticResult } from './PredictorSyntheticResult'
import { RidgeAugmentedResult } from './RidgeAugmentedResult'
import { SharpRdResult } from './SharpRdResult'
import { TLearnerIntervals } from './TLearnerIntervals'

const intervalText = (estimate: CausalEstimate): string => {
  if (estimate.interval.kind === 'none') return 'none'
  const figure = formatInterval(
    headlineValue(estimate.effect),
    estimate.interval.lower,
    estimate.interval.upper,
    intervalTypeOf(estimate.interval),
    scaleOf(estimate),
  )
  return `[${figure.bounds.lower}, ${figure.bounds.upper}]`
}

function RunRecord({ run }: { readonly run: EstimationRunArtifact }) {
  return (
    <>
      <dl className="mb-0 mt-2 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-micro text-faint">
        <dt>Run</dt>
        <dd className={literal('m-0 break-all')}>{run.id}</dd>
        <dt>Study</dt>
        <dd className={literal('m-0 break-all')}>{run.study}</dd>
        <dt>Identification</dt>
        <dd className={literal('m-0 break-all')}>{run.identification}</dd>
        <dt>Prepared dataset</dt>
        <dd className={literal('m-0 break-all')}>{run.preparedDataset}</dd>
        <dt>Columns</dt>
        <dd className="m-0">{run.columns.map((column) => column.name).join(', ')}</dd>
        {configurationSettings(run.configuration).map((setting) => (
          <Fragment key={setting.key}>
            <dt>{setting.label}</dt>
            <dd className="m-0">{setting.value}</dd>
          </Fragment>
        ))}
        {run.kind === 'causal-forest-run' && (
          <>
            <dt>Fitted forest settings</dt>
            <dd className={literal('m-0 break-all')}>{JSON.stringify(run.evidence.fitted)}</dd>
          </>
        )}
        <dt>Created</dt>
        <dd className={literal('m-0')}>{formatTimestamp(run.createdAt)}</dd>
        <dt>Method</dt>
        <dd className={literal('m-0')}>{run.method}</dd>
      </dl>
    </>
  )
}

/** Memoised: editing the form must not redraw the result, its diagnostics or its charts. */
export const ResultCard = memo(function ResultCard({
  run,
  study,
  current,
  stepLabel,
  onDelete,
  others = [],
}: {
  readonly others?: readonly EstimationRunArtifact[]
  readonly run: EstimationRunArtifact
  readonly study: StudySpecification
  readonly current: boolean
  readonly stepLabel: string
  readonly onDelete?: (run: EstimationRunArtifact) => void
}) {
  const theme = useChartTheme()
  const estimate = run.estimate
  const adjustmentVariables = useMemo(
    () => contemporaneousAdjustmentVariables(estimate.adjustment) ?? [],
    [estimate.adjustment],
  )
  const sentence = resultHeadline(run, study)
  const scaleLine = resultScaleLine(run, study, stepLabel)
  const sampleLine = resultSampleLine(run)
  // Other path-valued runs of the same question can be drawn as a ghost behind this one.
  const [ghostId, setGhostId] = useState('')
  const ghosts = others.filter(
    (other) => other.id !== run.id && other.estimate.effect.kind === 'path',
  )
  const ghostRun = ghosts.find((other) => String(other.id) === ghostId) ?? null
  const chart = useMemo(
    () =>
      estimate.effect.kind === 'path' && run.kind !== 'predictor-synthetic-control-run'
        ? impactPathOption(
            {
              outcome: study.outcome.name,
              before:
                run.kind === 'causal-impact-run'
                  ? preInterventionPoints(run.evidence.preInterventionPath)
                  : [],
              points: estimate.effect.values,
              stepLabel,
              ghost:
                ghostRun !== null && ghostRun.estimate.effect.kind === 'path'
                  ? {
                      name: `${describeRun(ghostRun.configuration)}, ${formatTime(ghostRun.createdAt)}`,
                      points: ghostRun.estimate.effect.values,
                    }
                  : undefined,
            },
            theme,
          )
        : null,
    [estimate.effect, run, stepLabel, study.outcome.name, theme, ghostRun],
  )
  // Runs recorded before the histogram was added carry no draws, so they keep the summary alone.
  const posterior = useMemo(
    () =>
      run.kind === 'bayesian-gaussian-run' && (run.evidence.histogramCounts ?? []).length > 0
        ? posteriorDensityOption(
            {
              outcome: study.outcome.name,
              treatment: study.treatment.name,
              histogramStart: run.evidence.histogramStart,
              histogramBinWidth: run.evidence.histogramBinWidth,
              histogramCounts: run.evidence.histogramCounts,
              mean: run.evidence.effectMean,
              hdiLower: run.evidence.hdiLower,
              hdiUpper: run.evidence.hdiUpper,
            },
            theme,
          )
        : null,
    [run, study.outcome.name, study.treatment.name, theme],
  )
  const overlap = useMemo(() => {
    const scored =
      run.kind === 'propensity-weighting-run' ||
      run.kind === 'propensity-matching-run' ||
      run.kind === 'doubly-robust-run'
    if (!scored) return null
    return propensityDistributionOption(
      {
        treatment: study.treatment.name,
        propensity: run.evidence.propensity,
        treated: run.evidence.treated,
        weights: run.kind === 'propensity-weighting-run' ? run.evidence.weights : undefined,
      },
      theme,
    )
  }, [run, study.treatment.name, theme])
  const overlapPanels = run.kind === 'propensity-weighting-run' ? 2 : 1
  const weightScatter = useMemo(
    () =>
      run.kind !== 'propensity-weighting-run'
        ? null
        : propensityWeightOption(
            {
              treatment: study.treatment.name,
              outcomeName: study.outcome.name,
              propensity: run.evidence.propensity,
              treated: run.evidence.treated,
              weights: run.evidence.weights,
              outcome: run.evidence.outcome,
            },
            theme,
          ),
    [run, study.outcome.name, study.treatment.name, theme],
  )
  const [scoreWindow, setScoreWindow] = useState<VisibleWindow | null>(null)
  const [chosenCurve, setChosenCurve] = useState(0)
  const curves = run.kind === 'bayesian-gaussian-run' ? (run.evidence.curves ?? []) : []
  const curveIndex = Math.min(chosenCurve, Math.max(0, curves.length - 1))
  const curveChart = useMemo(() => {
    const curve = curves[curveIndex]
    return curve === undefined
      ? null
      : counterfactualCurvesOption(
          {
            outcome: study.outcome.name,
            treatment: study.treatment.name,
            covariate: adjustmentVariables[curveIndex]?.name ?? `covariate ${curveIndex + 1}`,
            curve,
          },
          theme,
        )
  }, [curves, curveIndex, adjustmentVariables, study.outcome.name, study.treatment.name, theme])
  const stamp = (
    <RunMeta>
      {[
        describeRun(run.configuration),
        ...(run.kind === 'backdoor-linear-run'
          ? [
              describeFixedEffects(run.configuration.fixedEffects),
              describeCovariance(run.configuration.errors),
            ].filter((part): part is string => part !== null)
          : []),
        formatTime(run.createdAt),
      ]}
    </RunMeta>
  )
  // A grouped effect draws each group's interval on the shared axis, the whole-population average last.
  const groupChart = useMemo(
    () =>
      estimate.effect.kind === 'byGroup'
        ? runComparisonOption(
            [
              ...estimate.effect.groups.map((group): RunComparisonRow => ({
                label: group.label,
                estimate: group.value,
                lower: group.interval.lower,
                upper: group.interval.upper,
                current: false,
              })),
              {
                label: 'All rows',
                estimate: estimate.effect.overall,
                lower: estimate.interval.kind === 'none' ? null : estimate.interval.lower,
                upper: estimate.interval.kind === 'none' ? null : estimate.interval.upper,
                current,
              },
            ],
            theme,
          )
        : null,
    [estimate, current, theme],
  )
  // Per-row effects are drawn as their distribution, with the average marked, since a thousand points have no order to plot.
  const rowChart = useMemo(() => {
    const rows = run.kind === 't-learner-run' ? tLearnerRowEffects(run.evidence) : null
    if (run.kind !== 't-learner-run' || rows === null) return null
    const summary = summariseRowEffects(rows)
    return histogramOption(
      {
        name: `effect of ${study.treatment.name} on ${study.outcome.name}`,
        bins: summary.bins,
        nullCount: 0,
        marks: [
          { name: 'average', value: run.evidence.average, tone: 'signal' },
          { name: 'median', value: summary.median, tone: 'info' },
        ],
      },
      theme,
    )
  }, [run, study.treatment.name, study.outcome.name, theme])
  const body = (
    <>
      <DagRevisionNotice study={study} />
      {run.kind === 'panel-intervention-run' && run.evidence.kind === 'panelAdjusted' && (
        <AdjustedDidConvergenceWarning evidence={run.evidence} />
      )}
      <div className="mt-3">
        {run.kind === 'causal-forest-run' ? (
          <CausalForestResults
            evidence={run.evidence}
            outcome={study.outcome.name}
            columns={run.columns}
          />
        ) : (
          <EstimateHeadline
            estimate={estimate}
            sentence={sentence}
            scaleLine={scaleLine}
            sampleLine={sampleLine}
            stepLabel={stepLabel}
            accent={current}
            testId="effect-estimate"
          />
        )}
        {run.kind === 'panel-intervention-run' &&
          run.configuration.primary === 'syntheticDid' &&
          estimate.interval.kind === 'confidence' && (
            <p className="mt-2 text-sm text-muted">
              Approximate 95% confidence interval using the placebo standard error.
            </p>
          )}
        {run.kind === 'panel-intervention-run' &&
          run.evidence.kind === 'panelDid' &&
          run.evidence.uncertainty?.kind === 'available' && (
            <p className="mt-2 text-sm text-muted">
              Approximate 95% confidence interval using{' '}
              {conventionalDidInferenceLabels[run.evidence.uncertainty.method].toLowerCase()}{' '}
              uncertainty.
            </p>
          )}
      </div>
      {groupChart !== null && estimate.effect.kind === 'byGroup' && (
        <div className="mt-3">
          <ExpandableChart
            option={groupChart}
            label={`Effect of ${study.treatment.name} on ${study.outcome.name} by group of ${estimate.effect.modifier}`}
            className="h-[220px]"
            testId="group-effects"
          />
        </div>
      )}
      {(run.kind === 'propensity-weighting-run' ||
        run.kind === 'propensity-matching-run' ||
        run.kind === 'doubly-robust-run') && (
        <CovariateBalanceResult
          evidence={run.evidence.balance}
          augmented={run.kind === 'doubly-robust-run'}
        />
      )}
      {overlap !== null && (
        <div className="mt-3">
          <ExpandableChart
            option={overlap}
            window={scoreWindow}
            onWindow={setScoreWindow}
            label={`Fitted propensity score for ${study.treatment.name} by arm`}
            className={overlapPanels === 1 ? 'h-[240px]' : 'h-[480px]'}
            testId="propensity-overlap"
          />
        </div>
      )}
      {weightScatter !== null && (
        <div className="mt-3">
          <ExpandableChart
            option={weightScatter}
            window={scoreWindow}
            onWindow={setScoreWindow}
            label={`${study.outcome.name} against the fitted propensity score, each row at its weight`}
            className="h-[300px]"
            testId="propensity-weights"
          />
        </div>
      )}
      {rowChart !== null && (
        <div className="mt-3">
          <ExpandableChart
            option={rowChart}
            label={`Distribution of the per-row effect of ${study.treatment.name} on ${study.outcome.name}`}
            className="h-[220px]"
            testId="row-effects"
          />
        </div>
      )}
      {run.kind === 't-learner-run' && run.evidence.kind === 'tLearner' && (
        <TLearnerIntervals evidence={run.evidence} />
      )}
      {chart !== null && (
        <div className="mt-3">
          {ghosts.length > 0 && (
            <div className="mb-2 flex flex-wrap items-center justify-between gap-2">
              <p className={label('m-0 text-muted')}>Compare with</p>
              <Select
                aria-label="Compare with"
                className={field('text', 'w-64')}
                value={ghostId}
                onChange={(event) => setGhostId(event.target.value)}
              >
                <option value="">No other run</option>
                {ghosts.map((other) => (
                  <option key={other.id} value={String(other.id)}>
                    {describeRun(other.configuration)}, {formatTime(other.createdAt)}
                  </option>
                ))}
              </Select>
            </div>
          )}
          <ExpandableChart
            option={chart}
            label={`Observed ${study.outcome.name.replaceAll('_', ' ')} and its estimated no-intervention path`}
            className="h-[260px]"
            testId="impact-path"
          />
        </div>
      )}
      {posterior !== null && (
        <div className="mt-3">
          <ExpandableChart
            option={posterior}
            label={`Posterior density of the effect of ${study.treatment.name} on ${study.outcome.name}`}
            className="h-[220px]"
            testId="posterior-density"
          />
        </div>
      )}
      {curveChart !== null && (
        <div className="mt-3">
          <div className="flex flex-wrap items-center justify-between gap-2">
            <p className={label('m-0 text-muted')}>Expected outcome across a covariate</p>
            <Select
              aria-label="Curve covariate"
              className={field('text', 'w-40')}
              value={curveIndex}
              onChange={(event) => setChosenCurve(Number(event.target.value))}
            >
              {curves.map((_, index) => (
                <option key={index} value={index}>
                  {adjustmentVariables[index]?.name ?? `covariate ${index + 1}`}
                </option>
              ))}
            </Select>
          </div>
          <div className="mt-2">
            <ExpandableChart
              option={curveChart}
              label={`Expected ${study.outcome.name} across the chosen covariate under both interventions`}
              className="h-[240px]"
              testId="counterfactual-curves"
            />
          </div>
        </div>
      )}
      {run.kind === 'backdoor-linear-run' && (
        <p className="mb-0 mt-3 max-w-[75ch] text-body text-muted">
          {estimate.interval.kind === 'confidence' &&
          (estimate.interval.lower > 0 || estimate.interval.upper < 0)
            ? 'The interval excludes zero.'
            : 'The interval includes zero: the data do not rule out no effect.'}{' '}
          {run.configuration.errors.kind === 'arma' ? (
            <>
              For comparison, least squares gives{' '}
              <span className={num('text-bone')}>
                {formatStatistic('raw', run.evidence.estimate).text}
              </span>{' '}
              with a Newey–West interval of{' '}
              <span className={num('text-bone')}>
                {intervalText({
                  ...estimate,
                  effect: { kind: 'additive', value: run.evidence.estimate, unit: '' },
                  interval: {
                    kind: 'confidence',
                    level: run.evidence.level,
                    lower: run.evidence.hacInterval[0],
                    upper: run.evidence.hacInterval[1],
                  },
                })}
              </span>
              .
            </>
          ) : run.configuration.errors.kind === 'classical' ? (
            run.configuration.fixedEffects.kind !== 'none' ? null : (
              <>
                For comparison, the HAC interval is{' '}
                <span className={num('text-bone')}>
                  {intervalText({
                    ...estimate,
                    interval: {
                      kind: 'confidence',
                      level: run.evidence.level,
                      lower: run.evidence.hacInterval[0],
                      upper: run.evidence.hacInterval[1],
                    },
                  })}
                </span>
                .
              </>
            )
          ) : (
            <>
              For comparison, the classical interval is{' '}
              <span className={num('text-bone')}>
                {intervalText({
                  ...estimate,
                  interval: {
                    kind: 'confidence',
                    level: run.evidence.level,
                    lower: run.evidence.interval[0],
                    upper: run.evidence.interval[1],
                  },
                })}
              </span>
              .
            </>
          )}
        </p>
      )}
      {run.kind === 'backdoor-linear-run' && armaReading(run)?.errors.converged === false && (
        <Alert tone="warn" live={false} className="mt-3">
          <p className="m-0">
            The ARMA fit stopped without converging; review the specification and optimiser settings
            before reading the estimate.
          </p>
        </Alert>
      )}
      {run.kind === 'backdoor-linear-run' && (
        <ArmaUncertaintyAlert evidence={armaReading(run)?.errors} />
      )}
      {run.kind === 'count-glm-run' && !run.evidence.converged && (
        <Alert tone="warn" live={false} className="mt-3">
          <p className="m-0">
            The optimiser did not converge; treat the estimate and its interval as provisional.
          </p>
        </Alert>
      )}
      <ResultInterpretation
        interpretation={interpretEstimationResult(run, study, stepLabel)}
        className="mt-3"
      />
      <Diagnostics run={run} />
      {run.kind === 'backdoor-linear-run' && run.configuration.fixedEffects.kind !== 'none' && (
        <FeWeightDiagnosticResult
          evidence={run.evidence.weightDiagnostic}
          grouping={run.configuration.fixedEffects.name}
        />
      )}
      {run.kind === 'causal-impact-run' && (
        <ImpactEffectPanel evidence={run.evidence} stepLabel={stepLabel} />
      )}
      {run.kind === 'causal-impact-run' && run.evidence.kind !== 'causalImpact' && (
        <BayesianImpactResult
          evidence={run.evidence}
          columnNames={run.columns.map((column) => column.name)}
        />
      )}
      {run.kind === 'sharp-rd-run' && (
        <SharpRdResult
          evidence={run.evidence}
          running={run.columns[0].name}
          outcome={study.outcome.name}
        />
      )}
      {run.kind === 'synthetic-control-run' ? <SyntheticControlEvidenceDetails run={run} /> : null}
      {run.kind === 'ridge-augmented-synthetic-run' ? (
        <RidgeAugmentedResult
          evidence={run.evidence}
          sourcePeriods={run.sourcePeriods}
          outcome={study.outcome.name}
        />
      ) : null}
      {run.kind === 'predictor-synthetic-control-run' ? (
        <PredictorSyntheticResult run={run} outcome={study.outcome.name} />
      ) : null}
      {run.kind === 'panel-intervention-run' ? <PanelEvidenceDetails run={run} /> : null}
    </>
  )
  if (!current) {
    // History rows fold to one line in the runs drawer; only the current estimate keeps the stage.
    return (
      <RunFold
        title={sentence}
        figure={
          run.kind === 'causal-forest-run' && estimate.effect.kind === 'perRow'
            ? `${estimate.effect.effects.length} conditional predictions`
            : headlineFigure(estimate).text
        }
        stamp={stamp}
        onDelete={onDelete === undefined ? undefined : () => onDelete(run)}
        deleteLabel="Delete this run"
      >
        <RunDetails label="Estimation run details">
          <RunRecord run={run} />
        </RunDetails>
        {body}
      </RunFold>
    )
  }
  return (
    <article className="rounded-xl bg-panel lift p-4" aria-label={`${sentence} estimate`}>
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <span className={label('text-signal-text')}>Current estimate</span>
        <span className={num('text-micro text-faint')}>{stamp}</span>
      </div>
      {body}
    </article>
  )
})
