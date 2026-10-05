import { MARK_PATHS } from '@/lib/brand'

/** Directed H on the band's blue: the same mark on the band, the canvas and the favicon. */
export function HirmosMark({
  className,
  size = 20,
}: {
  readonly className?: string
  readonly size?: number
}) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 32 32"
      fill="none"
      aria-hidden
      className={className}
    >
      <rect width="32" height="32" rx="7" fill="var(--color-rail-active)" />
      <g fill="var(--color-rail-ink)">
        {MARK_PATHS.map((d) => (
          <path key={d} d={d} />
        ))}
      </g>
    </svg>
  )
}
