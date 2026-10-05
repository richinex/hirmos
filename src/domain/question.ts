import { assertNever, brand, err, ok, type Brand, type Result } from './dop'
import type { ColumnId, DatasetProfile, DatasetProfileId } from './dataset'

export type StudyQuestionId = Brand<string, 'StudyQuestionId'>

export type QuestionProblem =
  | { readonly kind: 'missing-treatment' }
  | { readonly kind: 'missing-outcome' }
  | { readonly kind: 'same-column' }
  | { readonly kind: 'unknown-column'; readonly value: string }

export type QuestionState =
  | {
      readonly kind: 'draft'
      readonly treatment: ColumnId | null
      readonly outcome: ColumnId | null
      readonly problem: QuestionProblem | null
    }
  | { readonly kind: 'framed'; readonly question: StudyQuestion }

export type QuestionEvent =
  | { readonly type: 'treatment-changed'; readonly value: string }
  | { readonly type: 'outcome-changed'; readonly value: string }
  | { readonly type: 'submitted' }
  | { readonly type: 'edit-requested' }

export interface StudyQuestion {
  readonly id: StudyQuestionId
  readonly dataset: DatasetProfileId
  readonly treatment: ColumnId
  readonly outcome: ColumnId
}

export const EMPTY_QUESTION: QuestionState = {
  kind: 'draft',
  treatment: null,
  outcome: null,
  problem: null,
}

export function columnFromProfile(
  profile: DatasetProfile,
  raw: string,
): Result<ColumnId, QuestionProblem> {
  const column = profile.columns.find((candidate) => candidate.id === raw)
  return column ? ok(column.id) : err({ kind: 'unknown-column', value: raw })
}

export function frameQuestion(
  profile: DatasetProfile,
  treatment: ColumnId | null,
  outcome: ColumnId | null,
): Result<StudyQuestion, QuestionProblem> {
  if (treatment === null) return err({ kind: 'missing-treatment' })
  if (outcome === null) return err({ kind: 'missing-outcome' })
  if (treatment === outcome) return err({ kind: 'same-column' })
  return ok({
    id: brand<string, 'StudyQuestionId'>(`${profile.id}:${treatment}->${outcome}`),
    dataset: profile.id,
    treatment,
    outcome,
  })
}

export function stepQuestion(
  profile: DatasetProfile,
  state: QuestionState,
  event: QuestionEvent,
): QuestionState {
  switch (state.kind) {
    case 'framed':
      return event.type === 'edit-requested'
        ? {
            kind: 'draft',
            treatment: state.question.treatment,
            outcome: state.question.outcome,
            problem: null,
          }
        : state
    case 'draft': {
      if (event.type === 'treatment-changed') {
        const parsed = columnFromProfile(profile, event.value)
        return parsed.ok
          ? { ...state, treatment: parsed.value, problem: null }
          : { ...state, problem: parsed.error }
      }
      if (event.type === 'outcome-changed') {
        const parsed = columnFromProfile(profile, event.value)
        return parsed.ok
          ? { ...state, outcome: parsed.value, problem: null }
          : { ...state, problem: parsed.error }
      }
      if (event.type === 'submitted') {
        const framed = frameQuestion(profile, state.treatment, state.outcome)
        return framed.ok
          ? { kind: 'framed', question: framed.value }
          : { ...state, problem: framed.error }
      }
      return state
    }
    default:
      return assertNever(state)
  }
}

export function describeQuestionProblem(problem: QuestionProblem): string {
  switch (problem.kind) {
    case 'missing-treatment':
      return 'Choose a treatment.'
    case 'missing-outcome':
      return 'Choose an outcome.'
    case 'same-column':
      return 'Treatment and outcome must be different columns.'
    case 'unknown-column':
      return 'That column is not part of this dataset profile.'
    default:
      return assertNever(problem)
  }
}

export function nameOfColumn(profile: DatasetProfile, id: ColumnId): string {
  const column = profile.columns.find((candidate) => candidate.id === id)
  if (!column)
    throw new Error(`Question column ${id} is absent from dataset profile ${profile.id}.`)
  return column.name
}
