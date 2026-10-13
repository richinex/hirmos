import { z } from 'zod'
import {
  DISCOVERY_METHOD_GROUPS,
  DISCOVERY_LAG_OPTIONS,
  PCMCI_ALPHA_OPTIONS,
  DYNOTEARS_PENALTY_OPTIONS,
  OCSE_SHUFFLE_OPTIONS,
  cdnotsContextSchema,
  cdnotsMissingStrategySchema,
  constraintCiTestSchema,
  type DiscoveryConfiguration,
} from './discovery'
import { brand } from './dop'
import type { ConfigurationSetting } from './estimation'

export const discoveryMethodSchema = z
  .object({ method: z.enum(DISCOVERY_METHOD_GROUPS.flatMap((group) => group.methods)) })
  .strict()

export function discoveryBackgroundProblem(
  configuration: DiscoveryConfiguration,
  columns: number,
): string | null {
  if (!('background' in configuration)) return null
  const b = configuration.background
  const pairs = [...b.required, ...b.forbidden]
  if (
    pairs.some(
      ([from, to]) => from === to || from < 0 || to < 0 || from >= columns || to >= columns,
    )
  )
    return 'Background directions must connect two different variables in the prepared data.'
  if (b.required.some(([from, to]) => b.forbidden.some(([a, c]) => a === from && c === to)))
    return 'A direction cannot be both required and forbidden.'
  if (b.tiers.length > columns)
    return 'The tier assignments exceed the number of prepared variables.'
  if (b.forbiddenWithinTiers.some((tier) => !b.tiers.includes(tier)))
    return 'A forbidden within-tier direction must refer to an assigned tier.'
  return null
}
const lag = z.union(DISCOVERY_LAG_OPTIONS.map((value) => z.literal(value)))
const alpha = z.union(PCMCI_ALPHA_OPTIONS.map((value) => z.literal(value)))
const penalty = z.union(DYNOTEARS_PENALTY_OPTIONS.map((value) => z.literal(value)))
const integer = z.number().int().nonnegative()
const finite = z.number().finite()
const seed = integer.max(4294967295)
const background = z
  .object({
    required: z.array(z.tuple([integer, integer])),
    forbidden: z.array(z.tuple([integer, integer])),
    requiredPatterns: z.array(z.tuple([z.string(), z.string()])),
    forbiddenPatterns: z.array(z.tuple([z.string(), z.string()])),
    tiers: z.array(integer.nullable()),
    forbiddenWithinTiers: z.array(integer),
  })
  .strict()
const constraint = {
  alpha,
  maxDepth: integer.max(5).nullable(),
  ciTest: constraintCiTestSchema,
  background,
}
const neural = {
  lambda: finite.nonnegative(),
  ridgeLambda: finite.nonnegative(),
  learningRate: finite.positive(),
  checkEvery: integer.positive(),
  lookback: integer.positive(),
  seed,
}
export const discoverySettingsSchema = z
  .discriminatedUnion('kind', [
    z.object({ kind: z.literal('pc-stable'), ...constraint }).strict(),
    z
      .object({
        kind: z.literal('fci'),
        ...constraint,
        maxPathLength: z
          .union([
            z.literal(1),
            z.literal(2),
            z.literal(3),
            z.literal(4),
            z.literal(5),
            z.literal(6),
            z.literal(8),
            z.literal(10),
          ])
          .nullable(),
      })
      .strict(),
    z
      .object({
        kind: z.literal('jpcmci-plus'),
        tauMax: lag,
        pcAlpha: alpha,
        assignments: z.array(
          z
            .object({
              column: z
                .string()
                .min(1)
                .transform((id) => brand<string, 'ColumnId'>(id)),
              role: z.enum(['system', 'timeContext', 'spaceContext']),
            })
            .strict(),
        ),
        timeDummy: z.boolean(),
        spaceDummy: z.boolean(),
      })
      .strict(),
    z
      .object({
        kind: z.literal('rpcmci'),
        numRegimes: integer.min(2).max(6),
        maxTransitions: integer,
        switchThres: finite.min(0).max(1),
        numIterations: integer.min(1).max(100),
        maxAnneal: integer.min(1).max(50),
        tauMin: integer.max(6),
        tauMax: lag,
        pcAlpha: alpha,
        alphaLevel: alpha,
        seed,
      })
      .strict(),
    z
      .object({
        kind: z.literal('cdnots'),
        maxLag: lag,
        alpha,
        missing: cdnotsMissingStrategySchema,
        context: cdnotsContextSchema,
      })
      .strict(),
    z
      .object({
        kind: z.literal('cdnots-plus'),
        maxLag: lag,
        alpha,
        missing: cdnotsMissingStrategySchema,
        context: cdnotsContextSchema,
      })
      .strict(),
    z
      .object({
        kind: z.literal('grace'),
        maxLag: lag,
        alpha,
        context: cdnotsContextSchema,
        gateThreshold: finite.min(0).max(1),
        epochs: integer.min(1).max(2000),
        patience: integer.min(1).max(500),
        seed,
      })
      .strict(),
    z
      .object({
        kind: z.literal('cmlp'),
        ...neural,
        lag,
        hidden: z.tuple([integer.min(1).max(256)]).rest(integer.min(1).max(256)),
        activation: z.enum(['sigmoid', 'tanh', 'relu', 'leakyRelu', 'identity']),
        penalty: z.enum(['groupLasso', 'groupSparseGroupLasso', 'hierarchical']),
        maxIter: integer.min(1).max(50000),
      })
      .strict(),
    z
      .object({
        kind: z.literal('clstm'),
        ...neural,
        context: integer.min(1).max(100),
        hidden: integer.min(1).max(256),
        maxIter: integer.min(1).max(20000),
      })
      .strict(),
    z.object({ kind: z.literal('direct-lingam') }).strict(),
    z.object({ kind: z.literal('pcmci-plus'), tauMax: lag, pcAlpha: alpha }).strict(),
    z.object({ kind: z.literal('lpcmci'), tauMax: lag, pcAlpha: alpha }).strict(),
    z
      .object({ kind: z.literal('dynotears'), maxLag: lag, lambdaW: penalty, lambdaA: penalty })
      .strict(),
    z.object({ kind: z.literal('var-lingam'), maxLag: lag, prune: z.boolean() }).strict(),
    z
      .object({
        kind: z.literal('ocse'),
        maxLag: lag,
        alpha,
        nShuffles: z.union(OCSE_SHUFFLE_OPTIONS.map((value) => z.literal(value))),
        method: z.enum(['gaussian', 'knn']),
        k: z.literal(5),
      })
      .strict(),
  ])
  .superRefine((value, ctx) => {
    if ((value.kind === 'cmlp' || value.kind === 'clstm') && value.checkEvery > value.maxIter)
      ctx.addIssue({
        code: 'custom',
        message: 'The check interval cannot exceed the maximum iterations.',
      })
    if (value.kind === 'cmlp' && value.hidden.length > 4)
      ctx.addIssue({ code: 'custom', message: 'Use at most four hidden layers.' })
    if (value.kind === 'grace' && value.patience > value.epochs)
      ctx.addIssue({ code: 'custom', message: 'Patience cannot exceed the number of epochs.' })
    if (value.kind === 'rpcmci' && value.tauMin > value.tauMax)
      ctx.addIssue({ code: 'custom', message: 'The minimum lag cannot exceed the maximum lag.' })
  })

const labels: Record<string, string> = {
  tauMax: 'Maximum lag',
  maxLag: 'Maximum lag',
  lag: 'Maximum lag',
  pcAlpha: 'PC alpha',
  alpha: 'Significance level',
  alphaLevel: 'Graph alpha',
  maxDepth: 'Maximum depth',
  maxPathLength: 'Maximum path length',
  ciTest: 'CI test',
  timeDummy: 'Generate period context',
  spaceDummy: 'Generate unit context',
  numRegimes: 'Regimes',
  maxTransitions: 'Maximum transitions',
  switchThres: 'Switching threshold',
  numIterations: 'Iterations per annealing',
  maxAnneal: 'Annealing runs',
  tauMin: 'Minimum lag',
  lambdaW: 'Contemporaneous λ',
  lambdaA: 'Lagged λ',
  prune: 'Prune',
  nShuffles: 'Permutation shuffles',
  method: 'Information estimator',
  k: 'Neighbours',
  missing: 'Missing observations',
  context: 'Time context',
  gateThreshold: 'Gate threshold',
  epochs: 'Epochs',
  patience: 'Patience',
  seed: 'Seed',
  hidden: 'Hidden units',
  activation: 'Activation',
  penalty: 'Penalty',
  lambda: 'Sparsity penalty',
  ridgeLambda: 'Ridge penalty',
  learningRate: 'Learning rate',
  maxIter: 'Maximum iterations',
  checkEvery: 'Check interval',
  lookback: 'Lookback',
}
const values: Record<string, string> = {
  fisherZ: 'Fisher Z',
  kci: 'KCI',
  gaussian: 'Gaussian',
  knn: 'Nearest neighbours',
  pairwiseComplete: 'Pairwise complete',
  varEm: 'VAR-EM imputation',
  none: 'None',
  linear: 'Linear time',
  linearSine: 'Linear + sine',
  linearExponential: 'Linear + exponential',
  linearQuadratic: 'Linear + quadratic',
  step: 'Step',
  stepLinear: 'Step + linear',
  system: 'System variable',
  timeContext: 'Observed time context',
  spaceContext: 'Observed unit context',
  sigmoid: 'Sigmoid',
  tanh: 'Tanh',
  relu: 'ReLU',
  leakyRelu: 'Leaky ReLU',
  identity: 'Identity',
  groupLasso: 'Group lasso',
  groupSparseGroupLasso: 'Group sparse group lasso',
  hierarchical: 'Hierarchical',
}

export function discoverySettings(
  configuration: DiscoveryConfiguration,
  method: string,
  columns: readonly { readonly id: string; readonly name: string }[],
): readonly ConfigurationSetting[] {
  const settings: ConfigurationSetting[] = [{ key: 'method', label: 'Method', value: method }]
  for (const [key, value] of Object.entries(configuration)) {
    if (key === 'kind' || key === 'background' || key === 'assignments') continue
    settings.push({
      key,
      label: key === 'context' && configuration.kind === 'clstm' ? 'Context length' : labels[key]!,
      value:
        value === null
          ? 'Automatic'
          : typeof value === 'boolean'
            ? value
              ? 'Yes'
              : 'No'
            : Array.isArray(value)
              ? value.join(', ')
              : (values[String(value)] ?? String(value)),
    })
  }
  if (configuration.kind === 'jpcmci-plus') {
    for (const column of columns)
      settings.push({
        key: `role:${column.id}`,
        label: `${column.name} role`,
        value:
          values[configuration.assignments.find((a) => a.column === column.id)?.role ?? 'system']!,
      })
  }
  if ('background' in configuration) {
    const b = configuration.background
    for (const kind of ['required', 'forbidden'] as const) {
      settings.push({
        key: kind,
        label: kind === 'required' ? 'Required directions' : 'Forbidden directions',
        value:
          b[kind]
            .map(
              ([from, to]) =>
                `${columns[from]?.name ?? 'Unknown variable'} → ${columns[to]?.name ?? 'Unknown variable'}`,
            )
            .join('; ') || 'None',
      })
      settings.push({
        key: `${kind}Patterns`,
        label: kind === 'required' ? 'Required patterns' : 'Forbidden patterns',
        value: b[`${kind}Patterns`].map(([from, to]) => `${from} → ${to}`).join('; ') || 'None',
      })
    }
    settings.push({
      key: 'tiers',
      label: 'Tiers',
      value:
        b.tiers
          .flatMap((tier, i) =>
            tier === null ? [] : [`${columns[i]?.name ?? 'Unknown variable'}: ${tier}`],
          )
          .join('; ') || 'None',
    })
    settings.push({
      key: 'withinTiers',
      label: 'Forbidden within tiers',
      value: b.forbiddenWithinTiers.join(', ') || 'None',
    })
  }
  return settings
}
