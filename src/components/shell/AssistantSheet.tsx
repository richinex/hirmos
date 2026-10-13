import { Group, Panel, Separator } from 'react-resizable-panels'
import { Slot } from './AssistantSplit'

export default function AssistantSheet({
  chapterHost,
  paneHost,
  expanded,
}: {
  readonly chapterHost: HTMLElement
  readonly paneHost: HTMLElement
  readonly expanded: boolean
}) {
  return (
    <div className="relative flex min-h-0 min-w-0 flex-1 flex-col">
      <Group orientation="vertical" className="min-h-0 min-w-0 flex-1">
        <Panel id="chapter" minSize="15%" className="flex min-h-0 min-w-0">
          <Slot host={chapterHost} />
        </Panel>
        <Separator
          aria-label="Resize the assistant"
          className="flex h-5 cursor-row-resize touch-none items-center justify-center border-t border-line bg-panel shadow-[0_-8px_24px_rgb(0_0_0/0.08)] focus-visible:outline-2 focus-visible:outline-signal"
        >
          <span aria-hidden className="h-1 w-10 rounded-full bg-dim" />
        </Separator>
        <Panel
          id="assistant"
          defaultSize="55%"
          minSize="25%"
          maxSize="85%"
          className="flex min-h-0 min-w-0 flex-col"
        >
          {expanded ? null : <Slot host={paneHost} />}
        </Panel>
      </Group>
      {expanded ? (
        <div className="absolute inset-0 z-10 flex min-h-0 flex-col bg-panel">
          <Slot host={paneHost} />
        </div>
      ) : null}
    </div>
  )
}
