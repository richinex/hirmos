import * as Popover from '@radix-ui/react-popover'
import { Icon } from '@/components/Icon'
import { button } from '@/components/ui/recipes'
import type { RootCauseGraph } from '@/domain/rootCause'

export function GraphDetails({
  name,
  graph,
  disabled,
  onOpen,
}: {
  readonly name: string
  readonly graph: RootCauseGraph
  readonly disabled: boolean
  readonly onOpen: () => void
}) {
  return (
    <Popover.Root>
      <Popover.Trigger asChild>
        <button
          type="button"
          disabled={disabled}
          className={button(
            'quiet',
            'inline-flex max-w-full items-center gap-2 whitespace-normal text-left',
          )}
        >
          <span className="shrink-0 text-muted">Graph</span>
          <span className="min-w-0 break-words">{name}</span>
          <Icon name="expand_more" size={14} className="shrink-0 text-faint" />
        </button>
      </Popover.Trigger>
      <Popover.Portal>
        <Popover.Content
          aria-label={`Graph details: ${name}`}
          side="bottom"
          align="start"
          sideOffset={8}
          collisionPadding={12}
          className="pop float z-(--z-dialog) flex max-h-[var(--radix-popover-content-available-height)] w-[22rem] max-w-[calc(100vw-24px)] flex-col gap-4 rounded-xl border border-line bg-panel p-5 text-body text-ink"
        >
          <div className="min-w-0 shrink-0">
            <h3 className="m-0 break-words text-body font-medium">{name}</h3>
            <p className="mb-0 mt-1 text-label text-muted">Relationships used for this analysis</p>
          </div>
          <div
            className="min-h-0 max-h-[40dvh] overflow-y-auto overscroll-contain"
            tabIndex={0}
            role="region"
            aria-label="Graph relationships"
          >
            {graph.edges.length === 0 ? (
              <p className="m-0 text-body text-muted">This graph has no directed relationships.</p>
            ) : (
              <ul className="m-0 grid list-none gap-3 p-0">
                {graph.edges.map(([cause, effect]) => (
                  <li
                    key={`${cause}-${effect}`}
                    className="grid grid-cols-[minmax(0,1fr)_auto_minmax(0,1fr)] items-start gap-2 text-label"
                  >
                    <span className="break-words">{graph.nodes[cause]!.name}</span>
                    <span>
                      <span aria-hidden className="text-muted">
                        →
                      </span>
                      <span className="sr-only">causes</span>
                    </span>
                    <span className="break-words">{graph.nodes[effect]!.name}</span>
                  </li>
                ))}
              </ul>
            )}
          </div>
          <Popover.Close asChild>
            <button
              type="button"
              className={button('quiet', 'shrink-0 self-start')}
              onClick={onOpen}
            >
              Open DAG workspace
            </button>
          </Popover.Close>
        </Popover.Content>
      </Popover.Portal>
    </Popover.Root>
  )
}
