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
  /** A Material Symbols ligature name, for example "terminal". */
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
