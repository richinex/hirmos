import { z } from 'zod'
import { brand, err, ok, type Result } from './dop'
import type { CountRegressionDraft } from './countRegression'
import type { PanelRegressionDraft } from './panelRegression'

const column = z
  .string()
  .min(1)
  .transform((id) => brand<string, 'ColumnId'>(id))
const finiteText = z
  .string()
  .refine((text) => text.trim() !== '' && Number.isFinite(Number(text)), 'Enter a finite number.')
const integerText = (min: number) =>
  finiteText.refine(
    (text) => Number.isSafeInteger(Number(text)) && Number(text) >= min,
    `Enter a whole number of at least ${min}.`,
  )
const confidence = finiteText.refine(
  (text) => Number(text) > 0 && Number(text) < 1,
  'Confidence must be between 0 and 1.',
)

export const countRegressionDraftSchema: z.ZodType<CountRegressionDraft> = z
  .object({
    outcome: column.nullable(),
    family: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('negativeBinomial'), exposure: column.nullable() }).strict(),
      z.object({ kind: z.literal('binomial'), trials: column.nullable() }).strict(),
    ]),
    model: z.discriminatedUnion('kind', [
      z
        .object({
          kind: z.literal('lags'),
          predictor: column.nullable(),
          lags: z
            .string()
            .refine(
              (text) =>
                text.trim() !== '' &&
                text
                  .split(',')
                  .every(
                    (value) =>
                      value.trim() !== '' &&
                      Number.isSafeInteger(Number(value)) &&
                      Number(value) >= 0,
                  ),
              'Supply comma-separated non-negative whole-number lags.',
            ),
        })
        .strict(),
      z
        .object({
          kind: z.literal('events'),
          onset: column.nullable(),
          cohort: z.string(),
          window: z.discriminatedUnion('kind', [
            z.object({ kind: z.literal('all') }).strict(),
            z
              .object({
                kind: z.literal('finite'),
                first: finiteText.refine(
                  (text) => Number.isSafeInteger(Number(text)) && Number(text) <= -1,
                ),
                last: integerText(0),
              })
              .strict(),
          ]),
        })
        .strict(),
      z
        .object({ kind: z.literal('summary'), onset: column.nullable(), cohort: z.string() })
        .strict(),
      z
        .object({
          kind: z.literal('interrupted'),
          intervention: integerText(3),
          horizon: integerText(0),
          bandwidth: integerText(0),
        })
        .strict(),
    ]),
    covariates: z.array(column).refine((ids) => new Set(ids).size === ids.length),
    confidence,
    iterations: integerText(1),
    tolerance: finiteText.refine((text) => Number(text) > 0),
  })
  .strict()

export function countRegressionColumns(value: CountRegressionDraft): readonly (string | null)[] {
  return [
    value.outcome,
    value.family.kind === 'binomial' ? value.family.trials : value.family.exposure,
    ...value.covariates,
    ...(value.model.kind === 'lags'
      ? [value.model.predictor]
      : value.model.kind === 'interrupted'
        ? []
        : [value.model.onset]),
  ]
}

export function checkCountRegressionDraft(
  value: CountRegressionDraft,
  panel: boolean,
): Result<CountRegressionDraft, string> {
  if (value.outcome === null) return err('Choose an outcome.')
  const denominator = value.family.kind === 'binomial' ? value.family.trials : value.family.exposure
  if (value.family.kind === 'binomial' && denominator === null)
    return err('Choose the trials column.')
  if (denominator === value.outcome) return err('The denominator and outcome must differ.')
  const model = value.model
  if (
    model.kind === 'lags' &&
    (model.predictor === null ||
      model.predictor === value.outcome ||
      model.predictor === denominator)
  )
    return err('Choose a predictor distinct from the outcome and denominator.')
  if (
    (model.kind === 'events' || model.kind === 'summary') &&
    (!panel ||
      model.onset === null ||
      model.cohort.trim() === '' ||
      value.family.kind !== 'negativeBinomial')
  )
    return err(
      'Cohort designs require panel data, a treatment indicator, a cohort period label and the negative-binomial model.',
    )
  if (model.kind === 'interrupted' && panel)
    return err('Use a single time series for this interrupted count model.')
  const reserved = [
    value.outcome,
    denominator,
    ...(model.kind === 'lags'
      ? [model.predictor]
      : model.kind === 'interrupted'
        ? []
        : [model.onset]),
  ]
  if (value.covariates.some((id) => reserved.includes(id)))
    return err(
      'Covariates must differ from the model’s outcome, denominator, predictor and treatment indicator.',
    )
  return ok(value)
}

export function panelRegressionColumns(value: PanelRegressionDraft): readonly (string | null)[] {
  return [
    value.outcome,
    value.cluster,
    value.weight,
    ...(value.model.kind === 'interactions'
      ? [...value.model.predictors.map((p) => p.column), ...value.model.terms.flat()]
      : [value.model.onset, ...value.model.covariates]),
  ]
}

export function checkPanelRegressionDraft(
  value: PanelRegressionDraft,
  panel: boolean,
): Result<PanelRegressionDraft, string> {
  if (value.outcome === null) return err('Choose an outcome.')
  if (!confidence.safeParse(value.confidence).success)
    return err('Confidence must be between 0 and 1.')
  const model = value.model
  if (model.kind !== 'interactions' && !panel)
    return err('Prepare panel data to use adoption timing.')
  if (model.kind !== 'interactions' && model.onset === null)
    return err('Choose an absorbing treatment indicator.')
  if (model.kind !== 'bacon' && !panel && value.cluster === null)
    return err('Choose a cluster column.')
  if (model.kind === 'interactions') {
    const ids = model.predictors.map((p) => p.column)
    if (
      !ids.length ||
      !model.terms.length ||
      new Set(ids).size !== ids.length ||
      model.terms.some(
        (term) =>
          term.length < 1 ||
          term.length > 3 ||
          new Set(term).size !== term.length ||
          term.some((id) => !ids.includes(id)),
      )
    )
      return err(
        'Choose distinct predictors and terms containing one to three selected predictors.',
      )
  }
  if (model.kind === 'eventStudy') {
    const [first, last, reference] = [model.first, model.last, model.reference].map(Number)
    if (
      [model.first, model.last, model.reference].some((text) => text.trim() === '') ||
      ![first, last, reference].every(Number.isSafeInteger) ||
      first! > last! ||
      reference! < first! ||
      reference! > last!
    )
      return err('Enter a whole-number event window and a reference period inside it.')
  }
  return ok(value)
}
