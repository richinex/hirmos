import { useRef, type ReactNode } from 'react'
import { ScrollArrow } from '@/components/ui/ScrollArrow'
import { useScrollOverflow } from '@/lib/useScrollOverflow'
import { cn } from '@/lib/utils'

export function BlockToolbar({ children }: { readonly children: ReactNode }) {
  const viewport = useRef<HTMLDivElement>(null)
  const content = useRef<HTMLDivElement>(null)
  const { overflow, page } = useScrollOverflow(viewport, content)
  return (
    <div
      role="toolbar"
      aria-label="Add a block"
      className="pipeline-block-toolbar relative mx-3 my-2 h-10 min-w-0 shrink-0 overflow-hidden rounded-lg bg-raised pointer-coarse:h-[52px]"
    >
      <div
        ref={viewport}
        data-block-scroll
        className="overflow-x-auto overscroll-x-contain scroll-px-12 [scrollbar-width:none] [&::-webkit-scrollbar]:hidden"
      >
        <div
          ref={content}
          className="flex h-10 w-max min-w-full items-center gap-1 p-1 pointer-coarse:h-[52px]"
        >
          {children}
        </div>
      </div>
      {(['left', 'right'] as const).map((side) => {
        const visible = side === 'left' ? overflow.start : overflow.end
        return (
          <div
            key={side}
            data-overflow-edge={side}
            className={cn(
              'absolute inset-y-0 flex w-14 items-center transition-opacity duration-(--motion-fast) motion-reduce:transition-none',
              side === 'left'
                ? 'left-0 justify-start bg-linear-to-r from-raised from-40% to-transparent'
                : 'right-0 justify-end bg-linear-to-l from-raised from-40% to-transparent',
              visible ? 'opacity-100' : 'invisible opacity-0',
            )}
          >
            <ScrollArrow
              end={side === 'right' ? 'end' : 'start'}
              className="mx-1"
              label={side === 'right' ? 'Scroll to more blocks' : 'Scroll back'}
              title={side === 'right' ? 'More blocks to the right' : 'Blocks to the left'}
              onClick={() => page(side === 'right' ? 1 : -1)}
            />
          </div>
        )
      })}
    </div>
  )
}
