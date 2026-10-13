import type { Trace } from './agent'

export type StampedTrace = Trace & { readonly at: number }

export function appendTrace(
  trace: readonly StampedTrace[],
  event: StampedTrace,
): readonly StampedTrace[] {
  let previous = trace.length - 1
  while (previous >= 0 && trace[previous].agent !== event.agent) previous--
  if (event.kind === 'writing' && previous >= 0 && trace[previous].kind === 'writing')
    return trace.map((item, index) => (index === previous ? event : item))
  return [...trace, event]
}

/** One model request and what came of it: the block it wrote, the tools that block called, and what it printed. */
export interface TraceStep {
  readonly index: number
  /** The agent step this request spent, from the budget snapshot before it; a retry repeats the step. */
  readonly step: number | null
  readonly at: number
  readonly code: string | null
  readonly draft: string | null
  readonly tools: readonly string[]
  readonly output: string | null
}

export type AgentEnd =
  | { readonly kind: 'open' }
  | { readonly kind: 'answered'; readonly at: number }
  | { readonly kind: 'failed'; readonly at: number; readonly message: string }

export interface AgentLog {
  readonly agent: string
  readonly task: string | null
  readonly steps: readonly TraceStep[]
  readonly end: AgentEnd
  /** The step from the latest budget snapshot, given to the next request. */
  readonly budgetStep: number | null
}

const PARENT = 'Assistant'

function withLast(
  steps: readonly TraceStep[],
  change: (step: TraceStep) => TraceStep,
): readonly TraceStep[] {
  const last = steps.at(-1)
  return last === undefined ? steps : [...steps.slice(0, -1), change(last)]
}

function append(log: AgentLog, step: StampedTrace): AgentLog {
  switch (step.kind) {
    case 'budget':
      return { ...log, budgetStep: step.budget.step }
    case 'usage':
    case 'provider-failure':
      return log
    case 'task':
      return { ...log, task: step.text }
    case 'request':
      return {
        ...log,
        steps: [
          ...log.steps,
          {
            index: log.steps.length + 1,
            step: log.budgetStep,
            at: step.at,
            code: null,
            draft: null,
            tools: [],
            output: null,
          },
        ],
      }
    case 'writing':
      return { ...log, steps: withLast(log.steps, (last) => ({ ...last, draft: step.text })) }
    case 'python':
      return {
        ...log,
        steps: withLast(log.steps, (last) => ({ ...last, code: step.text, draft: null })),
      }
    case 'tool':
      return {
        ...log,
        steps: withLast(log.steps, (last) => ({ ...last, tools: [...last.tools, step.text] })),
      }
    case 'output':
      return { ...log, steps: withLast(log.steps, (last) => ({ ...last, output: step.text })) }
    case 'answer':
      return { ...log, end: { kind: 'answered', at: step.at } }
    case 'failed':
      return { ...log, end: { kind: 'failed', at: step.at, message: step.text } }
  }
}

/** The parent's log and one per subagent, in the order each first appeared. */
export function agentLogs(trace: readonly StampedTrace[]): {
  readonly parent: AgentLog
  readonly subagents: readonly AgentLog[]
} {
  const empty = (agent: string): AgentLog => ({
    agent,
    task: null,
    steps: [],
    end: { kind: 'open' },
    budgetStep: null,
  })
  const byAgent = new Map<string, AgentLog>([[PARENT, empty(PARENT)]])
  for (const step of trace)
    byAgent.set(step.agent, append(byAgent.get(step.agent) ?? empty(step.agent), step))
  const [parent, ...subagents] = byAgent.values()
  return { parent, subagents }
}

/** Requests grouped by the step they spent, so a retry shares its step with the reply after it. */
export function stepGroups(steps: readonly TraceStep[]): readonly (readonly TraceStep[])[] {
  const groups: TraceStep[][] = []
  for (const step of steps) {
    const last = groups.at(-1)
    if (last !== undefined && step.step !== null && last[0].step === step.step) last.push(step)
    else groups.push([step])
  }
  return groups
}

/** How long a step took: until the next step began, or until the agent finished. */
export function stepDuration(log: AgentLog, i: number): number | null {
  const next = log.steps[i + 1]?.at ?? (log.end.kind === 'open' ? null : log.end.at)
  return next === null ? null : next - log.steps[i].at
}

export const seconds = (ms: number): string => `${(ms / 1000).toFixed(1)}s`

export interface StepVerb {
  readonly word: string
  /** Octopus's phase glyphs: terminal for written code, bolt for an execution, edit_note for the answer. */
  readonly icon: string
}

/** What a step did, read from the tools its block called. */
export function stepVerb(step: TraceStep): StepVerb {
  const called = (name: string) => step.tools.includes(name)
  if (step.code === null)
    return step.draft !== null
      ? { word: 'draft', icon: 'edit_note' }
      : { word: 'no block', icon: 'refresh' }
  if (called('finish')) return { word: 'answered', icon: 'edit_note' }
  if (called('delegate')) return { word: 'delegated', icon: 'account_tree' }
  if (called('run_analysis')) return { word: 'requested', icon: 'bolt' }
  if (called('propose_estimator') || called('propose_sql') || called('propose_pipeline'))
    return { word: 'proposed', icon: 'draft' }
  if (called('profile_table')) return { word: 'profiled', icon: 'query_stats' }
  if (called('project') || called('study')) return { word: 'read', icon: 'description' }
  return { word: 'computed', icon: 'terminal' }
}

const count = (n: number, one: string, many: string): string => `${n} ${n === 1 ? one : many}`

/** Empty parts are left out rather than shown as zero. */
export function traceSummary(trace: readonly StampedTrace[]): string {
  const { parent, subagents } = agentLogs(trace)
  const elapsed = trace.length < 2 ? 0 : trace[trace.length - 1].at - trace[0].at
  const parts = [
    count(parent.steps.filter((step) => step.code !== null).length, 'block', 'blocks'),
    parent.steps.some((step) => step.code === null)
      ? count(parent.steps.length, 'request', 'requests')
      : null,
    subagents.length === 0 ? null : count(subagents.length, 'subagent', 'subagents'),
    elapsed < 100 ? null : seconds(elapsed),
  ].filter((part): part is string => part !== null)
  return parts.join(', ')
}
