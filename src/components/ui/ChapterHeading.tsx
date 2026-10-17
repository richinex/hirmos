import type { ReactNode } from 'react'

/** The chapter has one heading; navigation supplies its position in the workflow. */
export function ChapterHeading({
  id,
  children,
  className = '',
}: {
  readonly id?: string
  readonly children: ReactNode
  readonly className?: string
}) {
  return (
    <h2
      id={id}
      className={`chapter-heading mt-0 min-w-0 font-reading font-normal text-heading text-ink [overflow-wrap:anywhere] ${className}`}
    >
      {children}
    </h2>
  )
}
