import { useState, type ReactNode } from 'react'
import { Group, Handle, Panel, Separator } from 'motion-panels/react'
import { cn } from '@/lib/utils'
import { Slot } from './AssistantSplit'

type Side = 'chapter' | 'assistant'

const LAYOUT = 'hirmos.shell.assistant.v2'
const HOUSE_CURVE = { duration: 0.28, ease: [0.2, 0.7, 0.2, 1] } as const

interface Layout {
  readonly width: number
  readonly order: readonly Side[]
}

function readLayout(): Layout {
  try {
    const parsed: unknown = JSON.parse(localStorage.getItem(LAYOUT) ?? 'null')
    if (parsed !== null && typeof parsed === 'object') {
      const { width, order } = parsed as Record<string, unknown>
      if (
        typeof width === 'number' &&
        Array.isArray(order) &&
        order.length === 2 &&
        order.includes('chapter') &&
        order.includes('assistant')
      )
        return { width, order: order as Side[] }
    }
  } catch {
    /* private mode or a corrupt entry */
  }
  return { width: 400, order: ['chapter', 'assistant'] }
}

function writeLayout(layout: Layout): void {
  try {
    localStorage.setItem(LAYOUT, JSON.stringify(layout))
  } catch {
    /* quota or private mode */
  }
}

/**
 * The chapter and the docked assistant side by side. The assistant folds open on the house curve while
 * the pinned chapter holds still, and its mark drags it to either side. Loaded on the first dock, so a
 * visitor who never opens the assistant does not download the panel library.
 */
export default function AssistantDock({
  chapterHost,
  paneHost,
  docked,
}: {
  readonly chapterHost: HTMLElement
  readonly paneHost: HTMLElement
  readonly docked: boolean
}) {
  const [layout, setLayout] = useState(readLayout)
  const change = (next: Layout) => {
    setLayout(next)
    writeLayout(next)
  }
  const assistantFirst = layout.order[0] === 'assistant'
  const panels: Record<Side, ReactNode> = {
    chapter: (
      <Panel key="chapter" value="chapter" pin className="flex min-w-0">
        <Slot host={chapterHost} />
      </Panel>
    ),
    assistant: (
      <Panel
        key="assistant"
        value="assistant"
        size={layout.width}
        onSizeChange={(width) => change({ ...layout, width })}
        minSize={320}
        maxSize={640}
        collapsed={!docked}
        transition={HOUSE_CURVE}
        className={cn(
          'relative flex min-w-0 border-line',
          docked && (assistantFirst ? 'border-r' : 'border-l'),
        )}
      >
        {docked ? (
          <>
            <Handle
              aria-label="Move the assistant to the other side"
              title="Drag to the other side, or use the arrow keys"
              className="absolute left-2 top-1 z-10 h-7 w-7 cursor-grab rounded-md active:cursor-grabbing"
            />
            <Slot host={paneHost} />
          </>
        ) : null}
      </Panel>
    ),
  }
  return (
    <Group
      orientation="horizontal"
      order={[...layout.order]}
      onOrderChange={(order) => change({ ...layout, order })}
      transition={HOUSE_CURVE}
      className="flex min-w-0 flex-1"
    >
      {panels[layout.order[0]]}
      <Separator
        aria-label="Resize the assistant"
        className={cn('seam seam-v', docked ? null : 'pointer-events-none opacity-0')}
      />
      {panels[layout.order[1]]}
    </Group>
  )
}
