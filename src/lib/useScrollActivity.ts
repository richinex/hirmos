import { useEffect } from 'react'

/** One listener for nested scroll surfaces, without React renders on scroll. */
export function useScrollActivity() {
  useEffect(() => {
    const pending = new Map<HTMLElement, ReturnType<typeof setTimeout>>()
    const scroll = (event: Event) => {
      const element = event.target === document ? document.documentElement : event.target
      if (!(element instanceof HTMLElement)) return
      clearTimeout(pending.get(element))
      element.dataset.scrolling = 'true'
      pending.set(element, setTimeout(() => {
        delete element.dataset.scrolling
        pending.delete(element)
      }, 700))
    }
    document.addEventListener('scroll', scroll, { capture: true, passive: true })
    return () => {
      document.removeEventListener('scroll', scroll, true)
      for (const [element, timer] of pending) {
        clearTimeout(timer)
        delete element.dataset.scrolling
      }
    }
  }, [])
}
