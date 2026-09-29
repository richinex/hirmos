import { z } from 'zod'
import type { DagDocument, DagDocumentId, DagNodeId, DagOriginChoice, DagRevisionId, EditableDag } from './dag'
import { analyseDagCausalFlow, describeDagCausalRole, type DagCausalFlow, type DagCausalRole } from './dagFlow'
import { inspectDagStudyBinding, type DagStudyBindingProblem } from './dagValidation'
import type { ColumnId } from './dataset'
import { assertNever, brand, err, isNonEmpty, mapNonEmpty, ok, type Brand, type NonEmptyArray, type Result } from './dop'
import { BACKDOOR_IDENTIFICATION_METHOD_ID, COUNTERFACTUAL_IDENTIFICATION_METHOD_ID, GRAPHICAL_IDENTIFICATION_METHOD_ID, RD_DESIGN_METHOD_ID, type MethodSource } from './methods'
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

/** How an effect modifier's values become the groups a conditional effect is averaged within. */
export type ModifierGrouping =
  | { readonly kind: 'levels' }
  | { readonly kind: 'quantiles'; readonly bins: number }

/** What a reader declares about a covariate before a run. */
export type CovariateEncoding =
  | { readonly kind: 'numeric' }
  | { readonly kind: 'categorical' }

/** The columns a declared encoding expands into. */
export type DesignLayout =
  | { readonly kind: 'numeric' }
  /** One column per level, as `pd.get_dummies` gives a classifier. */
  | { readonly kind: 'indicators' }
  /** The first level is the baseline and is dropped, as patsy's `C(x)` gives a regression. */
  | { readonly kind: 'treatment-contrast' }

export type Estimand =
  | { readonly kind: 'local-cutoff-effect'; readonly scale: 'additive'; readonly running: StudyVariable; readonly cutoff: number; readonly assignment: 'at-or-above' }
  | { readonly kind: 'average-treatment-effect'; readonly scale: 'additive' }
  | { readonly kind: 'average-treatment-effect-on-treated'; readonly scale: 'additive'; readonly treatedValue: 1 }
  | { readonly kind: 'average-treatment-effect-on-controls'; readonly scale: 'additive' }
  | { readonly kind: 'overlap-weighted-average-treatment-effect'; readonly scale: 'additive' }
  | { readonly kind: 'average-partial-effect'; readonly scale: 'additive' }
  | { readonly kind: 'variance-weighted-average-partial-effect'; readonly scale: 'additive' }
  | { readonly kind: 'conditional-partial-effect-per-row'; readonly scale: 'additive'; readonly modifiers: readonly StudyVariable[] }
  /** The average effect within each group of an effect modifier: an ATE stratified by a variable other than the treatment. */
  | { readonly kind: 'conditional-average-treatment-effect'; readonly scale: 'additive'; readonly modifier: StudyVariable; readonly grouping: ModifierGrouping }
  /** One effect per prepared row: the average treatment contrast conditioned on that row's values of the adjustment
   *  variables and of any effect modifiers named here, variables the treatment does not reach that the effect may vary with. */
  | { readonly kind: 'conditional-average-treatment-effect-per-row'; readonly scale: 'additive'; readonly modifiers: readonly StudyVariable[] }

export const QUANTILE_GROUP_CHOICES: readonly number[] = [2, 3, 4, 5]

export function describeGrouping(grouping: ModifierGrouping): string {
  switch (grouping.kind) {
    case 'levels': return 'each value'
    case 'quantiles': return `${grouping.bins} quantile groups`
    default: return assertNever(grouping)
  }
}

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

/** What a design assumption says and where it comes from; the study records the reader's rationale against it. */
export interface DesignAssumptionReference {
  readonly id: 'consistency' | 'no-interference'
  readonly name: string
  readonly statement: string
  readonly sources: NonEmptyArray<MethodSource>
}

const HERNAN_ROBINS_CH1: MethodSource = { kind: 'paper', title: 'Causal Inference: What If (Hern\u00e1n and Robins, 2020)', locator: 'chapter 1, consistency and no interference' }

export const DESIGN_ASSUMPTIONS: NonEmptyArray<DesignAssumptionReference> = [
  {
    id: 'consistency',
    name: 'Consistency',
    statement: CONSISTENCY_STATEMENT,
    sources: [
      { kind: 'paper', title: 'Concerning the consistency assumption in causal inference (VanderWeele, 2009)', locator: 'Epidemiology 20(6); doi:10.1097/EDE.0b013e3181bd5638' },
      HERNAN_ROBINS_CH1,
    ],
  },
  {
    id: 'no-interference',
    name: 'No interference',
    statement: NO_INTERFERENCE_STATEMENT,
    sources: [HERNAN_ROBINS_CH1],
  },
]

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
  readonly cutoff?: { readonly variable: DagNodeId | null; readonly value: string }
  readonly dagDocument: DagDocumentId | null
  readonly treatment: DagNodeId | null
  readonly outcome: DagNodeId | null
  readonly estimand: Estimand['kind']
  /** The effect modifier a conditional effect is grouped by; read only for that target. */
  readonly modifier: DagNodeId | null
  readonly grouping: ModifierGrouping
  /** The effect modifiers a per-row effect is conditioned on beside the adjustment set; read only for that target. */
  readonly modifiers: readonly DagNodeId[]
  readonly assignment: { readonly kind: AssignmentMechanism['kind'] | null; readonly description: string }
  readonly consistencyRationale: string
  readonly noInterferenceRationale: string
}

export type StudyDesignProblem =
  | { readonly kind: 'cutoff-required' }
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
  | { readonly kind: 'modifier-required' }
  | { readonly kind: 'latent-modifier'; readonly name: string }
  | { readonly kind: 'modifier-is-endpoint'; readonly name: string }
  | { readonly kind: 'modifier-after-treatment'; readonly name: string; readonly role: string }

export const EMPTY_STUDY_DRAFT: StudyDesignDraft = { dagDocument: null, treatment: null, outcome: null, estimand: 'average-treatment-effect', modifier: null, grouping: { kind: 'quantiles', bins: 3 }, modifiers: [], assignment: { kind: null, description: '' }, consistencyRationale: '', noInterferenceRationale: '' }

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
    if (from === to) continue
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
  if (document.current.validation.structure.kind !== 'sound') return err({ kind: 'dag-not-validated', name: document.name })
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
  const estimand = estimandOf(draft, document, treatmentNode.id, outcomeNode.id)
  if (!estimand.ok) return estimand
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
    estimand: estimand.value,
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

/** An effect modifier from the graph: observed, neither endpoint, and not reached by the treatment. */
function modifierVariable(document: DagDocument, treatment: DagNodeId, outcome: DagNodeId, id: DagNodeId): Result<StudyVariable, StudyDesignProblem> {
  const node = document.current.graph.nodes.find((candidate) => candidate.id === id)
  if (node === undefined || node.kind === 'latent' || node.column === null) return err({ kind: 'latent-modifier', name: node?.name ?? String(id) })
  if (node.id === treatment || node.id === outcome) return err({ kind: 'modifier-is-endpoint', name: node.name })
  const role = analyseDagCausalFlow(document.current.graph, treatment, outcome).roles.get(node.id) ?? { kind: 'unrelated' }
  if (role.kind === 'mediator' || role.kind === 'collider' || role.kind === 'post-treatment') return err({ kind: 'modifier-after-treatment', name: node.name, role: describeDagCausalRole(role) })
  return ok({ node: node.id, column: node.column, name: node.name })
}

/** The target from the draft; a conditional effect needs observed modifiers that the treatment does not reach. */
function estimandOf(draft: StudyDesignDraft, document: DagDocument, treatment: DagNodeId, outcome: DagNodeId): Result<Estimand, StudyDesignProblem> {
  switch (draft.estimand) {
    case 'local-cutoff-effect': {
      if (draft.cutoff?.variable == null || draft.cutoff.value.trim() === '' || !Number.isFinite(Number(draft.cutoff.value))) return err({ kind: 'cutoff-required' })
      const running = modifierVariable(document, treatment, outcome, draft.cutoff.variable)
      if (!running.ok) return running
      return ok({ kind: 'local-cutoff-effect', scale: 'additive', running: running.value, cutoff: Number(draft.cutoff.value), assignment: 'at-or-above' })
    }
    case 'average-treatment-effect': return ok({ kind: 'average-treatment-effect', scale: 'additive' })
    case 'average-treatment-effect-on-treated': return ok({ kind: 'average-treatment-effect-on-treated', scale: 'additive', treatedValue: 1 })
    case 'average-treatment-effect-on-controls':
    case 'overlap-weighted-average-treatment-effect':
    case 'average-partial-effect':
    case 'variance-weighted-average-partial-effect': return ok({ kind: draft.estimand, scale: 'additive' })
    case 'conditional-average-treatment-effect': {
      if (draft.modifier === null) return err({ kind: 'modifier-required' })
      const modifier = modifierVariable(document, treatment, outcome, draft.modifier)
      if (!modifier.ok) return modifier
      return ok({ kind: 'conditional-average-treatment-effect', scale: 'additive', modifier: modifier.value, grouping: draft.grouping })
    }
    case 'conditional-partial-effect-per-row':
    case 'conditional-average-treatment-effect-per-row': {
      const modifiers: StudyVariable[] = []
      for (const id of draft.modifiers) {
        const modifier = modifierVariable(document, treatment, outcome, id)
        if (!modifier.ok) return modifier
        modifiers.push(modifier.value)
      }
      return ok({ kind: draft.estimand, scale: 'additive', modifiers })
    }
    default: return assertNever(draft.estimand)
  }
}

const DESIGN_PROBLEMS: ReadonlySet<StudyDesignProblem['kind']> = new Set(['cutoff-required', 'assignment-required', 'assignment-description-required', 'randomised-needs-experimental-dag', 'modifier-required', 'latent-modifier', 'modifier-is-endpoint', 'modifier-after-treatment'])

/** The bound graph, treatment and outcome before the design fields are complete, for role and path previews; never recorded. */
export function previewStudyBinding(draft: StudyDesignDraft, documents: readonly DagDocument[], prepared: PreparedDatasetArtifact): StudySpecification | null {
  const ready = readyStudySpecification(draft, documents, prepared)
  if (ready.ok) return ready.value
  if (!DESIGN_PROBLEMS.has(ready.error.kind)) return null
  // The preview only needs the binding, so the target falls back to the plain average while the modifier is unsettled.
  const filled = readyStudySpecification({ ...draft, estimand: 'average-treatment-effect', assignment: { kind: 'observed-choice', description: 'pending' } }, documents, prepared)
  return filled.ok ? filled.value : null
}

export const estimandSentence = (study: StudySpecification): string => {
  switch (study.estimand.kind) {
    case 'local-cutoff-effect': return `Local effect of ${study.treatment.name} on ${study.outcome.name} at ${study.estimand.running.name} = ${study.estimand.cutoff}`
    case 'average-treatment-effect': return `Average effect of ${study.treatment.name} on ${study.outcome.name}`
    case 'average-treatment-effect-on-treated': return `Average effect of ${study.treatment.name} on ${study.outcome.name} among treated rows`
    case 'average-treatment-effect-on-controls': return `Average effect of ${study.treatment.name} on ${study.outcome.name} among control rows`
    case 'overlap-weighted-average-treatment-effect': return `Overlap-weighted effect of ${study.treatment.name} on ${study.outcome.name}`
    case 'average-partial-effect': return `Average partial effect of ${study.treatment.name} on ${study.outcome.name}`
    case 'variance-weighted-average-partial-effect': return `Variance-weighted partial effect of ${study.treatment.name} on ${study.outcome.name}`
    case 'conditional-partial-effect-per-row': return `Conditional partial effect of ${study.treatment.name} on ${study.outcome.name}`
    case 'conditional-average-treatment-effect': return `Effect of ${study.treatment.name} on ${study.outcome.name} within groups of ${study.estimand.modifier.name}`
    case 'conditional-average-treatment-effect-per-row': return `Effect of ${study.treatment.name} on ${study.outcome.name} for each row`
    default: return assertNever(study.estimand)
  }
}

/** The estimand in full: scale, what the paths contribute, and the population it averages over. */
export const describeEstimand = (study: StudySpecification): string => {
  const effect = 'the total effect through every causal path, mediators included, on the additive scale'
  switch (study.estimand.kind) {
    case 'local-cutoff-effect': return `The additive treatment effect at ${study.estimand.running.name} = ${study.estimand.cutoff}, where treatment switches from 0 below the cutoff to 1 at or above it. This is not an average effect over all prepared rows.`
    case 'average-treatment-effect': return `Average treatment effect of ${study.treatment.name} on ${study.outcome.name}: ${effect}, averaged over all ${study.population.observations} prepared rows.`
    case 'average-treatment-effect-on-treated': return `Average treatment effect on the treated of ${study.treatment.name} on ${study.outcome.name}: ${effect}, averaged over prepared rows with ${study.treatment.name} = 1.`
    case 'average-treatment-effect-on-controls': return `Average the binary treatment contrast among rows with ${study.treatment.name} = 0.`
    case 'overlap-weighted-average-treatment-effect': return 'Average the binary treatment contrast with weights proportional to e(X)(1 − e(X)), where e(X) is the treatment probability given the covariates.'
    case 'average-partial-effect': return 'Average the conditional outcome-treatment covariance divided by conditional treatment variance. A causal slope interpretation requires unconfoundedness and the treatment model assumptions.'
    case 'variance-weighted-average-partial-effect': return 'Average conditional partial effects with weights proportional to conditional treatment variance. This is not an equally weighted population average.'
    case 'conditional-partial-effect-per-row': return 'Estimate the conditional outcome-treatment covariance divided by conditional treatment variance at each row’s covariate values. These are slopes, not arbitrary treatment contrasts.'
    case 'conditional-average-treatment-effect': return `Conditional average treatment effect of ${study.treatment.name} on ${study.outcome.name}: ${effect}, averaged within each group of ${study.estimand.modifier.name} (${describeGrouping(study.estimand.grouping)}) over the ${study.population.observations} prepared rows.`
    case 'conditional-average-treatment-effect-per-row': {
      const modifiers = study.estimand.modifiers.map((variable) => variable.name)
      const conditioned = modifiers.length === 0 ? 'the adjustment variables' : `the adjustment variables and ${modifiers.join(', ')}`
      return `Conditional average treatment effect of ${study.treatment.name} on ${study.outcome.name} for each row: ${effect}, conditioned on the row's values of ${conditioned}, reported for every one of the ${study.population.observations} prepared rows.`
    }
    default: return assertNever(study.estimand)
  }
}

/** The target quantity and the identifying functional are separate parts of the study record. */
export function identifiedExpression(study: StudySpecification, adjustmentSet: readonly StudyVariable[]): string {
  const t = study.treatment.name
  const y = study.outcome.name
  const z = adjustmentSet.map((variable) => variable.name).join(', ')
  switch (study.estimand.kind) {
    case 'local-cutoff-effect': return `τ(${study.estimand.cutoff}) = lim(x↓c) E[${y} | ${study.estimand.running.name}=x] − lim(x↑c) E[${y} | ${study.estimand.running.name}=x], c=${study.estimand.cutoff}; under continuity and sharp assignment.`
    case 'average-treatment-effect-on-controls': return `ATC = E[τ(Z) | ${t}=0], τ(z)=E[${y}|${t}=1,Z=z]−E[${y}|${t}=0,Z=z], Z={${z}}`
    case 'overlap-weighted-average-treatment-effect': return `E[e(Z)(1−e(Z))τ(Z)] / E[e(Z)(1−e(Z))], e(z)=P(${t}=1|Z=z), τ(z)=E[${y}|${t}=1,Z=z]−E[${y}|${t}=0,Z=z], Z={${z}}`
    case 'average-partial-effect': return `E[β(Z)], β(z)=Cov(${y},${t}|Z=z)/Var(${t}|Z=z), Z={${z}}; causal interpretation additionally requires the treatment model assumptions.`
    case 'variance-weighted-average-partial-effect': return `E[Var(${t}|Z)β(Z)]/E[Var(${t}|Z)], β(z)=Cov(${y},${t}|Z=z)/Var(${t}|Z=z), Z={${z}}`
    case 'conditional-partial-effect-per-row': return `β(x)=Cov(${y},${t}|X=x)/Var(${t}|X=x), X contains the adjustment variables and recorded effect modifiers; causal interpretation additionally requires the treatment model assumptions.`
    case 'average-treatment-effect':
      return adjustmentSet.length === 0
        ? `ATE(t₁,t₀) = E[${y} | ${t}=t₁] − E[${y} | ${t}=t₀]`
        : `ATE(t₁,t₀) = Σ_z {E[${y} | ${t}=t₁, Z=z] − E[${y} | ${t}=t₀, Z=z]} P(Z=z), Z={${z}}`
    case 'average-treatment-effect-on-treated':
      return adjustmentSet.length === 0
        ? `ATT = E[${y} | ${t}=1] − E[${y} | ${t}=0]`
        : `ATT = Σ_z {E[${y} | ${t}=1, Z=z] − E[${y} | ${t}=0, Z=z]} P(Z=z | ${t}=1), Z={${z}}`
    case 'conditional-average-treatment-effect': {
      const m = study.estimand.modifier.name
      return adjustmentSet.length === 0
        ? `CATE(t₁,t₀ | ${m}=x) = E[${y} | ${t}=t₁, ${m}=x] − E[${y} | ${t}=t₀, ${m}=x]`
        : `CATE(t₁,t₀ | ${m}=x) = Σ_z {E[${y} | ${t}=t₁, Z=z, ${m}=x] − E[${y} | ${t}=t₀, Z=z, ${m}=x]} P(Z=z | ${m}=x), Z={${z}}`
    }
    case 'conditional-average-treatment-effect-per-row': {
      const conditioned = [...adjustmentSet, ...study.estimand.modifiers.filter((modifier) => !adjustmentSet.some((variable) => variable.column === modifier.column))].map((variable) => variable.name).join(', ')
      return conditioned.length === 0
        ? `CATE(t₁,t₀) = E[${y} | ${t}=t₁] − E[${y} | ${t}=t₀], with no variable to condition on, so the same for every row`
        : `CATE(t₁,t₀ | Z=z) = E[${y} | ${t}=t₁, Z=z] − E[${y} | ${t}=t₀, Z=z] at each row's z, Z={${conditioned}}`
    }
    default: return assertNever(study.estimand)
  }
}

/** A variable name inside TeX: upright text, with TeX's special characters escaped. */
const texName = (name: string): string =>
  `\\text{${name.replace(/[\\{}$&#_%^~]/g, (char) => (char === '\\' ? '\\textbackslash{}' : char === '^' || char === '~' ? `\\${char}{}` : `\\${char}`))}}`

/** The identified expression as TeX for typesetting; `identifiedExpression` stays the plain-text record. */
export function identifiedExpressionTex(study: StudySpecification, adjustmentSet: readonly StudyVariable[]): string {
  const t = texName(study.treatment.name)
  const y = texName(study.outcome.name)
  const z = adjustmentSet.map((variable) => texName(variable.name)).join(', ')
  switch (study.estimand.kind) {
    case 'local-cutoff-effect': return String.raw`\tau(c)=\lim_{x\downarrow c}\mathbb{E}[${y}\mid ${texName(study.estimand.running.name)}=x]-\lim_{x\uparrow c}\mathbb{E}[${y}\mid ${texName(study.estimand.running.name)}=x],\quad c=${study.estimand.cutoff}`
    case 'average-treatment-effect-on-controls': return String.raw`\mathrm{ATC}=\mathbb{E}[\tau(Z)\mid ${t}=0],\quad \tau(z)=\mathbb{E}[${y}\mid ${t}=1,Z=z]-\mathbb{E}[${y}\mid ${t}=0,Z=z]`
    case 'overlap-weighted-average-treatment-effect': return String.raw`\frac{\mathbb{E}[e(Z)(1-e(Z))\tau(Z)]}{\mathbb{E}[e(Z)(1-e(Z))]},\quad e(z)=P(${t}=1\mid Z=z)`
    case 'average-partial-effect': return String.raw`\mathbb{E}[\beta(Z)],\quad \beta(z)=\frac{\operatorname{Cov}(${y},${t}\mid Z=z)}{\operatorname{Var}(${t}\mid Z=z)}`
    case 'variance-weighted-average-partial-effect': return String.raw`\frac{\mathbb{E}[\operatorname{Var}(${t}\mid Z)\beta(Z)]}{\mathbb{E}[\operatorname{Var}(${t}\mid Z)]},\quad \beta(z)=\frac{\operatorname{Cov}(${y},${t}\mid Z=z)}{\operatorname{Var}(${t}\mid Z=z)}`
    case 'conditional-partial-effect-per-row': return String.raw`\beta(x)=\frac{\operatorname{Cov}(${y},${t}\mid X=x)}{\operatorname{Var}(${t}\mid X=x)}`
    case 'average-treatment-effect':
      return adjustmentSet.length === 0
        ? String.raw`\mathrm{ATE}(t_1,t_0) = \mathbb{E}[${y} \mid ${t}=t_1] - \mathbb{E}[${y} \mid ${t}=t_0]`
        : String.raw`\mathrm{ATE}(t_1,t_0) = \sum_{z} \bigl\{\mathbb{E}[${y} \mid ${t}=t_1, Z=z] - \mathbb{E}[${y} \mid ${t}=t_0, Z=z]\bigr\}\, P(Z=z),\allowbreak\quad Z=\{${z}\}`
    case 'average-treatment-effect-on-treated':
      return adjustmentSet.length === 0
        ? String.raw`\mathrm{ATT} = \mathbb{E}[${y} \mid ${t}=1] - \mathbb{E}[${y} \mid ${t}=0]`
        : String.raw`\mathrm{ATT} = \sum_{z} \bigl\{\mathbb{E}[${y} \mid ${t}=1, Z=z] - \mathbb{E}[${y} \mid ${t}=0, Z=z]\bigr\}\, P(Z=z \mid ${t}=1),\allowbreak\quad Z=\{${z}\}`
    case 'conditional-average-treatment-effect': {
      const m = texName(study.estimand.modifier.name)
      return adjustmentSet.length === 0
        ? String.raw`\mathrm{CATE}(t_1,t_0 \mid ${m}=x) = \mathbb{E}[${y} \mid ${t}=t_1, ${m}=x] - \mathbb{E}[${y} \mid ${t}=t_0, ${m}=x]`
        : String.raw`\mathrm{CATE}(t_1,t_0 \mid ${m}=x) = \sum_{z} \bigl\{\mathbb{E}[${y} \mid ${t}=t_1, Z=z, ${m}=x] - \mathbb{E}[${y} \mid ${t}=t_0, Z=z, ${m}=x]\bigr\}\, P(Z=z \mid ${m}=x),\allowbreak\quad Z=\{${z}\}`
    }
    case 'conditional-average-treatment-effect-per-row': {
      const conditioned = [...adjustmentSet, ...study.estimand.modifiers.filter((modifier) => !adjustmentSet.some((variable) => variable.column === modifier.column))].map((variable) => texName(variable.name)).join(', ')
      return conditioned.length === 0
        ? String.raw`\mathrm{CATE}(t_1,t_0) = \mathbb{E}[${y} \mid ${t}=t_1] - \mathbb{E}[${y} \mid ${t}=t_0]`
        : String.raw`\mathrm{CATE}(t_1,t_0 \mid Z=z) = \mathbb{E}[${y} \mid ${t}=t_1, Z=z] - \mathbb{E}[${y} \mid ${t}=t_0, Z=z],\allowbreak\quad Z=\{${conditioned}\}`
    }
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
  readonly names: readonly string[]
  readonly edges: readonly (readonly [number, number])[]
  readonly treatment: number
  readonly outcome: number
  readonly unobserved: readonly number[]
  readonly estimand: 'ate' | 'att'
} {
  const position = (node: DagNodeId): number => study.graph.nodes.findIndex((candidate) => candidate.node === node)
  return {
    nodes: study.graph.nodes.length,
    names: study.graph.nodes.map((node) => node.name),
    edges: study.graph.edges,
    treatment: position(study.treatment.node),
    outcome: position(study.outcome.node),
    unobserved: study.graph.nodes.flatMap((node, index) => (node.column === null ? [index] : [])),
    // A conditional effect is identified the way the average is; only the treated target needs the counterfactual search.
    estimand: study.estimand.kind === 'average-treatment-effect-on-treated' ? 'att' : 'ate',
  }
}

const adjustmentIndexSetSchema = z.array(z.number().int().nonnegative())
const projectedEdgeSchema = z.tuple([z.number().int().nonnegative(), z.number().int().nonnegative()])
const latentProjectionEvidenceSchema = z.object({
  directedEdges: z.array(projectedEdgeSchema),
  bidirectedEdges: z.array(projectedEdgeSchema),
}).strict()

const graphicalIdentificationEvidenceSchema = z.discriminatedUnion('kind', [
  z.object({
    kind: z.literal('identified'),
    expression: z.string().min(1),
    latex: z.string().min(1),
    projection: latentProjectionEvidenceSchema,
  }).strict(),
  z.object({
    kind: z.literal('unidentifiable'),
    hedgeGraph: z.array(z.number().int().nonnegative()).min(1),
    hedgeSubgraph: z.array(z.number().int().nonnegative()).min(1),
    projection: latentProjectionEvidenceSchema,
  }).strict(),
])
export type GraphicalIdentificationEvidence = z.infer<typeof graphicalIdentificationEvidenceSchema>

const frontdoorSetEvidenceSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('identified'), mediators: z.array(z.number().int().nonnegative()).min(1) }).strict(),
  z.object({ kind: z.literal('notIdentified') }).strict(),
])

const instrumentSetEvidenceSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('identified'), instruments: z.array(z.number().int().nonnegative()).min(1) }).strict(),
  z.object({ kind: z.literal('notIdentified') }).strict(),
])

const counterfactualIdentificationEvidenceSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('notApplicable') }).strict(),
  z.object({
    kind: z.literal('identified'),
    treatedExpression: z.string().min(1),
    untreatedExpression: z.string().min(1),
  }).strict(),
  z.object({ kind: z.literal('unidentifiable'), reason: z.string().min(1) }).strict(),
])

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
  frontdoor: frontdoorSetEvidenceSchema,
  instruments: instrumentSetEvidenceSchema,
  graphicalIdentification: graphicalIdentificationEvidenceSchema,
  counterfactualIdentification: counterfactualIdentificationEvidenceSchema,
}).strict()

export type BackdoorIdentificationEvidence = z.infer<typeof backdoorIdentificationEvidenceSchema>

export type BackdoorIdentificationEvidenceProblem = { readonly kind: 'invalid-backdoor-identification-evidence'; readonly detail: string }

export function parseBackdoorIdentificationEvidence(value: unknown): Result<BackdoorIdentificationEvidence, BackdoorIdentificationEvidenceProblem> {
  const parsed = backdoorIdentificationEvidenceSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-backdoor-identification-evidence', detail: z.prettifyError(parsed.error) })
  const adjustmentNodes = parsed.data.result.kind === 'identified'
    ? [...parsed.data.result.canonicalSet, ...parsed.data.result.minimalSets.flat()]
    : []
  const graphicalNodes = parsed.data.graphicalIdentification.kind === 'unidentifiable'
    ? [...parsed.data.graphicalIdentification.hedgeGraph, ...parsed.data.graphicalIdentification.hedgeSubgraph]
    : []
  const frontdoorNodes = parsed.data.frontdoor.kind === 'identified' ? parsed.data.frontdoor.mediators : []
  const instrumentNodes = parsed.data.instruments.kind === 'identified' ? parsed.data.instruments.instruments : []
  const projectedNodes = [
    ...parsed.data.graphicalIdentification.projection.directedEdges.flat(),
    ...parsed.data.graphicalIdentification.projection.bidirectedEdges.flat(),
  ]
  const inRange = [parsed.data.treatment, parsed.data.outcome, ...parsed.data.unobserved, ...adjustmentNodes, ...graphicalNodes, ...frontdoorNodes, ...instrumentNodes, ...projectedNodes]
    .every((index) => index < parsed.data.nodes)
  if (!inRange) return err({ kind: 'invalid-backdoor-identification-evidence', detail: 'An index exceeds the node count.' })
  if (parsed.data.frontdoor.kind === 'identified') {
    const mediators = new Set(parsed.data.frontdoor.mediators)
    const unobserved = new Set(parsed.data.unobserved)
    if (mediators.size !== parsed.data.frontdoor.mediators.length) return err({ kind: 'invalid-backdoor-identification-evidence', detail: 'The front-door mediator set contains a duplicate node.' })
    if (mediators.has(parsed.data.treatment) || mediators.has(parsed.data.outcome)) return err({ kind: 'invalid-backdoor-identification-evidence', detail: 'A front-door mediator must differ from the treatment and outcome.' })
    if (parsed.data.frontdoor.mediators.some((index) => unobserved.has(index))) return err({ kind: 'invalid-backdoor-identification-evidence', detail: 'A front-door mediator must be observed.' })
  }
  if (parsed.data.instruments.kind === 'identified') {
    const instruments = new Set(parsed.data.instruments.instruments)
    const unobserved = new Set(parsed.data.unobserved)
    if (instruments.size !== parsed.data.instruments.instruments.length) return err({ kind: 'invalid-backdoor-identification-evidence', detail: 'The instrument set contains a duplicate node.' })
    if (instruments.has(parsed.data.treatment) || instruments.has(parsed.data.outcome)) return err({ kind: 'invalid-backdoor-identification-evidence', detail: 'An instrument must differ from the treatment and outcome.' })
    if (parsed.data.instruments.instruments.some((index) => unobserved.has(index))) return err({ kind: 'invalid-backdoor-identification-evidence', detail: 'An instrument must be observed.' })
  }
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
  | { readonly kind: 'id-hedge'; readonly graph: NonEmptyArray<string>; readonly subgraph: NonEmptyArray<string> }
  | { readonly kind: 'att-requires-counterfactual-identification'; readonly estimand: 'ATT' }

export type AdjustmentSetChoice =
  | { readonly kind: 'canonical' }
  | { readonly kind: 'minimal'; readonly ordinal: number }

export type IdentifiedBackdoor = Extract<BackdoorIdentificationEvidence['result'], { readonly kind: 'identified' }>

const sameIndexSet = (left: readonly number[], right: readonly number[]): boolean => {
  const a = [...left].sort((x, y) => x - y)
  const b = [...right].sort((x, y) => x - y)
  return a.length === b.length && a.every((value, index) => value === b[index])
}

/**
 * Whether identification leaves a choice of adjustment set to make: several minimal sets, or a
 * canonical set that adds outcome predictors to the only minimal set. With no choice the minimal set
 * is the identification; with one, the study is recorded only once the set is chosen.
 */
export const offersAdjustmentChoice = (result: IdentifiedBackdoor): boolean =>
  result.minimalSets.length > 1 || !sameIndexSet(result.minimalSets[0], result.canonicalSet)

export type AdjustmentSetSelection =
  | { readonly kind: 'canonical'; readonly variables: readonly StudyVariable[] }
  | { readonly kind: 'minimal'; readonly ordinal: number; readonly variables: readonly StudyVariable[] }

export type MinimalAdjustmentSetEnumeration =
  | { readonly kind: 'complete'; readonly sets: NonEmptyArray<readonly StudyVariable[]> }
  | { readonly kind: 'truncated'; readonly sets: NonEmptyArray<readonly StudyVariable[]> }

/** The observed instruments DoWhy's search accepts for the treatment and outcome, when there are any. */
export type InstrumentSet =
  | { readonly kind: 'identified'; readonly instruments: NonEmptyArray<StudyVariable> }
  | { readonly kind: 'not-identified' }

export type Identification =
  | { readonly kind: 'cutoff-design'; readonly strategy: 'sharp-rd-continuity'; readonly basis: NonEmptyArray<IdentificationBasisEntry> }
  | {
      readonly kind: 'identified'
      readonly strategy: 'backdoor-adjustment'
      readonly adjustment: AdjustmentSetSelection
      readonly canonicalAdjustmentSet: readonly StudyVariable[]
      readonly minimalAdjustmentSets: MinimalAdjustmentSetEnumeration
      readonly mediators: readonly StudyGraphNode[]
      readonly instruments: InstrumentSet
      readonly basis: NonEmptyArray<IdentificationBasisEntry>
    }
  | {
      readonly kind: 'graphically-identified'
      readonly strategy: 'id-algorithm'
      readonly expression: string
      readonly latex: string
      readonly projection: GraphicalIdentificationEvidence['projection']
      readonly frontdoor:
        | { readonly kind: 'identified'; readonly mediators: NonEmptyArray<StudyVariable> }
        | { readonly kind: 'not-identified' }
      readonly instruments: InstrumentSet
      readonly basis: NonEmptyArray<IdentificationBasisEntry>
    }
  /** No observational expression exists, but the graph names instruments; the effect is identified only under the instrumental-variable estimator's assumptions. */
  | {
      readonly kind: 'instrument-identified'
      readonly strategy: 'instrumental-variable'
      readonly instruments: NonEmptyArray<StudyVariable>
      readonly reasons: NonEmptyArray<IdentificationFailure>
      readonly basis: NonEmptyArray<IdentificationBasisEntry>
    }
  | {
      readonly kind: 'counterfactually-identified'
      readonly strategy: 'idc-star'
      readonly treatedExpression: string
      readonly untreatedExpression: string
      readonly projection: GraphicalIdentificationEvidence['projection']
      readonly basis: NonEmptyArray<IdentificationBasisEntry>
    }
  | { readonly kind: 'backdoor-not-identified'; readonly reasons: NonEmptyArray<IdentificationFailure> }

/** The strategy an identification record settled on, named once for every view that shows it. */
export function describeIdentificationStrategy(identification: Identification): string {
  switch (identification.kind) {
    case 'cutoff-design': return 'Sharp RD continuity assumptions'
    case 'identified': return 'Back-door adjustment'
    case 'graphically-identified': return identification.frontdoor.kind === 'identified' ? 'Front-door identification' : 'General ID expression'
    case 'counterfactually-identified': return 'IDC* counterfactual identification'
    case 'instrument-identified': return 'Instrumental variable'
    case 'backdoor-not-identified': return 'Not identified'
    default: return assertNever(identification)
  }
}

/** Whether some estimator in the catalogue can take this record: an ID expression alone has no evaluator yet. */
export function estimableIdentification(identification: Identification): boolean {
  switch (identification.kind) {
    case 'cutoff-design': return true
    case 'identified':
    case 'counterfactually-identified':
    case 'instrument-identified':
      return true
    case 'graphically-identified':
      return identification.frontdoor.kind === 'identified' || identification.instruments.kind === 'identified'
    case 'backdoor-not-identified':
      return false
    default:
      return assertNever(identification)
  }
}

export function identificationAllowsEstimation(kind: Identification['kind']): boolean {
  switch (kind) {
    case 'cutoff-design': return true
    case 'identified':
    case 'graphically-identified':
    case 'counterfactually-identified':
    case 'instrument-identified':
      return true
    case 'backdoor-not-identified':
      return false
    default:
      return assertNever(kind)
  }
}

export interface IdentificationArtifact {
  readonly kind: 'identification'
  readonly id: IdentificationId
  readonly study: StudyId
  readonly createdAt: string
  readonly method: typeof BACKDOOR_IDENTIFICATION_METHOD_ID | typeof GRAPHICAL_IDENTIFICATION_METHOD_ID | typeof COUNTERFACTUAL_IDENTIFICATION_METHOD_ID | typeof RD_DESIGN_METHOD_ID
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
  if (study.estimand.kind === 'local-cutoff-effect') return ok({
    kind: 'cutoff-design', strategy: 'sharp-rd-continuity', basis: [
      { kind: 'graph-assumption', id: 'rd-continuity', statement: 'Both potential-outcome regression functions are continuous at the cutoff; no other intervention changes there.' },
      { kind: 'qualification', id: 'rd-design-not-graph-test', statement: 'The DAG does not establish continuity or rule out precise manipulation of the running variable. Sharp assignment is checked against the data when fitting.' },
    ],
  })
  const latent = study.graph.nodes.filter((node) => node.column === null).map((node) => node.name)
  const variablesFrom = (indexes: readonly number[]): readonly StudyVariable[] => indexes.flatMap((index): StudyVariable[] => {
    const node = study.graph.nodes[index]
    return node === undefined || node.column === null ? [] : [{ node: node.node, column: node.column, name: node.name }]
  })
  const instrumentVariables = evidence.instruments.kind === 'identified' ? variablesFrom(evidence.instruments.instruments) : []
  const instruments: InstrumentSet = isNonEmpty(instrumentVariables)
    ? { kind: 'identified', instruments: instrumentVariables }
    : { kind: 'not-identified' }
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
    if (study.estimand.kind === 'average-treatment-effect-on-treated' && evidence.counterfactualIdentification.kind === 'identified') {
      const basis: NonEmptyArray<IdentificationBasisEntry> = [
        {
          kind: 'graph-result',
          id: 'idc-star-expressions',
          statement: `IDC* derived observational expressions for E[${study.outcome.name}(1) | ${study.treatment.name}=1] and E[${study.outcome.name}(0) | ${study.treatment.name}=1].`,
        },
        {
          kind: 'graph-assumption',
          id: 'counterfactual-latent-projection',
          statement: `The expressions use the observed-variable projection of “${study.dagName}”; bidirected edges represent the common causes recorded as unmeasured nodes.`,
        },
        {
          kind: 'qualification',
          id: 'binary-ett-estimator',
          statement: 'The current evaluator requires every observed graph variable to be recorded as 0 or 1 and reports a plug-in estimate without a sampling interval.',
        },
      ]
      return ok({
        kind: 'counterfactually-identified',
        strategy: 'idc-star',
        treatedExpression: evidence.counterfactualIdentification.treatedExpression,
        untreatedExpression: evidence.counterfactualIdentification.untreatedExpression,
        projection: evidence.graphicalIdentification.projection,
        basis,
      })
    }
    if (study.estimand.kind !== 'average-treatment-effect-on-treated' && evidence.graphicalIdentification.kind === 'identified') {
      const frontdoorVariables = evidence.frontdoor.kind === 'identified' ? variablesFrom(evidence.frontdoor.mediators) : []
      const frontdoor = isNonEmpty(frontdoorVariables)
        ? { kind: 'identified' as const, mediators: frontdoorVariables }
        : { kind: 'not-identified' as const }
      const basis: NonEmptyArray<IdentificationBasisEntry> = [
        {
          kind: 'graph-result',
          id: 'id-expression',
          statement: `The ID algorithm derived an observational expression for P(${study.outcome.name} | do(${study.treatment.name})).`,
        },
        {
          kind: 'graph-assumption',
          id: 'latent-projection',
          statement: `The expression uses the observed-variable projection of “${study.dagName}”; bidirected edges represent the common causes recorded as unmeasured nodes.`,
        },
        {
          kind: 'qualification',
          id: 'estimator-required',
          statement: frontdoor.kind === 'identified'
            ? 'The front-door two-stage estimator can evaluate this expression under its linear stage-model assumptions.'
            : 'This record establishes graphical identification. Estimation requires an estimator that evaluates this identified expression.',
        },
      ]
      return ok({
        kind: 'graphically-identified',
        strategy: 'id-algorithm',
        expression: evidence.graphicalIdentification.expression,
        latex: evidence.graphicalIdentification.latex,
        projection: evidence.graphicalIdentification.projection,
        frontdoor,
        instruments,
        basis,
      })
    }
    if (evidence.graphicalIdentification.kind === 'unidentifiable') {
      const nameAt = (index: number): string => study.graph.nodes[index]?.name ?? String(index)
      reasons.push({
        kind: 'id-hedge',
        graph: evidence.graphicalIdentification.hedgeGraph.map(nameAt) as unknown as NonEmptyArray<string>,
        subgraph: evidence.graphicalIdentification.hedgeSubgraph.map(nameAt) as unknown as NonEmptyArray<string>,
      })
    }
    if (study.estimand.kind === 'average-treatment-effect-on-treated') {
      reasons.push({ kind: 'att-requires-counterfactual-identification', estimand: 'ATT' })
    }
    if (study.estimand.kind !== 'average-treatment-effect-on-treated' && instruments.kind === 'identified') {
      const names = instruments.instruments.map((variable) => variable.name).join(', ')
      const plural = instruments.instruments.length > 1
      const basis: NonEmptyArray<IdentificationBasisEntry> = [
        { kind: 'design-record', id: 'assignment-mechanism', statement: `${describeAssignmentKind(study.assignment.kind)} treatment: ${study.assignment.description} ${describeStudyDesignCategory(studyDesignCategory(study))}` },
        {
          kind: 'graph-result',
          id: 'instrument-set',
          statement: `${names} ${plural ? 'meet' : 'meets'} the two level 2 definitional requirements for a valid instrument in “${study.dagName}”. As-if-random: any backdoor paths between the instrument and ${study.outcome.name} can be blocked. Exclusion: the instrument is a cause of ${study.outcome.name} only indirectly through ${study.treatment.name}.`,
        },
        {
          kind: 'graph-assumption',
          id: 'instrument-exclusion',
          statement: `None of the other causes of ${study.outcome.name} are also causes of ${names}, so there are no backdoor paths between the ${plural ? 'instruments' : 'instrument'} and the outcome; and if the causal path from ${names} to ${study.treatment.name} were removed there would be no causal path from ${names} to ${study.outcome.name}.`,
        },
        { kind: 'design-assumption', id: 'consistency', statement: study.designAssumptions.consistency.statement, rationale: study.designAssumptions.consistency.rationale },
        { kind: 'design-assumption', id: 'no-interference', statement: study.designAssumptions.noInterference.statement, rationale: study.designAssumptions.noInterference.rationale },
        {
          kind: 'qualification',
          id: 'instrument-estimator',
          statement: `No observational expression for the effect exists: ${reasons.map(describeIdentificationFailure).join(' ')} The level 2 graphical assumptions are not sufficient for instrumental variable identification; additional parametric assumptions are needed. The estimator makes a linearity assumption and derives the ATE from the coefficients of linear models of ${study.outcome.name} and ${study.treatment.name} given the instrument.`,
        },
      ]
      return ok({ kind: 'instrument-identified', strategy: 'instrumental-variable', instruments: instruments.instruments, reasons: reasons as unknown as NonEmptyArray<IdentificationFailure>, basis })
    }
    return ok({ kind: 'backdoor-not-identified', reasons: reasons as unknown as NonEmptyArray<IdentificationFailure> })
  }
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
        ? `No unmeasured common cause of ${treatment} and ${outcome} exists. Test sensitivity to this assumption in the sensitivity section.`
        : `No unmeasured common cause of ${treatment} and ${outcome} remains after adjusting for ${setText}. Test sensitivity to this assumption in the sensitivity section.`,
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
  if (instruments.kind === 'identified') {
    const names = instruments.instruments.map((variable) => variable.name).join(', ')
    basis.push({
      kind: 'graph-result',
      id: 'instrument-set',
      statement: `${names} also ${instruments.instruments.length > 1 ? 'meet' : 'meets'} the two definitional requirements for a valid instrument, as-if-random and exclusion, so an instrumental variable estimand is available as well.`,
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
    instruments,
    basis: basis as unknown as NonEmptyArray<IdentificationBasisEntry>,
  })
}

export const selectedAdjustmentSet = (identification: Extract<Identification, { kind: 'identified' }>): readonly StudyVariable[] => identification.adjustment.variables

/** The instruments an instrumental-variable estimate may use, whichever strategy the record settled on. */
export function identifiedInstruments(identification: Identification): NonEmptyArray<StudyVariable> | null {
  switch (identification.kind) {
    case 'cutoff-design': return null
    case 'instrument-identified': return identification.instruments
    case 'identified':
    case 'graphically-identified': return identification.instruments.kind === 'identified' ? identification.instruments.instruments : null
    case 'counterfactually-identified':
    case 'backdoor-not-identified': return null
    default: return assertNever(identification)
  }
}

export function describeStudyDesignProblem(problem: StudyDesignProblem): string {
  switch (problem.kind) {
    case 'cutoff-required': return 'Choose a measured running variable and enter a finite cutoff.'
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
    case 'modifier-required': return 'Choose the effect modifier whose groups the effect is averaged within.'
    case 'latent-modifier': return `${problem.name} is unmeasured. Choose a measured effect modifier.`
    case 'modifier-is-endpoint': return `${problem.name} is the treatment or the outcome; the effect modifier must be a third variable.`
    case 'modifier-after-treatment': return `${problem.name} is ${problem.role}, so its groups are set after the treatment; choose a variable the treatment does not reach.`
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
    case 'no-observed-backdoor-set': return 'No measured adjustment set blocks every back-door path in this graph.'
    case 'id-hedge': return `The ID algorithm found a hedge: ${failure.subgraph.join(', ')} remains inside the confounded component ${failure.graph.join(', ')} after intervention. The level-2 effect is not identifiable from the observational distribution under this graph.`
    case 'att-requires-counterfactual-identification': return `${failure.estimand} conditions on the factual treated group. The unconditional level-2 ID expression does not identify this counterfactual target; it requires counterfactual identification or a valid adjustment strategy.`
    default: return assertNever(failure)
  }
}
