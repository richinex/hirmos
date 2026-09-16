import type { ReactNode } from 'react'
import { Metadata } from './Metadata'

/** Separate metadata fields without joining their text into a sentence. */
export function RunMeta({ children }: { readonly children: readonly ReactNode[] }) {
  return <Metadata>{children}</Metadata>
}
