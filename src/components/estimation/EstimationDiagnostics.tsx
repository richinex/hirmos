import { MetricGrid, MetricTile } from '@/components/ui/figures'
import { Metadata } from '@/components/ui/Metadata'
import { assertNever } from '@/domain/dop'
import {
  additive,
  adjustmentLabels,
  boundsReading,
  describeDiscreteStatePreparations,
  describeInstrumentalVariableRoute,
  effectiveSampleSize,
  summariseRowEffects,
  tLearnerRowEffects,
  weightSpread,
  type CausalEffectsAdjustment,
  type EstimationRunArtifact,
  type TreatmentModelEvidence,
} from '@/domain/estimation'
import {
  recordedStaggeredAdjustment,
  staggeredAdjustmentDescriptions,
  staggeredHeadlineDescriptions,
  staggeredRunHeadline,
} from '@/domain/staggeredDid'
import { namesFigure, namesInProse } from '@/lib/format/names'
import {
  formatCount,
  formatInterval,
  formatP,
  formatPercent,
  formatStatistic,
  formatWords,
  type Formatted,
} from '@/lib/format/number'

/** An adjustment set as a figure: named when short, counted when long. */
const adjustmentFigure = (names: readonly string[], none: string): Formatted =>
  formatWords(names.length === 0 ? none : namesFigure(names, 'variables').value)

const adjustmentStrategyLabel = (adjustment: CausalEffectsAdjustment): string => {
  switch (adjustment.kind) {
    case 'optimal':
      return 'Complete O-set'
    case 'minimizedOptimal':
      return 'Minimized O-set'
    case 'collidersMinimizedOptimal':
      return 'Collider-minimized O-set'
    case 'explicit':
      return 'User-supplied set'
    default:
      return assertNever(adjustment)
  }
}

const treatmentModelTiles = (
  model: TreatmentModelEvidence,
): readonly {
  readonly label: string
  readonly value: Formatted
  readonly context?: React.ReactNode
}[] =>
  model.kind === 'logistic'
    ? [
        {
          label: 'Treatment model',
          value: formatWords(model.converged ? 'Converged' : 'Stopped early'),
          context: `${formatCount(model.parameters).text} parameters`,
        },
      ]
    : [
        {
          label: 'Treatment model',
          value: formatWords(
            model.scoring.kind === 'crossFitted' ? 'Boosted, cross-fitted' : 'Boosted trees',
          ),
          context: (
            <Metadata>
              <span>learning rate {model.learningRate}</span>
              <span>depth {model.maxDepth}</span>
              <span>{formatCount(model.nEstimators).text} trees</span>
              <span>
                {formatCount(model.candidates).text} candidate{model.candidates === 1 ? '' : 's'}
              </span>
            </Metadata>
          ),
        },
        ...boostedScoringTiles(model.scoring),
      ]

const boostedScoringTiles = (
  scoring: Extract<TreatmentModelEvidence, { kind: 'boosted' }>['scoring'],
): readonly {
  readonly label: string
  readonly value: Formatted
  readonly context?: React.ReactNode
}[] => {
  switch (scoring.kind) {
    case 'oneModel':
      return [
        {
          label: 'Cross-validated AUC',
          value: formatStatistic('score', scoring.validationAuc),
          context: 'the chosen candidate, averaged over the search folds',
        },
        {
          label: 'AUC on the fitted rows',
          value: formatStatistic('score', scoring.fittedAuc),
          context: 'the model scoring the rows it was fitted on',
        },
      ]
    case 'crossFitted':
      return [
        {
          label: 'Cross-fitted AUC',
          value: formatStatistic('score', scoring.auc),
          context:
            'each half predicted from the other half; hyperparameters selected on the full sample first',
        },
      ]
    default:
      return assertNever(scoring)
  }
}

export function Diagnostics({ run }: { readonly run: EstimationRunArtifact }) {
  const tiles = ((): readonly {
    readonly label: string
    readonly value: Formatted
    readonly context?: React.ReactNode
  }[] => {
    switch (run.kind) {
      case 'frontdoor-two-stage-run': {
        const { evidence } = run
        const mediator = run.columns[evidence.mediator]?.name ?? 'Mediator'
        return [
          {
            label: 'Mediator',
            value: formatWords(mediator),
            context: `${formatStatistic('raw', evidence.controlValue).text} → ${formatStatistic('raw', evidence.treatmentValue).text} treatment contrast`,
          },
          {
            label: 'Treatment → mediator',
            value: formatStatistic('raw', evidence.firstStageEffect),
            context: `${formatCount(evidence.firstStageParams.length).text} first-stage parameters`,
          },
          {
            label: 'Mediator → outcome',
            value: formatStatistic('raw', evidence.secondStageEffect),
            context: `${formatCount(evidence.secondStageParams.length).text} second-stage parameters`,
          },
        ]
      }
      case 'instrumental-variable-run': {
        const { evidence } = run
        const instruments = evidence.instruments
          .map((index) => run.columns[index]?.name ?? String(index))
          .join(', ')
        return [
          {
            label: 'Estimator route',
            value: formatWords(describeInstrumentalVariableRoute(evidence.route)),
            context: (
              <Metadata>
                <span>{formatCount(evidence.instruments.length, { noun: 'instrument' }).text}</span>
                <span>{formatCount(evidence.params.length, { noun: 'coefficient' }).text}</span>
              </Metadata>
            ),
          },
          {
            label: 'Instruments',
            value: formatWords(instruments),
            context: `${formatCount(evidence.observations).text} rows`,
          },
          {
            label: 'Bootstrap SE',
            value:
              evidence.standardError === null
                ? formatWords('none')
                : formatStatistic('raw', evidence.standardError),
            context:
              evidence.uncertainty.kind === 'bootstrap' ? (
                <Metadata>
                  <span>{formatCount(evidence.uncertainty.simulations).text} resamples</span>
                  <span>seed {evidence.uncertainty.seed}</span>
                </Metadata>
              ) : (
                'no bootstrap requested'
              ),
          },
        ]
      }
      case 'backdoor-linear-run': {
        const { evidence } = run
        const absorbedNames =
          evidence.fixedEffects.kind === 'none'
            ? []
            : evidence.fixedEffects.absorbed.map(
                (column) => run.columns[column]?.name ?? String(column),
              )
        const counted = (n: number, singular: string, plural: string) =>
          formatCount(n, { noun: n === 1 ? singular : plural })
        return [
          evidence.fixedEffects.kind !== 'none'
            ? {
                label: 'R squared, within',
                value: formatStatistic('score', evidence.rSquared),
                context: (
                  <Metadata>
                    <span>{counted(evidence.parameters, 'regressor', 'regressors').text}</span>
                    <span>
                      {evidence.fixedEffects.kind === 'time'
                        ? counted(evidence.fixedEffects.periods, 'period', 'periods').text
                        : counted(evidence.fixedEffects.units, 'unit', 'units').text}
                    </span>
                    {evidence.fixedEffects.kind === 'unitAndTime' && (
                      <span>
                        {counted(evidence.fixedEffects.periods, 'period', 'periods').text}
                      </span>
                    )}
                  </Metadata>
                ),
              }
            : {
                label: 'R squared',
                value: formatStatistic('score', evidence.rSquared),
                context: `${formatCount(evidence.parameters).text} parameters`,
              },
          {
            label: 'Residual SD',
            value: formatStatistic('sd', evidence.residualSd),
            context: (
              <Metadata>
                <span>{formatCount(evidence.degreesOfFreedom).text} degrees of freedom</span>
                <span>Durbin–Watson {formatStatistic('raw', evidence.durbinWatson).text}</span>
              </Metadata>
            ),
          },
          evidence.fixedEffects.kind !== 'none'
            ? {
                label: 'Absorbed by the fixed effects',
                value:
                  absorbedNames.length === 0
                    ? formatWords('none')
                    : formatCount(absorbedNames.length),
                context:
                  absorbedNames.length === 0
                    ? 'No adjustment columns were removed during absorption.'
                    : absorbedNames.join(', '),
              }
            : {
                label: 'Newey–West bandwidth',
                value: formatCount(evidence.hacMaxLags, { noun: 'lag' }),
                context: `heteroskedasticity and autocorrelation consistent p ${formatP(evidence.hacPValue, { withLabel: false }).text}`,
              },
          ...(evidence.errorModel.kind === 'hc1'
            ? [
                {
                  label: 'Robust SE (HC1)',
                  value: formatStatistic('raw', evidence.errorModel.standardError),
                  context: `p ${formatP(evidence.errorModel.pValue, { withLabel: false }).text}`,
                },
              ]
            : []),
          ...(evidence.errorModel.kind === 'cluster'
            ? [
                {
                  label: 'Clustered SE',
                  value: formatStatistic('raw', evidence.errorModel.standardError),
                  context: (
                    <Metadata>
                      <span>
                        {formatCount(evidence.errorModel.clusters, { noun: 'clusters' }).text}
                      </span>
                      <span>
                        p {formatP(evidence.errorModel.pValue, { withLabel: false }).text}
                      </span>
                    </Metadata>
                  ),
                },
              ]
            : []),
          ...(evidence.errorModel.kind === 'arma'
            ? [
                {
                  label: `ARMA(${evidence.errorModel.errors.p}, ${evidence.errorModel.errors.q}) errors`,
                  value: formatWords(
                    [...evidence.errorModel.errors.ar, ...evidence.errorModel.errors.ma]
                      .map(
                        (term) => `${term.name} ${formatStatistic('raw', term.coefficient).text}`,
                      )
                      .join(', '),
                  ),
                  context: (
                    <Metadata>
                      <span>
                        innovation variance{' '}
                        {formatStatistic('raw', evidence.errorModel.errors.sigma2).text}
                      </span>
                      <span>AIC {formatStatistic('raw', evidence.errorModel.errors.aic).text}</span>
                      <span>
                        {evidence.errorModel.errors.converged
                          ? `converged in ${evidence.errorModel.errors.iterations} iterations`
                          : `stopped at ${evidence.errorModel.errors.iterations} iterations`}
                      </span>
                    </Metadata>
                  ),
                },
              ]
            : []),
        ]
      }
      case 'count-glm-run': {
        const { evidence } = run
        return [
          {
            label: 'Coefficient',
            value: formatStatistic('raw', evidence.coefficient),
            context: (
              <Metadata>
                <span>SE {formatStatistic('raw', evidence.standardError).text}</span>
                <span>p {formatP(evidence.pValue, { withLabel: false }).text}</span>
              </Metadata>
            ),
          },
          evidence.family === 'poisson'
            ? {
                label: 'Deviance',
                value: formatStatistic('raw', evidence.deviance ?? Number.NaN),
                context: `${formatCount(evidence.degreesOfFreedom).text} degrees of freedom`,
              }
            : {
                label: 'Dispersion alpha',
                value: formatStatistic('raw', evidence.alpha ?? Number.NaN),
                context: `log likelihood ${formatStatistic('raw', evidence.logLikelihood ?? Number.NaN).text}`,
              },
          {
            label: 'Convergence',
            value: formatWords(evidence.converged ? 'converged' : 'not converged'),
            context: `${formatCount(evidence.iterations).text} iterations`,
          },
        ]
      }
      case 'negative-binomial-ingarch-run': {
        const { evidence } = run
        return [
          {
            label: 'Average path difference',
            value: formatStatistic('raw', evidence.averageEffect),
            context: (
              <Metadata>
                <span>{formatCount(evidence.horizon).text} forecast periods</span>
                <span>cumulative {formatStatistic('raw', evidence.cumulativeEffect).text}</span>
              </Metadata>
            ),
          },
          {
            label: 'Overdispersion',
            value: formatStatistic('raw', evidence.dispersion),
            context: `negative-binomial size ${formatStatistic('raw', evidence.size).text}`,
          },
          {
            label: 'Recursion',
            value: formatWords(
              `${evidence.link === 'identity' ? 'additive' : 'multiplicative'}, count lag ${evidence.pastObservationLags.join(', ')}, mean lag ${evidence.pastMeanLags.join(', ')}`,
            ),
            context: (
              <Metadata>
                <span>{formatCount(evidence.iterations).text} optimizer iterations</span>
                <span>
                  {evidence.functionEvaluations}/{evidence.gradientEvaluations} function/gradient
                  evaluations
                </span>
              </Metadata>
            ),
          },
        ]
      }
      case 'ardl-run': {
        const { evidence } = run
        const reading = boundsReading(evidence)
        const [lower, upper] = evidence.boundsCritical[1] ?? [Number.NaN, Number.NaN]
        return [
          {
            label: 'Bounds test',
            value: formatWords(
              reading === 'level-relation'
                ? 'level relation'
                : reading === 'no-level-relation'
                  ? 'no level relation'
                  : 'inconclusive',
            ),
            context: `F ${formatStatistic('raw', evidence.boundsStatistic).text} against 5% bounds ${formatStatistic('raw', lower).text} to ${formatStatistic('raw', upper).text}`,
          },
          {
            label: 'Bounds p',
            value: formatP(evidence.boundsPUpper, { withLabel: false }),
            context: (
              <Metadata>
                <span>I(1) bound</span>
                <span>
                  I(0) bound p {formatP(evidence.boundsPLower, { withLabel: false }).text}
                </span>
              </Metadata>
            ),
          },
          {
            label: 'Lag orders',
            value: formatWords(`ARDL(${evidence.arLag}, ${evidence.dlLag})`),
            context: (
              <Metadata>
                <span>AIC over {formatCount(evidence.grid.length).text} candidates</span>
                <span>{evidence.trend === 'ct' ? 'constant and trend' : 'constant'}</span>
                <span>case {evidence.case}</span>
              </Metadata>
            ),
          },
        ]
      }
      case 'vecm-run': {
        const { evidence } = run
        return [
          {
            label: 'Cointegration rank',
            value: formatCount(evidence.rank),
            context: (
              <Metadata>
                <span>Johansen trace at {['90', '95', '99'][evidence.significance] ?? ''}%</span>
                <span>{evidence.kArDiff} lagged differences</span>
              </Metadata>
            ),
          },
          {
            label: 'Adjustment p',
            value: formatWords(
              evidence.pvaluesAlpha
                .map((row) => formatP(row[0] ?? Number.NaN, { withLabel: false }).text)
                .join('; '),
            ),
            context: (
              <Metadata>
                <span>alpha per equation</span>
                <span>terms “{evidence.deterministic}”</span>
              </Metadata>
            ),
          },
          evidence.chow === null
            ? {
                label: 'Chow break',
                value: formatWords('not requested'),
                context: 'set a split after a row to test stability',
              }
            : {
                label: 'Chow break',
                value: formatP(evidence.chow[1], { withLabel: false }),
                context: (
                  <Metadata>
                    <span>F {formatStatistic('raw', evidence.chow[0]).text}</span>
                    <span>split after row {run.configuration.breakIndex}</span>
                  </Metadata>
                ),
              },
        ]
      }
      case 'ridge-augmented-synthetic-run':
        return [
          { label: 'Selected lambda', value: formatStatistic('raw', run.evidence.lambda) },
          { label: 'Donor units', value: formatCount(run.evidence.request.donors.length) },
          { label: 'Pre-treatment periods', value: formatCount(run.evidence.request.prePeriods) },
        ]
      case 'predictor-synthetic-control-run':
        return [
          {
            label: 'Pre-intervention outcome MSPE',
            value: formatStatistic('raw', run.evidence.outcomeMspe),
          },
          { label: 'Donor units', value: formatCount(run.evidence.donors.length) },
          { label: 'Predictor summaries', value: formatCount(run.evidence.balance.length) },
        ]
      case 'synthetic-control-run': {
        const { evidence } = run
        const donorNames = run.columns.slice(2).map((variable) => variable.name)
        return [
          {
            label: 'Donor weights',
            value: formatWords(
              evidence.weights
                .map((weight, index) => ({ name: donorNames[index] ?? String(index), weight }))
                .filter((donor) => donor.weight >= 0.0005)
                .sort((first, second) => second.weight - first.weight)
                .map((donor) => `${donor.name} ${formatStatistic('score', donor.weight).text}`)
                .join('; '),
            ),
            context: (
              <Metadata>
                <span>
                  {formatCount(evidence.weights.filter((weight) => weight >= 0.0005).length).text}{' '}
                  of {evidence.weights.length} donors carry weight
                </span>
                <span>sum to one</span>
              </Metadata>
            ),
          },
          {
            label: 'Pre-period loss',
            value: formatStatistic('raw', evidence.loss),
            context: (
              <Metadata>
                <span>{formatCount(evidence.nPre).text} pre rows</span>
                <span>{formatCount(evidence.iterations).text} active-set steps</span>
              </Metadata>
            ),
          },
          {
            label: 'Average post gap',
            value: formatStatistic('raw', evidence.att),
            context: `over ${formatCount(evidence.nPost).text} post rows`,
          },
          evidence.crossFit.kind === 'available'
            ? {
                label: 'Bias-corrected estimate',
                value: formatStatistic('raw', evidence.crossFit.att),
                context: (
                  <Metadata>
                    <span>pre-period blocks left out in turn</span>
                    <span>SE {formatStatistic('raw', evidence.crossFit.standardError).text}</span>
                    <span>
                      {
                        formatInterval(
                          evidence.crossFit.att,
                          evidence.crossFit.confidenceInterval[0],
                          evidence.crossFit.confidenceInterval[1],
                          { kind: 'confidence', level: 0.95 },
                          additive,
                        ).text
                      }
                    </span>
                    <span>p {formatP(evidence.crossFit.pValue, { withLabel: false }).text}</span>
                  </Metadata>
                ),
              }
            : {
                label: 'Bias-corrected estimate',
                value: formatWords('unavailable'),
                context: evidence.crossFit.reason,
              },
          evidence.donorPlacebo.kind === 'available'
            ? {
                label: 'Donor placebo rank',
                value: formatP(evidence.donorPlacebo.pValue, { withLabel: false }),
                context: `compared with ${formatCount(evidence.donorPlacebo.nValidPlacebos).text} donors treated in turn; smaller means fewer donors looked as unusual`,
              }
            : {
                label: 'Donor placebo rank',
                value: formatWords('unavailable'),
                context: evidence.donorPlacebo.reason,
              },
          evidence.conformalBand.kind === 'available'
            ? {
                label: 'Conformal band',
                value: formatStatistic('raw', evidence.conformalBand.halfWidth),
                context: `${Math.round((1 - evidence.conformalBand.alpha) * 100)}% fixed-weight prediction half-width`,
              }
            : {
                label: 'Conformal band',
                value: formatWords('unavailable'),
                context: evidence.conformalBand.reason,
              },
        ]
      }
      case 'panel-intervention-run': {
        const { evidence } = run
        if (evidence.kind === 'sunAbraham')
          return [
            { label: 'Method', value: formatWords('Sun–Abraham') },
            { label: 'Retained clusters', value: formatCount(evidence.clusters) },
            {
              label: 'Overall ATT',
              value: formatWords('Post-treatment aggregation'),
              context: 'Weighted by supported treated observations',
            },
          ]
        if (evidence.kind === 'staggeredDid')
          return [
            {
              label: 'Adjustment method',
              value: formatWords(
                staggeredAdjustmentDescriptions[
                  recordedStaggeredAdjustment(evidence.specification).kind
                ].label,
              ),
              context: 'Group-time ATT estimation',
            },
            {
              label: 'Cohorts',
              value: formatCount(evidence.cohorts.keys.length),
              context: 'Distinct first-treatment periods',
            },
            {
              label: 'Retained units',
              value: formatCount(evidence.units.length),
              context: `${formatCount(evidence.times.length).text} periods`,
            },
            {
              label: 'Overall ATT',
              value: formatWords(
                `${staggeredHeadlineDescriptions[staggeredRunHeadline(run)].label} average`,
              ),
              context: staggeredHeadlineDescriptions[staggeredRunHeadline(run)].description,
            },
          ]
        if (evidence.kind === 'panelAdjusted')
          return [
            {
              label: 'Method',
              value: formatWords(
                evidence.specification.kind === 'regression'
                  ? 'Regression DiD'
                  : 'Doubly robust DiD',
              ),
              context: 'Average effect on the treated group',
            },
            panelLayoutTile(evidence, 'One before and one after period'),
          ]
        if (evidence.kind === 'panelDid')
          return [
            {
              label: 'Method',
              value: formatWords('Conventional difference-in-differences'),
              context: 'Change in treated outcomes minus change in comparison outcomes',
            },
            panelLayoutTile(
              evidence,
              `${formatCount(evidence.nPre).text} pre- and ${formatCount(evidence.nPost).text} post-periods`,
            ),
          ]
        const topWeights = evidence.syntheticDid.omega
          .map((weight, index) => ({
            weight,
            unit: evidence.units[index] ?? `control ${index + 1}`,
          }))
          .sort((left, right) => right.weight - left.weight)
          .slice(0, 3)
        const [heaviest, ...nextWeights] = topWeights
        return [
          {
            label: 'Synthetic DID',
            value: formatStatistic('raw', evidence.syntheticDid.estimate),
            context: (
              <Metadata>
                <span>chosen as the main result before fitting</span>
                <span>DID {formatStatistic('raw', evidence.did.estimate).text}</span>
                <span>
                  synthetic control{' '}
                  {formatStatistic('raw', evidence.syntheticControl.estimate).text}
                </span>
                <span>{formatCount(evidence.nPost).text} post periods</span>
              </Metadata>
            ),
          },
          panelLayoutTile(
            evidence,
            `${formatCount(evidence.units.length).text} units × ${formatCount(evidence.times.length).text} periods`,
          ),
          heaviest === undefined
            ? {
                label: 'Largest comparison weight',
                value: formatWords('none'),
                context: 'no synthetic-DID unit weights',
              }
            : {
                label: 'Largest comparison weight',
                value: formatStatistic('score', heaviest.weight),
                context: (
                  <Metadata>
                    <span>unit {heaviest.unit}</span>
                    {nextWeights.map(({ unit, weight }) => (
                      <span key={unit}>
                        {unit} {formatStatistic('score', weight).text}
                      </span>
                    ))}
                  </Metadata>
                ),
              },
        ]
      }
      case 'causal-forest-run':
        return []
      case 't-learner-run': {
        const { evidence } = run
        const rows = tLearnerRowEffects(evidence)
        const summary = rows === null ? null : summariseRowEffects(rows)
        const rowEffects = {
          label: 'Row effects',
          value: formatWords(
            summary === null
              ? 'none'
              : `${formatStatistic('raw', summary.minimum).text} to ${formatStatistic('raw', summary.maximum).text}`,
          ),
          context:
            summary === null ? (
              ''
            ) : (
              <Metadata>
                <span>median {formatStatistic('raw', summary.median).text}</span>
                <span>
                  {formatPercent(summary.positiveShare, { precision: 0 }).text} above zero
                </span>
              </Metadata>
            ),
        }
        switch (evidence.kind) {
          case 'tLearner':
            return [
              {
                label: 'Arms',
                value: formatWords(
                  `${formatCount(evidence.controlRows).text} control, ${formatCount(evidence.treatedRows).text} treated`,
                ),
                context: 'one outcome forest each',
              },
              rowEffects,
              {
                label: 'Forests',
                value: formatWords(`${formatCount(evidence.trees).text} trees`),
                context: (
                  <Metadata>
                    <span>minimum leaf {evidence.minLeaf}</span>
                    <span>learner seed {evidence.seed}</span>
                  </Metadata>
                ),
              },
            ]
          case 'crossFittedTLearner':
            return [
              {
                label: 'Arms',
                value: formatWords(
                  `${formatCount(evidence.controlRows).text} control, ${formatCount(evidence.treatedRows).text} treated`,
                ),
                context: 'two halves of equal size, stratified on the outcome',
              },
              rowEffects,
              ...(
                [
                  ['Treated model', evidence.selected.treated],
                  ['Control model', evidence.selected.control],
                ] as const
              ).map(([label, choice]) => ({
                label,
                value: formatWords(
                  `learning rate ${choice.learningRate}, depth ${choice.maxDepth}, ${formatCount(choice.nEstimators).text} trees`,
                ),
                context: (
                  <Metadata>
                    <span>best AUC {formatStatistic('score', choice.validationAuc).text}</span>
                    <span>
                      {formatCount(evidence.candidates).text} candidates, {evidence.splits}-fold
                    </span>
                  </Metadata>
                ),
              })),
            ]
          default:
            return assertNever(evidence)
        }
      }
      case 'negbin-nuts-run': {
        const { evidence } = run
        return [
          {
            label: 'Divergences',
            value: formatCount(evidence.divergences),
            context: (
              <Metadata>
                <span>acceptance {formatStatistic('score', evidence.acceptanceRate).text}</span>
                <span>step {formatStatistic('raw', evidence.stepSize).text}</span>
              </Metadata>
            ),
          },
          {
            label: 'Rate ratio per SD',
            value: formatStatistic('raw', Math.exp(evidence.betaTreatmentMean)),
            context: (
              <Metadata>
                <span>per standard deviation of {run.columns[0]?.name ?? 'the treatment'}</span>
                <span>
                  beta {formatStatistic('raw', evidence.betaTreatmentMean).text}, posterior sd{' '}
                  {formatStatistic('raw', evidence.betaTreatmentSd).text}
                </span>
              </Metadata>
            ),
          },
          {
            label: 'Dispersion r',
            value: formatStatistic('raw', evidence.dispersionMean),
            context: (
              <Metadata>
                <span>{formatCount(evidence.warmup).text} warmup</span>
                <span>{formatCount(evidence.samples).text} draws</span>
                <span>seed {evidence.seed}</span>
              </Metadata>
            ),
          },
        ]
      }
      case 'bayesian-gaussian-run': {
        const { evidence } = run
        return [
          {
            label: 'Divergences',
            value: formatCount(evidence.divergences),
            context: (
              <Metadata>
                <span>{formatCount(evidence.chains).text} chains</span>
                <span>acceptance {formatStatistic('score', evidence.acceptanceRate).text}</span>
                <span>step {formatStatistic('raw', evidence.stepSize).text}</span>
              </Metadata>
            ),
          },
          {
            label: 'P(effect > 0)',
            value: formatStatistic('score', evidence.probabilityPositive),
            context: (
              <Metadata>
                <span>posterior mean {formatStatistic('raw', evidence.effectMean).text}</span>
                <span>sd {formatStatistic('raw', evidence.effectSd).text}</span>
                <span>median {formatStatistic('raw', evidence.effectMedian).text}</span>
              </Metadata>
            ),
          },
          {
            label: 'Residual sd',
            value: formatStatistic('raw', evidence.sigmaMean),
            context: (
              <Metadata>
                <span>{formatCount(evidence.warmup).text} warmup</span>
                <span>{formatCount(evidence.samples).text} draws per chain</span>
                <span>seed {evidence.seed}</span>
              </Metadata>
            ),
          },
        ]
      }
      case 'discrete-bn-run': {
        const { evidence } = run
        return [
          {
            label: 'Treatment states',
            value: formatWords(`${evidence.treatmentStates[0]} → ${evidence.treatmentStates[1]}`),
            context: (
              <Metadata>
                <span>state budget {evidence.bins}</span>
                <span>counts {evidence.stateCounts.join('/')}</span>
              </Metadata>
            ),
          },
          {
            label: 'State preparation',
            value: formatWords(
              `${evidence.statePreparations.filter((entry) => entry.strategy.kind === 'observedStates').length} observed, ${evidence.statePreparations.filter((entry) => entry.strategy.kind === 'quantiles').length} quantile`,
            ),
            context: describeDiscreteStatePreparations(evidence.statePreparations),
          },
          {
            label: 'Expected outcome',
            value: formatWords(
              `${formatStatistic('raw', evidence.expectations[0]).text} → ${formatStatistic('raw', evidence.expectations[1]).text}`,
            ),
            context: 'under do(low) and do(high)',
          },
          ...(evidence.parentsAdjusted.join(', ') ===
          adjustmentLabels(run.estimate.adjustment).join(', ')
            ? []
            : [
                {
                  label: 'Adjustment set',
                  value: adjustmentFigure(evidence.parentsAdjusted, 'none'),
                  context:
                    evidence.minimalAdjustmentSet === null
                      ? 'no minimal adjustment set'
                      : `minimal set ${evidence.minimalAdjustmentSet.length === 0 ? 'empty' : namesInProse(evidence.minimalAdjustmentSet, (count) => `of ${count} variables`)}`,
                },
              ]),
        ]
      }
      case 'binary-ett-run': {
        const { evidence } = run
        return [
          {
            label: 'E[Y(1) | X=1]',
            value: formatStatistic('raw', evidence.treatedPotentialOutcomeMean),
            context: 'treated potential-outcome mean among treated rows',
          },
          {
            label: 'E[Y(0) | X=1]',
            value: formatStatistic('raw', evidence.untreatedPotentialOutcomeMean),
            context: 'untreated potential-outcome mean among treated rows',
          },
          {
            label: 'ETT',
            value: formatStatistic('raw', evidence.effectOnTreated),
            context: (
              <Metadata>
                <span>{formatCount(evidence.observations).text} rows</span>
                <span>plug-in estimate</span>
              </Metadata>
            ),
          },
        ]
      }
      case 'double-ml-run': {
        const { evidence } = run
        return [
          {
            label: 'Model',
            value: formatWords(
              evidence.model === 'plr'
                ? 'partially linear'
                : evidence.att
                  ? 'interactive (effect on the treated)'
                  : 'interactive (average effect)',
            ),
            context: evidence.treatBinary ? 'binary treatment' : 'continuous treatment',
          },
          {
            label: 'Standard error',
            value: formatStatistic('raw', evidence.standardError),
            context: 'sandwich, cross-fitted',
          },
          {
            label: 'Fold seed',
            value: formatCount(evidence.seed),
            context: (
              <Metadata>
                <span>5 folds</span>
                <span>200 trees</span>
                <span>learner seed 7</span>
              </Metadata>
            ),
          },
        ]
      }
      case 'causal-effects-run': {
        const { evidence } = run
        const nodeName = (node: readonly [number, number]) =>
          `${run.columns[node[0]]?.name ?? node[0]}${node[1] === 0 ? '' : ` (t−${Math.abs(node[1])})`}`
        const fitTiles = (() => {
          switch (evidence.fit.kind) {
            case 'unfitted':
              return []
            case 'invalidAdjustment':
              return []
            case 'adjustedLinear':
              return [
                {
                  label: 'Adjustment set',
                  value: adjustmentFigure(evidence.fit.adjustmentSet.map(nodeName), 'None'),
                  context: (
                    <Metadata>
                      <span>{adjustmentStrategyLabel(evidence.fit.selection)}</span>
                      <span>linear</span>
                      <span>τ max {evidence.tauMax}</span>
                    </Metadata>
                  ),
                },
              ]
            case 'adjustedKnn':
              return [
                {
                  label: 'Adjustment set',
                  value: adjustmentFigure(evidence.fit.adjustmentSet.map(nodeName), 'None'),
                  context: (
                    <Metadata>
                      <span>{adjustmentStrategyLabel(evidence.fit.selection)}</span>
                      <span>{evidence.fit.k}-neighbour</span>
                      <span>τ max {evidence.tauMax}</span>
                    </Metadata>
                  ),
                },
              ]
            case 'wrightParents':
              return [
                {
                  label: 'Direct effect',
                  value: formatStatistic('raw', evidence.fit.directEffect),
                  context: 'sum of direct path contrasts',
                },
                {
                  label: 'Indirect effect',
                  value: formatStatistic('raw', evidence.fit.indirectEffect),
                  context: (
                    <Metadata>
                      <span>{evidence.fit.paths.length} directed paths</span>
                      <span>{evidence.fit.coefficients.length} parent coefficients</span>
                    </Metadata>
                  ),
                },
              ]
            default:
              return assertNever(evidence.fit)
          }
        })()
        return [
          ...fitTiles,
          {
            label: 'Predictions',
            value: formatWords(
              evidence.predictions.map((value) => formatStatistic('raw', value).text).join(' → '),
            ),
            context: `at ${evidence.interventions[0]} and ${evidence.interventions[1]}`,
          },
          {
            label: 'Fitted rows',
            value: formatCount(evidence.fittedObservations),
            context:
              evidence.mediators.length === 0
                ? 'no mediators'
                : `${evidence.mediators.length} mediator nodes`,
          },
          ...(evidence.uncertainty.kind === 'bootstrap'
            ? [
                {
                  label: 'Bootstrap',
                  value: formatCount(evidence.uncertainty.samples),
                  context: (
                    <Metadata>
                      <span>
                        {Math.round(evidence.uncertainty.confidenceLevel * 100)}% percentile
                        interval
                      </span>
                      <span>block {evidence.uncertainty.resolvedBlockLength}</span>
                      <span>seed {evidence.uncertainty.seed}</span>
                    </Metadata>
                  ),
                },
              ]
            : []),
        ]
      }
      case 'sharp-rd-run':
        return [
          {
            label: 'Estimation bandwidth',
            value: formatStatistic('raw', run.evidence.bandwidth),
            context: 'mserd, triangular kernel',
          },
          {
            label: 'Bias bandwidth',
            value: formatStatistic('raw', run.evidence.biasBandwidth),
            context: 'local quadratic bias correction',
          },
          {
            label: 'Local sample',
            value: formatCount(
              run.evidence.effectiveObservations[0] + run.evidence.effectiveObservations[1],
            ),
            context: `${run.evidence.effectiveObservations[0]} below; ${run.evidence.effectiveObservations[1]} at or above cutoff`,
          },
        ]
      case 'causal-impact-run': {
        const { evidence } = run
        return [
          {
            label: 'Cumulative effect',
            value: formatStatistic('raw', evidence.cumulative),
            context: `over ${formatCount(evidence.nPost).text} post rows`,
          },
          {
            label: 'Average effect',
            value: formatStatistic('raw', evidence.average),
            context: `pre window ${formatCount(evidence.nPre).text} rows`,
          },
          ...(evidence.kind !== 'causalImpact'
            ? [
                {
                  label: 'Posterior draws',
                  value: formatCount(evidence.draws),
                  context: `${evidence.warmup} warmup iterations`,
                },
              ]
            : [
                {
                  label: 'Log likelihood',
                  value: formatStatistic('raw', evidence.logLikelihood),
                  context: `${formatCount(evidence.controls.length).text} controls`,
                },
              ]),
        ]
      }
      case 'propensity-weighting-run': {
        const { evidence } = run
        const sampleSize = effectiveSampleSize(evidence.weights, evidence.treated)
        return [
          {
            label: 'Treated mean',
            value: formatStatistic('raw', evidence.treatedMean),
            context: `${formatCount(evidence.treatedRows).text} treated rows`,
          },
          {
            label: 'Control mean',
            value: formatStatistic('raw', evidence.controlMean),
            context: `${formatCount(evidence.controlRows).text} control rows`,
          },
          {
            label: 'Weight sums',
            value: formatWords(
              `${evidence.treatedWeightSum.toFixed(0)} treated, ${evidence.controlWeightSum.toFixed(0)} control`,
            ),
            context: 'the total weight each arm carries',
          },
          {
            label: 'Effective sample size',
            value: formatWords(
              `${sampleSize.treated.toFixed(0)} treated, ${sampleSize.control.toFixed(0)} control`,
            ),
            context: 'lower values relative to each arm’s row count indicate more uneven weights',
          },
          ...treatmentModelTiles(evidence.treatmentModel),
        ]
      }
      case 'propensity-matching-run': {
        const { evidence } = run
        return [
          {
            label: 'Paired rows',
            value: formatCount(
              evidence.target === 'att' ? evidence.treatedRows : evidence.matches.length,
            ),
            context:
              evidence.target === 'att'
                ? 'one nearest control neighbour for each treated row'
                : 'one nearest neighbour from the other arm for every row',
          },
          {
            label: 'Arms',
            value: formatWords(`${evidence.treatedRows} treated, ${evidence.controlRows} control`),
            context:
              evidence.target === 'att'
                ? 'matched differences are averaged over treated rows'
                : 'pairs are averaged over every row',
          },
          ...treatmentModelTiles(evidence.treatmentModel),
        ]
      }
      case 'doubly-robust-run': {
        const { evidence } = run
        return [
          {
            label: 'Treated term',
            value: formatStatistic('raw', evidence.treatedTerm),
            context:
              evidence.target === 'att'
                ? 'observed outcome mean among treated rows'
                : 'weighted residual plus the fitted outcome under treatment',
          },
          {
            label: 'Control term',
            value: formatStatistic('raw', evidence.controlTerm),
            context:
              evidence.target === 'att'
                ? 'control-outcome predictions for treated rows plus the treatment-odds-weighted control residual correction'
                : 'the same under no treatment',
          },
          {
            label: 'Treatment model',
            value: formatWords(evidence.converged ? 'Converged' : 'Stopped early'),
            context: `${formatCount(evidence.parameters).text} parameters`,
          },
        ]
      }
      case 'continuous-gps-run': {
        const { evidence } = run
        const rows = formatCount(evidence.observations).text
        const spreadTiles =
          evidence.weights === null
            ? [
                {
                  label: 'Effective sample size',
                  value: formatWords('Not recorded'),
                  context:
                    'this run was saved before weights were recorded; run it again to see them',
                },
              ]
            : (() => {
                const spread = weightSpread(evidence.weights, 10)
                return [
                  {
                    label: 'Effective sample size',
                    value: formatCount(Math.round(spread.effectiveSampleSize)),
                    context: `of ${rows} rows; the closer to the row count, the less a few weights dominate`,
                  },
                  {
                    label: 'Largest weight',
                    value: formatStatistic('raw', spread.largestWeight),
                    context: `the ${spread.largestShare.count} largest weights carry ${formatPercent(spread.largestShare.share).text} of the total`,
                  },
                ]
              })()
        return [
          {
            label: 'Standard error',
            value: formatStatistic('raw', evidence.standardError),
            context: 'from the weighted regression, with the weights taken as fixed',
          },
          ...spreadTiles,
          {
            label: 'Residual scale',
            value: formatStatistic('raw', evidence.residualScale),
            context: 'the spread of the treatment around its fitted value',
          },
        ]
      }
      default:
        return assertNever(run)
    }
  })()
  // The design-specific tiles say what stands in for an adjustment set; otherwise the set itself, counted when
  // long with its first names beneath. The identification record lists every one.
  const adjustmentTile = ((): { readonly value: Formatted; readonly preview: string | null } => {
    if (run.kind === 'panel-intervention-run') {
      switch (run.evidence.kind) {
        case 'sunAbraham':
          return { value: formatWords('Cohort-by-event-time interactions'), preview: null }
        case 'staggeredDid':
          return { value: formatWords('Adoption-cohort comparisons'), preview: null }
        case 'panelAdjusted':
          return {
            value: formatWords(
              run.evidence.specification.kind === 'doublyRobust'
                ? 'Baseline-covariate adjustment'
                : 'Covariate-adjusted regression',
            ),
            preview:
              run.columns
                .slice(2)
                .map((column) => column.name)
                .join(', ') || 'No covariates',
          }
        case 'panelDid':
          return { value: formatWords('Treated and control changes'), preview: null }
        case 'panelIntervention':
          return { value: formatWords('Unit and time weights'), preview: null }
        default:
          return assertNever(run.evidence)
      }
    }
    if (run.kind === 'frontdoor-two-stage-run') {
      const stage = (columns: readonly number[]) =>
        columns.length === 0
          ? 'none'
          : columns.map((index) => run.columns[index]?.name ?? index).join(', ')
      return {
        value: formatWords(
          `stage 1: ${stage(run.evidence.firstStageAdjustment)}, stage 2: ${stage(run.evidence.secondStageAdjustment)}`,
        ),
        preview: null,
      }
    }
    if (run.kind === 'instrumental-variable-run')
      return { value: formatWords('None; the estimator uses no covariates'), preview: null }
    if (run.estimate.adjustment.kind === 'structural-parent-model')
      return {
        value: formatWords(
          `${run.estimate.adjustment.coefficients} parent coefficients, ${run.estimate.adjustment.paths} directed paths`,
        ),
        preview: null,
      }
    const names = adjustmentLabels(run.estimate.adjustment)
    return {
      value: adjustmentFigure(names, 'None'),
      preview: names.length === 0 ? null : namesFigure(names, 'variables').preview,
    }
  })()
  const adjustmentValue = adjustmentTile.value

  // A method that reports the set it actually fitted names the same members as the recorded estimate
  // whenever it accepted them, so the two tiles would print one value twice and leave the grid's last
  // row part-filled. Keep the one tile and let it carry how the set was chosen.
  const restated = tiles.find(
    (tile) => tile.label === 'Adjustment set' && tile.value.text === adjustmentValue.text,
  )
  const declaredCategorical = run.columns
    .filter((column) => run.encodings[column.column]?.kind === 'categorical')
    .map((column) => column.name)

  return (
    <MetricGrid className="mt-4" label="Diagnostics">
      <MetricTile
        label={
          run.kind === 'panel-intervention-run'
            ? 'Comparison design'
            : run.kind === 'frontdoor-two-stage-run'
              ? 'Stage adjustments'
              : run.kind === 'instrumental-variable-run'
                ? 'Covariates'
                : run.estimate.adjustment.kind === 'structural-parent-model'
                  ? 'Structural model'
                  : 'Adjustment set'
        }
        size="compact"
        frame="cell"
        value={adjustmentValue}
        help={
          run.kind === 'panel-intervention-run'
            ? 'This design does not use a DAG adjustment set.'
            : undefined
        }
        context={
          run.kind === 'panel-intervention-run'
            ? (adjustmentTile.preview ?? undefined)
            : declaredCategorical.length > 0
              ? `Categorical: ${namesInProse(declaredCategorical, (count) => `${count} variables`)}. One column per level.`
              : (adjustmentTile.preview ?? restated?.context)
        }
      />
      {tiles
        .filter((tile) => tile !== restated)
        .map((tile) => (
          <MetricTile
            key={tile.label}
            label={tile.label}
            size="compact"
            frame="cell"
            value={tile.value}
            context={tile.context}
          />
        ))}
    </MetricGrid>
  )
}

/** The treated and comparison unit counts every two-group panel design reports. */
const panelLayoutTile = (
  layout: { readonly treatedUnits: number; readonly controlUnits: number },
  context: string,
) => ({
  label: 'Panel layout',
  value: formatWords(
    `${formatCount(layout.treatedUnits).text} treated, ${formatCount(layout.controlUnits).text} comparison`,
  ),
  context,
})
