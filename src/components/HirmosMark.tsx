/**
 * The brand mark: the back-door triangle. A common cause above points to the treatment and the
 * outcome, and the treatment points to the outcome along the base. All currentColor, so it follows
 * its host's ink; `public/hirmos.svg` carries the same geometry with heavier strokes for the tab.
 */
export function HirmosMark({ className, size = 20 }: { readonly className?: string; readonly size?: number }) {
  return (
    <svg width={size} height={size} viewBox="0 0 24 24" fill="none" aria-hidden className={className}>
      <g stroke="currentColor" strokeWidth="1.8" strokeLinecap="round">
        <path d="M10.59 7.93L7.88 12.8" />
        <path d="M13.41 7.93L16.12 12.8" />
        <path d="M8.1 17.6L13.3 17.6" />
      </g>
      <g fill="currentColor">
        <path d="M6.61 15.07L6.56 12.29L9 13.65Z" />
        <path d="M17.39 15.07L15 13.65L17.44 12.29Z" />
        <path d="M15.9 17.6L13.5 19L13.5 16.2Z" />
        <circle cx="12" cy="5.4" r="2" />
        <circle cx="5.2" cy="17.6" r="2" />
        <circle cx="18.8" cy="17.6" r="2" />
      </g>
    </svg>
  )
}
