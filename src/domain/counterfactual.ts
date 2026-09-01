import { z } from 'zod'
import { assertNever, brand, isNonEmpty, type Brand, type NonEmptyArray } from './dop'
import type { CaveatEvaluation, MethodDefinition, MethodEligibility } from './methods'
import { DYNAMIC_LINEAR_SCM_METHOD_ID, LINEAR_SCM_METHOD_ID } from './methods'
import type { PreparedDatasetArtifact, PreparedDatasetVersionId } from './preprocessing'
import type { Identification, IdentificationId, StudyId, StudySpecification, StudyVariable } from './study'

/**
 * A counterfactual run keeps the fitted structural equations beside the row-level answers, so a
 * reader can see exactly which model produced each "would have been".
 */

export type CounterfactualRunId = Brand<string, 'CounterfactualRunId'>

export const newCounterfactualRunId = (): CounterfactualRunId => brand<string, 'CounterfactualRunId'>(crypto.randomUUID())

export interface LinearScmConfiguration {
  readonly kind: 'linear-scm'
  /** The two treatment values every row is set to; the effect is the second minus the first. */
  readonly interventions: readonly [number, number]
  /** Observation noise scale for abduction; null inverts the noise map exactly. */
  readonly observationNoise: number | null
}

export const DEFAULT_LINEAR_SCM: LinearScmConfiguration = { kind: 'linear-scm', interventions: [0, 1], observationNoise: null }

export type DynamicInterventionTiming =
  | { readonly kind: 'point'; readonly time: number }
  | { readonly kind: 'persistent'; readonly start: number }

export type DynamicInterventionSchedule =
  | { readonly kind: 'point'; readonly row: number }
  | { readonly kind: 'persistent'; readonly startRow: number }

export const dynamicCounterfactualBlockLengthSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('fixed'), length: z.number().int().positive() }).strict(),
  z.object({ kind: z.literal('cubeRoot') }).strict(),
])
export type DynamicCounterfactualBlockLength = z.infer<typeof dynamicCounterfactualBlockLengthSchema>

export const dynamicCounterfactualUncertaintySchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('none') }).strict(),
  z.object({
    kind: z.literal('blockBootstrap'),
    samples: z.number().int().min(20).max(5000),
    blockLength: dynamicCounterfactualBlockLengthSchema,
    confidenceLevel: z.number().gt(0).lt(1),
    seed: z.number().int().min(0).max(0xffff_ffff),
  }).strict(),
])
export type DynamicCounterfactualUncertainty = z.infer<typeof dynamicCounterfactualUncertaintySchema>

export interface DynamicLinearScmConfiguration {
  readonly kind: 'dynamic-linear-scm'
  readonly interventions: readonly [number, number]
  readonly schedule: DynamicInterventionSchedule
  /** Number of reported time points, including the intervention time. */
  readonly steps: number
  readonly uncertainty: DynamicCounterfactualUncertainty
}

export const DEFAULT_DYNAMIC_LINEAR_SCM: DynamicLinearScmConfiguration = {
  kind: 'dynamic-linear-scm',
  interventions: [0, 1],
  schedule: { kind: 'point', row: 2 },
  steps: 12,
  uncertainty: {
    kind: 'blockBootstrap',
    samples: 200,
    blockLength: { kind: 'cubeRoot' },
    confidenceLevel: 0.9,
    seed: 0,
  },
}

export type CounterfactualConfiguration = LinearScmConfiguration | DynamicLinearScmConfiguration

export const linearScmEvidenceSchema = z.object({
  kind: z.literal('linearScmCounterfactual'),
  observations: z.number().int().positive(),
  order: z.array(z.number().int().nonnegative()).min(2),
  equations: z.array(z.object({
    node: z.number().int().nonnegative(),
    intercept: z.number().finite(),
    parents: z.array(z.tuple([z.number().int().nonnegative(), z.number().finite()])),
    residualSd: z.number().finite().nonnegative(),
    rSquared: z.number().finite(),
  }).strict()).min(2),
  interventions: z.tuple([z.number().finite(), z.number().finite()]),
  observationNoise: z.number().positive().nullable(),
  factualOutcome: z.array(z.number().finite()),
  counterfactualLow: z.array(z.number().finite()),
  counterfactualHigh: z.array(z.number().finite()),
  effects: z.array(z.number().finite()),
  averageEffect: z.number().finite(),
  sharePositive: z.number().min(0).max(1),
}).strict()

export type LinearScmEvidence = z.infer<typeof linearScmEvidenceSchema>

const dynamicInterventionTimingSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('point'), time: z.number().int().nonnegative() }).strict(),
  z.object({ kind: z.literal('persistent'), start: z.number().int().nonnegative() }).strict(),
])

const dynamicCounterfactualUncertaintyEvidenceSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('none') }).strict(),
  z.object({
    kind: z.literal('blockBootstrap'),
    samples: z.number().int().min(20).max(5000),
    blockLength: dynamicCounterfactualBlockLengthSchema,
    resolvedBlockLength: z.number().int().positive(),
    confidenceLevel: z.number().gt(0).lt(1),
    seed: z.number().int().min(0).max(0xffff_ffff),
    effectDraws: z.array(z.array(z.number().finite()).min(1)).min(1),
    pointwiseEffectInterval: z.tuple([
      z.array(z.number().finite()).min(1),
      z.array(z.number().finite()).min(1),
    ]),
    averageDraws: z.array(z.number().finite()).min(1),
    averageInterval: z.tuple([z.number().finite(), z.number().finite()]),
    cumulativeDraws: z.array(z.number().finite()).min(1),
    cumulativeInterval: z.tuple([z.number().finite(), z.number().finite()]),
  }).strict(),
])
export type DynamicCounterfactualUncertaintyEvidence = z.infer<typeof dynamicCounterfactualUncertaintyEvidenceSchema>

export function dynamicCounterfactualUncertaintyMatches(requested: DynamicCounterfactualUncertainty, returned: DynamicCounterfactualUncertaintyEvidence): boolean {
  switch (requested.kind) {
    case 'none': return returned.kind === 'none'
    case 'blockBootstrap':
      if (returned.kind !== 'blockBootstrap') return false
      return requested.samples === returned.samples
        && requested.confidenceLevel === returned.confidenceLevel
        && requested.seed === returned.seed
        && (requested.blockLength.kind === 'cubeRoot'
          ? returned.blockLength.kind === 'cubeRoot'
          : returned.blockLength.kind === 'fixed' && requested.blockLength.length === returned.blockLength.length)
    default: return assertNever(requested)
  }
}

export const dynamicLinearScmEvidenceSchema = z.object({
  kind: z.literal('dynamicLinearScmCounterfactual'),
  observations: z.number().int().positive(),
  fittedObservations: z.number().int().positive(),
  maxLag: z.number().int().nonnegative(),
  order: z.array(z.number().int().nonnegative()).min(2),
  equations: z.array(z.object({
    variable: z.number().int().nonnegative(),
    intercept: z.number().finite(),
    parents: z.array(z.object({
      variable: z.number().int().nonnegative(),
      lag: z.number().int().nonnegative(),
      coefficient: z.number().finite(),
    }).strict()),
    residualScale: z.number().finite().nonnegative(),
  }).strict()).min(2),
  timing: dynamicInterventionTimingSchema,
  interventions: z.tuple([z.number().finite(), z.number().finite()]),
  start: z.number().int().nonnegative(),
  factualOutcome: z.array(z.number().finite()).min(1),
  counterfactualLow: z.array(z.number().finite()).min(1),
  counterfactualHigh: z.array(z.number().finite()).min(1),
  effects: z.array(z.number().finite()).min(1),
  averageEffect: z.number().finite(),
  cumulativeEffect: z.number().finite(),
  uncertainty: dynamicCounterfactualUncertaintyEvidenceSchema,
}).strict().superRefine((evidence, context) => {
  const length = evidence.effects.length
  if (evidence.factualOutcome.length !== length || evidence.counterfactualLow.length !== length || evidence.counterfactualHigh.length !== length) {
    context.addIssue({ code: 'custom', message: 'Every dynamic counterfactual path must cover the same horizon.' })
  }
  const cumulative = evidence.effects.reduce((sum, effect) => sum + effect, 0)
  const scale = 1 + Math.abs(evidence.cumulativeEffect)
  if (Math.abs(cumulative - evidence.cumulativeEffect) > 1e-9 * scale || Math.abs(evidence.averageEffect - cumulative / length) > 1e-9 * scale) {
    context.addIssue({ code: 'custom', message: 'Dynamic counterfactual aggregate effects do not match the horizon path.' })
  }
  switch (evidence.uncertainty.kind) {
    case 'none': break
    case 'blockBootstrap': {
      const uncertainty = evidence.uncertainty
      if (uncertainty.effectDraws.length !== uncertainty.samples || uncertainty.averageDraws.length !== uncertainty.samples || uncertainty.cumulativeDraws.length !== uncertainty.samples) {
        context.addIssue({ code: 'custom', message: 'Dynamic counterfactual bootstrap draw counts must match the requested samples.' })
      }
      if (uncertainty.pointwiseEffectInterval[0].length !== length || uncertainty.pointwiseEffectInterval[1].length !== length || uncertainty.effectDraws.some((draw) => draw.length !== length)) {
        context.addIssue({ code: 'custom', message: 'Every dynamic counterfactual bootstrap path and pointwise interval must cover the reported horizon.' })
      }
      if (uncertainty.pointwiseEffectInterval[0].some((lower, index) => lower > (uncertainty.pointwiseEffectInterval[1][index] ?? Number.NEGATIVE_INFINITY)) || uncertainty.averageInterval[0] > uncertainty.averageInterval[1] || uncertainty.cumulativeInterval[0] > uncertainty.cumulativeInterval[1]) {
        context.addIssue({ code: 'custom', message: 'Dynamic counterfactual bootstrap interval bounds are reversed.' })
      }
      uncertainty.effectDraws.forEach((draw, index) => {
        const drawCumulative = draw.reduce((sum, effect) => sum + effect, 0)
        const returnedCumulative = uncertainty.cumulativeDraws[index] ?? Number.NaN
        const returnedAverage = uncertainty.averageDraws[index] ?? Number.NaN
        const drawScale = 1 + Math.abs(drawCumulative)
        if (Math.abs(returnedCumulative - drawCumulative) > 1e-9 * drawScale || Math.abs(returnedAverage - drawCumulative / length) > 1e-9 * drawScale) {
          context.addIssue({ code: 'custom', message: 'Dynamic counterfactual bootstrap aggregates do not match their effect paths.' })
        }
      })
      break
    }
    default: assertNever(evidence.uncertainty)
  }
})

export type DynamicLinearScmEvidence = z.infer<typeof dynamicLinearScmEvidenceSchema>

interface CounterfactualRunIdentity {
  readonly id: CounterfactualRunId
  readonly study: StudyId
  readonly identification: IdentificationId
  readonly preparedDataset: PreparedDatasetVersionId
  readonly createdAt: string
  /** The DAG nodes in the order the matrix columns were sent. */
  readonly nodes: NonEmptyArray<StudyVariable>
  readonly eligibility: Exclude<MethodEligibility, { readonly kind: 'refused' }>
}

export type CounterfactualRunArtifact =
  | CounterfactualRunIdentity & { readonly kind: 'linear-scm-run'; readonly method: typeof LINEAR_SCM_METHOD_ID; readonly configuration: LinearScmConfiguration; readonly evidence: LinearScmEvidence }
  | CounterfactualRunIdentity & { readonly kind: 'dynamic-linear-scm-run'; readonly method: typeof DYNAMIC_LINEAR_SCM_METHOD_ID; readonly configuration: DynamicLinearScmConfiguration; readonly evidence: DynamicLinearScmEvidence }

type Satisfied = Extract<CaveatEvaluation, { readonly kind: 'satisfied' }>
type Unresolved = Extract<CaveatEvaluation, { readonly kind: 'unresolved' }>
type Violated = Extract<CaveatEvaluation, { readonly kind: 'violated' }>

const findCaveat = (method: MethodDefinition, id: string): MethodDefinition['caveats'][number] =>
  method.caveats.find((caveat) => caveat.id === id) ?? (() => {
    throw new Error(`Method catalogue invariant failed: ${method.name} has no condition “${id}”.`)
  })()

/** Rules over the study's graph, the identification, and the configuration. */
export function evaluateCounterfactualEligibility(method: MethodDefinition, context: {
  readonly study: StudySpecification
  readonly identification: Identification
  readonly prepared: PreparedDatasetArtifact
  readonly configuration: CounterfactualConfiguration
}): MethodEligibility {
  const satisfied: Satisfied[] = []
  const unresolved: Unresolved[] = []
  const violations: Violated[] = []
  const satisfy = (id: string, evidence: string) => satisfied.push({ kind: 'satisfied', caveat: findCaveat(method, id), evidence })
  const leave = (id: string, missingEvidence: string) => unresolved.push({ kind: 'unresolved', caveat: findCaveat(method, id), missingEvidence })
  const violate = (id: string, evidence: string) => violations.push({ kind: 'violated', caveat: findCaveat(method, id), evidence })
  const { study, identification, prepared, configuration } = context

  const latent = study.graph.nodes.filter((node) => node.column === null)
  switch (configuration.kind) {
    case 'linear-scm':
      if (latent.length > 0) violate('scm-graph-complete', `${latent.map((node) => node.name).join(', ')} ${latent.length === 1 ? 'is' : 'are'} unmeasured; every equation needs its parents in the data.`)
      else if (identification.kind !== 'identified') violate('scm-graph-complete', 'No measured back-door adjustment set was found; the fitted equations would carry an open back-door path.')
      else satisfy('scm-graph-complete', `All ${study.graph.nodes.length} nodes are measured and the study is identified by back-door adjustment.`)
      leave('scm-structural-equations', 'Linearity with additive noise is assumed; the equations’ R² and residual scales are reported with the run.')
      if (configuration.interventions[0] === configuration.interventions[1]) violate('scm-abduction', 'The two intervention values are equal, so every row-level effect is zero by construction.')
      else satisfy('scm-abduction', configuration.observationNoise === null ? 'Exact inversion of the affine noise map.' : `Gaussian posterior mean under observation noise ${configuration.observationNoise}.`)
      leave('scm-modularity', 'The run assumes intervention changes only the treatment equation and carries each row’s inferred disturbance terms unchanged into both intervention worlds; this cross-world structural assumption is not testable from the observed rows.')
      if (prepared.kind === 'prepared-time-series' || study.graph.laggedArrows > 0) leave('scm-contemporaneous', study.graph.laggedArrows > 0 ? `${study.graph.laggedArrows} lagged arrows are collapsed; each row is treated as one unit.` : 'Rows are a time series; each row is treated as one unit.')
      else satisfy('scm-contemporaneous', 'Independent observations with no lagged arrows.')
      break
    case 'dynamic-linear-scm': {
      if (prepared.kind !== 'prepared-time-series') violate('dynamic-scm-time-series', 'This prepared dataset is not a regular time series.')
      else satisfy('dynamic-scm-time-series', `Prepared as a regular ${prepared.sampling.frequency} time series in chronological order.`)
      leave('dynamic-scm-structural-equations', 'Linearity with additive innovations and time-invariant coefficients is assumed; every fitted coefficient and residual scale is retained with the run.')
      if (latent.length > 0) violate('dynamic-scm-graph-complete', `${latent.map((node) => node.name).join(', ')} ${latent.length === 1 ? 'is' : 'are'} unmeasured; the dynamic fitter requires every parent series.`)
      else satisfy('dynamic-scm-graph-complete', `All ${study.graph.nodes.length} graph nodes are measured; contemporaneous and lagged arrows are passed at their recorded time indices.`)
      const startRow = configuration.schedule.kind === 'point' ? configuration.schedule.row : configuration.schedule.startRow
      if (startRow < 2) violate('dynamic-scm-history', 'The intervention starts before one complete lag of factual history is available; the exact maximum lag is checked by the kernel.')
      else satisfy('dynamic-scm-history', `The intervention begins at row ${startRow}; the kernel verifies that the preceding rows cover the graph’s maximum lag.`)
      leave('dynamic-scm-modularity', 'The replay holds every non-treatment equation and each time point’s recovered innovation fixed across worlds; this cross-world invariance is a structural assumption.')
      if (configuration.interventions[0] === configuration.interventions[1]) violate('dynamic-scm-intervention-schedule', 'The two intervention values are equal, so the contrast is zero by construction.')
      else satisfy('dynamic-scm-intervention-schedule', configuration.schedule.kind === 'point' ? `One-time intervention at row ${configuration.schedule.row}.` : `Persistent intervention from row ${configuration.schedule.startRow} through the ${configuration.steps}-point horizon.`)
      switch (configuration.uncertainty.kind) {
        case 'none':
          leave('dynamic-scm-no-interval', 'No sampling interval was requested; the fitted path and its aggregates will be reported as point results.')
          break
        case 'blockBootstrap':
          if (configuration.uncertainty.samples < 20) violate('dynamic-scm-no-interval', 'At least 20 block-bootstrap refits are required.')
          else satisfy('dynamic-scm-no-interval', `${configuration.uncertainty.samples} block-bootstrap refits at ${Math.round(configuration.uncertainty.confidenceLevel * 100)}% confidence; the kernel verifies the selected block length leaves at least 2 blocks.`)
          break
        default: assertNever(configuration.uncertainty)
      }
      break
    }
    default:
      assertNever(configuration)
  }
  if (isNonEmpty(violations)) return { kind: 'refused', satisfied, unresolved, violations }
  if (isNonEmpty(unresolved)) return { kind: 'caution', satisfied, unresolved }
  return { kind: 'eligible', satisfied }
}
