import { useEffect, useRef, useState } from 'react'

/** The rendered width of an element, updated through a ResizeObserver; 0 until it is first measured. */
export function useElementWidth<T extends HTMLElement>(): readonly [React.RefObject<T | null>, number] {
  const ref = useRef<T>(null)
  const [width, setWidth] = useState(0)
  useEffect(() => {
    const element = ref.current
    if (element === null) return
    const observer = new ResizeObserver((entries) => {
      const next = Math.floor(entries[0]?.contentRect.width ?? 0)
      setWidth((current) => (current === next ? current : next))
    })
    observer.observe(element)
    return () => observer.disconnect()
  }, [])
  return [ref, width]
}

/** The rendered width and height of an element, updated through a ResizeObserver; 0 until first measured. */
export function useElementSize<T extends HTMLElement>(): readonly [React.RefObject<T | null>, number, number] {
  const ref = useRef<T>(null)
  const [size, setSize] = useState({ width: 0, height: 0 })
  useEffect(() => {
    const element = ref.current
    if (element === null) return
    const observer = new ResizeObserver((entries) => {
      const rect = entries[0]?.contentRect
      const next = { width: Math.floor(rect?.width ?? 0), height: Math.floor(rect?.height ?? 0) }
      setSize((current) => (current.width === next.width && current.height === next.height ? current : next))
    })
    observer.observe(element)
    return () => observer.disconnect()
  }, [])
  return [ref, size.width, size.height]
}
