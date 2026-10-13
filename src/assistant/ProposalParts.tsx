import { createContext, useContext, useId, useState, type ReactNode } from 'react'
import { Icon } from '@/components/Icon'
import { ParameterHelp } from '@/components/ui/ParameterLabel'
import { button, chromeAction, label } from '@/components/ui/recipes'
import type { ConfigurationSetting } from '@/domain/estimation'
import { cn } from '@/lib/utils'
import { askAssistant } from './assistantStore'
import { proposalAvailability } from './proposalAvailability'

export const ProposalActivity = createContext(false)

/** The frame every proposal card shares: the suggestion notice, then what the card proposes. */
export function ProposalCard({
  name,
  title,
  children,
}: {
  readonly name: string
  readonly title: string
  readonly children: ReactNode
}) {
  return (
    <section
      aria-label={name}
      className="my-4 flex flex-col gap-3 rounded-xl border border-signal/40 bg-signal/[0.06] p-3.5"
    >
      <div className="flex flex-wrap items-baseline gap-x-2">
        <span className={label('text-signal-text')}>Suggestion</span>
        <span className="text-label text-faint">
          Assistant guidance, not a saved analysis result.
        </span>
      </div>
      <h3 className="m-0 text-body font-medium">{title}</h3>
      {children}
    </section>
  )
}

/** The proposal's own checks as a list the user ticks off while verifying; the ticks stay in the card and are not sent. */
export function Checks({ checks }: { readonly checks: readonly string[] }) {
  const [done, setDone] = useState<ReadonlySet<number>>(new Set())
  return (
    <fieldset className="m-0 flex flex-col gap-1 border-0 p-0">
      <legend className={label('mb-1 flex items-center gap-1.5 p-0 text-ink')}>
        <Icon name="fact_check" size={15} />
        Checks
        <span className="font-normal text-faint">
          {done.size} of {checks.length}
        </span>
      </legend>
      {checks.map((check, i) => (
        <label
          key={i}
          className="flex cursor-pointer items-start gap-2 rounded-md px-1 py-0.5 text-body hover:bg-panel/60"
        >
          <input
            type="checkbox"
            className="mt-[3px] accent-[var(--color-ok)]"
            checked={done.has(i)}
            onChange={() =>
              setDone((previous) => {
                const next = new Set(previous)
                if (next.has(i)) next.delete(i)
                else next.add(i)
                return next
              })
            }
          />
          <span className={cn('min-w-0', done.has(i) ? 'text-faint line-through' : 'text-bone')}>
            {check}
          </span>
        </label>
      ))}
    </fieldset>
  )
}

/** The proposed settings, labelled as the page's controls label them. */
export function ProposalSettings({
  settings,
}: {
  readonly settings: readonly ConfigurationSetting[]
}) {
  return (
    <dl className="m-0 grid grid-cols-2 gap-2">
      {settings.map((setting) => (
        <div key={setting.key}>
          <dt className="text-label text-muted">{setting.label}</dt>
          <dd className="m-0 text-body">{setting.value}</dd>
        </div>
      ))}
    </dl>
  )
}

/** The primary action with its consequences behind the ⓘ beside it, then Dismiss. */
export function ProposalActions({
  action,
  help,
  disabled = false,
  onAction,
  onDismiss,
}: {
  readonly action: string
  readonly help: string
  readonly disabled?: boolean
  readonly onAction: () => void
  readonly onDismiss: () => void
}) {
  const running = useContext(ProposalActivity)
  const availability = proposalAvailability(running, disabled)
  const reasonId = useId()
  return (
    <div className="flex flex-wrap items-center gap-2">
      <button
        type="button"
        className={button('signal')}
        disabled={availability.kind === 'blocked'}
        aria-describedby={availability.kind === 'blocked' ? reasonId : undefined}
        onClick={() => {
          if (availability.kind === 'ready') onAction()
        }}
      >
        {action}
      </button>
      <ParameterHelp label={action} help={help} />
      <button type="button" className={button('quiet', 'ml-auto')} onClick={onDismiss}>
        Dismiss
      </button>
      {availability.kind === 'blocked' ? (
        <p id={reasonId} className="m-0 basis-full text-label text-muted">
          {availability.reason}
        </p>
      ) : null}
    </div>
  )
}

/** The next step after a change: profile the result against the baseline. It fills the composer; the user sends it. */
export function CompareAction({ children }: { readonly children?: ReactNode }) {
  return (
    <div role="status" className="flex flex-wrap items-center gap-2">
      <span className="inline-flex items-center gap-1.5 rounded-full bg-ok/10 px-2 py-0.5 text-label text-ok">
        <Icon name="check" size={13} />
        {children}
      </span>
      <button
        type="button"
        className={chromeAction('signal')}
        onClick={() =>
          askAssistant(
            'Profile the output and compare it with your baseline profile, using its receipt.',
          )
        }
      >
        <Icon name="difference" size={14} />
        Compare with baseline
      </button>
    </div>
  )
}
