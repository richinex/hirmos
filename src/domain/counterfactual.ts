import { z } from 'zod'
import { brand, isNonEmpty, type Brand, type NonEmptyArray } from './dop'
import type { CaveatEvaluation, MethodDefinition, MethodEligibility } from './methods'
import { LINEAR_SCM_METHOD_ID } from './methods'
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

export interface CounterfactualRunArtifact {
  readonly kind: 'linear-scm-run'
  readonly id: CounterfactualRunId
  readonly study: StudyId
  readonly identification: IdentificationId
  readonly preparedDataset: PreparedDatasetVersionId
  readonly createdAt: string
  readonly method: typeof LINEAR_SCM_METHOD_ID
  readonly configuration: LinearScmConfiguration
  /** The DAG nodes in the order the matrix columns were sent. */
  readonly nodes: NonEmptyArray<StudyVariable>
  readonly evidence: LinearScmEvidence
  readonly eligibility: Exclude<MethodEligibility, { readonly kind: 'refused' }>
}

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
  readonly configuration: LinearScmConfiguration
}): MethodEligibility {
  const satisfied: Satisfied[] = []
  const unresolved: Unresolved[] = []
  const violations: Violated[] = []
  const satisfy = (id: string, evidence: string) => satisfied.push({ kind: 'satisfied', caveat: findCaveat(method, id), evidence })
  const leave = (id: string, missingEvidence: string) => unresolved.push({ kind: 'unresolved', caveat: findCaveat(method, id), missingEvidence })
  const violate = (id: string, evidence: string) => violations.push({ kind: 'violated', caveat: findCaveat(method, id), evidence })
  const { study, identification, prepared, configuration } = context

  const latent = study.graph.nodes.filter((node) => node.column === null)
  if (latent.length > 0) violate('scm-graph-complete', `${latent.map((node) => node.name).join(', ')} ${latent.length === 1 ? 'is' : 'are'} unmeasured; every equation needs its parents in the data.`)
  else if (identification.kind !== 'identified') violate('scm-graph-complete', 'No measured back-door adjustment set was found; the fitted equations would carry an open back-door path.')
  else satisfy('scm-graph-complete', `All ${study.graph.nodes.length} nodes are measured and the study is identified by back-door adjustment.`)
  leave('scm-structural-equations', 'Linearity with additive noise is assumed; the equations’ R² and residual scales are reported with the run.')
  if (configuration.interventions[0] === configuration.interventions[1]) violate('scm-abduction', 'The two intervention values are equal, so every row-level effect is zero by construction.')
  else satisfy('scm-abduction', configuration.observationNoise === null ? 'Exact inversion of the affine noise map.' : `Gaussian posterior mean under observation noise ${configuration.observationNoise}.`)
  leave('scm-modularity', 'The run assumes intervention changes only the treatment equation and carries each row’s inferred disturbance terms unchanged into both intervention worlds; this cross-world structural assumption is not testable from the observed rows.')
  if (prepared.kind === 'prepared-time-series' || study.graph.laggedArrows > 0) leave('scm-contemporaneous', study.graph.laggedArrows > 0 ? `${study.graph.laggedArrows} lagged arrows are collapsed; each row is treated as one unit.` : 'Rows are a time series; each row is treated as one unit.')
  else satisfy('scm-contemporaneous', 'Independent observations with no lagged arrows.')
  if (isNonEmpty(violations)) return { kind: 'refused', satisfied, unresolved, violations }
  if (isNonEmpty(unresolved)) return { kind: 'caution', satisfied, unresolved }
  return { kind: 'eligible', satisfied }
}
