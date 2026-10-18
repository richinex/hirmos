import { z } from 'zod'
import { brand, err, ok, type Result } from './dop'
import { actionLabel, planRows, type DecisionEvidence, type PlanEstimate } from './decisionEvidence'
import type { DecisionFrame } from './decisionFrame'

const unit = z.number().min(0).max(1)
const count = z.number().int().nonnegative()

const columnRole = z
  .object({
    column: z
      .string()
      .min(1)
      .transform((value) => brand<string, 'ColumnId'>(value)),
    name: z.string().min(1),
  })
  .strict()

const estimate = z
  .object({
    value: z.number(),
    interval: z.object({ lower: z.number(), upper: z.number() }).strict().nullable(),
  })
  .strict()

/** The exact rule chosen, written out so it can be deployed or audited without the run that scored it. */
const policySchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('equal-chance'), actions: z.array(z.number()).min(2) }).strict(),
  z.object({ kind: z.literal('always'), action: z.number() }).strict(),
  z
    .object({
      kind: z.literal('copied'),
      inputs: z.array(columnRole),
      actions: z.array(z.number()).min(2),
      patterns: z.array(z.array(z.number())),
      probabilities: z.array(z.array(unit)),
    })
    .strict(),
])

export type DecisionPolicy = z.infer<typeof policySchema>

/** What the evidence run said about the chosen plan; a missing source was not evaluated for it. */
const basisSchema = z
  .object({
    weighted: z
      .object({
        propensity: columnRole,
        estimator: z.enum(['selfNormalized', 'inverseProbability']),
        rows: count,
        heldOut: z.boolean(),
        value: estimate,
        difference: estimate,
        effectiveRows: z.number().nonnegative(),
      })
      .strict()
      .nullable(),
    randomised: z.object({ mean: unit, rows: count }).strict().nullable(),
    bounds: z.object({ lower: unit, upper: unit }).strict().nullable(),
    observed: estimate.nullable(),
  })
  .strict()

export type DecisionBasis = z.infer<typeof basisSchema>

const isoDate = /^\d{4}-\d{2}-\d{2}$/

/** A decision made from one evidence run, with what was known and expected when it was made. */
export const decisionRecordSchema = z
  .object({
    kind: z.literal('decision-record'),
    id: z
      .string()
      .uuid()
      .transform((value) => brand<string, 'DecisionRecordId'>(value)),
    evidence: z
      .string()
      .uuid()
      .transform((value) => brand<string, 'DecisionEvidenceId'>(value)),
    frame: z
      .string()
      .uuid()
      .transform((value) => brand<string, 'DecisionFrameId'>(value)),
    dagDocument: z.string().min(1),
    dagRevision: z.string().min(1),
    prepared: z.string().min(1),
    createdAt: z.string().datetime(),
    action: columnRole,
    outcome: columnRole,
    plan: z.string().min(1),
    policy: policySchema,
    /** Share of cases kept on current practice; the deployed rule is then a mixture of the two. */
    control: z
      .object({ share: z.number().gt(0).lt(1) })
      .strict()
      .nullable(),
    basis: basisSchema,
    assumptions: z.array(z.string().min(1)).min(1),
    rationale: z.string().trim().min(1),
    reconsider: z.string().trim().min(1),
    reviewOn: z.string().regex(isoDate),
  })
  .strict()

export type DecisionRecord = z.infer<typeof decisionRecordSchema>

/** A plan the evidence run can support a decision for, with its exact rule and its evidence. */
export type RecordablePlan = {
  readonly key: string
  readonly label: string
  readonly policy: DecisionPolicy
  readonly basis: DecisionBasis
}

const NATURAL_BOUNDS_ASSUMPTION =
  'The natural bounds assume an outcome between 0 and 1 and describe the population the log covers; they are computed from a sample.'
const RANDOMISED_ASSUMPTION =
  'The randomised estimate applies to the population and assignment design of the randomised rows, and has its own sampling error.'
const OBSERVED_ASSUMPTION =
  'The observed average describes the logged rows; as a forecast of future performance it is an estimate.'

const weightedAssumption = (propensity: string) =>
  `The weighted estimate assumes ${propensity} records the chance the logging policy gave each logged action, and that the plan’s actions were sometimes logged where it would take them.`

/** The assumptions behind each source of evidence the plan rests on. */
export function basisAssumptions(basis: DecisionBasis): readonly string[] {
  return [
    ...(basis.weighted === null ? [] : [weightedAssumption(basis.weighted.propensity.name)]),
    ...(basis.weighted?.heldOut === true
      ? ['The copied rule is scored only on logged rows held out from copying.']
      : []),
    ...(basis.randomised === null ? [] : [RANDOMISED_ASSUMPTION]),
    ...(basis.bounds === null ? [] : [NATURAL_BOUNDS_ASSUMPTION]),
    ...(basis.observed === null ? [] : [OBSERVED_ASSUMPTION]),
  ]
}

/** Every plan the run can support: its scored plans, or each action's always plan when nothing was scored. */
export function recordablePlans(run: DecisionEvidence): readonly RecordablePlan[] {
  const actions = run.result.actions.map((action) => action.value)
  const scored = planRows(run)
  if (run.scoring === null)
    return run.result.actions.map((action) => ({
      key: `always-${action.value}`,
      label: `Always ${run.action.name} ${actionLabel(action.value)}`,
      policy: { kind: 'always', action: action.value },
      basis: {
        weighted: null,
        randomised: action.randomised,
        bounds: { lower: action.lower, upper: action.upper },
        observed: null,
      },
    }))
  const { propensity, request, result } = run.scoring
  return scored.map((row) => {
    const policy: DecisionPolicy =
      row.rule.kind === 'uniform'
        ? { kind: 'equal-chance', actions }
        : row.rule.kind === 'always'
          ? { kind: 'always', action: actions[row.rule.action]! }
          : {
              kind: 'copied',
              inputs: run.imitation?.inputs ?? [],
              actions,
              patterns: run.imitation?.patterns ?? [],
              probabilities: row.rule.probabilities,
            }
    return {
      key: `plan-${row.index}`,
      label: row.label,
      policy,
      basis: {
        weighted: {
          propensity,
          estimator: request.estimator.kind,
          rows: row.scored.rows,
          heldOut: request.plans[row.index]!.rows === 'heldOut',
          value: row.scored.value,
          difference: row.scored.difference,
          effectiveRows: row.scored.weights.effectiveRows,
        },
        randomised: row.randomised,
        bounds: row.bounds,
        observed: result.observed,
      },
    }
  })
}

export type DecisionRecordDraft = {
  readonly plan: string
  /** Percent of cases kept on current practice, as typed; empty for none. */
  readonly control: string
  readonly rationale: string
  readonly reconsider: string
  readonly reviewOn: string
}

export type DecisionRecordProblem =
  | { readonly kind: 'no-plan' }
  | { readonly kind: 'control' }
  | { readonly kind: 'rationale' }
  | { readonly kind: 'reconsider' }
  | { readonly kind: 'review-date' }

/** Check a draft against the run and build the record it describes. */
export function draftDecisionRecord(
  run: DecisionEvidence,
  frame: DecisionFrame,
  draft: DecisionRecordDraft,
  now: Date,
): Result<DecisionRecord, DecisionRecordProblem> {
  const plan = recordablePlans(run).find((candidate) => candidate.key === draft.plan)
  if (plan === undefined) return err({ kind: 'no-plan' })
  const percent = draft.control.trim() === '' ? null : Number(draft.control)
  if (percent !== null && !(Number.isFinite(percent) && percent > 0 && percent < 100))
    return err({ kind: 'control' })
  if (draft.rationale.trim() === '') return err({ kind: 'rationale' })
  if (draft.reconsider.trim() === '') return err({ kind: 'reconsider' })
  if (!isoDate.test(draft.reviewOn) || Number.isNaN(Date.parse(draft.reviewOn)))
    return err({ kind: 'review-date' })
  return ok({
    kind: 'decision-record',
    id: brand<string, 'DecisionRecordId'>(crypto.randomUUID()),
    evidence: run.id,
    frame: frame.id,
    dagDocument: frame.dagDocument,
    dagRevision: frame.dagRevision,
    prepared: run.prepared,
    createdAt: now.toISOString(),
    action: run.action,
    outcome: run.outcome,
    plan: plan.label,
    policy: plan.policy,
    control: percent === null ? null : { share: percent / 100 },
    basis: plan.basis,
    assumptions: [...basisAssumptions(plan.basis)] as [string, ...string[]],
    rationale: draft.rationale.trim(),
    reconsider: draft.reconsider.trim(),
    reviewOn: draft.reviewOn,
  })
}

export function describeDecisionRecordProblem(problem: DecisionRecordProblem): string {
  switch (problem.kind) {
    case 'no-plan':
      return 'Choose the plan you are deciding on.'
    case 'control':
      return 'Enter a share kept on current practice from 1 to 99 percent, or leave it empty.'
    case 'rationale':
      return 'Enter the reason for the decision.'
    case 'reconsider':
      return 'Enter what would change this decision.'
    case 'review-date':
      return 'Choose a review date.'
  }
}

/** The plan's expected value as the evidence states it, most direct source first. */
export function expectedValue(basis: DecisionBasis): PlanEstimate | null {
  if (basis.weighted !== null) return basis.weighted.value
  if (basis.randomised !== null) return { value: basis.randomised.mean, interval: null }
  return null
}
