import { useCallback, useLayoutEffect, useState, type RefObject } from 'react'

/** Which ends of a horizontal scroller have content out of view. */
export interface ScrollOverflow {
  readonly start: boolean
  readonly end: boolean
}

const NONE: ScrollOverflow = { start: false, end: false }

/** Smooth, unless the reader asked for reduced motion; read at the moment of scrolling. */
export const scrollBehavior = (): ScrollBehavior =>
  window.matchMedia('(prefers-reduced-motion: reduce)').matches ? 'auto' : 'smooth'

/**
 * Tracks a horizontal scroller whose content may be wider than it, and pages it by most of its width.
 * The ends are measured on scroll and whenever the scroller or its content resizes, at most once a frame.
 * `active` lets a caller that renders the scroller only in some forms switch the tracking with it.
 */
export function useScrollOverflow(
  viewport: RefObject<HTMLElement | null>,
  content: RefObject<HTMLElement | null>,
  active = true,
): { readonly overflow: ScrollOverflow; readonly page: (direction: 1 | -1) => void } {
  const [overflow, setOverflow] = useState<ScrollOverflow>(NONE)

  useLayoutEffect(() => {
    const scroller = viewport.current
    const row = content.current
    if (!active || scroller === null || row === null) {
      setOverflow(NONE)
      return
    }
    let pending = 0
    const measure = () => {
      const start = scroller.scrollLeft > 1
      const end = scroller.scrollLeft + scroller.clientWidth < scroller.scrollWidth - 1
      setOverflow((current) =>
        current.start === start && current.end === end ? current : { start, end },
      )
    }
    const schedule = () => {
      cancelAnimationFrame(pending)
      pending = requestAnimationFrame(measure)
    }
    const observer = new ResizeObserver(schedule)
    observer.observe(scroller)
    observer.observe(row)
    scroller.addEventListener('scroll', schedule, { passive: true })
    measure()
    return () => {
      observer.disconnect()
      cancelAnimationFrame(pending)
      scroller.removeEventListener('scroll', schedule)
    }
  }, [viewport, content, active])

  const page = useCallback(
    (direction: 1 | -1) => {
      const scroller = viewport.current
      scroller?.scrollBy({
        left: direction * scroller.clientWidth * 0.7,
        behavior: scrollBehavior(),
      })
    },
    [viewport],
  )

  return { overflow, page }
}
