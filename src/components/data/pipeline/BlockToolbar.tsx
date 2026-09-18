import { useLayoutEffect, useRef, useState, type ReactNode } from 'react'
import { cn } from '@/lib/utils'

type Overflow = 'none' | 'right' | 'both' | 'left'

export function BlockToolbar({ children }: { readonly children: ReactNode }) {
  const viewport = useRef<HTMLDivElement>(null)
  const content = useRef<HTMLDivElement>(null)
  const [overflow, setOverflow] = useState<Overflow>('none')
  useLayoutEffect(() => {
    const scroller = viewport.current
    const row = content.current
    if (scroller === null || row === null) return
    let frame = 0
    const measure = () => {
      const left = scroller.scrollLeft > 1
      const right = scroller.scrollLeft + scroller.clientWidth < scroller.scrollWidth - 1
      setOverflow(left ? right ? 'both' : 'left' : right ? 'right' : 'none')
    }
    const schedule = () => {
      cancelAnimationFrame(frame)
      frame = requestAnimationFrame(measure)
    }
    const observer = new ResizeObserver(schedule)
    observer.observe(scroller)
    observer.observe(row)
    scroller.addEventListener('scroll', schedule, { passive: true })
    measure()
    return () => { observer.disconnect(); cancelAnimationFrame(frame); scroller.removeEventListener('scroll', schedule) }
  }, [])
  return <div role="toolbar" aria-label="Add a block" className="pipeline-block-toolbar relative mx-3 my-2 h-10 min-w-0 shrink-0 overflow-hidden rounded-lg bg-raised pointer-coarse:h-[52px]">
    <div ref={viewport} data-block-scroll className="overflow-x-auto overscroll-x-contain scroll-px-12 [scrollbar-width:none] [&::-webkit-scrollbar]:hidden">
      <div ref={content} className="flex h-10 w-max min-w-full items-center gap-1 p-1 pointer-coarse:h-[52px]">{children}</div>
    </div>
    {(['left', 'right'] as const).map((side) => {
      const visible = overflow === 'both' || overflow === side
      return <div key={side} aria-hidden data-overflow-edge={side} className={cn('pointer-events-none absolute inset-y-0 w-6 transition-opacity duration-(--motion-fast) motion-reduce:transition-none', side === 'left' ? 'left-0 bg-linear-to-r from-raised to-transparent' : 'right-0 bg-linear-to-l from-raised to-transparent', visible ? 'opacity-100' : 'invisible opacity-0')} />
    })}
  </div>
}
