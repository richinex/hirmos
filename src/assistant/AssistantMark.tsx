import { cn } from '@/lib/utils'

export type AssistantMarkState = 'idle' | 'open' | 'working' | 'offline'

/** A confounder over a treatment and an outcome. Inline SVG, not the icon font, so it never shows as a ligature name while the font loads. */
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
  const edges = state === 'working' ? 'mark-flow' : undefined
  return (
    <svg
      aria-hidden
      width={size}
      height={size}
      viewBox="0 0 20 20"
      fill="none"
      stroke="currentColor"
      strokeWidth={1.6}
      strokeLinecap="round"
      className={cn('shrink-0', className)}
    >
      <path
        className={edges}
        d="M9 6.2 5.2 13.4M11 6.2l3.8 7.2"
        strokeDasharray={state === 'offline' ? '1.5 2' : undefined}
      />
      <path
        className={edges}
        d="M6.6 16h6.4"
        stroke={lit ? accent : undefined}
        strokeDasharray={state === 'offline' ? '1.5 2' : undefined}
      />
      <circle cx="10" cy="4" r="2" />
      <circle cx="4" cy="16" r="2" fill={lit ? 'currentColor' : 'none'} />
      <circle
        cx="16"
        cy="16"
        r="2"
        fill={lit ? accent : 'none'}
        stroke={lit ? accent : undefined}
      />
    </svg>
  )
}
