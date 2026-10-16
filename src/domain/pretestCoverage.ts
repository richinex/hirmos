import { z } from 'zod'
import { brand, err, ok, type Result } from './dop'
import {
  honestCandidates,
  honestEventStudySchema,
  honestSourceSchema,
  type HonestCandidate,
  type HonestEventStudy,
} from './honestDid'
import type { EstimationRunArtifact, ConfigurationSetting } from './estimation'
import type { TimeSeriesRun } from './timeSeries'

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
    prePeriod: z.number().int().max(-2),
    postPeriod: z.number().int().nonnegative(),
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

export function pretestRequest(
  study: HonestEventStudy,
  settings: PretestSettings,
): Result<PretestRequest, string> {
  const pre = study.events.indexOf(settings.prePeriod),
    post = study.events.indexOf(settings.postPeriod)
  if (pre < 0 || post < 0)
    return err('Choose pre-treatment and post-treatment periods present in this event study.')
  const preVariance = study.covariance[pre]![pre]!,
    postVariance = study.covariance[post]![post]!
  const correlation = study.covariance[pre]![post]! / Math.sqrt(preVariance * postVariance)
  const pretestBias =
    settings.scenario.kind === 'specified'
      ? settings.scenario.bias
      : study.coefficients[pre]! / Math.sqrt(preVariance) + settings.scenario.offset
  // Linear drift is measured from the omitted period, -1, on each side of treatment.
  const distanceRatio = (settings.postPeriod + 1) / (-settings.prePeriod - 1)
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
    eventStudy: honestEventStudySchema,
    settings: pretestSettingsSchema,
    evidence: pretestEvidenceSchema,
  })
  .strict()
  .superRefine((run, ctx) => {
    const request = pretestRequest(run.eventStudy, run.settings)
    if (
      run.source.run !== run.settings.sourceRun ||
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

export function pretestRunMatches(
  run: PretestRun,
  estimates: readonly EstimationRunArtifact[],
  designs: readonly TimeSeriesRun[],
): boolean {
  if (!pretestRunSchema.safeParse(run).success) return false
  const candidate = honestCandidates(estimates, designs).find(
    (c) => c.source.kind === run.source.kind && c.source.run === run.source.run,
  )
  return (
    candidate !== undefined &&
    candidate.preparedDataset === run.preparedDataset &&
    candidate.input.ok &&
    JSON.stringify(candidate.input.value) === JSON.stringify(run.eventStudy)
  )
}

export function pretestSettings(
  value: PretestSettings,
  candidates: readonly HonestCandidate[],
): readonly ConfigurationSetting[] {
  return [
    {
      key: 'sourceRun',
      label: 'Event-study run',
      value: candidates.find((c) => c.source.run === value.sourceRun)?.label ?? 'Not selected',
    },
    { key: 'prePeriod', label: 'Pre-treatment period', value: String(value.prePeriod) },
    { key: 'postPeriod', label: 'Post-treatment period', value: String(value.postPeriod) },
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
