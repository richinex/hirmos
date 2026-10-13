import { useState } from 'react'
import { CodeHighlight } from '@/components/ui/CodeHighlight'
import { DisclosureSummary } from '@/components/ui/DisclosureSummary'
import { button, label } from '@/components/ui/recipes'
import { useAssistantBridge } from './applicationTools'
import type { AssistantRunOutcome, RunCapability } from './analysisRun'
import { runSubjectProblem, type RunSubject } from './analysisRun'

type Request = {
  readonly id: string
  readonly turnId?: string
  readonly capability: RunCapability
  readonly reason: string
} & (
  | { readonly kind: 'pending'; readonly decide: (approved: boolean) => void }
  | { readonly kind: 'running' }
  | { readonly kind: 'settled'; readonly outcome: AssistantRunOutcome }
)

export function useRunApproval() {
  const bridge = useAssistantBridge()
  const [requests, setRequests] = useState<readonly Request[]>([])
  const request = async (
    key: string,
    revision: string,
    reason: string,
    signal: AbortSignal,
    onApproved: () => void,
    subject?: RunSubject,
    waiting?: (pending: boolean) => void,
    turnId?: string,
  ): Promise<AssistantRunOutcome> => {
    const capability = bridge.store.getState().runs[key]
    if (!capability || capability.revision !== revision)
      return { kind: 'refused', reason: 'The specification changed. Request a new run.' }
    const mismatch = runSubjectProblem(capability.inspection, subject)
    if (mismatch) return { kind: 'refused', reason: mismatch }
    if (signal.aborted) return { kind: 'cancelled' }
    const id = crypto.randomUUID()
    const controller = new AbortController()
    const executionSignal = AbortSignal.any([signal, controller.signal])
    const unsubscribe = bridge.store.subscribe((state) => {
      if (state.runs[key]?.revision !== revision) controller.abort()
    })
    const update = (next: Request) =>
      setRequests((previous) => previous.map((entry) => (entry.id === id ? next : entry)))
    let outcome: AssistantRunOutcome = { kind: 'cancelled' }
    try {
      waiting?.(true)
      const approved = await new Promise<boolean>((resolve) => {
        const cancel = () => decide(false)
        const decide = (value: boolean) => {
          executionSignal.removeEventListener('abort', cancel)
          resolve(value)
        }
        executionSignal.addEventListener('abort', cancel, { once: true })
        setRequests((previous) => [
          ...previous,
          { id, turnId, capability, reason, kind: 'pending', decide },
        ])
      })
      waiting?.(false)
      if (executionSignal.aborted) return outcome
      if (!approved) {
        outcome = { kind: 'refused', reason: 'The user declined this run.' }
        return outcome
      }
      onApproved()
      update({ id, turnId, capability, reason, kind: 'running' })
      outcome = await bridge.run(key, revision, executionSignal)
      if (
        outcome.kind === 'completed' &&
        subject?.kind === 'saved-study' &&
        runSubjectProblem(capability.inspection, outcome.subject)
      ) {
        outcome = {
          kind: 'failed',
          reason: `Run ${outcome.runId} was saved, but its study does not match the approved study. Do not use it to answer this question; inspect the saved result.`,
        }
      }
      return outcome
    } catch (error) {
      outcome = executionSignal.aborted
        ? { kind: 'cancelled' }
        : {
            kind: 'failed',
            reason: error instanceof Error ? error.message : 'The analysis could not finish.',
          }
      return outcome
    } finally {
      waiting?.(false)
      unsubscribe()
      update({ id, turnId, capability, reason, kind: 'settled', outcome })
    }
  }
  return { requests, request, clear: () => setRequests([]) }
}

export function RunApprovalCards({ requests }: { readonly requests: readonly Request[] }) {
  return requests.map((entry) => (
    <section
      key={entry.id}
      aria-label="Analysis approval"
      className="my-4 grid grid-cols-[minmax(0,1fr)] gap-3 rounded-xl border border-line p-3.5"
    >
      <span className={label()}>{entry.capability.inspection.section}</span>
      <h3 className="m-0 text-body font-medium">Run {entry.capability.inspection.method}</h3>
      {entry.capability.inspection.study ? (
        <>
          <p className="m-0">{entry.capability.inspection.study.question}</p>
          <dl className="m-0 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-label">
            {entry.capability.inspection.study.settings.map((setting, index) => (
              <div key={index} className="contents">
                <dt>{setting.label}</dt>
                <dd className="m-0 break-words">{setting.value}</dd>
              </div>
            ))}
          </dl>
        </>
      ) : null}
      <details>
        <DisclosureSummary icon="psychology" className="cursor-pointer text-label">
          Assistant rationale
        </DisclosureSummary>
        <p className="m-0">{entry.reason}</p>
      </details>
      <details>
        <DisclosureSummary icon="receipt_long" className="cursor-pointer text-label">
          Specification
        </DisclosureSummary>
        <CodeHighlight
          code={JSON.stringify(JSON.parse(entry.capability.inspection.specification), null, 2)}
          lang="json"
          copyable
          copyLabel="Copy specification"
          className="mt-2 max-h-64 overflow-auto rounded-md bg-panel p-2.5 font-mono text-label"
        />
      </details>
      <p className="m-0 text-label text-muted">
        Approving runs the analysis locally and allows the assistant to send this summary to
        OpenRouter: {entry.capability.inspection.sharedResult}
      </p>
      {entry.kind === 'pending' ? (
        <div className="flex flex-wrap gap-2">
          <button type="button" className={button('signal')} onClick={() => entry.decide(true)}>
            Run and share summary
          </button>
          <button type="button" className={button('quiet')} onClick={() => entry.decide(false)}>
            Decline run
          </button>
        </div>
      ) : (
        <p role="status" className="m-0 text-label">
          {entry.kind === 'running'
            ? 'Running through the existing analysis controls…'
            : entry.outcome.kind === 'completed'
              ? `Run saved. Results are available in ${entry.capability.inspection.section}.`
              : entry.outcome.kind === 'calculated'
                ? 'Calculation saved. Review it in Sample size and power planning.'
                : entry.outcome.kind === 'cancelled'
                  ? 'Run cancelled.'
                  : entry.outcome.reason}
        </p>
      )}
    </section>
  ))
}
