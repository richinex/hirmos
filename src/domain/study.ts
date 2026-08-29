import { z } from 'zod'
import type { DagDocument, DagDocumentId, DagNodeId, DagOriginChoice, DagRevisionId, EditableDag } from './dag'
import { analyseDagCausalFlow, describeDagCausalRole, type DagCausalFlow, type DagCausalRole } from './dagFlow'
import { inspectDagStudyBinding, type DagStudyBindingProblem } from './dagValidation'
import type { ColumnId } from './dataset'
import { assertNever, brand, err, isNonEmpty, mapNonEmpty, ok, type Brand, type NonEmptyArray, type Result } from './dop'
import { BACKDOOR_IDENTIFICATION_METHOD_ID } from './methods'
import type { PreparedDatasetArtifact, PreparedDatasetVersionId } from './preprocessing'

/**
 * A study binds one prepared dataset, one validated DAG revision, a treatment, an outcome, and an
 * estimand. Identification is a separate artifact so a refusal is recorded, not merely displayed.
 */

export type StudyId = Brand<string, 'StudyId'>
export type IdentificationId = Brand<string, 'IdentificationId'>

export interface StudyVariable {
  readonly node: DagNodeId
  readonly column: ColumnId
  readonly name: string
}

export type Estimand =
  | { readonly kind: 'average-treatment-effect'; readonly scale: 'additive' }
  | { readonly kind: 'average-treatment-effect-on-treated'; readonly scale: 'additive'; readonly treatedValue: 1 }

/** Estimands the chapter does not offer yet, with the check each one waits on. */
export const ESTIMAND_DEFERRALS: readonly { readonly kind: 'conditional-effect'; readonly name: string; readonly reason: string }[] = [
  { kind: 'conditional-effect', name: 'Effect within subgroups', reason: 'needs a declared modifier and enough rows per group; no estimator here reports it.' },
]

/** Why the treatment varied: the question every credibility judgement starts from. */
export type AssignmentMechanism =
  | { readonly kind: 'randomised'; readonly description: string }
  | { readonly kind: 'policy-change'; readonly description: string }
  | { readonly kind: 'observed-choice'; readonly description: string }

export type StudyDesignCategory = 'experimental' | 'quasi-experimental' | 'observational'

/** A design assumption the estimate presumes; the rationale is the user's, or null when nobody stated one. */
export interface DesignAssumption {
  readonly statement: string
  readonly rationale: string | null
}

export const CONSISTENCY_STATEMENT = 'Consistency: each recorded treatment value represents one well-defined intervention, and a unit\u2019s observed outcome equals its potential outcome under the treatment it received.'
export const NO_INTERFERENCE_STATEMENT = 'No interference: one unit\u2019s treatment does not affect another unit\u2019s outcome.'

export interface StudyGraphNode {
  readonly node: DagNodeId
  readonly name: string
  /** Null for an unmeasured node. */
  readonly column: ColumnId | null
}

/** The DAG at variable level: one node per variable, lags collapsed, duplicate arrows merged. */
export interface StudyGraph {
  readonly nodes: NonEmptyArray<StudyGraphNode>
  readonly edges: readonly (readonly [number, number])[]
  /** Lagged arrows folded into the variable-level graph. */
  readonly laggedArrows: number
}

export interface StudySpecification {
  readonly kind: 'study-specification'
  readonly id: StudyId
  readonly createdAt: string
  readonly preparedDataset: PreparedDatasetVersionId
  readonly dataset: DagDocument['dataset']
  readonly dagDocument: DagDocumentId
  readonly dagRevision: DagRevisionId
  readonly dagName: string
  readonly treatment: StudyVariable
  readonly outcome: StudyVariable
  readonly estimand: Estimand
  readonly population: { readonly kind: 'all-prepared-rows'; readonly observations: number }
  readonly dagBasis: DagOriginChoice
  readonly assignment: AssignmentMechanism
  readonly designAssumptions: { readonly consistency: DesignAssumption; readonly noInterference: DesignAssumption }
  readonly graph: StudyGraph
  /** The bound DAG's editable graph at the study's revision, for path and role analysis. */
  readonly editableGraph: EditableDag
}

export interface StudyDesignDraft {
  readonly dagDocument: DagDocumentId | null
  readonly treatment: DagNodeId | null
  readonly outcome: DagNodeId | null
  readonly estimand: Estimand['kind']
  readonly assignment: { readonly kind: AssignmentMechanism['kind'] | null; readonly description: string }
  readonly consistencyRationale: string
  readonly noInterferenceRationale: string
}

export type StudyDesignProblem =
  | { readonly kind: 'dag-required' }
  | { readonly kind: 'unknown-dag'; readonly document: DagDocumentId }
  | { readonly kind: 'dag-not-validated'; readonly name: string }
  | { readonly kind: 'dag-dataset-mismatch'; readonly name: string }
  | { readonly kind: 'treatment-required' }
  | { readonly kind: 'outcome-required' }
  | { readonly kind: 'latent-treatment'; readonly name: string }
  | { readonly kind: 'latent-outcome'; readonly name: string }
  | { readonly kind: 'binding'; readonly problem: DagStudyBindingProblem }
  | { readonly kind: 'assignment-required' }
  | { readonly kind: 'assignment-description-required' }
  | { readonly kind: 'randomised-needs-experimental-dag'; readonly name: string }

export const EMPTY_STUDY_DRAFT: StudyDesignDraft = { dagDocument: null, treatment: null, outcome: null, estimand: 'average-treatment-effect', assignment: { kind: null, description: '' }, consistencyRationale: '', noInterferenceRationale: '' }

export const dagBasisOf = (document: DagDocument): DagOriginChoice => (document.origin.kind === 'user-authored' ? document.origin.basis : 'discovery-informed')

export const newStudyId = (): StudyId => brand<string, 'StudyId'>(crypto.randomUUID())
export const newIdentificationId = (): IdentificationId => brand<string, 'IdentificationId'>(crypto.randomUUID())

export function studyGraphOf(graph: EditableDag): StudyGraph {
  const nodes = graph.nodes.map((node): StudyGraphNode => ({
    node: node.id,
    name: node.name,
    column: node.kind === 'observed' ? node.column : null,
  })) as unknown as NonEmptyArray<StudyGraphNode>
  const index = new Map(nodes.map((node, position) => [node.node, position]))
  const seen = new Set<string>()
  const edges: (readonly [number, number])[] = []
  let laggedArrows = 0
  for (const edge of graph.edges) {
    const from = index.get(edge.cause)
    const to = index.get(edge.effect)
    if (from === undefined || to === undefined) continue
    if (edge.timing.kind === 'lagged') laggedArrows += 1
    const key = `${from}>${to}`
    if (seen.has(key)) continue
    seen.add(key)
    edges.push([from, to])
  }
  return { nodes, edges, laggedArrows }
}

/** Every check that does not need the graph search: the draft, the DAG's state, and the binding. */
export function readyStudySpecification(
  draft: StudyDesignDraft,
  documents: readonly DagDocument[],
  prepared: PreparedDatasetArtifact,
): Result<StudySpecification, StudyDesignProblem> {
  if (draft.dagDocument === null) return err({ kind: 'dag-required' })
  const document = documents.find((candidate) => candidate.id === draft.dagDocument)
  if (document === undefined) return err({ kind: 'unknown-dag', document: draft.dagDocument })
  if (document.preparedDataset !== prepared.id) return err({ kind: 'dag-dataset-mismatch', name: document.name })
  if (document.current.validation.kind !== 'structurally-valid') return err({ kind: 'dag-not-validated', name: document.name })
  if (draft.treatment === null) return err({ kind: 'treatment-required' })
  if (draft.outcome === null) return err({ kind: 'outcome-required' })
  const treatmentNode = document.current.graph.nodes.find((node) => node.id === draft.treatment)
  const outcomeNode = document.current.graph.nodes.find((node) => node.id === draft.outcome)
  const binding = inspectDagStudyBinding(document, draft.treatment, draft.outcome)
  if (!binding.ok) return err({ kind: 'binding', problem: binding.error })
  if (treatmentNode === undefined || treatmentNode.kind === 'latent') {
    return err({ kind: 'latent-treatment', name: treatmentNode?.name ?? String(draft.treatment) })
  }
  if (outcomeNode === undefined || outcomeNode.kind === 'latent') {
    return err({ kind: 'latent-outcome', name: outcomeNode?.name ?? String(draft.outcome) })
  }
  if (draft.assignment.kind === null) return err({ kind: 'assignment-required' })
  const description = draft.assignment.description.trim()
  if (description === '') return err({ kind: 'assignment-description-required' })
  const dagBasis = dagBasisOf(document)
  if (draft.assignment.kind === 'randomised' && dagBasis !== 'experimental-design') return err({ kind: 'randomised-needs-experimental-dag', name: document.name })
  const rationale = (text: string): string | null => (text.trim() === '' ? null : text.trim())
  return ok({
    kind: 'study-specification',
    id: newStudyId(),
    createdAt: new Date().toISOString(),
    preparedDataset: prepared.id,
    dataset: document.dataset,
    dagDocument: document.id,
    dagRevision: document.current.id,
    dagName: document.name,
    treatment: { node: treatmentNode.id, column: treatmentNode.column, name: treatmentNode.name },
    outcome: { node: outcomeNode.id, column: outcomeNode.column, name: outcomeNode.name },
    estimand: draft.estimand === 'average-treatment-effect'
      ? { kind: 'average-treatment-effect', scale: 'additive' }
      : { kind: 'average-treatment-effect-on-treated', scale: 'additive', treatedValue: 1 },
    population: { kind: 'all-prepared-rows', observations: prepared.observations },
    dagBasis,
    assignment: { kind: draft.assignment.kind, description },
    designAssumptions: {
      consistency: { statement: CONSISTENCY_STATEMENT, rationale: rationale(draft.consistencyRationale) },
      noInterference: { statement: NO_INTERFERENCE_STATEMENT, rationale: rationale(draft.noInterferenceRationale) },
    },
    graph: studyGraphOf(document.current.graph),
    editableGraph: document.current.graph,
  })
}

const DESIGN_PROBLEMS: ReadonlySet<StudyDesignProblem['kind']> = new Set(['assignment-required', 'assignment-description-required', 'randomised-needs-experimental-dag'])

/** The bound graph, treatment and outcome before the design fields are complete, for role and path previews; never recorded. */
export function previewStudyBinding(draft: StudyDesignDraft, documents: readonly DagDocument[], prepared: PreparedDatasetArtifact): StudySpecification | null {
  const ready = readyStudySpecification(draft, documents, prepared)
  if (ready.ok) return ready.value
  if (!DESIGN_PROBLEMS.has(ready.error.kind)) return null
  const filled = readyStudySpecification({ ...draft, assignment: { kind: 'observed-choice', description: 'pending' } }, documents, prepared)
  return filled.ok ? filled.value : null
}

export const estimandSentence = (study: StudySpecification): string => {
  switch (study.estimand.kind) {
    case 'average-treatment-effect': return `Average effect of ${study.treatment.name} on ${study.outcome.name}`
    case 'average-treatment-effect-on-treated': return `Average effect of ${study.treatment.name} on ${study.outcome.name} among treated rows`
    default: return assertNever(study.estimand)
  }
}

/** The estimand in full: scale, what the paths contribute, and the population it averages over. */
export const describeEstimand = (study: StudySpecification): string => {
  const effect = 'the total effect through every causal path, mediators included, on the additive scale'
  switch (study.estimand.kind) {
    case 'average-treatment-effect': return `Average treatment effect of ${study.treatment.name} on ${study.outcome.name}: ${effect}, averaged over all ${study.population.observations} prepared rows.`
    case 'average-treatment-effect-on-treated': return `Average treatment effect on the treated of ${study.treatment.name} on ${study.outcome.name}: ${effect}, averaged over prepared rows with ${study.treatment.name} = 1.`
    default: return assertNever(study.estimand)
  }
}

/** The target quantity and the identifying functional are separate parts of the study record. */
export function identifiedExpression(study: StudySpecification, adjustmentSet: readonly StudyVariable[]): string {
  const t = study.treatment.name
  const y = study.outcome.name
  const z = adjustmentSet.map((variable) => variable.name).join(', ')
  switch (study.estimand.kind) {
    case 'average-treatment-effect':
      return adjustmentSet.length === 0
        ? `ATE(t₁,t₀) = E[${y} | ${t}=t₁] − E[${y} | ${t}=t₀]`
        : `ATE(t₁,t₀) = Σ_z {E[${y} | ${t}=t₁, Z=z] − E[${y} | ${t}=t₀, Z=z]} P(Z=z), Z={${z}}`
    case 'average-treatment-effect-on-treated':
      return adjustmentSet.length === 0
        ? `ATT = E[${y} | ${t}=1] − E[${y} | ${t}=0]`
        : `ATT = Σ_z {E[${y} | ${t}=1, Z=z] − E[${y} | ${t}=0, Z=z]} P(Z=z | ${t}=1), Z={${z}}`
    default: return assertNever(study.estimand)
  }
}

/** Study-design category implied by the recorded assignment mechanism and DAG basis. */
export function studyDesignCategory(study: StudySpecification): StudyDesignCategory {
  switch (study.assignment.kind) {
    case 'randomised': return study.dagBasis === 'experimental-design' ? 'experimental' : 'observational'
    case 'policy-change': return 'quasi-experimental'
    case 'observed-choice': return 'observational'
    default: return assertNever(study.assignment)
  }
}

export function describeAssignmentKind(kind: AssignmentMechanism['kind']): string {
  switch (kind) {
    case 'randomised': return 'Randomised'
    case 'policy-change': return 'Policy change'
    case 'observed-choice': return 'Observed choice'
    default: return assertNever(kind)
  }
}

export function describeStudyDesignCategory(category: StudyDesignCategory): string {
  switch (category) {
    case 'experimental': return 'Experimental design: implemented random assignment supports exchangeability; non-compliance, attrition and treatment versions still require assessment.'
    case 'quasi-experimental': return 'Quasi-experimental design: identification depends on the selected design\u2019s assumptions, such as parallel trends, exclusion or continuity.'
    case 'observational': return 'Observational design: back-door identification requires conditional exchangeability under the recorded graph, together with consistency, no interference and adequate overlap.'
    default: return assertNever(category)
  }
}

/** The façade's identification command, derived from the specification and nothing else. */
export function backdoorIdentificationCommand(study: StudySpecification): {
  readonly nodes: number
  readonly edges: readonly (readonly [number, number])[]
  readonly treatment: number
  readonly outcome: number
  readonly unobserved: readonly number[]
} {
  const position = (node: DagNodeId): number => study.graph.nodes.findIndex((candidate) => candidate.node === node)
  return {
    nodes: study.graph.nodes.length,
    edges: study.graph.edges,
    treatment: position(study.treatment.node),
    outcome: position(study.outcome.node),
    unobserved: study.graph.nodes.flatMap((node, index) => (node.column === null ? [index] : [])),
  }
}

const adjustmentIndexSetSchema = z.array(z.number().int().nonnegative())

export const backdoorIdentificationEvidenceSchema = z.object({
  kind: z.literal('backdoorIdentification'),
  nodes: z.number().int().min(2),
  treatment: z.number().int().nonnegative(),
  outcome: z.number().int().nonnegative(),
  unobserved: z.array(z.number().int().nonnegative()),
  result: z.discriminatedUnion('kind', [
    z.object({
      kind: z.literal('identified'),
      canonicalSet: adjustmentIndexSetSchema,
      minimalSets: z.tuple([adjustmentIndexSetSchema]).rest(adjustmentIndexSetSchema),
      truncated: z.boolean(),
    }).strict(),
    z.object({ kind: z.literal('notIdentified') }).strict(),
  ]),
}).strict()

export type BackdoorIdentificationEvidence = z.infer<typeof backdoorIdentificationEvidenceSchema>

export type BackdoorIdentificationEvidenceProblem = { readonly kind: 'invalid-backdoor-identification-evidence'; readonly detail: string }

export function parseBackdoorIdentificationEvidence(value: unknown): Result<BackdoorIdentificationEvidence, BackdoorIdentificationEvidenceProblem> {
  const parsed = backdoorIdentificationEvidenceSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-backdoor-identification-evidence', detail: z.prettifyError(parsed.error) })
  const adjustmentNodes = parsed.data.result.kind === 'identified'
    ? [...parsed.data.result.canonicalSet, ...parsed.data.result.minimalSets.flat()]
    : []
  const inRange = [parsed.data.treatment, parsed.data.outcome, ...parsed.data.unobserved, ...adjustmentNodes]
    .every((index) => index < parsed.data.nodes)
  if (!inRange) return err({ kind: 'invalid-backdoor-identification-evidence', detail: 'An index exceeds the node count.' })
  return ok(parsed.data)
}

export type IdentificationBasisEntry =
  | { readonly kind: 'graph-assumption'; readonly id: string; readonly statement: string }
  | { readonly kind: 'graph-result'; readonly id: string; readonly statement: string }
  /** One back-door path written with its arrows and the variable that blocks it: an adjustment variable, or a collider left alone. */
  | { readonly kind: 'backdoor-path'; readonly id: string; readonly statement: string; readonly nodes: readonly string[]; readonly arrows: readonly ('→' | '←')[]; readonly blockedAt: readonly string[]; readonly closure: 'adjustment' | 'collider' }
  | { readonly kind: 'design-record'; readonly id: string; readonly statement: string }
  | { readonly kind: 'design-assumption'; readonly id: string; readonly statement: string; readonly rationale: string | null }
  | { readonly kind: 'qualification'; readonly id: string; readonly statement: string }

export type IdentificationFailure =
  | { readonly kind: 'unmeasured-confounding'; readonly latent: NonEmptyArray<string> }
  | { readonly kind: 'open-backdoor-path'; readonly path: NonEmptyArray<string>; readonly through: readonly string[] }
  | { readonly kind: 'no-observed-backdoor-set' }

export type AdjustmentSetChoice =
  | { readonly kind: 'canonical' }
  | { readonly kind: 'minimal'; readonly ordinal: number }

export type AdjustmentSetSelection =
  | { readonly kind: 'canonical'; readonly variables: readonly StudyVariable[] }
  | { readonly kind: 'minimal'; readonly ordinal: number; readonly variables: readonly StudyVariable[] }

export type MinimalAdjustmentSetEnumeration =
  | { readonly kind: 'complete'; readonly sets: NonEmptyArray<readonly StudyVariable[]> }
  | { readonly kind: 'truncated'; readonly sets: NonEmptyArray<readonly StudyVariable[]> }

export type Identification =
  | {
      readonly kind: 'identified'
      readonly strategy: 'backdoor-adjustment'
      readonly adjustment: AdjustmentSetSelection
      readonly canonicalAdjustmentSet: readonly StudyVariable[]
      readonly minimalAdjustmentSets: MinimalAdjustmentSetEnumeration
      readonly mediators: readonly StudyGraphNode[]
      readonly basis: NonEmptyArray<IdentificationBasisEntry>
    }
  | { readonly kind: 'backdoor-not-identified'; readonly reasons: NonEmptyArray<IdentificationFailure> }

export interface IdentificationArtifact {
  readonly kind: 'identification'
  readonly id: IdentificationId
  readonly study: StudyId
  readonly createdAt: string
  readonly method: typeof BACKDOOR_IDENTIFICATION_METHOD_ID
  readonly evidence: BackdoorIdentificationEvidence
  readonly result: Identification
}

export type IdentificationConstructionProblem = {
  readonly kind: 'minimal-adjustment-set-out-of-range'
  readonly ordinal: number
  readonly available: number
}

/** The bound DAG's roles and paths for this study, from the contemporaneous graph. */
export const studyFlow = (study: StudySpecification): DagCausalFlow => analyseDagCausalFlow(study.editableGraph, study.treatment.node, study.outcome.node)

export type VariableRole = DagCausalRole

/** Every node the treatment can reach along directed arrows, lagged or not: anything here is moved by the intervention. */
export function treatmentDescendants(study: StudySpecification): ReadonlySet<DagNodeId> {
  const children = new Map<DagNodeId, DagNodeId[]>()
  for (const edge of study.editableGraph.edges) children.set(edge.cause, [...(children.get(edge.cause) ?? []), edge.effect])
  const reached = new Set<DagNodeId>()
  const frontier = [study.treatment.node]
  while (frontier.length > 0) {
    const current = frontier.pop()
    if (current === undefined) break
    for (const child of children.get(current) ?? []) {
      if (reached.has(child)) continue
      reached.add(child)
      frontier.push(child)
    }
  }
  return reached
}

/** Each variable's place relative to the treatment and outcome, for the inspector and the identification card. */
export function variableRoles(study: StudySpecification): readonly { readonly node: StudyGraphNode; readonly role: DagCausalRole }[] {
  const flow = studyFlow(study)
  return study.graph.nodes.map((node) => ({ node, role: flow.roles.get(node.node) ?? { kind: 'unrelated' } }))
}

export const describeVariableRole = (role: DagCausalRole): string => describeDagCausalRole(role)

/** Convert back-door evidence into a typed identification record with graph results, assumptions, and qualifications kept distinct. */
export function identificationFrom(
  study: StudySpecification,
  evidence: BackdoorIdentificationEvidence,
  choice: AdjustmentSetChoice = { kind: 'canonical' },
): Result<Identification, IdentificationConstructionProblem> {
  const latent = study.graph.nodes.filter((node) => node.column === null).map((node) => node.name)
  if (evidence.result.kind === 'notIdentified') {
    const flow = studyFlow(study)
    const name = (node: DagNodeId): string => study.graph.nodes.find((candidate) => candidate.node === node)?.name ?? String(node)
    const openPaths = flow.paths.flatMap((path): IdentificationFailure[] => (path.status.kind === 'open' || path.status.kind === 'open-through-unmeasured'
      ? [{ kind: 'open-backdoor-path', path: path.nodes.map(name) as unknown as NonEmptyArray<string>, through: path.status.kind === 'open-through-unmeasured' ? path.status.unmeasured.map(name) : [] }]
      : []))
    const reasons: IdentificationFailure[] = [
      ...(isNonEmpty(latent) ? [{ kind: 'unmeasured-confounding', latent } as const] : []),
      ...openPaths,
      { kind: 'no-observed-backdoor-set' },
    ]
    return ok({ kind: 'backdoor-not-identified', reasons: reasons as unknown as NonEmptyArray<IdentificationFailure> })
  }
  const variablesFrom = (indexes: readonly number[]): readonly StudyVariable[] => indexes.flatMap((index): StudyVariable[] => {
    const node = study.graph.nodes[index]
    return node === undefined || node.column === null ? [] : [{ node: node.node, column: node.column, name: node.name }]
  })
  const canonicalAdjustmentSet = variablesFrom(evidence.result.canonicalSet)
  const minimalSets = mapNonEmpty(evidence.result.minimalSets, variablesFrom)
  let adjustment: AdjustmentSetSelection
  switch (choice.kind) {
    case 'canonical':
      adjustment = { kind: 'canonical', variables: canonicalAdjustmentSet }
      break
    case 'minimal': {
      const variables = minimalSets[choice.ordinal]
      if (variables === undefined) return err({ kind: 'minimal-adjustment-set-out-of-range', ordinal: choice.ordinal, available: minimalSets.length })
      adjustment = { kind: 'minimal', ordinal: choice.ordinal, variables }
      break
    }
    default:
      return assertNever(choice)
  }
  const adjustmentSet = adjustment.variables
  const flow = studyFlow(study)
  const mediators = variableRoles(study).filter((entry) => entry.role.kind === 'mediator').map((entry) => entry.node)
  const setText = adjustmentSet.length === 0 ? 'nothing' : adjustmentSet.map((variable) => variable.name).join(', ')
  const treatment = study.treatment.name
  const outcome = study.outcome.name
  const nodeName = (node: DagNodeId): string => study.graph.nodes.find((candidate) => candidate.node === node)?.name ?? String(node)
  const indexOf = (node: DagNodeId): number => study.graph.nodes.findIndex((candidate) => candidate.node === node)
  const arrows = (nodes: readonly DagNodeId[]): ('→' | '←')[] =>
    nodes.slice(1).map((node, index) => {
      const from = indexOf(nodes[index] ?? node)
      const to = indexOf(node)
      return study.graph.edges.some(([cause, effect]) => cause === from && effect === to) ? '→' : '←'
    })
  const spell = (nodes: readonly DagNodeId[]): string => {
    const names = nodes.map(nodeName)
    const directions = arrows(nodes)
    return names.map((name, index) => (index === 0 ? name : ` ${directions[index - 1] ?? '–'} ${name}`)).join('')
  }
  const paths = flow.paths.flatMap((path, index): IdentificationBasisEntry[] => {
    if (path.status.kind === 'closed-by-adjustment') {
      const blockedAt = path.status.by.map(nodeName)
      return [{ kind: 'backdoor-path', id: `backdoor-path-${index}`, nodes: path.nodes.map(nodeName), arrows: arrows(path.nodes), blockedAt, closure: 'adjustment', statement: `Back-door path ${spell(path.nodes)} is blocked at ${blockedAt.join(', ')}.` }]
    }
    if (path.status.kind === 'closed-at-collider') {
      const blockedAt = path.status.colliders.map(nodeName)
      return [{ kind: 'backdoor-path', id: `backdoor-path-${index}`, nodes: path.nodes.map(nodeName), arrows: arrows(path.nodes), blockedAt, closure: 'collider', statement: `Path ${spell(path.nodes)} is blocked at collider ${blockedAt.join(', ')}, which is left alone.` }]
    }
    return []
  })
  const keptOut = variableRoles(study).flatMap((entry): IdentificationBasisEntry[] => {
    const role = entry.role
    const reason = role.kind === 'mediator'
      ? (role.alsoCollider ? 'a mediator and a collider' : 'a mediator; the estimand is the total effect')
      : role.kind === 'collider' ? 'a collider; adjusting would open a path'
      : role.kind === 'post-treatment' ? 'a descendant of the treatment'
      : null
    return reason === null ? [] : [{ kind: 'graph-result', id: `kept-out-${entry.node.name}`, statement: `${entry.node.name} is left out of the adjustment set: ${reason}.` }]
  })
  const category = studyDesignCategory(study)
  const basis: IdentificationBasisEntry[] = [
    { kind: 'design-record', id: 'assignment-mechanism', statement: `${describeAssignmentKind(study.assignment.kind)} treatment: ${study.assignment.description} ${describeStudyDesignCategory(category)}` },
    {
      kind: 'graph-assumption',
      id: 'graph-theory',
      statement: `The graph “${study.dagName}” (revision ${study.dagRevision.slice(0, 8)}) states the causes of ${treatment} and ${outcome}; a missing arrow claims there is no direct cause.`,
    },
    {
      kind: 'graph-assumption',
      id: 'no-unmeasured-confounding',
      statement: adjustmentSet.length === 0
        ? `No unmeasured common cause of ${treatment} and ${outcome} exists. Test sensitivity to this assumption in the sensitivity chapter.`
        : `No unmeasured common cause of ${treatment} and ${outcome} remains after adjusting for ${setText}. Test sensitivity to this assumption in the sensitivity chapter.`,
    },
    { kind: 'design-assumption', id: 'consistency', statement: study.designAssumptions.consistency.statement, rationale: study.designAssumptions.consistency.rationale },
    { kind: 'design-assumption', id: 'no-interference', statement: study.designAssumptions.noInterference.statement, rationale: study.designAssumptions.noInterference.rationale },
    {
      kind: 'graph-result',
      id: 'backdoor-criterion',
      statement: adjustmentSet.length === 0
        ? `No back-door path from ${treatment} to ${outcome} is open; the back-door criterion holds without adjustment.`
        : `Adjusting for ${setText} satisfies the back-door criterion for the total effect of ${treatment} on ${outcome}: ${paths.length === 1 ? 'the one back-door path is' : `all ${paths.length} back-door paths are`} blocked, and no mediator, collider or post-treatment variable is adjusted for.`,
    },
    ...keptOut,
    ...paths,
  ]
  if (study.graph.laggedArrows > 0) {
    basis.push({
      kind: 'qualification',
      id: 'lags-collapsed',
      statement: `Identification projects ${study.graph.laggedArrows} lagged arrow${study.graph.laggedArrows === 1 ? '' : 's'} onto the variable-level graph. The adjustment set uses same-period variables.`,
    })
  }
  return ok({
    kind: 'identified',
    strategy: 'backdoor-adjustment',
    adjustment,
    canonicalAdjustmentSet,
    minimalAdjustmentSets: evidence.result.truncated
      ? { kind: 'truncated', sets: minimalSets }
      : { kind: 'complete', sets: minimalSets },
    mediators,
    basis: basis as unknown as NonEmptyArray<IdentificationBasisEntry>,
  })
}

export const selectedAdjustmentSet = (identification: Extract<Identification, { kind: 'identified' }>): readonly StudyVariable[] => identification.adjustment.variables

export function describeStudyDesignProblem(problem: StudyDesignProblem): string {
  switch (problem.kind) {
    case 'dag-required': return 'Choose a validated causal graph.'
    case 'unknown-dag': return 'The chosen causal graph no longer exists. Select another graph.'
    case 'dag-not-validated': return `Resolve the structural issues in “${problem.name}” in the DAG workspace.`
    case 'dag-dataset-mismatch': return `“${problem.name}” uses a different prepared dataset. Select a matching graph.`
    case 'treatment-required': return 'Choose the treatment variable.'
    case 'outcome-required': return 'Choose the outcome variable.'
    case 'latent-treatment': return `${problem.name} is unmeasured. Choose a measured treatment.`
    case 'latent-outcome': return `${problem.name} is unmeasured. Choose a measured outcome.`
    case 'binding': return describeBindingProblem(problem.problem)
    case 'assignment-required': return 'Record the treatment-assignment mechanism: randomised, policy change, or observed choice.'
    case 'assignment-description-required': return 'Describe who or what assigned the treatment, and when.'
    case 'randomised-needs-experimental-dag': return `“${problem.name}” was drawn from domain knowledge or discovery, not an experimental design, so the treatment cannot be recorded as randomised.`
    default: return assertNever(problem)
  }
}

const describeBindingProblem = (problem: DagStudyBindingProblem): string => {
  switch (problem.kind) {
    case 'unknown-treatment': return 'The treatment is not in the chosen graph. Select another treatment or revise the graph.'
    case 'unknown-outcome': return 'The outcome is not in the chosen graph. Select another outcome or revise the graph.'
    case 'same-treatment-and-outcome': return 'The treatment and the outcome must be different variables.'
    case 'no-causal-path': return 'The graph encodes no directed causal path from the treatment to the outcome, so its total effect is zero. Check that the selected variables and graph represent the intended data-generating process.'
    default: return assertNever(problem)
  }
}

export function describeIdentificationFailure(failure: IdentificationFailure): string {
  switch (failure.kind) {
    case 'unmeasured-confounding': return `${failure.latent.join(', ')} ${failure.latent.length === 1 ? 'is' : 'are'} unmeasured, so the back-door criterion cannot be satisfied with the recorded columns.`
    case 'open-backdoor-path': return `The back-door path ${failure.path.join(' – ')} remains open${failure.through.length > 0 ? ` because ${failure.through.join(', ')} ${failure.through.length === 1 ? 'is' : 'are'} unmeasured` : ''}.`
    case 'no-observed-backdoor-set': return 'No measured adjustment set blocks every back-door path in this graph. Front-door and instrumental-variable identification were not assessed.'
    default: return assertNever(failure)
  }
}
