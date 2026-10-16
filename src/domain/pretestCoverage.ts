import { z } from 'zod'
import { brand, err, ok, type Result } from './dop'
import {
  eventStudyCandidates,
  honestEventStudySchema,
  honestSourceSchema,
  type HonestCandidate,
} from './honestDid'
import type { EstimationRunArtifact, ConfigurationSetting } from './estimation'
import type { TimeSeriesRun } from './timeSeries'
import { clusterCovarianceLabels } from './panelRegression'

const finite = z.number().finite()
const level = finite.gt(0).lt(1)
const probability = finite.min(0).max(1)
export const pretestRequestSchema = z
  .object({
    targetBias: finite,
    pretestBias: finite,
    correlation: finite.min(-1).max(1),
    intervalAlpha: level,
    pretestAlpha: level,
  })
  .strict()
export type PretestRequest = z.infer<typeof pretestRequestSchema>
export const pretestSettingsSchema = z
  .object({
    sourceRun: z.string().uuid(),
    prePeriod: z.number().int().safe(),
    postPeriod: z.number().int().safe(),
    coefficients: z
      .object({
        pre: z.number().int().nonnegative(),
        post: z.number().int().nonnegative(),
        reference: z.number().int().safe(),
      })
      .strict()
      .optional(),
    scenario: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('calibrated'), offset: finite }).strict(),
      z.object({ kind: z.literal('specified'), bias: finite }).strict(),
    ]),
    confidence: level,
    pretestAlpha: level,
  })
  .strict()
export type PretestSettings = z.infer<typeof pretestSettingsSchema>
export const pretestEvidenceSchema = z
  .object({
    kind: z.literal('pretestCoverage'),
    request: pretestRequestSchema,
    unconditional: probability,
    conditional: probability,
    passProbability: probability.positive(),
    jointProbability: probability,
    integrationError: finite.nonnegative(),
    evaluations: z.number().int().nonnegative(),
  })
  .strict()
  .refine(
    (e) =>
      Math.abs(e.jointProbability - e.conditional * e.passProbability) < 1e-10 &&
      e.jointProbability <= e.unconditional + 1e-10,
    'The coverage probabilities are inconsistent.',
  )
export type PretestEvidence = z.infer<typeof pretestEvidenceSchema>

const customStudySchema = z
  .object({
    kind: z.literal('custom'),
    reference: z.number().int().safe(),
    terms: z.tuple([z.number().int().nonnegative(), z.number().int().nonnegative()]),
    names: z.tuple([z.string().min(1), z.string().min(1)]),
    events: z.tuple([z.number().int().safe(), z.number().int().safe()]),
    coefficients: z.tuple([finite, finite]),
    covariance: z.tuple([z.tuple([finite, finite]), z.tuple([finite, finite])]),
  })
  .strict()
  .refine(
    (v) =>
      v.terms[0] !== v.terms[1] &&
      v.events[0] < v.reference &&
      v.reference < v.events[1] &&
      v.covariance[0][0] > 0 &&
      v.covariance[1][1] > 0 &&
      Math.abs(v.covariance[0][1] - v.covariance[1][0]) <=
        1e-10 * Math.max(1, Math.abs(v.covariance[0][1])),
    'Choose distinct coefficients on opposite sides of their shared reference, with a valid covariance.',
  )
const pretestStudySchema = z.union([honestEventStudySchema, customStudySchema])
type PretestStudy = z.infer<typeof pretestStudySchema>
export type PretestCandidate = HonestCandidate & {
  readonly custom?: Extract<TimeSeriesRun, { kind: 'panel-regression' }>
}

export function pretestInput(
  candidate: PretestCandidate,
  settings: PretestSettings,
): Result<PretestStudy, string> {
  if (!candidate.custom)
    return settings.coefficients
      ? err('Coefficient mapping is only available for custom-regression runs.')
      : candidate.input
  const mapping = settings.coefficients
  if (!mapping)
    return err('Choose two event coefficients and their shared omitted reference period.')
  const e = candidate.custom.evidence
  const pre = e.terms.findIndex((t) => t.index === mapping.pre)
  const post = e.terms.findIndex((t) => t.index === mapping.post)
  if (pre < 0 || post < 0) return err('Choose retained coefficients from the saved regression.')
  const parsed = customStudySchema.safeParse({
    kind: 'custom',
    reference: mapping.reference,
    terms: [mapping.pre, mapping.post],
    names: [e.terms[pre]!.name, e.terms[post]!.name],
    events: [settings.prePeriod, settings.postPeriod],
    coefficients: [e.terms[pre]!.estimate, e.terms[post]!.estimate],
    covariance: [
      [e.covariance[pre]![pre], e.covariance[pre]![post]],
      [e.covariance[post]![pre], e.covariance[post]![post]],
    ],
  })
  return parsed.success
    ? ok(parsed.data)
    : err(
        'Choose distinct event coefficients with the pre-period before the omitted reference and the post-period after it.',
      )
}

export function pretestRequest(
  study: PretestStudy,
  settings: PretestSettings,
): Result<PretestRequest, string> {
  const pre = study.events.indexOf(settings.prePeriod),
    post = study.events.indexOf(settings.postPeriod)
  if (pre < 0 || post < 0)
    return err(
      'Choose individual pre-treatment and post-treatment periods from this event study, not binned endpoints.',
    )
  const preVariance = study.covariance[pre]![pre]!,
    postVariance = study.covariance[post]![post]!
  const correlation = study.covariance[pre]![post]! / Math.sqrt(preVariance * postVariance)
  const pretestBias =
    settings.scenario.kind === 'specified'
      ? settings.scenario.bias
      : study.coefficients[pre]! / Math.sqrt(preVariance) + settings.scenario.offset
  const reference = 'kind' in study ? study.reference : -1
  if (settings.prePeriod >= reference || settings.postPeriod <= reference)
    return err('The selected periods must lie on opposite sides of the omitted reference period.')
  const distanceRatio = (settings.postPeriod - reference) / (reference - settings.prePeriod)
  const parsed = pretestRequestSchema.safeParse({
    targetBias: -distanceRatio * Math.sqrt(preVariance / postVariance) * pretestBias,
    pretestBias,
    correlation,
    intervalAlpha: 1 - settings.confidence,
    pretestAlpha: settings.pretestAlpha,
  })
  return parsed.success
    ? ok(parsed.data)
    : err(
        'The selected estimates must have a valid correlation, finite biases and test levels strictly between zero and one.',
      )
}

export const pretestRunSchema = z
  .object({
    kind: z.literal('pretest-coverage-run'),
    id: z
      .string()
      .uuid()
      .transform((v) => brand<string, 'SensitivityRunId'>(v)),
    preparedDataset: z
      .string()
      .uuid()
      .transform((v) => brand<string, 'PreparedDatasetVersionId'>(v)),
    createdAt: z.iso.datetime(),
    source: honestSourceSchema,
    eventStudy: pretestStudySchema,
    settings: pretestSettingsSchema,
    evidence: pretestEvidenceSchema,
  })
  .strict()
  .superRefine((run, ctx) => {
    const request = pretestRequest(run.eventStudy, run.settings)
    if (
      run.source.run !== run.settings.sourceRun ||
      ('kind' in run.eventStudy
        ? run.settings.coefficients?.pre !== run.eventStudy.terms[0] ||
          run.settings.coefficients?.post !== run.eventStudy.terms[1] ||
          run.settings.coefficients?.reference !== run.eventStudy.reference
        : run.settings.coefficients !== undefined) ||
      !request.ok ||
      Object.entries(request.value).some(
        ([key, value]) => value !== run.evidence.request[key as keyof PretestRequest],
      )
    )
      ctx.addIssue({
        code: 'custom',
        message: 'The coverage result must match the recorded event study and scenario.',
      })
  })
export type PretestRun = z.infer<typeof pretestRunSchema>

export function pretestCandidates(
  estimates: readonly EstimationRunArtifact[],
  designs: readonly TimeSeriesRun[],
): readonly PretestCandidate[] {
  return [
    ...eventStudyCandidates(estimates, designs, 'pretestCoverage'),
    ...designs.flatMap((run) =>
      run.kind === 'panel-regression' && run.specification.specification.kind === 'interactions'
        ? [
            {
              source: { kind: 'regressionDesign' as const, run: run.id },
              label: `Custom regression for ${run.outcome.name}, ${clusterCovarianceLabels[run.specification.specification.covariance ?? 'cr2']}`,
              createdAt: run.createdAt,
              preparedDataset: run.preparedDataset,
              custom: run,
              input: err(
                'Choose two event coefficients and their shared omitted reference period.',
              ),
            },
          ]
        : [],
    ),
  ]
}

export function pretestRunMatches(
  run: PretestRun,
  estimates: readonly EstimationRunArtifact[],
  designs: readonly TimeSeriesRun[],
): boolean {
  if (!pretestRunSchema.safeParse(run).success) return false
  const candidate = pretestCandidates(estimates, designs).find(
    (c) => c.source.kind === run.source.kind && c.source.run === run.source.run,
  )
  const input = candidate ? pretestInput(candidate, run.settings) : null
  return (
    candidate !== undefined &&
    candidate.preparedDataset === run.preparedDataset &&
    input?.ok === true &&
    JSON.stringify(input.value) === JSON.stringify(run.eventStudy)
  )
}

export function pretestSettings(
  value: PretestSettings,
  candidates: readonly PretestCandidate[],
): readonly ConfigurationSetting[] {
  return [
    {
      key: 'sourceRun',
      label: 'Source run',
      value: candidates.find((c) => c.source.run === value.sourceRun)?.label ?? 'Not selected',
    },
    { key: 'prePeriod', label: 'Pre-treatment period', value: String(value.prePeriod) },
    { key: 'postPeriod', label: 'Post-treatment period', value: String(value.postPeriod) },
    ...(value.coefficients
      ? [
          {
            key: 'preCoefficient',
            label: 'Pre-treatment coefficient',
            value:
              candidates
                .find((c) => c.source.run === value.sourceRun)
                ?.custom?.evidence.terms.find((t) => t.index === value.coefficients!.pre)?.name ??
              'Not selected',
          },
          {
            key: 'postCoefficient',
            label: 'Post-treatment coefficient',
            value:
              candidates
                .find((c) => c.source.run === value.sourceRun)
                ?.custom?.evidence.terms.find((t) => t.index === value.coefficients!.post)?.name ??
              'Not selected',
          },
          {
            key: 'reference',
            label: 'Omitted reference period',
            value: String(value.coefficients.reference),
          },
        ]
      : []),
    {
      key: 'scenario',
      label: 'Pre-trend bias scenario',
      value:
        value.scenario.kind === 'specified'
          ? `Specified bias: ${value.scenario.bias} standard errors`
          : `Recorded pre-trend t-statistic plus ${value.scenario.offset}`,
    },
    { key: 'confidence', label: 'Confidence level', value: `${value.confidence * 100}%` },
    {
      key: 'pretestAlpha',
      label: 'Pre-test significance level',
      value: `${value.pretestAlpha * 100}%`,
    },
  ]
}

export const pretestMeaning =
  'Coverage describes repeated samples under the recorded linear-trend scenario, not the probability that this saved interval contains the true effect. Passing one pre-test does not establish parallel trends. This calculation does not cover a joint test of several leads or replace HonestDiD confidence sets.'
