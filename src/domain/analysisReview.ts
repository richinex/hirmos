import type { DagDocument } from './dag'
import type { StudySpecification, IdentificationArtifact } from './study'
import type { PreparedDatasetVersionId } from './preprocessing'
import type { EstimationRunArtifact } from './estimation'
import {
  adjustmentLabels,
  configurationSettings,
  describeRun,
  effectiveSampleSize,
  treatmentModelStoppedEarly,
} from './estimation'
import { resolveStudyDagRevision, studyRevisionNotice } from './dagProvenance'
import { honestCandidates, honestRunMatches } from './honestDid'
import { didSensitivitySource, didSensitivityRunMatches } from './didSensitivity'
import type { SensitivityRunArtifact } from './sensitivity'
import { formatCount, formatStatistic } from '@/lib/format/number'

type Source = { readonly workspace: 'estimation' | 'sensitivity'; readonly run: string }
type Finding =
  | { readonly kind: 'available'; readonly concern: boolean; readonly sources: readonly Source[] }
  | {
      readonly kind:
        'not-recorded' | 'not-assessable' | 'not-applicable' | 'unavailable' | 'mismatched'
      readonly reason: string
    }

export interface ReviewItem {
  readonly id: string
  readonly title: string
  readonly evidence: Finding
  readonly reading: string
  readonly next: string | null
}

export const reviewEvidenceLabels: Record<Finding['kind'], string> = {
  available: 'Recorded evidence',
  'not-recorded': 'Not recorded',
  'not-assessable': 'Not assessable',
  'not-applicable': 'Not applicable',
  unavailable: 'Unavailable',
  mismatched: 'Source mismatch',
}

export interface AnalysisReviewContext {
  readonly studies: readonly StudySpecification[]
  readonly identifications: readonly IdentificationArtifact[]
  readonly documents: readonly DagDocument[]
  readonly preparedDataset: PreparedDatasetVersionId | null
  readonly sensitivityRuns: readonly SensitivityRunArtifact[]
}

export type AnalysisReview =
  | { readonly kind: 'not-supported'; readonly reason: string }
  | {
      readonly kind: 'available'
      readonly source: Source & {
        readonly study: string
        readonly identification: string
        readonly preparedDataset: string
        readonly dagRevision: string | null
      }
      readonly method: string
      readonly estimand: EstimationRunArtifact['estimate']['estimand']
      readonly sample: EstimationRunArtifact['estimate']['sample']
      readonly subject: { readonly treatment: string; readonly outcome: string } | null
      readonly settings: ReturnType<typeof configurationSettings>
      readonly adjustment: readonly string[]
      readonly provenance: readonly string[]
      readonly questionCheck: string
      readonly items: readonly ReviewItem[]
      readonly assumptions: readonly string[]
    }

const number = (value: number) => formatStatistic('raw', value).text
const count = (value: number) => formatCount(value).text
const maximum = (values: readonly number[]) => values.reduce((a, b) => Math.max(a, b), -Infinity)
const minimum = (values: readonly number[]) => values.reduce((a, b) => Math.min(a, b), Infinity)

function recorded(
  run: EstimationRunArtifact,
  id: string,
  title: string,
  reading: string,
  next: string | null = null,
  concern = false,
): ReviewItem {
  return {
    id,
    title,
    evidence: { kind: 'available', concern, sources: [{ workspace: 'estimation', run: run.id }] },
    reading,
    next,
  }
}

function absent(
  id: string,
  title: string,
  kind: Exclude<Finding['kind'], 'available'>,
  reason: string,
  next: string | null = null,
): ReviewItem {
  return { id, title, evidence: { kind, reason }, reading: reason, next }
}

type PropensityRun = Extract<
  EstimationRunArtifact,
  { kind: 'propensity-weighting-run' | 'propensity-matching-run' | 'doubly-robust-run' }
>

function propensityReview(run: PropensityRun): ReviewItem[] {
  const e = run.evidence
  const b = e.balance
  const balance =
    b.kind === 'available'
      ? b.rows.length === 0
        ? absent(
            'balance',
            'Covariate balance',
            'not-applicable',
            'No adjustment covariates were recorded. This does not establish that the groups are comparable.',
          )
        : recorded(
            run,
            'balance',
            'Covariate balance',
            `Across ${count(b.rows.length)} covariates, the largest absolute standardised difference is ${number(maximum(b.rows.map((r) => Math.abs(r.before))))} before adjustment and ${number(maximum(b.rows.map((r) => Math.abs(r.after))))} after adjustment.${run.kind === 'doubly-robust-run' ? ' After adjustment describes propensity weighting, not the outcome regression.' : ''}`,
            'Inspect the Love plot and individual covariates in Estimation. Balance on measured covariates does not establish absence of unmeasured confounding.',
          )
      : absent(
          'balance',
          'Covariate balance',
          b.kind === 'notRecorded' ? 'not-recorded' : 'unavailable',
          b.kind === 'notRecorded' ? 'This saved run contains no balance calculation.' : b.reason,
          'If balance is needed, review the specification in Estimation before approving a new run.',
        )
  const scores = (treated: boolean) => e.propensity.filter((_, i) => e.treated[i] === treated)
  const treated = scores(true),
    control = scores(false)
  const overlap =
    treated.length > 0 && control.length > 0 && e.propensity.length === e.treated.length
      ? recorded(
          run,
          'overlap',
          'Treatment overlap',
          `Recorded propensity scores range from ${number(minimum(treated))} to ${number(maximum(treated))} among treated rows and from ${number(minimum(control))} to ${number(maximum(control))} among control rows.`,
          'Inspect both score distributions in Estimation. Overlapping ranges alone do not establish adequate support.',
        )
      : absent(
          'overlap',
          'Treatment overlap',
          'unavailable',
          'The saved scores cannot be compared across both treatment groups.',
        )
  const stopped =
    e.kind === 'doublyRobust' ? !e.converged : treatmentModelStoppedEarly(e.treatmentModel)
  const treatedRows = e.treated.filter(Boolean).length
  const controlRows = e.treated.length - treatedRows
  const items = [
    recorded(
      run,
      'population',
      'Target population',
      `${e.target === 'att' ? `The ATT averages over the ${count(treatedRows)} treated rows` : `The ATE targets both groups in the ${count(e.observations)} analysed rows`}. The fit includes ${count(treatedRows)} treated and ${count(controlRows)} control rows.`,
      'Check the source transformations for exclusions. These counts do not describe rows removed before estimation.',
    ),
    recorded(
      run,
      'convergence',
      'Treatment model',
      stopped
        ? 'The treatment model stopped before convergence.'
        : 'The recorded treatment model has no early-stop flag.',
      stopped
        ? 'Review the treatment model in Estimation before relying on its estimate or diagnostics.'
        : null,
      stopped,
    ),
    balance,
    overlap,
  ]
  if (run.kind === 'propensity-weighting-run') {
    const size = effectiveSampleSize(run.evidence.weights, run.evidence.treated)
    items.push(
      recorded(
        run,
        'weights',
        'Weight concentration',
        `Effective sample sizes are ${number(size.treated)} treated and ${number(size.control)} control rows.`,
        'Compare these with the group counts. A much smaller effective sample indicates uneven weights; it is not a test of identification.',
      ),
    )
  }
  if ('interval' in e && e.interval !== null && e.interval.unconverged.length > 0) {
    items.push(
      recorded(
        run,
        'bootstrap-convergence',
        'Interval refits',
        `${count(e.interval.unconverged.length)} of ${count(e.interval.rounds)} bootstrap refits stopped before convergence.`,
        'Review the interval refits in Estimation before relying on the reported uncertainty.',
        true,
      ),
    )
  }
  items.push(
    absent(
      'hidden-confounding',
      'Unmeasured confounding',
      'not-assessable',
      'Balance and overlap cannot measure bias from an omitted common cause. The current estimator-probe tools do not refit this propensity estimator.',
      'Review the causal assumptions and plausible omitted causes. A sensitivity analysis from a different estimator would answer a different question.',
    ),
  )
  return items
}

function sensitivityFinding(probe: SensitivityRunArtifact): string {
  if (probe.kind === 'did-sensitivity-run') {
    const e = probe.evidence
    return `The omitted-variable analysis reports robustness values of ${number(e.robustnessValue)} for the estimate and ${number(e.robustnessValueCi)} for its interval, expressed as shares, with null ${number(e.request.null)}, level ${number(e.request.level)} and rho ${number(e.request.rho)}.`
  }
  if (probe.kind === 'honest-did-run') {
    const e = probe.evidence
    const config = e.request.configuration
    const results = e.results
      .map((result) => {
        const interval = result.interval
        if (interval.kind === 'fixedLength')
          return `bound ${number(result.bound)}: ${number(interval.lower)} to ${number(interval.upper)} (${interval.accuracy})`
        const accepted = interval.grid.filter((_, i) => interval.accepted[i])
        if (accepted.length === 0) return `bound ${number(result.bound)}: no accepted grid points`
        return `bound ${number(result.bound)}: accepted grid envelope ${number(accepted[0]!)} to ${number(accepted.at(-1)!)}${interval.openLower || interval.openUpper ? ', reaching the grid boundary' : ''}`
      })
      .join('; ')
    return `Under ${config.restriction.kind === 'smoothness' ? 'smoothness' : 'relative-magnitude'} restrictions at confidence ${number(config.confidence)}, the saved target estimate is ${number(e.estimate)}. ${results}. The target uses the recorded post-period contrast, which need not equal the headline average.`
  }
  return ''
}

function sensitivityReview(
  run: EstimationRunArtifact,
  context: AnalysisReviewContext,
): ReviewItem[] {
  const honest = honestCandidates([run], [])[0]
  const dr = didSensitivitySource(run)
  return (
    [
      {
        id: 'parallel-trends',
        title: 'Parallel-trends sensitivity',
        kind: 'honest-did-run',
        supported: honest?.input.ok === true,
        reason:
          honest && !honest.input.ok
            ? honest.input.error
            : 'This saved fit does not supply the event-study inputs required by the current HonestDiD route.',
        matches: (probe: SensitivityRunArtifact) =>
          probe.kind === 'honest-did-run' && honestRunMatches(probe, [run], []),
      },
      {
        id: 'did-confounding',
        title: 'DiD omitted-variable sensitivity',
        kind: 'did-sensitivity-run',
        supported: dr !== null,
        reason:
          'The current DiD omitted-variable calculation requires a supported two-period, doubly robust fit.',
        matches: (probe: SensitivityRunArtifact) =>
          probe.kind === 'did-sensitivity-run' && didSensitivityRunMatches(probe, [run]),
      },
    ] as const
  ).map((check) => {
    const linked = context.sensitivityRuns.filter(
      (probe) =>
        probe.kind === check.kind &&
        (probe.kind === 'honest-did-run'
          ? probe.source.kind === 'estimation' && probe.source.run === run.id
          : probe.estimationRun === run.id),
    )
    const valid = linked.filter((probe) => check.matches(probe))
    if (valid.length > 0)
      return {
        id: check.id,
        title: check.title,
        evidence: {
          kind: 'available' as const,
          concern: valid.length !== linked.length,
          sources: valid.map((probe) => ({ workspace: 'sensitivity' as const, run: probe.id })),
        },
        reading: `${count(valid.length)} saved sensitivity ${valid.length === 1 ? 'analysis matches' : 'analyses match'} this run's recorded inputs.${valid.length !== linked.length ? ' Other linked records do not match and are excluded.' : ''} ${valid.map(sensitivityFinding).join(' ')} These are conditional results, not measured violations.`,
        next: 'Read the linked results in Sensitivity, including their restrictions, target and intervals, before interpreting robustness.',
      }
    if (linked.length > 0)
      return absent(
        check.id,
        check.title,
        'mismatched',
        'Linked sensitivity records do not match this run’s saved inputs and cannot support its review.',
        'Review the source selection in Sensitivity.',
      )
    return check.supported
      ? absent(
          check.id,
          check.title,
          'not-recorded',
          'No matching sensitivity result is saved for this run.',
          'Open Sensitivity and select this source run to review a specification before running it.',
        )
      : absent(check.id, check.title, 'not-assessable', check.reason)
  })
}

function didReview(
  run: Extract<EstimationRunArtifact, { kind: 'panel-intervention-run' }>,
  context: AnalysisReviewContext,
): ReviewItem[] {
  const e = run.evidence
  if (e.kind === 'sunAbraham') {
    const pre = e.events.filter(
      (event) => event.key < 0 && !e.request.referencePeriods.includes(event.key),
    )
    const excludesZero = pre.filter(
      (event) => event.interval.lower > 0 || event.interval.upper < 0,
    ).length
    return [
      recorded(
        run,
        'comparison',
        'Comparison and timing',
        `${count(e.retainedRows.length)} rows were retained and ${count(e.removedRows.length)} removed. The fit uses ${count(e.clusters)} clusters and omits ${count(e.omitted.length)} cohort–event cells. Reference periods: ${e.request.referencePeriods.join(', ')}. Explicit reference cohorts: ${e.request.referenceCohorts.join(', ') || 'None'}.`,
        'Inspect the recorded adoption histories and reference cohorts before interpreting comparisons. Event-time effects may average over different cohorts.',
        e.removedRows.length > 0 || e.omitted.length > 0,
      ),
      pre.length === 0
        ? absent(
            'pre-trends',
            'Pre-treatment evidence',
            'not-assessable',
            'No estimated pre-treatment event intervals outside the reference periods are recorded.',
          )
        : recorded(
            run,
            'pre-trends',
            'Pre-treatment evidence',
            `${count(pre.length)} pre-treatment event intervals are recorded; ${count(excludesZero)} exclude zero. This is not a joint test, and non-rejection does not establish parallel trends.`,
            'Inspect anticipation and cohort support alongside the event-study estimates.',
            excludesZero > 0,
          ),
      ...sensitivityReview(run, context),
    ]
  }
  if (e.kind === 'staggeredDid') {
    const pre = e.events.keys.flatMap((event, i) =>
      event < -e.specification.anticipation && e.events.intervals[i]?.kind !== 'reference'
        ? [e.events.intervals[i]!]
        : [],
    )
    const estimated = pre.filter((p) => p.kind === 'estimated')
    const excludesZero = estimated.filter((p) => p.lower > 0 || p.upper < 0).length
    const limited = e.fits.filter((f) => f.status.kind === 'iterationLimit').length
    return [
      recorded(
        run,
        'comparison',
        'Comparison and timing',
        `The comparison uses ${e.specification.controls === 'never-treated' ? 'never-treated' : 'not-yet-treated'} units, with ${count(e.specification.anticipation)} anticipation periods. ${count(e.retainedObservations)} of ${count(e.observations)} input observations were retained. ${count(e.changes.length)} support changes and ${count(e.smallCohorts.length)} small-cohort notices were recorded.`,
        'Inspect cohort support and excluded periods in Estimation. Event-time effects may average over different cohorts.',
        e.changes.length > 0 || e.smallCohorts.length > 0,
      ),
      recorded(
        run,
        'convergence',
        'Nuisance fits',
        limited > 0
          ? `${count(limited)} propensity fits reached their iteration limit.`
          : 'No propensity fit is recorded as reaching its iteration limit.',
        limited > 0
          ? 'Review the convergence warnings in Estimation before interpreting the estimates.'
          : null,
        limited > 0,
      ),
      estimated.length === 0
        ? absent(
            'pre-trends',
            'Pre-treatment evidence',
            'not-assessable',
            'This saved event study has no estimated pre-treatment intervals outside the anticipation window.',
          )
        : recorded(
            run,
            'pre-trends',
            'Pre-treatment evidence',
            `${count(estimated.length)} pre-treatment intervals are available; ${count(excludesZero)} exclude zero. Coverage is ${e.events.coverage.kind}.${pre.length > estimated.length ? ` ${count(pre.length - estimated.length)} further pre-treatment estimates have no interval.` : ''}`,
            'Inspect the event-study plot in Estimation. This count is not a joint pre-trend test, and intervals containing zero do not establish parallel trends.',
            excludesZero > 0,
          ),
      ...sensitivityReview(run, context),
    ]
  }
  const items = [
    recorded(
      run,
      'comparison',
      'Comparison and timing',
      `${count(e.treatedUnits)} treated units are compared with ${count(e.controlUnits)} control units across ${count(e.nPre)} pre-treatment and ${count(e.nPost)} post-treatment periods.`,
      'Check treatment timing, anticipation and whether the comparison units provide a credible untreated trend.',
    ),
  ]
  items.push(
    e.nPre < 2
      ? absent(
          'pre-trends',
          'Pre-treatment evidence',
          'not-assessable',
          'With one pre-treatment period, this panel cannot show a pre-treatment trend.',
        )
      : recorded(
          run,
          'pre-trends',
          'Pre-treatment evidence',
          `${count(e.nPre)} pre-treatment periods are recorded. This review does not compute a pre-trend test.`,
          'Inspect the pre-treatment trajectories in Estimation. Similar observed trajectories do not establish parallel post-treatment trends.',
        ),
  )
  if (e.kind === 'panelAdjusted' && e.inference.kind === 'crossFitted') {
    const failed = e.inference.optimizerStatus.filter(
      (s) => s === 'iteration-limit' || s === 'line-search-failed',
    ).length
    items.push(
      recorded(
        run,
        'convergence',
        'Propensity fits',
        failed > 0
          ? `${count(failed)} folds did not converge.`
          : 'The recorded propensity fits converged in every fold.',
        failed > 0 ? 'Review the propensity fit before interpreting this result.' : null,
        failed > 0,
      ),
    )
    if (e.specification.kind === 'doublyRobust')
      items.push(
        recorded(
          run,
          'trimming',
          'Propensity trimming',
          `The recorded score threshold is ${number(e.specification.trimming)}. The estimate retains the recorded ATT target.`,
          'Do not interpret score clipping as proof of overlap or as evidence that observations were removed.',
        ),
      )
  }
  if (
    e.kind === 'panelIntervention' &&
    (run.configuration.primary ?? 'syntheticDid') === 'syntheticDid'
  ) {
    const method = e.syntheticDid
    const placebo = e.syntheticDidPlacebo
    const inTime = e.syntheticDidInTime
    items.push(
      recorded(
        run,
        'donor-weights',
        'Donor concentration',
        `The largest recorded donor weight is ${number(maximum(method.omega))}.`,
        'Inspect the donor and period weights in Estimation alongside pre-treatment fit. Weight concentration alone does not measure bias.',
      ),
    )
    items.push(
      placebo.kind === 'available'
        ? recorded(
            run,
            'placebo-uncertainty',
            'Placebo uncertainty',
            `${count(placebo.replications)} placebo replications give a standard error of ${number(placebo.standardError)}.`,
            'This is an uncertainty calculation, not a pre-treatment placebo test. Its interpretation depends on comparable error variation across units.',
          )
        : absent('placebo-uncertainty', 'Placebo uncertainty', 'unavailable', placebo.reason),
    )
    items.push(
      inTime.kind === 'available'
        ? recorded(
            run,
            'in-time-placebo',
            'Pre-treatment placebo',
            `The recorded in-time placebo estimate is ${number(inTime.estimate)}.`,
            'Inspect the in-time placebo in Estimation. Its magnitude alone, without uncertainty and context, does not establish a failed or passed check.',
          )
        : absent('in-time-placebo', 'Pre-treatment placebo', 'unavailable', inTime.reason),
    )
  }
  items.push(...sensitivityReview(run, context))
  return items
}

export function reviewAnalysis(
  run: EstimationRunArtifact,
  context: AnalysisReviewContext,
): AnalysisReview {
  const supported =
    run.kind === 'propensity-weighting-run' ||
    run.kind === 'propensity-matching-run' ||
    run.kind === 'doubly-robust-run' ||
    run.kind === 'panel-intervention-run' ||
    run.kind === 'sharp-rd-run' ||
    run.kind === 'instrumental-variable-run' ||
    run.kind === 'synthetic-control-run'
  if (!supported)
    return {
      kind: 'not-supported',
      reason:
        'This review covers propensity weighting, matching, doubly robust estimation, panel DiD, sharp RD, instrumental variables and standalone synthetic control. Other designs need their own evidence review.',
    }
  const study = context.studies.find((s) => s.id === run.study)
  const identification = context.identifications.find(
    (i) => i.id === run.identification && i.study === run.study,
  )
  const graph = study ? resolveStudyDagRevision(study, context.documents) : null
  const provenance = [
    ...(!study
      ? [
          'The recorded study is unavailable. Its question cannot be recovered from the current controls.',
        ]
      : []),
    ...(!identification ? ['The recorded identification is unavailable.'] : []),
    ...(study && study.preparedDataset !== run.preparedDataset
      ? ['The study and run refer to different prepared datasets.']
      : []),
    ...(context.preparedDataset !== run.preparedDataset
      ? [
          'The current prepared data are not the data recorded for this run. The review describes the saved result only.',
        ]
      : []),
    ...(graph && studyRevisionNotice(graph) ? [studyRevisionNotice(graph)!] : []),
  ]
  return {
    kind: 'available',
    source: {
      workspace: 'estimation',
      run: run.id,
      study: run.study,
      identification: run.identification,
      preparedDataset: run.preparedDataset,
      dagRevision: study?.dagRevision ?? null,
    },
    method: describeRun(run.configuration),
    estimand: run.estimate.estimand,
    sample: run.estimate.sample,
    subject: study ? { treatment: study.treatment.name, outcome: study.outcome.name } : null,
    settings: configurationSettings(run.configuration),
    adjustment: adjustmentLabels(run.estimate.adjustment),
    provenance,
    questionCheck:
      'Compare the saved target, population and settings with your intended question. This review cannot certify agreement with an unrecorded intention or reconstruct the original approval from the estimate.',
    items:
      run.kind === 'panel-intervention-run'
        ? didReview(run, context)
        : run.kind === 'sharp-rd-run'
          ? [
              recorded(
                run,
                'local-sample',
                'Local comparison',
                `The effect is local to cutoff ${number(run.evidence.cutoff)}. The fitting bandwidth is ${number(run.evidence.bandwidth)}; ${count(run.evidence.effectiveObservations[0])} observations below and ${count(run.evidence.effectiveObservations[1])} at or above the cutoff contribute.`,
                'Inspect the running-variable distribution and local outcome plot. These counts do not establish continuity.',
              ),
              recorded(
                run,
                'uncertainty',
                'Bias correction and uncertainty',
                `The robust bias-corrected interval is ${number(run.evidence.robust.interval[0])} to ${number(run.evidence.robust.interval[1])}. The conventional estimate is ${number(run.evidence.conventional.value)} and the bias-corrected estimate is ${number(run.evidence.biasCorrected.value)}.`,
                'Read the robust interval as sampling uncertainty, not evidence that assignment is valid.',
              ),
              absent(
                'manipulation',
                'Sorting at the cutoff',
                'not-recorded',
                'This run does not record a density or manipulation test. The outcome plot cannot establish absence of sorting.',
              ),
              absent(
                'bandwidth',
                'Bandwidth sensitivity',
                'not-recorded',
                'This record contains one selected fitting bandwidth, not a bandwidth-sensitivity analysis.',
              ),
            ]
          : run.kind === 'instrumental-variable-run'
            ? [
                recorded(
                  run,
                  'instruments',
                  'Instrument specification',
                  `${count(run.evidence.instruments.length)} instruments are recorded for ${count(run.evidence.observations)} observations.`,
                  'Read the saved identification and specification for the named instruments and target population.',
                ),
                absent(
                  'strength',
                  'Instrument strength',
                  'not-recorded',
                  'This estimator records no first-stage F statistic or weak-instrument-robust interval. A reported standard error does not establish instrument strength.',
                ),
                absent(
                  'exclusion',
                  'Exclusion and assignment',
                  'not-assessable',
                  'The numerical fit does not establish exclusion or as-if-random instrument assignment. Review the recorded graph and substantive assumptions.',
                ),
              ]
            : run.kind === 'synthetic-control-run'
              ? [
                  recorded(
                    run,
                    'pre-fit',
                    'Pre-treatment fit and donors',
                    `There are ${count(run.evidence.nPre)} pre-treatment and ${count(run.evidence.nPost)} post-treatment periods. The largest donor weight is ${number(maximum(run.evidence.weights))}.`,
                    'Inspect the saved pre-treatment gaps and donor weights. Good fit alone does not establish a credible counterfactual.',
                  ),
                  run.evidence.donorPlacebo.kind === 'available'
                    ? recorded(
                        run,
                        'placebos',
                        'Donor placebos',
                        `${count(run.evidence.donorPlacebo.nValidPlacebos)} valid donor placebos are recorded, with p = ${number(run.evidence.donorPlacebo.pValue)}.`,
                        'Compare pre-treatment fit across donors before interpreting the placebo ranking; it is not automatically a randomised-assignment test.',
                      )
                    : absent(
                        'placebos',
                        'Donor placebos',
                        'unavailable',
                        run.evidence.donorPlacebo.reason,
                      ),
                ]
              : propensityReview(run),
    assumptions:
      run.kind === 'panel-intervention-run'
        ? [
            'Identification depends on the design’s parallel-trends, anticipation and interference assumptions. Saved diagnostics do not establish them.',
            'Sensitivity bounds describe specified departures from assumptions, not the departures actually present in the data.',
          ]
        : run.kind === 'sharp-rd-run'
          ? [
              'Identification requires continuity of untreated and treated potential outcomes at the cutoff and a credible assignment rule. The local effect need not apply away from the cutoff.',
            ]
          : run.kind === 'instrumental-variable-run'
            ? [
                'Relevance, exclusion and instrument assignment need justification. Interpret the target under the recorded model; do not automatically call it a population ATE or a complier effect.',
              ]
            : run.kind === 'synthetic-control-run'
              ? [
                  'The donor combination must represent the untreated trajectory after intervention. Pre-treatment fit does not establish absence of spillovers, anticipation or other simultaneous changes.',
                ]
              : [
                  'Identification depends on adequate overlap, consistency and no unmeasured confounding under the recorded design. Balance on measured covariates does not establish these assumptions.',
                ],
  }
}
