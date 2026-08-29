import type { ReactNode } from 'react'
import { cn } from '@/lib/utils'
import { label as labelCn } from './recipes'

export type AlertTone = 'danger' | 'warn' | 'ok' | 'info'

const TONE: Record<AlertTone, string> = {
  danger: 'border-danger/40 bg-danger/5 text-danger',
  warn: 'border-warn/40 bg-warn/5 text-warn',
  ok: 'border-ok/40 bg-ok/5 text-ok',
  info: 'border-[var(--color-info)]/40 bg-[var(--color-info)]/5 text-[var(--color-info)]',
}

/** The one status banner: every surface renders alerts through this instead of hand-templated border/bg/text triples. */
export function Alert({ tone, title, live = true, testId, className, children }: {
  readonly tone: AlertTone
  /** Optional micro-cap heading above the body, in the same tone. */
  readonly title?: string
  /** role="alert" for announcements; pass false for static caveats that should not interrupt. */
  readonly live?: boolean
  readonly testId?: string
  readonly className?: string
  readonly children: ReactNode
}) {
  return (
    <div role={live ? 'alert' : undefined} data-testid={testId} className={cn('rounded-lg border px-3 py-2.5 text-body', TONE[tone], className)}>
      {title && <p className={labelCn('mb-1')}>{title}</p>}
      {children}
    </div>
  )
}
