import { cn } from '@/lib/utils'

/**
 * Material Symbols Sharp at weight 300. The squared terminals and uniform monoline stroke let icons
 * sit flush against the 1px chrome, which the old mixed lucide and phosphor set could not.
 *
 * Ligature-based, so the glyph name is the child text. The font is self-hosted and imported once in
 * main.tsx. The landing page does not load it, so a ligature there renders its name as plain text.
 */
export function Icon({
  name,
  size = 16,
  className,
  fill = false,
  weight = 300,
}: {
  /** A Material Symbols ligature, or the application's root_cause glyph. */
  name: string
  size?: number
  className?: string
  /** Filled variant, reserved for the active or selected state. */
  fill?: boolean
  /**
   * The font's weight axis. 300 is the house line, so the toolbar reads as one monoline drawing.
   * Raise it only for a control alone on an empty bar, such as the phone header's overflow dots,
   * which read as texture at 300.
   */
  weight?: number
}) {
  if (name === 'root_cause')
    return (
      <svg
        aria-hidden
        viewBox="0 0 26 26"
        width={size}
        height={size}
        className={cn('shrink-0 select-none', className)}
        fill="none"
        stroke="currentColor"
        strokeWidth={1.4}
        strokeLinecap="square"
        strokeLinejoin="miter"
      >
        <g transform="translate(-4 -4) scale(1.2)">
          <path
            d="M9 4.8h2l.4 1.5 1 .6 1.5-.4 1 1.7-1.1 1.1v1.3l1.1 1.1-1 1.7-1.5-.4-1 .6-.4 1.5H9l-.4-1.5-1-.6-1.5.4-1-1.7 1.1-1.1V9.3L5.1 8.2l1-1.7 1.5.4 1-.6Z"
            strokeWidth={1.1}
          />
          <circle cx="10" cy="10" r="1.8" fill={fill ? 'currentColor' : 'none'} strokeWidth={1.1} />
        </g>
        <circle cx="18.5" cy="16" r="4.5" />
        <path d="m22 19.5 3 3" strokeWidth={fill ? 2 : 1.4} />
      </svg>
    )
  return (
    <span
      aria-hidden
      className={cn('msym shrink-0 select-none', className)}
      style={{
        fontSize: size,
        fontVariationSettings: `"FILL" ${fill ? 1 : 0}, "wght" ${weight}, "GRAD" 0, "opsz" 24`,
      }}
    >
      {name}
    </span>
  )
}
