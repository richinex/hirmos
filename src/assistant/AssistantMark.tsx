import { cn } from '@/lib/utils'

export type AssistantMarkState = 'idle' | 'open' | 'working' | 'offline'

/** A helper whose antenna is a graph node. Inline SVG, not the icon font, so it never shows as a ligature name while the font loads. */
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
      <path className={state === 'working' ? 'mark-flow' : undefined} d="M12 5.25V8.25" />
      <circle
        cx="12"
        cy="3.75"
        r="1.5"
        fill={lit ? accent : 'none'}
        stroke={lit ? accent : undefined}
      />
      <rect
        x="4.5"
        y="8.25"
        width="15"
        height="11.25"
        strokeDasharray={state === 'offline' ? '1.5 2' : undefined}
      />
      <circle cx="9" cy="12.75" r="1.125" fill="currentColor" stroke="none" />
      <circle cx="15" cy="12.75" r="1.125" fill="currentColor" stroke="none" />
      <path d="M9.75 16.125H14.25" stroke={lit ? accent : undefined} />
    </svg>
  )
}
