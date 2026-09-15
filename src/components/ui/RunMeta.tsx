import type { ReactNode } from 'react'

/** Separate metadata fields without joining their text into a sentence. */
export function RunMeta({ children }: { readonly children: readonly ReactNode[] }) {
  return <span className="inline-flex flex-wrap items-baseline gap-x-4 gap-y-1">{children.map((item, index) => <span key={index}>{item}</span>)}</span>
}
