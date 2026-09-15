import { MARK_PATHS } from '@/lib/brand'

/** Directed H, shared by the home link and workbench navigation. */
export function HirmosMark({ className, size = 20 }: { readonly className?: string; readonly size?: number }) {
  return (
    <svg width={size} height={size} viewBox="0 0 32 32" fill="none" aria-hidden className={className}>
      <rect width="32" height="32" rx="7" fill="var(--color-panel)" />
      <g fill="var(--color-signal)">
        {MARK_PATHS.map(d => <path key={d} d={d} />)}
      </g>
    </svg>
  )
}
