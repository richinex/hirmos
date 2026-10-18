import { z } from 'zod'
import { brand, err, ok, type Result } from './dop'
import { categoricalMembershipSchema } from './sampleMembership'
import type { DagDocument } from './dag'
import type { DecisionFrame } from './decisionFrame'
import type { InterpretationStatement, ResultInterpretation } from './resultInterpretation'
import { formatCount, formatStatistic } from '../lib/format/number'

const count = z.number().int().nonnegative()
const unit = z.number().min(0).max(1)

export const decisionEvidenceRequestSchema = z
  .object({
    actions: z.array(z.number().finite()).min(2).max(256),
    learning: z
      .object({
        horizon: z.number().int().min(1).max(1_000_000),
        seeds: z.array(z.number().int().nonnegative()).min(1).max(20),
      })
      .strict()
      .nullable(),
  })
  .strict()

export type DecisionEvidenceRequest = z.infer<typeof decisionEvidenceRequestSchema>

const sampleSchema = z.object({ rows: count, mean: unit }).strict()
const curveSchema = z
  .object({
    mean: z.array(z.number()),
    low: z.array(z.number()),
    high: z.array(z.number()),
    finals: z.array(z.number()),
  })
  .strict()

export const decisionEvidenceResultSchema = z
  .object({
    actions: z.array(
      z
        .object({
          value: z.number(),
          logged: sampleSchema,
          share: unit,
          lower: unit,
          upper: unit,
          randomised: sampleSchema.nullable(),
        })
        .strict(),
    ),
    loggedRows: count,
    randomisedRows: count,
    excludedRows: count,
    learning: z
      .object({
        steps: z.array(z.number().int().positive()),
        best: unit,
        scratch: curveSchema,
        clipped: curveSchema,
      })
      .strict()
      .nullable(),
  })
  .strict()

export type DecisionEvidenceResult = z.infer<typeof decisionEvidenceResultSchema>

const ruleSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('uniform') }).strict(),
  z.object({ kind: z.literal('always'), action: z.number().int().nonnegative() }).strict(),
  z.object({ kind: z.literal('table'), probabilities: z.array(z.array(unit)) }).strict(),
])

/** A seeded share of the logged rows kept back from copying, so the copy is scored only on rows it never saw. */
const holdoutSchema = z
  .object({ share: z.number().gt(0).lt(1), seed: z.number().int().min(0).max(4_294_967_295) })
  .strict()

export type Holdout = z.infer<typeof holdoutSchema>

/** Most distinct input patterns a copied rule is reported and scored on. */
export const IMITATION_PATTERN_LIMIT = 5_000

export const imitationRuleRequestSchema = z
  .object({
    width: z.number().int().min(0).max(64),
    actions: z
      .object({
        kind: z.literal('discrete'),
        count: z.number().int().min(2).max(256),
        chosen: z.array(z.number().int().nonnegative()),
      })
      .strict(),
    probes: z.array(z.array(z.number().finite())).min(1).max(IMITATION_PATTERN_LIMIT),
    settings: z
      .object({
        hidden: z.number().int().min(1).max(512),
        learningRate: z.number().positive(),
        epochs: z.number().int().min(1).max(1_000),
        batchSize: z.number().int().min(1).max(65_536),
        validationShare: z.number().gt(0).lt(1),
        patience: z.number().int().min(1),
        seed: z.number().int().nonnegative(),
      })
      .strict(),
    sample: z
      .object({ rows: z.number().int().positive(), seed: z.number().int().nonnegative() })
      .strict()
      .nullable(),
    holdout: holdoutSchema.nullable(),
  })
  .strict()

export type ImitationRuleRequest = z.infer<typeof imitationRuleRequestSchema>

export const imitationRuleEvidenceSchema = z
  .object({ kind: z.literal('discrete'), probabilities: z.array(z.array(unit)) })
  .strict()

export type ImitationRuleEvidence = z.infer<typeof imitationRuleEvidenceSchema>

/** A copied rule: the demonstrator's action counts and the learned probabilities for each input pattern. */
const imitationSchema = z
  .object({
    inputs: z.array(
      z
        .object({
          column: z
            .string()
            .min(1)
            .transform((value) => brand<string, 'ColumnId'>(value)),
          name: z.string().min(1),
        })
        .strict(),
    ),
    seeds: z.number().int().min(1).max(16),
    trainingRows: z.number().int().positive(),
    /** Logged rows kept back from training for scoring the copy; null when every row was available to it. */
    heldOutRows: count.nullable().default(null),
    patterns: z.array(z.array(z.number().finite())).min(1).max(IMITATION_PATTERN_LIMIT),
    demonstrated: z.array(z.array(count)),
    learned: z
      .object({
        mean: z.array(z.array(unit)),
        low: z.array(z.array(unit)),
        high: z.array(z.array(unit)),
      })
      .strict(),
  })
  .strict()

export type DecisionImitation = z.infer<typeof imitationSchema>

export type ScoredRule = z.infer<typeof ruleSchema>

/** The rows a plan is scored on: every logged row, or only the rows held out from copying. */
const scoredRows = z.enum(['all', 'heldOut'])

export const scoredPlanSchema = z
  .object({ rule: ruleSchema, rows: scoredRows, compare: z.boolean() })
  .strict()

export type ScoredPlan = z.infer<typeof scoredPlanSchema>

/** Most plans bootstrapped for comparison in one run; each costs two resamples of every row per round. */
export const COMPARED_PLAN_LIMIT = 6

/**
 * The plans a run scores: equal chance, always each action, and the copied rule when there is one. The copied rule is
 * scored on held-out rows. Equal chance and the copied rule are compared, and every always plan too when the total
 * stays within the comparison limit.
 */
export function comparedPlans(
  actions: number,
  copied: readonly (readonly number[])[] | null,
): ScoredPlan[] {
  const fixed = 1 + (copied === null ? 0 : 1)
  const everyAction = fixed + actions <= COMPARED_PLAN_LIMIT
  return [
    { rule: { kind: 'uniform' }, rows: 'all', compare: true },
    ...Array.from({ length: actions }, (_, action) => ({
      rule: { kind: 'always' as const, action },
      rows: 'all' as const,
      compare: everyAction,
    })),
    ...(copied === null
      ? []
      : [
          {
            rule: { kind: 'table' as const, probabilities: copied.map((row) => [...row]) },
            rows: 'heldOut' as const,
            compare: true,
          },
        ]),
  ]
}

export const loggedPolicyValueRequestSchema = z
  .object({
    actions: z.array(z.number().finite()).min(2).max(256),
    plans: z
      .array(scoredPlanSchema)
      .min(1)
      .max(258)
      .refine((plans) => plans.filter((plan) => plan.compare).length <= COMPARED_PLAN_LIMIT),
    estimator: z.discriminatedUnion('kind', [
      z
        .object({
          kind: z.literal('inverseProbability'),
          maxWeight: z.number().positive().nullable(),
        })
        .strict(),
      z.object({ kind: z.literal('selfNormalized') }).strict(),
    ]),
    bootstrap: z
      .object({
        rounds: z.number().int().min(1).max(10_000),
        seed: z.number().int().min(0).max(4_294_967_295),
        alpha: z.number().gt(0).lt(1),
      })
      .strict(),
    holdout: holdoutSchema.nullable(),
  })
  .strict()

export type LoggedPolicyValueRequest = z.infer<typeof loggedPolicyValueRequestSchema>

const estimateSchema = z
  .object({
    value: z.number(),
    interval: z.object({ lower: z.number(), upper: z.number() }).strict().nullable(),
  })
  .strict()

export type PlanEstimate = z.infer<typeof estimateSchema>

export const loggedPolicyValueResultSchema = z
  .object({
    rows: count,
    heldOutRows: count.nullable(),
    /** The average logged outcome over every logged row, with its bootstrap interval. */
    observed: estimateSchema,
    plans: z.array(
      z
        .object({
          rows: count,
          observed: z.number(),
          value: estimateSchema,
          /** The plan's value minus the observed average on the same rows, with a paired interval. */
          difference: estimateSchema,
          weights: z
            .object({
              support: unit,
              maxWeight: z.number().nonnegative(),
              effectiveRows: z.number().nonnegative(),
            })
            .strict(),
        })
        .strict(),
    ),
  })
  .strict()

export type LoggedPolicyValueResult = z.infer<typeof loggedPolicyValueResultSchema>

const columnRole = z
  .object({
    column: z
      .string()
      .min(1)
      .transform((value) => brand<string, 'ColumnId'>(value)),
    name: z.string().min(1),
  })
  .strict()

/** Scores saved before plans were compared lack paired differences and weight checks; drop them and say so. */
function supersedeLegacyScoring(value: unknown): unknown {
  if (value === null || typeof value !== 'object') return value
  const scoring: unknown = Reflect.get(value, 'scoring')
  if (scoring === null || typeof scoring !== 'object') return value
  const request: unknown = Reflect.get(scoring, 'request')
  if (request === null || typeof request !== 'object' || !('rules' in request)) return value
  return { ...value, scoring: null, scoringSuperseded: true }
}

/** One run of the data checks for a framed decision, against the prepared dataset it read. */
export const decisionEvidenceSchema = z.preprocess(
  supersedeLegacyScoring,
  z
    .object({
      kind: z.literal('decision-evidence'),
      id: z
        .string()
        .uuid()
        .transform((value) => brand<string, 'DecisionEvidenceId'>(value)),
      frame: z
        .string()
        .uuid()
        .transform((value) => brand<string, 'DecisionFrameId'>(value)),
      prepared: z.string().min(1),
      createdAt: z.string().datetime(),
      action: columnRole,
      outcome: columnRole,
      randomised: categoricalMembershipSchema.nullable(),
      request: decisionEvidenceRequestSchema,
      result: decisionEvidenceResultSchema,
      scoring: z
        .object({
          propensity: columnRole,
          request: loggedPolicyValueRequestSchema,
          result: loggedPolicyValueResultSchema,
        })
        .strict()
        .nullable(),
      imitation: imitationSchema.nullable().default(null),
      /** The run was scored before plans were compared, so its scores were set aside on loading. */
      scoringSuperseded: z.boolean().default(false),
    })
    .strict()
    .superRefine((record, context) => {
      if (record.result.actions.length !== record.request.actions.length)
        context.addIssue({
          code: 'custom',
          message: 'The evidence does not cover the requested actions.',
        })
      if (
        record.scoring !== null &&
        record.scoring.result.plans.length !== record.scoring.request.plans.length
      )
        context.addIssue({
          code: 'custom',
          message: 'The scores do not cover the requested plans.',
        })
    }),
)

export type DecisionEvidence = z.infer<typeof decisionEvidenceSchema>

export type DecisionColumns = {
  readonly action: z.infer<typeof columnRole>
  readonly outcome: z.infer<typeof columnRole>
  /** The recorded variables the decision's rule may read, in graph order. */
  readonly inputs: readonly z.infer<typeof columnRole>[]
}

export type DecisionColumnsProblem =
  | { readonly kind: 'stale-frame' }
  | { readonly kind: 'several-decisions'; readonly count: number }
  | { readonly kind: 'unmeasured'; readonly name: string }

/** The recorded columns behind a single-decision frame, read from the DAG revision it was framed on. */
export function decisionColumns(
  frame: DecisionFrame,
  document: DagDocument,
): Result<DecisionColumns, DecisionColumnsProblem> {
  if (document.id !== frame.dagDocument || document.current.id !== frame.dagRevision)
    return err({ kind: 'stale-frame' })
  if (frame.request.decisions.length !== 1)
    return err({ kind: 'several-decisions', count: frame.request.decisions.length })
  const role = (index: number) => {
    const node = document.current.graph.nodes[index]
    return node?.kind === 'observed' ? { column: node.column, name: node.name } : null
  }
  const action = role(frame.request.decisions[0]!.node)
  const outcome = role(frame.request.objective)
  if (action === null)
    return err({ kind: 'unmeasured', name: frame.request.names[frame.request.decisions[0]!.node]! })
  if (outcome === null)
    return err({ kind: 'unmeasured', name: frame.request.names[frame.request.objective]! })
  const inputs = frame.request.decisions[0]!.reads.map(role)
  const unread = inputs.findIndex((input) => input === null)
  if (unread !== -1)
    return err({
      kind: 'unmeasured',
      name: frame.request.names[frame.request.decisions[0]!.reads[unread]!]!,
    })
  return ok({ action, outcome, inputs: inputs as z.infer<typeof columnRole>[] })
}

export function describeDecisionColumnsProblem(problem: DecisionColumnsProblem): string {
  switch (problem.kind) {
    case 'stale-frame':
      return 'The DAG has changed since this decision was framed. Frame it again to check it against the data.'
    case 'several-decisions':
      return `The data checks cover one decision; this frame has ${problem.count}. Frame the first decision on its own.`
    case 'unmeasured':
      return `${problem.name} has no column in the dataset.`
  }
}

/** Distinct actions in the logged cells, in ascending order. */
export function cellActions(cells: Float64Array): readonly number[] {
  const values = new Set<number>()
  for (let i = 0; i < cells.length; i += 4) if (cells[i + 2] !== 2) values.add(cells[i]!)
  return [...values].sort((a, b) => a - b)
}

/** An action's recorded value as the reader typed it: an integer code, or a number. */
export const actionLabel = (value: number): string =>
  Number.isInteger(value) ? String(value) : formatStatistic('raw', value).text

const rate = (value: number): string => formatStatistic('mean', value).text

/** Words joined as a reader would list them: "a", "a and b", "a, b and c". */
export const listWords = (words: readonly string[]): string =>
  words.length <= 1 ? (words[0] ?? '') : `${words.slice(0, -1).join(', ')} and ${words.at(-1)}`

const signed = (value: number): string => (value > 0 ? `+${rate(value)}` : rate(value))

/** A plan's name as a reader sees it in tables and charts. */
export function planLabel(
  rule: ScoredRule,
  action: string,
  actions: DecisionEvidenceResult['actions'],
): string {
  switch (rule.kind) {
    case 'uniform':
      return 'Equal chance'
    case 'always':
      return `Always ${action} ${actionLabel(actions[rule.action]!.value)}`
    case 'table':
      return 'Copied rule'
  }
}

const average = (values: readonly number[]): number =>
  values.reduce((total, value) => total + value, 0) / values.length

/** Actions whose randomised estimate lies outside their natural bounds. */
export const actionsOutsideBounds = (result: DecisionEvidenceResult) =>
  result.actions.filter(
    (action) =>
      action.randomised !== null &&
      (action.randomised.mean < action.lower || action.randomised.mean > action.upper),
  )

/** What a recorded evidence run says, as tagged statements derived from its numbers. */
export function interpretDecisionEvidence(run: DecisionEvidence): ResultInterpretation {
  const { result, scoring } = run
  const name = run.action.name
  const best = (pick: (action: DecisionEvidenceResult['actions'][number]) => number) =>
    result.actions.reduce((top, action) => (pick(action) > pick(top) ? action : top))
  const logBest = best((action) => action.logged.mean)
  const everyRandomised = result.actions.every((action) => action.randomised !== null)
  const anyRandomised = result.actions.some((action) => action.randomised !== null)
  const statements: InterpretationStatement[] = []

  if (everyRandomised) {
    const trialBest = best((action) => action.randomised!.mean)
    statements.push({
      kind: 'magnitude',
      text:
        trialBest.value === logBest.value
          ? `The log and the randomised rows both rank ${name} ${actionLabel(logBest.value)} first.`
          : `The randomised estimates rank ${name} ${actionLabel(trialBest.value)} first, at ${rate(trialBest.randomised!.mean)}, while the log ranks ${actionLabel(logBest.value)} first. On these estimates, choosing from the log alone would pick a different action.`,
    })
  } else {
    statements.push({
      kind: 'magnitude',
      text: anyRandomised
        ? `The randomised rows give ${listWords(result.actions.filter((action) => action.randomised !== null).map((action) => `${name} ${actionLabel(action.value)} a mean of ${rate(action.randomised!.mean)}`))}, while ${listWords(result.actions.filter((action) => action.randomised === null).map((action) => `${name} ${actionLabel(action.value)}`))}, with no randomised rows, is known only within its natural bounds: ${listWords(result.actions.filter((action) => action.randomised === null).map((action) => `${rate(action.lower)} to ${rate(action.upper)}`))}.`
        : `The log ranks ${name} ${actionLabel(logBest.value)} first, with a mean of ${rate(logBest.logged.mean)}, but without randomised rows the natural bounds are the only causal evidence.`,
    })
  }

  if (scoring !== null) {
    const { result: scored, request } = scoring
    const describe = (index: number) => planLabel(request.plans[index]!.rule, name, result.actions)
    statements.push({
      kind: 'comparison',
      text: `Weighting ${formatCount(scored.rows).text} logged rows by ${scoring.propensity.name} scores the equal-chance rule at ${rate(scored.plans[0]!.value.value)}; the observed average of those rows is ${rate(scored.observed.value)}.`,
    })
    const compared = request.plans.flatMap((plan, index) =>
      plan.compare && scored.plans[index]!.difference.interval !== null ? [index] : [],
    )
    if (compared.length > 0)
      statements.push({
        kind: 'uncertainty',
        text: `Against the observed average on the same rows, with 95% paired bootstrap intervals: ${listWords(
          compared.map((index) => {
            const { difference } = scored.plans[index]!
            return `${describe(index).toLowerCase()} ${signed(difference.value)} (${signed(difference.interval!.lower)} to ${signed(difference.interval!.upper)})`
          }),
        )}.`,
      })
    const thinnest = scored.plans.reduce(
      (low, plan, index) =>
        plan.weights.effectiveRows < scored.plans[low]!.weights.effectiveRows ? index : low,
      0,
    )
    statements.push({
      kind: 'qualification',
      text: `Weighted estimates assume ${scoring.propensity.name} records the chance the logging policy actually gave each logged action. The plan resting on the fewest rows is ${describe(thinnest).toLowerCase()}, with ${formatCount(Math.round(scored.plans[thinnest]!.weights.effectiveRows)).text} effective rows of ${formatCount(scored.plans[thinnest]!.rows).text}.`,
    })
  }
  if (run.scoringSuperseded)
    statements.push({
      kind: 'qualification',
      text: 'This run was scored before plans were compared on held-out rows with paired intervals, so its scores are not shown. Run the checks again to compare plans.',
    })

  if (run.imitation !== null) {
    const patterns = copiedPatterns(
      run.imitation,
      result.actions.map((action) => action.value),
    )
    const gaps = patterns.map((pattern) => pattern.gap)
    const weighted =
      gaps.reduce((total, gap, i) => total + gap * patterns[i]!.rows, 0) /
      patterns.reduce((total, pattern) => total + pattern.rows, 0)
    statements.push({
      kind: 'comparison',
      text: `Copied from ${formatCount(run.imitation.trainingRows).text} logged rows, the rule’s action probabilities differ from the logged shares by ${rate(weighted)} on average across ${formatCount(patterns.length).text} input ${patterns.length === 1 ? 'pattern' : 'patterns'}, and by at most ${rate(Math.max(...gaps))}.`,
    })
    const copied = scoring?.request.plans.findIndex((plan) => plan.rule.kind === 'table') ?? -1
    if (scoring != null && copied !== -1) {
      const plan = scoring.result.plans[copied]!
      statements.push({
        kind: 'comparison',
        text: `Scored only on the ${formatCount(plan.rows).text} logged rows held out from copying, the copied rule reaches ${rate(plan.value.value)} against an observed average of ${rate(plan.observed)} on those rows.`,
      })
    }
    statements.push({
      kind: 'uncertainty',
      text: `Across ${formatCount(run.imitation.seeds).text} training seeds, the copy’s probability of the most logged action varies by up to ${rate(Math.max(...patterns.map((pattern) => pattern.copiedHigh - pattern.copiedLow)))}.`,
    })
    statements.push({
      kind: 'qualification',
      text: 'A close copy reproduces how the demonstrator chose; it keeps the demonstrator’s value only when the decision checks find that copying from these variables is safe.',
    })
  }

  if (result.learning !== null) {
    const { scratch, clipped, steps } = result.learning
    statements.push({
      kind: 'comparison',
      text: `Over ${formatCount(steps.at(-1) ?? 0).text} replayed steps, learning from scratch gives up ${rate(average(scratch.finals))} of the outcome on average, and learning within the natural bounds gives up ${rate(average(clipped.finals))}.`,
    })
    statements.push({
      kind: 'uncertainty',
      text: `Across ${formatCount(scratch.finals.length).text} seeds, final regret ranges from ${rate(Math.min(...scratch.finals))} to ${rate(Math.max(...scratch.finals))} from scratch and from ${rate(Math.min(...clipped.finals))} to ${rate(Math.max(...clipped.finals))} within the bounds.`,
    })
  }

  statements.push({
    kind: 'qualification',
    text: 'The natural bounds assume an outcome between 0 and 1 and describe the population the log covers. They are computed from a sample, so they carry sampling error as well.',
  })
  if (anyRandomised)
    statements.push({
      kind: 'qualification',
      text: 'Each randomised estimate has its own sampling error and applies to the population and assignment design of the randomised rows. It tests the bounds only if those rows come from the same population and measure the outcome the same way.',
    })
  const [first, ...rest] = statements
  return { kind: 'result-interpretation', statements: [first!, ...rest] }
}

/** Where each column sits in a materialised matrix of logged rows. */
export type LoggedLayout = {
  readonly action: number
  readonly outcome: number
  readonly propensity: number | null
  readonly inputs: readonly number[]
  /** The sample-membership column; only rows coded 0, the logged rows, are kept. */
  readonly membership: number | null
}

/** Logged rows shaped for copying and scoring: one pattern per distinct combination of inputs. */
export type LoggedRows = {
  readonly rows: number
  /** One row of the rule's inputs per logged row. */
  readonly inputs: Float64Array
  /** Each logged row's action, as an index into the listed actions. */
  readonly chosen: readonly number[]
  readonly patterns: readonly (readonly number[])[]
  /** For each pattern, how often the demonstrator chose each action. */
  readonly demonstrated: readonly (readonly number[])[]
  /** Action, outcome, propensity and pattern per logged row, when a propensity column is read. */
  readonly scoring: Float64Array | null
}

export type LoggedRowsProblem =
  | { readonly kind: 'no-logged-rows' }
  | { readonly kind: 'unlisted-action'; readonly value: number }
  | { readonly kind: 'too-many-patterns' }

/** Keep the complete logged rows of a column-major matrix and group them by their inputs. */
export function shapeLoggedRows(
  matrix: {
    readonly values: Float64Array
    readonly validity: Uint8Array
    readonly rowCount: number
    readonly columns: number
  },
  layout: LoggedLayout,
  actions: readonly number[],
): Result<LoggedRows, LoggedRowsProblem> {
  const { values, validity, rowCount, columns } = matrix
  const valid = (index: number) => (validity[index >> 3]! & (1 << (index & 7))) !== 0
  const at = (column: number, row: number) => values[column * rowCount + row]!
  const arms = new Map(actions.map((value, index) => [value, index]))
  const index = new Map<string, number>()
  const patterns: number[][] = []
  const demonstrated: number[][] = []
  const inputs: number[] = []
  const chosen: number[] = []
  const scoring: number[] = []
  for (let row = 0; row < rowCount; row += 1) {
    let complete = true
    for (let column = 0; column < columns; column += 1)
      if (!valid(column * rowCount + row)) complete = false
    if (!complete || (layout.membership !== null && at(layout.membership, row) !== 0)) continue
    const action = at(layout.action, row)
    const arm = arms.get(action)
    if (arm === undefined) return err({ kind: 'unlisted-action', value: action })
    const pattern = layout.inputs.map((column) => at(column, row))
    const key = pattern.join('\u0000')
    let slot = index.get(key)
    if (slot === undefined) {
      if (patterns.length === IMITATION_PATTERN_LIMIT) return err({ kind: 'too-many-patterns' })
      slot = patterns.length
      index.set(key, slot)
      patterns.push(pattern)
      demonstrated.push(actions.map(() => 0))
    }
    demonstrated[slot]![arm]! += 1
    inputs.push(...pattern)
    chosen.push(arm)
    if (layout.propensity !== null)
      scoring.push(action, at(layout.outcome, row), at(layout.propensity, row), slot)
  }
  if (chosen.length === 0) return err({ kind: 'no-logged-rows' })
  return ok({
    rows: chosen.length,
    inputs: Float64Array.from(inputs),
    chosen,
    patterns,
    demonstrated,
    scoring: layout.propensity === null ? null : Float64Array.from(scoring),
  })
}

export function describeLoggedRowsProblem(problem: LoggedRowsProblem): string {
  switch (problem.kind) {
    case 'no-logged-rows':
      return 'No logged row has a value in every column the run reads.'
    case 'unlisted-action':
      return `The logged rows hold action ${actionLabel(problem.value)}, which the bounds did not cover.`
    case 'too-many-patterns':
      return `The rule's inputs take more than ${IMITATION_PATTERN_LIMIT.toLocaleString('en-GB')} distinct combinations. Read fewer or coarser variables.`
  }
}

/** The mean and the range across seeds of each pattern's action probabilities. */
export function summariseSeeds(
  seeds: readonly (readonly (readonly number[])[])[],
): DecisionImitation['learned'] {
  const first = seeds[0]!
  const across = (pick: (values: readonly number[]) => number) =>
    first.map((row, p) => row.map((_, a) => pick(seeds.map((seed) => seed[p]![a]!))))
  return {
    mean: across((values) => values.reduce((total, v) => total + v, 0) / values.length),
    low: across((values) => Math.min(...values)),
    high: across((values) => Math.max(...values)),
  }
}

/** How closely the copied rule matches the demonstrator on one input pattern. */
export type CopiedPattern = {
  readonly label: string
  readonly rows: number
  /** The demonstrator's most frequent action and its share of this pattern's rows. */
  readonly action: number
  readonly loggedShare: number
  readonly copied: number
  readonly copiedLow: number
  readonly copiedHigh: number
  /** The largest difference, over all actions, between the copy's mean probability and the logged share. */
  readonly gap: number
}

export function copiedPatterns(
  imitation: DecisionImitation,
  actions: readonly number[],
): readonly CopiedPattern[] {
  return imitation.patterns
    .map((pattern, p) => {
      const counts = imitation.demonstrated[p]!
      const rows = counts.reduce((total, n) => total + n, 0)
      const top = counts.reduce((best, n, a) => (n > counts[best]! ? a : best), 0)
      const mean = imitation.learned.mean[p]!
      return {
        label:
          imitation.inputs.length === 0
            ? 'Every row'
            : imitation.inputs
                .map((input, i) => `${input.name} ${actionLabel(pattern[i]!)}`)
                .join(', '),
        rows,
        action: actions[top]!,
        loggedShare: counts[top]! / rows,
        copied: mean[top]!,
        copiedLow: imitation.learned.low[p]![top]!,
        copiedHigh: imitation.learned.high[p]![top]!,
        gap: Math.max(...counts.map((n, a) => Math.abs(mean[a]! - n / rows))),
      }
    })
    .sort((a, b) => b.rows - a.rows)
}

/** One scored plan beside the other evidence that bears on it. */
export type PlanRow = {
  readonly index: number
  readonly label: string
  readonly rule: ScoredRule
  readonly compared: boolean
  readonly scored: LoggedPolicyValueResult['plans'][number]
  /** Natural bounds and the randomised estimate exist only for plans that always choose one action. */
  readonly bounds: { readonly lower: number; readonly upper: number } | null
  readonly randomised: { readonly mean: number; readonly rows: number } | null
}

/** Every plan a run scored, in request order; empty when the run read no propensity column. */
export function planRows(run: DecisionEvidence): readonly PlanRow[] {
  if (run.scoring === null) return []
  const { request, result } = run.scoring
  return request.plans.map((plan, index) => {
    const action = plan.rule.kind === 'always' ? run.result.actions[plan.rule.action]! : null
    return {
      index,
      label: planLabel(plan.rule, run.action.name, run.result.actions),
      rule: plan.rule,
      compared: plan.compare,
      scored: result.plans[index]!,
      bounds: action === null ? null : { lower: action.lower, upper: action.upper },
      randomised: action?.randomised ?? null,
    }
  })
}
