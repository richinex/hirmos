import { z } from 'zod'
import { brand } from './dop'
import type { EstimationRunArtifact } from './estimation'
import {
  adjustedDidConfigurationSchema,
  adjustedDidEvidenceSchema,
  didPropensityFitSchema,
} from './adjustedDid'

const share = z.number().finite().min(0).lt(1)
const axis = z
  .array(share)
  .min(2)
  .max(51)
  .refine(
    (v) => v.every((x, i) => i === 0 || x > v[i - 1]!),
    'Share axes must be strictly increasing.',
  )
export const didSensitivityRequestSchema = z
  .object({
    propensityFit: didPropensityFitSchema,
    folds: z.number().int().min(2),
    seed: z.number().int().min(0).max(0xffffffff),
    trimming: z.number().finite().gt(0).lt(0.5),
    normalization: z.enum(['in-sample', 'population']),
    scenarios: z
      .array(z.tuple([share, share]))
      .min(1)
      .max(100),
    rho: z.number().finite().min(-1).max(1),
    level: z.number().finite().gt(0.5).lt(1),
    null: z.number().finite(),
    outcomeShares: axis,
    rieszShares: axis,
  })
  .strict()
export type DidSensitivityRequest = z.infer<typeof didSensitivityRequestSchema>
const interval = z.tuple([z.number().finite(), z.number().finite()]).refine((v) => v[0] <= v[1])
const bounds = z
  .object({ effect: interval, interval })
  .strict()
  .refine((v) => v.interval[0] <= v.effect[0] && v.interval[1] >= v.effect[1])
// Historical records remain readable, but are never submitted as current requests.
const savedDidSensitivityEvidenceSchema = z
  .object({
    request: didSensitivityRequestSchema.extend({
      propensityFit: didPropensityFitSchema.optional(),
    }),
    units: z
      .array(z.string().min(1))
      .min(2)
      .refine((v) => new Set(v).size === v.length),
    times: z.tuple([z.number().int(), z.number().int()]).refine((v) => v[0] < v[1]),
    estimate: z.number().finite(),
    standardError: z.number().finite().nonnegative(),
    sigma2: z.number().finite().positive(),
    nu2: z.number().finite().positive(),
    rieszEstimate: z.enum(['orthogonal', 'nonOrthogonalReferenceRecovery']),
    optimizerStatus: z.array(
      z.enum(['projected-gradient', 'function-tolerance', 'iteration-limit', 'line-search-failed']),
    ),
    baseline: bounds,
    scenarios: z.array(bounds),
    grid: z.array(z.array(bounds)),
    robustnessValue: z.number().finite().min(0).max(1),
    robustnessValueCi: z.number().finite().min(0).max(1),
  })
  .strict()
  .superRefine((e, ctx) => {
    const fail = (message: string) => ctx.addIssue({ code: 'custom', message })
    if (e.optimizerStatus.length !== e.request.folds)
      fail('Optimizer status must be recorded for every fold.')
    if (
      e.request.propensityFit !== undefined &&
      e.optimizerStatus.some((s) => s === 'iteration-limit' || s === 'line-search-failed')
    )
      fail('Sensitivity requires converged propensity fits.')
    if (e.scenarios.length !== e.request.scenarios.length)
      fail('Scenario results must match the requested shares.')
    if (
      e.grid.length !== e.request.outcomeShares.length ||
      e.grid.some((r) => r.length !== e.request.rieszShares.length)
    )
      fail('The contour grid must match both share axes.')
    if (e.baseline.effect.some((v) => v !== e.estimate))
      fail('The zero-confounding effect must equal the fitted ATT.')
  })
export const didSensitivityEvidenceSchema = savedDidSensitivityEvidenceSchema.refine(
  (e) => e.request.propensityFit !== undefined,
  'A new sensitivity result must record the standardized propensity fit.',
)
export type DidSensitivityEvidence = z.infer<typeof savedDidSensitivityEvidenceSchema>
export const didSensitivityRunSchema = z
  .object({
    kind: z.literal('did-sensitivity-run'),
    id: z
      .string()
      .uuid()
      .transform((v) => brand<string, 'SensitivityRunId'>(v)),
    estimationRun: z
      .string()
      .uuid()
      .transform((v) => brand<string, 'EstimationRunId'>(v)),
    preparedDataset: z
      .string()
      .uuid()
      .transform((v) => brand<string, 'PreparedDatasetVersionId'>(v)),
    createdAt: z.iso.datetime(),
    evidence: savedDidSensitivityEvidenceSchema,
  })
  .strict()
export type DidSensitivityRun = z.infer<typeof didSensitivityRunSchema>

// Only the existing observational, two-period DR specification can supply this fit.
export function didSensitivitySource(run: EstimationRunArtifact) {
  if (
    run.kind !== 'panel-intervention-run' ||
    run.configuration.primary !== 'adjusted' ||
    run.configuration.specification.kind !== 'doublyRobust' ||
    run.evidence.kind !== 'panelAdjusted'
  )
    return null
  const c = adjustedDidConfigurationSchema.safeParse(run.configuration),
    e = adjustedDidEvidenceSchema.safeParse(run.evidence)
  if (
    !c.success ||
    !e.success ||
    e.data.specification.kind !== 'doublyRobust' ||
    c.data.specification.kind !== 'doublyRobust'
  )
    return null
  return {
    run,
    specification: c.data.specification,
    covariates: c.data.covariates,
    evidence: e.data,
  }
}
export function didSensitivityRunMatches(
  run: DidSensitivityRun,
  estimates: readonly EstimationRunArtifact[],
): boolean {
  const original = estimates.find((e) => e.id === run.estimationRun)
  const source = original === undefined ? null : didSensitivitySource(original)
  if (source === null || source.run.preparedDataset !== run.preparedDataset) return false
  const a = source.specification,
    b = run.evidence.request,
    e = run.evidence
  const close = (x: number, y: number) => Math.abs(x - y) <= 1e-9 * Math.max(1, Math.abs(y))
  return (
    a.folds === b.folds &&
    a.seed === b.seed &&
    a.trimming === b.trimming &&
    a.normalization === b.normalization &&
    source.evidence.inference.kind === 'crossFitted' &&
    source.evidence.inference.propensityFit === b.propensityFit &&
    e.units.length === source.evidence.units.length &&
    e.units.every((u, i) => u === source.evidence.units[i]) &&
    e.times.every((t, i) => t === source.evidence.times[i]) &&
    close(e.estimate, source.evidence.estimate) &&
    close(e.standardError, source.evidence.standardError)
  )
}
