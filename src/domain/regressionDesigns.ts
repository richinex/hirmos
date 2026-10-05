import { z } from 'zod'
import type { PanelRegressionDraft } from './panelRegression'
import type { CountRegressionDraft } from './countRegression'
import { assertNever, err, ok, type Result } from './dop'

/** Only supported specifications are representable. Analysis/model pairs are editable choices, not requests. */
export const regressionDesignSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('linear-event-study') }).strict(),
  z.object({ kind: z.literal('count-event-study') }).strict(),
  z.object({ kind: z.literal('count-cohort-summary') }).strict(),
  z.object({ kind: z.literal('interactions') }).strict(),
  z.object({ kind: z.literal('bacon') }).strict(),
])
export type RegressionDesign = z.infer<typeof regressionDesignSchema>
export type RegressionAnalysis = 'event-study' | 'cohort-summary' | 'interactions' | 'bacon'
export type RegressionOutcomeModel = 'linear' | 'count'
export const regressionAnalyses = [
  { value: 'event-study', label: 'Event study', requiresPanel: true },
  { value: 'cohort-summary', label: 'Cohort summary', requiresPanel: true },
  { value: 'interactions', label: 'Interactions / DDD', requiresPanel: false },
  { value: 'bacon', label: 'Bacon decomposition', requiresPanel: true },
] as const satisfies readonly { value: RegressionAnalysis; label: string; requiresPanel: boolean }[]
export const regressionDesigns = [
  {
    kind: 'linear-event-study',
    analysis: 'event-study',
    model: 'linear',
    runLabel: 'Fit event study',
    description:
      'Pooled event-time coefficients with unit and period fixed effects. Coefficients describe additive outcome differences relative to the omitted event period.',
  },
  {
    kind: 'count-event-study',
    analysis: 'event-study',
    model: 'count',
    runLabel: 'Fit event study',
    description:
      'One adopting cohort compared with never-treated units, using negative-binomial regression with unit and period fixed effects. Coefficients describe log rate contrasts relative to the period before adoption.',
  },
  {
    kind: 'count-cohort-summary',
    analysis: 'cohort-summary',
    model: 'count',
    runLabel: 'Fit cohort summary',
    description:
      'One adopting cohort compared with never-treated units, using negative-binomial regression with unit and period fixed effects and a post-adoption contrast.',
  },
  {
    kind: 'interactions',
    analysis: 'interactions',
    model: null,
    runLabel: 'Fit interaction regression',
    description:
      'Regression with main effects and lower-order interactions included alongside the selected interaction terms.',
  },
  {
    kind: 'bacon',
    analysis: 'bacon',
    model: null,
    runLabel: 'Decompose coefficient',
    description:
      'Decompose the two-way fixed-effects coefficient into weighted comparisons. This is a diagnostic, not a separate ATT estimator.',
  },
] as const satisfies readonly {
  kind: RegressionDesign['kind']
  analysis: RegressionAnalysis
  model: RegressionOutcomeModel | null
  runLabel: string
  description: string
}[]

export function regressionDesignInfo(design: RegressionDesign) {
  const definition = regressionDesigns.find((candidate) => candidate.kind === design.kind)
  if (definition === undefined) throw new Error('The regression specification is not registered.')
  return definition
}
export function selectRegressionDesign(
  analysis: RegressionAnalysis,
  model: RegressionOutcomeModel | null,
): Result<RegressionDesign, string> {
  const definition = regressionDesigns.find(
    (candidate) => candidate.analysis === analysis && candidate.model === model,
  )
  if (definition === undefined)
    return err(
      analysis === 'cohort-summary' && model === 'linear'
        ? 'A linear cohort-summary specification is not implemented.'
        : 'This analysis and outcome-model combination is not supported.',
    )
  return ok(regressionDesignSchema.parse({ kind: definition.kind }))
}
export function changeRegressionAnalysis(
  current: RegressionDesign,
  analysis: RegressionAnalysis,
): Result<RegressionDesign, string> {
  const previous = regressionDesignInfo(current)
  const model =
    analysis === 'event-study'
      ? (previous.model ?? 'linear')
      : analysis === 'cohort-summary'
        ? 'count'
        : null
  return selectRegressionDesign(analysis, model)
}

export type RegressionDesignState = {
  readonly selection: RegressionDesign
  readonly linear: PanelRegressionDraft
  readonly cohort: CountRegressionDraft
}
/** Pure transition: temporal drafts are not an input and therefore cannot be overwritten. */
export function transitionRegressionDesign(
  state: RegressionDesignState,
  next: RegressionDesign,
): RegressionDesignState {
  const previous = regressionDesignInfo(state.selection)
  switch (next.kind) {
    case 'linear-event-study': {
      const model =
        state.linear.model.kind === 'eventStudy'
          ? state.linear.model
          : {
              kind: 'eventStudy' as const,
              onset: null,
              covariates: [],
              first: '-3',
              last: '3',
              reference: '-1',
              tails: 'reference' as const,
            }
      return {
        ...state,
        selection: next,
        linear: {
          ...state.linear,
          outcome: previous.model === 'count' ? state.cohort.outcome : state.linear.outcome,
          model,
        },
      }
    }
    case 'count-event-study':
    case 'count-cohort-summary': {
      const kind = next.kind === 'count-event-study' ? 'events' : 'summary'
      const onset =
        state.cohort.model.kind === 'events' || state.cohort.model.kind === 'summary'
          ? state.cohort.model.onset
          : null
      const cohort =
        state.cohort.model.kind === 'events' || state.cohort.model.kind === 'summary'
          ? state.cohort.model.cohort
          : ''
      const model: CountRegressionDraft['model'] =
        state.cohort.model.kind === kind
          ? state.cohort.model
          : kind === 'events'
            ? { kind, onset, cohort, window: { kind: 'all' } }
            : { kind, onset, cohort }
      return {
        ...state,
        selection: next,
        cohort: {
          ...state.cohort,
          outcome: previous.model === 'linear' ? state.linear.outcome : state.cohort.outcome,
          model,
        },
      }
    }
    case 'interactions':
    case 'bacon': {
      const kind = next.kind
      const model: PanelRegressionDraft['model'] =
        state.linear.model.kind === kind
          ? state.linear.model
          : kind === 'interactions'
            ? { kind, predictors: [], terms: [] }
            : { kind, onset: null, covariates: [] }
      return { ...state, selection: next, linear: { ...state.linear, model } }
    }
    default:
      return assertNever(next)
  }
}

export type CountOutcomeReadiness =
  | { readonly kind: 'unselected' }
  | { readonly kind: 'checking' }
  | { readonly kind: 'ready' }
  | { readonly kind: 'refused'; readonly reason: string }
export function assessCountOutcome(values: Iterable<number>): CountOutcomeReadiness {
  let observations = 0
  for (const value of values) {
    observations++
    if (!Number.isFinite(value) || !Number.isInteger(value) || value < 0)
      return {
        kind: 'refused',
        reason: 'Count models require finite, non-negative integer outcomes in the prepared data.',
      }
  }
  return observations === 0
    ? { kind: 'refused', reason: 'There are no prepared outcome values to check.' }
    : { kind: 'ready' }
}
