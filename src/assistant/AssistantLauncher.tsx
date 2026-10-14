import { lazy, Suspense } from 'react'
import { iconControl } from '@/components/ui/recipes'
import { cn } from '@/lib/utils'
import { AssistantMark } from './AssistantMark'
import { toggleAssistant, useAssistantOpen, useAssistantRunning } from './assistantStore'

const AssistantPanel = lazy(() => import('./AssistantPanel'))

export function AssistantPane() {
  return (
    <aside aria-label="Assistant" className="flex min-h-0 min-w-0 flex-1 flex-col bg-panel">
      <Suspense
        fallback={
          <p role="status" className="p-4 text-body text-muted">
            Loading CausalBuddy…
          </p>
        }
      >
        <AssistantPanel />
      </Suspense>
    </aside>
  )
}

export function AssistantRailButton({
  expanded = false,
  className,
}: {
  readonly expanded?: boolean
  readonly className?: string
}) {
  const open = useAssistantOpen()
  const running = useAssistantRunning()
  return (
    <button
      type="button"
      aria-pressed={open}
      aria-label={open ? 'Close the assistant' : 'Open the assistant'}
      aria-keyshortcuts="Meta+I Control+I"
      title={expanded ? undefined : 'CausalBuddy (⌘I)'}
      onClick={toggleAssistant}
      className={cn(
        'flex h-10 items-center overflow-hidden rounded-full bg-rail transition-[width,color] duration-(--motion-base) motion-reduce:transition-none',
        expanded ? 'w-[224px]' : 'w-10',
        open || running ? 'text-rail-ink' : 'text-rail-faint hover:text-rail-ink',
        className,
      )}
    >
      <span className="grid h-10 w-10 shrink-0 place-items-center">
        <AssistantMark
          state={running ? 'working' : open ? 'open' : 'idle'}
          accent="var(--color-rail-signal)"
        />
      </span>
      <span
        aria-hidden
        className={cn(
          'whitespace-nowrap pl-2 text-body transition-opacity duration-(--motion-base)',
          expanded ? 'opacity-100' : 'opacity-0',
        )}
      >
        CausalBuddy
      </span>
    </button>
  )
}

/** The rail is slid away on a phone, so the launcher moves to the header. */
export function AssistantHeaderButton({ className }: { readonly className?: string }) {
  const open = useAssistantOpen()
  const running = useAssistantRunning()
  return (
    <button
      type="button"
      aria-pressed={open}
      aria-label={open ? 'Close the assistant' : 'Open the assistant'}
      onClick={toggleAssistant}
      className={iconControl(
        open ? 'selected' : 'quiet',
        cn('border-transparent text-ink', className),
      )}
    >
      <AssistantMark state={running ? 'working' : open ? 'open' : 'idle'} />
    </button>
  )
}
