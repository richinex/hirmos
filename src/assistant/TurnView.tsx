import { memo, useState } from 'react'
import { Icon } from '@/components/Icon'
import { Alert } from '@/components/ui/Alert'
import { label } from '@/components/ui/recipes'
import { cn } from '@/lib/utils'
import type { Answer } from './agent'
import { costText, stepLimit, turnCost, type TurnLimits } from './budget'
import { ProviderFailureNotice } from './BudgetMeter'
import { ExecutionTrace } from './ExecutionTrace'
import { traceSummary, type StampedTrace } from './traceCells'

export type Outcome =
  | { readonly kind: 'running' }
  | { readonly kind: 'answered'; readonly answer: Answer }
  | { readonly kind: 'failed'; readonly message: string }
  | { readonly kind: 'cancelled' }

export interface Turn {
  readonly id: string
  readonly question: string
  readonly context: string
  readonly trace: readonly StampedTrace[]
  readonly outcome: Outcome
  /** The limits the question started with; absent on turns restored from before they were recorded. */
  readonly limits?: TurnLimits
}

/** Memoised: a streaming trace re-renders only the turn it belongs to. */
export const TurnView = memo(function TurnView({ turn }: { readonly turn: Turn }) {
  const [traceOpen, setTraceOpen] = useState(false)
  const running = turn.outcome.kind === 'running'
  const cost = turnCost(turn.trace)
  const failure = turn.trace.filter((event) => event.kind === 'provider-failure').at(-1)
  return (
    <article className="flex flex-col gap-3 border-t border-line py-5 first:border-t-0">
      <div className="flex items-start gap-2.5">
        <span aria-hidden className="mt-2 h-1.5 w-1.5 shrink-0 bg-signal" />
        <h3 className="m-0 text-subtitle font-medium text-ink">{turn.question}</h3>
      </div>
      <div className="flex flex-col gap-3 pl-4">
        {turn.trace.length === 0 ? null : (
          <div className="flex flex-col gap-1.5">
            {running ? null : (
              <button
                type="button"
                aria-expanded={traceOpen}
                onClick={() => setTraceOpen((open) => !open)}
                className={label('flex w-fit items-center gap-1 text-faint hover:text-muted')}
              >
                <Icon name="code" size={14} />
                {traceSummary(turn.trace)}
                {cost === null ? null : `, ${costText(cost)}`}
                <Icon
                  name="chevron_right"
                  size={13}
                  className={cn('transition-transform', traceOpen && 'rotate-90')}
                />
              </button>
            )}
            {running || traceOpen ? (
              <ExecutionTrace
                trace={turn.trace}
                running={running}
                maxSteps={
                  running && turn.limits !== undefined
                    ? stepLimit(turn.trace, turn.limits.maxSteps)
                    : null
                }
              />
            ) : null}
          </div>
        )}
        {turn.outcome.kind === 'running' ? null : turn.outcome.kind === 'cancelled' ? (
          <p role="status" className={label('m-0 text-faint')}>
            Stopped
          </p>
        ) : turn.outcome.kind === 'failed' ? (
          failure?.kind === 'provider-failure' ? (
            <ProviderFailureNotice failure={failure.failure} at={failure.at} />
          ) : (
            <Alert tone="danger">{turn.outcome.message}</Alert>
          )
        ) : (
          <section aria-label="Assistant answer" className="flex flex-col gap-3">
            <p className="m-0 max-w-[68ch] whitespace-pre-wrap text-body leading-relaxed text-ink">
              {turn.outcome.answer.answer}
            </p>
          </section>
        )}
      </div>
    </article>
  )
})
