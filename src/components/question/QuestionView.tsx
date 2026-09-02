import { Select } from '@/components/ui/Select'
import type { FormEvent } from 'react'
import { button, field, label, panel } from '@/components/ui/recipes'
import type { DatasetProfile } from '@/domain/dataset'
import { assertNever } from '@/domain/dop'
import {
  describeQuestionProblem,
  nameOfColumn,
  type QuestionEvent,
  type QuestionState,
} from '@/domain/question'

export function QuestionView({ profile, state, dispatch }: {
  readonly profile: DatasetProfile
  readonly state: QuestionState
  readonly dispatch: (event: QuestionEvent) => void
}) {
  switch (state.kind) {
    case 'draft': {
      const submit = (event: FormEvent) => {
        event.preventDefault()
        dispatch({ type: 'submitted' })
      }
      return (
        <section className="rise my-auto w-full max-w-2xl" aria-labelledby="question-title">
          <span className={label('text-faint')}>05 · Study design</span>
          <h2 id="question-title" className="mb-6 mt-3 text-heading text-ink">Frame the causal question</h2>
          <form onSubmit={submit} className="grid gap-4 sm:grid-cols-2">
            <label className="block">
              <span className="mb-1.5 block text-body font-medium text-ink">Treatment</span>
              <Select
                className={field('text')}
                value={state.treatment ?? ''}
                onChange={(event) => dispatch({ type: 'treatment-changed', value: event.target.value })}
              >
                <option value="" disabled>Choose a column</option>
                {profile.columns.map((column) => <option key={column.id} value={column.id}>{column.name}</option>)}
              </Select>
            </label>
            <label className="block">
              <span className="mb-1.5 block text-body font-medium text-ink">Outcome</span>
              <Select
                className={field('text')}
                value={state.outcome ?? ''}
                onChange={(event) => dispatch({ type: 'outcome-changed', value: event.target.value })}
              >
                <option value="" disabled>Choose a column</option>
                {profile.columns.map((column) => <option key={column.id} value={column.id}>{column.name}</option>)}
              </Select>
            </label>
            {state.problem && <p role="alert" className="m-0 text-body text-danger sm:col-span-2">{describeQuestionProblem(state.problem)}</p>}
            <div className="sm:col-span-2">
              <button type="submit" className={button('signal')}>Set question</button>
            </div>
          </form>
        </section>
      )
    }
    case 'framed':
      return (
        <section className="rise my-auto w-full max-w-2xl" aria-labelledby="framed-question-title">
          <span className={label('text-faint')}>05 · Study design</span>
          <h2 id="framed-question-title" className="mb-6 mt-3 text-heading text-ink">Causal question</h2>
          <div className={panel('lift flex items-center gap-3 px-4 py-5 text-title text-ink')}>
            <span>{nameOfColumn(profile, state.question.treatment)}</span>
            <span aria-label="affects" className="text-signal">→</span>
            <span>{nameOfColumn(profile, state.question.outcome)}</span>
          </div>
          <button type="button" className={button('quiet', 'mt-4')} onClick={() => dispatch({ type: 'edit-requested' })}>
            Edit question
          </button>
        </section>
      )
    default:
      return assertNever(state)
  }
}
