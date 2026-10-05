import { z } from 'zod'
import type { ColumnId } from './dataset'
import { brand } from './dop'

const index = z.number().int().nonnegative()
const finite = z.number().finite()
export const codingSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('numeric') }).strict(),
  z.object({ kind: z.literal('categorical'), reference: z.string().min(1) }).strict(),
])
export const regressionSpecificationSchema = z.discriminatedUnion('kind', [
  z
    .object({
      kind: z.literal('eventStudy'),
      keys: z.array(z.tuple([index, z.number().int().safe()])),
      adoption: z.array(z.tuple([index, z.number().int().safe().nullable()])),
      covariates: z.array(index),
      window: z
        .object({
          first: z.number().int().safe(),
          last: z.number().int().safe(),
          reference: z.number().int().safe(),
          tails: z.enum(['reference', 'bin']),
        })
        .strict(),
    })
    .strict(),
  z
    .object({
      kind: z.literal('interactions'),
      predictors: z
        .array(z.object({ column: index, name: z.string().min(1), coding: codingSchema }).strict())
        .min(1),
      terms: z.array(z.array(index).min(1).max(3)).min(1),
    })
    .strict(),
])
export const panelRegressionRequestSchema = z
  .object({
    rows: index.min(1),
    columns: index.min(1),
    outcome: index,
    names: z.array(z.string().min(1)),
    weights: index.nullable(),
    cluster: index,
    confidence: finite.gt(0).lt(1),
    specification: regressionSpecificationSchema,
  })
  .strict()
  .superRefine((r, ctx) => {
    const fail = (message: string) => ctx.addIssue({ code: 'custom', message })
    if (
      r.names.length !== r.columns ||
      new Set(r.names).size !== r.columns ||
      [r.outcome, r.cluster, ...(r.weights === null ? [] : [r.weights])].some((i) => i >= r.columns)
    )
      fail('Model columns do not match the recorded matrix.')
    if (r.outcome === r.cluster || r.weights === r.outcome || r.weights === r.cluster)
      fail('Outcome, cluster and weight columns must have separate roles.')
    const s = r.specification
    if (s.kind === 'eventStudy') {
      if (
        s.keys.length !== r.rows ||
        new Set(s.adoption.map(([u]) => u)).size !== s.adoption.length
      )
        fail('Panel keys and adoption records must match the rows.')
      if (
        s.window.first >= s.window.last ||
        s.window.reference < s.window.first ||
        s.window.reference > s.window.last ||
        (s.window.tails === 'bin' &&
          (s.window.reference === s.window.first || s.window.reference === s.window.last))
      )
        fail(
          'Choose an event window containing its reference; binned endpoints cannot be the reference.',
        )
      if (
        new Set(s.covariates).size !== s.covariates.length ||
        s.covariates.some(
          (i) => i >= r.columns || i === r.outcome || i === r.cluster || i === r.weights,
        )
      )
        fail('Covariates must be distinct from outcome, cluster and weights.')
    } else {
      if (
        new Set(s.predictors.map((p) => p.column)).size !== s.predictors.length ||
        s.predictors.some(
          (p) =>
            p.column >= r.columns ||
            p.name !== r.names[p.column] ||
            p.column === r.outcome ||
            p.column === r.weights,
        )
      )
        fail('Choose distinct model predictors with their recorded column names.')
      if (
        s.terms.some((t) => new Set(t).size !== t.length || t.some((i) => i >= s.predictors.length))
      )
        fail('Each term must contain distinct selected predictors.')
    }
  })
export type PanelRegressionRequest = z.infer<typeof panelRegressionRequestSchema>
export const coefficientSchema = z
  .object({
    index,
    name: z.string(),
    estimate: finite,
    standardError: finite.nonnegative(),
    degreesOfFreedom: finite.positive(),
    pValue: finite.min(0).max(1),
    lower: finite,
    upper: finite,
  })
  .strict()
export const panelRegressionEvidenceSchema = z
  .object({
    version: z.literal(1),
    request: panelRegressionRequestSchema,
    observations: index.min(1),
    clusters: index.min(2),
    terms: z.array(coefficientSchema).min(1),
    omitted: z.array(z.string()),
    covariance: z.array(z.array(finite)),
    events: z.array(z.object({ index, period: z.number().int(), support: index.min(1) }).strict()),
    leadTest: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('unavailable'), reason: z.string().min(1) }).strict(),
      z
        .object({
          kind: z.literal('recorded'),
          statistic: finite.nonnegative(),
          numeratorDf: index.min(1),
          denominatorDf: finite.positive(),
          pValue: finite.min(0).max(1),
        })
        .strict(),
    ]),
  })
  .strict()
  .superRefine((e, ctx) => {
    const fail = (message: string) => ctx.addIssue({ code: 'custom', message })
    if (
      e.observations > e.request.rows ||
      e.covariance.length !== e.terms.length ||
      e.covariance.some((row) => row.length !== e.terms.length) ||
      new Set(e.terms.map((t) => t.index)).size !== e.terms.length
    )
      fail('Regression coefficient identities and covariance dimensions disagree.')
    if (e.terms.some((t) => t.lower > t.upper || t.estimate < t.lower || t.estimate > t.upper))
      fail('Regression intervals must contain their coefficient estimates.')
    if (
      e.request.specification.kind === 'interactions' &&
      (e.events.length > 0 || e.leadTest.kind !== 'unavailable')
    )
      fail('Event-time evidence cannot describe an interaction-only regression.')
  })
export type PanelRegressionEvidence = z.infer<typeof panelRegressionEvidenceSchema>

export const baconSpecificationSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('unadjusted') }).strict(),
  z.object({ kind: z.literal('adjusted'), controls: z.array(index).min(1) }).strict(),
])
export const baconRequestSchema = z
  .object({
    rows: index.min(1),
    columns: index.min(2),
    units: z.array(z.string().min(1)),
    times: z.array(z.number().int().safe()),
    outcome: index,
    treatment: index,
    specification: baconSpecificationSchema,
  })
  .strict()
  .superRefine((r, ctx) => {
    const used = [
      r.outcome,
      r.treatment,
      ...(r.specification.kind === 'adjusted' ? r.specification.controls : []),
    ]
    if (
      r.units.length !== r.rows ||
      r.times.length !== r.rows ||
      new Set(used).size !== used.length ||
      used.some((i) => i >= r.columns)
    )
      ctx.addIssue({
        code: 'custom',
        message:
          'Decomposition columns and panel keys must match the matrix and have distinct roles.',
      })
  })
export type BaconRequest = z.infer<typeof baconRequestSchema>
const timing = z.number().int().safe()
const comparison = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('treatedVsNever'), adoption: timing }).strict(),
  z.object({ kind: z.literal('earlierVsLater'), earlier: timing, later: timing }).strict(),
  z.object({ kind: z.literal('laterVsEarlier'), earlier: timing, later: timing }).strict(),
  z.object({ kind: z.literal('laterVsAlways'), adoption: timing }).strict(),
  z.object({ kind: z.literal('bothTreated'), earlier: timing, later: timing }).strict(),
])
const component = z.object({ comparison, estimate: finite, weight: finite }).strict()
export const baconEvidenceSchema = z
  .object({
    version: z.literal(1),
    specification: baconSpecificationSchema,
    observations: index.min(1),
    units: index.min(2),
    twfe: finite,
    reconstructed: finite,
    decomposition: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('unadjusted'), components: z.array(component).min(1) }).strict(),
      z
        .object({
          kind: z.literal('adjusted'),
          withinEstimate: finite,
          withinWeight: finite,
          between: z.array(component).min(1),
        })
        .strict(),
    ]),
  })
  .strict()
  .superRefine((e, ctx) => {
    if (e.specification.kind !== e.decomposition.kind)
      ctx.addIssue({
        code: 'custom',
        message: 'The decomposition and adjustment specification disagree.',
      })
  })
export type BaconEvidence = z.infer<typeof baconEvidenceSchema>

/** Editable text is kept separate from a validated numerical request. */
export interface PanelRegressionDraft {
  readonly outcome: ColumnId | null
  readonly cluster: ColumnId | null
  readonly weight: ColumnId | null
  readonly confidence: string
  readonly model:
    | {
        readonly kind: 'eventStudy'
        readonly onset: ColumnId | null
        readonly covariates: readonly ColumnId[]
        readonly first: string
        readonly last: string
        readonly reference: string
        readonly tails: 'reference' | 'bin'
      }
    | {
        readonly kind: 'interactions'
        readonly predictors: readonly {
          readonly column: ColumnId
          readonly coding: z.infer<typeof codingSchema>
        }[]
        readonly terms: readonly (readonly ColumnId[])[]
      }
    | {
        readonly kind: 'bacon'
        readonly onset: ColumnId | null
        readonly covariates: readonly ColumnId[]
      }
}
export const initialPanelRegression = (): PanelRegressionDraft => ({
  outcome: null,
  cluster: null,
  weight: null,
  confidence: '0.95',
  model: {
    kind: 'eventStudy',
    onset: null,
    covariates: [],
    first: '-3',
    last: '3',
    reference: '-1',
    tails: 'reference',
  },
})

const columnId = z
  .string()
  .min(1)
  .transform((v) => brand<string, 'ColumnId'>(v))
export const panelRegressionDraftSchema = z
  .object({
    outcome: columnId.nullable(),
    cluster: columnId.nullable(),
    weight: columnId.nullable(),
    confidence: z.string(),
    model: z.discriminatedUnion('kind', [
      z
        .object({
          kind: z.literal('eventStudy'),
          onset: columnId.nullable(),
          covariates: z.array(columnId),
          first: z.string(),
          last: z.string(),
          reference: z.string(),
          tails: z.enum(['reference', 'bin']),
        })
        .strict(),
      z
        .object({
          kind: z.literal('interactions'),
          predictors: z.array(z.object({ column: columnId, coding: codingSchema }).strict()),
          terms: z.array(z.array(columnId)),
        })
        .strict(),
      z
        .object({
          kind: z.literal('bacon'),
          onset: columnId.nullable(),
          covariates: z.array(columnId),
        })
        .strict(),
    ]),
  })
  .strict()

export function sameSpecification(a: unknown, b: unknown): boolean {
  if (a === b) return true
  if (Array.isArray(a) && Array.isArray(b))
    return a.length === b.length && a.every((v, i) => sameSpecification(v, b[i]))
  if (typeof a !== 'object' || a === null || typeof b !== 'object' || b === null) return false
  const aa = Object.entries(a),
    bb = Object.entries(b)
  return (
    aa.length === bb.length &&
    aa.every(([k, v]) => Object.hasOwn(b, k) && sameSpecification(v, Reflect.get(b, k)))
  )
}

/** Saved controls must recreate the same numerical specification, not merely the same method name. */
export function regressionControlsMatch(
  d: PanelRegressionDraft,
  r: PanelRegressionRequest,
  variables: readonly { readonly id: ColumnId; readonly name: string }[],
): boolean {
  const at = (id: ColumnId | null) => (id === null ? null : variables.findIndex((v) => v.id === id))
  if (
    at(d.outcome) !== r.outcome ||
    at(d.weight) !== r.weights ||
    Number(d.confidence) !== r.confidence ||
    (d.cluster === null
      ? r.cluster !== variables.length || r.columns !== variables.length + 1
      : at(d.cluster) !== r.cluster || r.columns !== variables.length)
  )
    return false
  const model = d.model,
    s = r.specification
  switch (model.kind) {
    case 'bacon':
      return false
    case 'eventStudy':
      return (
        s.kind === 'eventStudy' &&
        model.onset !== null &&
        at(model.onset) !== -1 &&
        sameSpecification(s.covariates, model.covariates.map(at)) &&
        sameSpecification(s.window, {
          first: Number(model.first),
          last: Number(model.last),
          reference: Number(model.reference),
          tails: model.tails,
        })
      )
    case 'interactions':
      return (
        s.kind === 'interactions' &&
        sameSpecification(
          s.predictors,
          model.predictors.map((p) => ({
            column: at(p.column),
            name: variables.find((v) => v.id === p.column)?.name,
            coding: p.coding,
          })),
        ) &&
        sameSpecification(
          s.terms,
          model.terms.map((t) => t.map((id) => model.predictors.findIndex((p) => p.column === id))),
        )
      )
  }
}
export function baconControlsMatch(
  d: PanelRegressionDraft,
  r: BaconRequest,
  variables: readonly { readonly id: ColumnId }[],
): boolean {
  const at = (id: ColumnId | null) => (id === null ? -1 : variables.findIndex((v) => v.id === id))
  return (
    d.model.kind === 'bacon' &&
    at(d.outcome) === r.outcome &&
    at(d.model.onset) === r.treatment &&
    sameSpecification(
      r.specification,
      d.model.covariates.length === 0
        ? { kind: 'unadjusted' }
        : { kind: 'adjusted', controls: d.model.covariates.map(at) },
    )
  )
}
