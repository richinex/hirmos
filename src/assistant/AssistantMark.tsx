import { cn } from '@/lib/utils'

export type AssistantMarkState = 'idle' | 'open' | 'working' | 'offline'

/** The directed H in a speech bubble. Inline SVG, not the icon font, so it never shows as a ligature name while the font loads. */
export function AssistantMark({
  state = 'idle',
  size = 20,
  accent = 'var(--color-signal)',
  className,
}: {
  readonly state?: AssistantMarkState
  readonly size?: number
  readonly accent?: string
  readonly className?: string
}) {
  const lit = state === 'open' || state === 'working'
  return (
    <svg
      aria-hidden
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth={1.5}
      strokeLinecap="square"
      strokeLinejoin="miter"
      className={cn('shrink-0', className)}
    >
      <g transform="translate(12 12) scale(1.1) translate(-12 -12)">
        <path
          d="M3.75 3.75H20.25V16.5H10.5L6.75 20.25V16.5H3.75Z"
          strokeDasharray={state === 'offline' ? '1.5 2' : undefined}
        />
        <g transform="translate(3.75 2.125) scale(0.5)" stroke="none" fill="currentColor">
          <path d="M7 7h5v18H7z" />
          <path d="M21 7h4v5l-4-3zm0 16 4-3v5h-4z" />
          <path
            d="M12 14h6v-4l8 6-8 6v-4h-6z"
            fill={lit ? accent : 'currentColor'}
            className={state === 'working' ? 'mark-nudge' : undefined}
          />
        </g>
      </g>
    </svg>
  )
}
