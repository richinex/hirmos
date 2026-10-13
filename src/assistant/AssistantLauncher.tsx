import { lazy, Suspense } from 'react'
import { iconControl } from '@/components/ui/recipes'
import { cn } from '@/lib/utils'
import { AssistantMark } from './AssistantMark'
import { toggleAssistant, useAssistantOpen, useAssistantRunning } from './assistantStore'

const AssistantPanel = lazy(() => import('./AssistantPanel'))

export function AssistantPane() {
  return (
    <aside
      aria-label="Assistant"
      className="flex min-h-0 min-w-0 flex-1 flex-col bg-panel"
    >
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

export function AssistantRailButton({ className }: { readonly className?: string }) {
  const open = useAssistantOpen()
  const running = useAssistantRunning()
  return (
    <button
      type="button"
      aria-pressed={open}
      aria-label={open ? 'Close the assistant' : 'Open the assistant'}
      aria-keyshortcuts="Meta+I Control+I"
      title="CausalBuddy (⌘I)"
      onClick={toggleAssistant}
      className={cn(
        'grid h-10 w-10 place-items-center transition-colors duration-(--motion-fast)',
        open || running ? 'text-rail-ink' : 'text-rail-faint hover:text-rail-ink',
        className,
      )}
    >
      <AssistantMark
        state={running ? 'working' : open ? 'open' : 'idle'}
        accent="var(--color-rail-signal)"
      />
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
      className={iconControl(open ? 'selected' : 'quiet', cn('text-ink', className))}
    >
      <AssistantMark state={running ? 'working' : open ? 'open' : 'idle'} />
    </button>
  )
}
