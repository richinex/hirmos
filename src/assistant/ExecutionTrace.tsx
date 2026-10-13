import { useState } from 'react'
import { Icon } from '@/components/Icon'
import { CodeHighlight } from '@/components/ui/CodeHighlight'
import { Orb } from '@/components/ui/Orb'
import { label, num } from '@/components/ui/recipes'
import { cn } from '@/lib/utils'
import { WARN_AT } from './budget'
import { SETTLE } from './BudgetMeter'
import {
  agentLogs,
  seconds,
  stepDuration,
  stepGroups,
  stepVerb,
  type AgentLog,
  type StampedTrace,
  type TraceStep,
} from './traceCells'

type AgentStatus = 'running' | 'done' | 'failed'

const statusOf = (log: AgentLog, running: boolean): AgentStatus =>
  log.end.kind === 'failed' ? 'failed' : log.end.kind === 'open' && running ? 'running' : 'done'

const ACCENT: Record<AgentStatus, string> = {
  running: 'var(--color-info)',
  done: 'var(--color-ok)',
  failed: 'var(--color-danger)',
}

/** A block as a notebook cell: In [n] with the code, Out [n] with what it printed. Folded, it shows the code's first line. */
function Cell({
  step,
  live,
  duration,
}: {
  readonly step: TraceStep
  readonly live: boolean
  readonly duration: number | null
}) {
  const [open, setOpen] = useState(false)
  const shown = open || live
  const code = step.code ?? step.draft ?? ''
  return (
    <div className="min-w-0 flex-1">
      <button
        type="button"
        aria-expanded={shown}
        onClick={() => setOpen((value) => !value)}
        className="flex w-full items-baseline gap-2 text-left text-label text-muted transition-colors hover:text-ink"
      >
        <span className={num('shrink-0 font-medium text-bone')}>
          {step.code === null ? 'Draft' : 'In'} [{step.index}]:
        </span>
        <span className="min-w-0 flex-1 truncate font-mono text-faint">
          {shown ? null : code.split('\n')[0]}
        </span>
        {live || duration === null ? null : (
          <span className={num('shrink-0 text-faint')}>{seconds(duration)}</span>
        )}
      </button>
      {shown ? (
        <>
          <CodeHighlight
            code={code}
            copyable
            copyLabel={`Copy block ${step.index}`}
            className="py-0.5 font-mono text-label"
          />
          {step.code === null || step.output === null ? null : (
            <>
              <div className={num('pt-0.5 text-label text-muted')}>Out [{step.index}]:</div>
              <pre className="m-0 max-h-56 overflow-auto whitespace-pre-wrap break-words font-mono text-label text-bone">
                {step.output}
              </pre>
            </>
          )}
        </>
      ) : null}
    </div>
  )
}

/** One row of the ledger: the verb before the material, so a scan of the turn shows the shape of the work. */
function Step({
  step,
  live,
  duration,
}: {
  readonly step: TraceStep
  readonly live: boolean
  readonly duration: number | null
}) {
  const verb = stepVerb(step)
  return (
    <div className="flex items-start gap-2.5 py-1">
      {live ? (
        <span className="mt-px grid size-3.5 shrink-0 place-items-center">
          <Orb state={step.code === null ? 'composing' : 'searching'} aria-label="Working" />
        </span>
      ) : (
        <Icon
          name={verb.icon}
          size={14}
          className={cn('mt-px shrink-0', step.code === null ? 'text-signal/70' : 'text-dim')}
        />
      )}
      {live && step.code === null && step.draft === null ? null : (
        <>
          <span className={label('mt-px hidden w-[62px] shrink-0 text-muted md:inline')}>
            {verb.word}
          </span>
          {step.code === null && step.draft === null ? (
            <span className="min-w-0 text-label text-muted">
              Request {step.index}: {step.output ?? 'No executable block was recorded.'}
              {duration === null ? null : ` (${seconds(duration)})`}
            </span>
          ) : step.code === null ? (
            <div className="min-w-0 flex-1">
              <Cell step={step} live={live} duration={duration} />
              {step.output === null ? null : (
                <p className="m-0 text-label text-muted">
                  Request {step.index}: {step.output}
                </p>
              )}
            </div>
          ) : (
            <Cell step={step} live={live} duration={duration} />
          )}
        </>
      )}
    </div>
  )
}

/** A running badge carries its index inside a spinning ring, so identity and activity are one glyph. */
function Badge({ status, index }: { readonly status: AgentStatus; readonly index: number }) {
  const accent = ACCENT[status]
  return (
    <span
      className="relative flex size-4 shrink-0 items-center justify-center rounded-full"
      style={
        status === 'running'
          ? { border: `1.5px solid color-mix(in srgb, ${accent} 35%, transparent)`, color: accent }
          : { background: accent, color: 'var(--color-signal-ink)' }
      }
    >
      {status === 'running' ? (
        <>
          <span
            className="absolute inset-[-1.5px] animate-spin rounded-full [animation-duration:0.9s]"
            style={{ border: '1.5px solid transparent', borderTopColor: accent }}
          />
          <span className={num('text-[10px] leading-none')}>{index}</span>
        </>
      ) : (
        <Icon name={status === 'done' ? 'check' : 'close'} size={11} />
      )}
    </span>
  )
}

function Agent({
  log,
  index,
  running,
}: {
  readonly log: AgentLog
  readonly index: number
  readonly running: boolean
}) {
  const [open, setOpen] = useState(false)
  const status = statusOf(log, running)
  const blocks = log.steps.filter((step) => step.code !== null)
  return (
    <div className="px-1.5">
      <button
        type="button"
        aria-expanded={open}
        disabled={log.steps.length === 0}
        onClick={() => setOpen((value) => !value)}
        className="group flex w-full items-center gap-1.5 rounded-full px-1.5 py-[3px] text-left transition-colors hover:bg-raised disabled:cursor-default disabled:hover:bg-transparent"
      >
        <Icon
          name={open ? 'expand_more' : 'chevron_right'}
          size={13}
          className={cn(
            'shrink-0 text-faint group-hover:text-ink',
            log.steps.length === 0 && 'invisible',
          )}
        />
        <Badge status={status} index={index} />
        <span
          className="min-w-0 flex-1 truncate font-medium text-bone"
          title={log.task ?? undefined}
        >
          {log.task ?? log.agent}
        </span>
        <span className={num('shrink-0 text-faint')}>
          {blocks.length} {blocks.length === 1 ? 'block' : 'blocks'}
        </span>
        {log.end.kind === 'failed' ? (
          <span className="max-w-[40%] shrink-0 truncate text-danger">{log.end.message}</span>
        ) : null}
      </button>
      {open ? (
        <div className="ml-[9px] border-l border-hair py-0.5 pl-2.5">
          {log.steps.map((step, i) => (
            <Step
              key={step.index}
              step={step}
              live={status === 'running' && i === log.steps.length - 1}
              duration={stepDuration(log, i)}
            />
          ))}
        </div>
      ) : null}
    </div>
  )
}

/** A count chip: the status glyph and a number, dimmed at zero so the eye skips it. */
function Chip({
  n,
  status,
  word,
}: {
  readonly n: number
  readonly status: AgentStatus
  readonly word: string
}) {
  return (
    <span className={cn('inline-flex shrink-0 items-center gap-1', n === 0 && 'opacity-40')}>
      <span className="inline-flex" style={{ color: ACCENT[status] }}>
        <Icon
          name={status === 'running' ? 'progress_activity' : status === 'done' ? 'check' : 'error'}
          size={11}
          className={
            status === 'running' && n > 0 ? 'animate-spin [animation-duration:0.9s]' : undefined
          }
        />
      </span>
      <span className={num()}>{n}</span> {word}
    </span>
  )
}

/** The fan-out: one box that opens to a capsule per subagent, each opening to its own cells. */
function Subagents({
  logs,
  running,
}: {
  readonly logs: readonly AgentLog[]
  readonly running: boolean
}) {
  const [collapsed, setCollapsed] = useState(false)
  const statuses = logs.map((log) => statusOf(log, running))
  const tally = (status: AgentStatus) => statuses.filter((value) => value === status).length
  return (
    <div className="my-1 rounded-md border border-hair bg-well text-label">
      <button
        type="button"
        aria-expanded={!collapsed}
        onClick={() => setCollapsed((value) => !value)}
        className="flex w-full items-center gap-1.5 px-1.5 py-1 text-left text-muted transition-colors hover:text-ink"
      >
        <Icon
          name={collapsed ? 'chevron_right' : 'expand_more'}
          size={13}
          className="shrink-0 text-faint"
        />
        <Icon name="account_tree" size={12} className="shrink-0" />
        <span className={label('min-w-0 truncate')}>
          Subagents, <span className={num()}>{logs.length}</span>
        </span>
        <span className="ml-auto flex shrink-0 items-center gap-2 pl-1 text-faint">
          <Chip n={tally('running')} status="running" word="running" />
          <Chip n={tally('done')} status="done" word="done" />
          <Chip n={tally('failed')} status="failed" word="failed" />
        </span>
      </button>
      {collapsed ? null : (
        <div className="border-t border-hair py-0.5">
          {logs.map((log, i) => (
            <Agent key={log.agent} log={log} index={i + 1} running={running} />
          ))}
        </div>
      )}
    </div>
  )
}

/** Kinetics' Segment Loader set on end as the trace's rail: one segment per step, the running one breathing. */
function StepSegment({ live, warn }: { readonly live: boolean; readonly warn: boolean }) {
  return (
    <span
      aria-hidden
      className={cn(
        'segment-grow absolute top-[2px] bottom-[2px] left-0.5 w-[3px] origin-top rounded-full transition-colors',
        SETTLE,
        warn ? 'bg-warn' : 'bg-info',
        live && 'segment-live',
      )}
    />
  )
}

export function ExecutionTrace({
  trace,
  running,
  maxSteps,
}: {
  readonly trace: readonly StampedTrace[]
  readonly running: boolean
  /** The parent's step limit while the question runs; the steps not yet taken show as stubs under the rail. */
  readonly maxSteps: number | null
}) {
  const { parent, subagents } = agentLogs(trace)
  const fork = parent.steps.findIndex((step) => step.tools.includes('delegate'))
  const groups = stepGroups(parent.steps)
  const used = groups.length
  const open = running && parent.end.kind === 'open'
  const warn = maxSteps !== null && used / maxSteps >= WARN_AT
  const left = maxSteps === null ? 0 : Math.max(maxSteps - used, 0)
  return (
    <div className="flex flex-col">
      {groups.map((group, g) => (
        <div key={group[0].index} className="relative pl-4">
          <StepSegment live={open && g === used - 1} warn={warn} />
          {group.map((step) => {
            const i = step.index - 1
            return (
              <div key={step.index}>
                <Step
                  step={step}
                  live={open && i === parent.steps.length - 1}
                  duration={stepDuration(parent, i)}
                />
                {i === fork && subagents.length > 0 ? (
                  <Subagents logs={subagents} running={running} />
                ) : null}
              </div>
            )
          })}
        </div>
      ))}
      {left > 0 ? (
        <div aria-hidden className="ml-0.5 flex flex-col gap-1 pt-[2px]">
          {Array.from({ length: left }, (_, i) => (
            <span key={i} className="h-2 w-[3px] rounded-full bg-raised" />
          ))}
        </div>
      ) : null}
    </div>
  )
}
