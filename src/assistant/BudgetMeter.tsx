import { useEffect, useState } from 'react'
import { Icon } from '@/components/Icon'
import { label, num } from '@/components/ui/recipes'
import { cn } from '@/lib/utils'
import { clock, credits, WARN_AT, type CreditStatus } from './budget'
import type { ProviderFailure } from './telemetry'

export const SETTLE = 'duration-(--motion-kinetic) ease-(--motion-kinetic-ease)'

/** Kinetics' Countdown Ring, run forwards: the arc is the share of the wait that has passed. */
function Ring({ share }: { readonly share: number }) {
  const r = 6
  const c = 2 * Math.PI * r
  return (
    <svg aria-hidden width={16} height={16} viewBox="0 0 16 16" className="-rotate-90">
      <circle cx={8} cy={8} r={r} fill="none" strokeWidth={2} className="stroke-raised" />
      <circle
        cx={8}
        cy={8}
        r={r}
        fill="none"
        strokeWidth={2}
        strokeLinecap="round"
        strokeDasharray={c}
        strokeDashoffset={c * (1 - Math.min(share, 1))}
        className={cn(
          'transition-[stroke-dashoffset] duration-1000 ease-linear',
          share >= WARN_AT ? 'stroke-warn' : 'stroke-info',
        )}
      />
    </svg>
  )
}

/** Kinetics' Battery Charge for the key's own limit, in credits. It is not the account balance. */
export function CreditGauge({ credit }: { readonly credit: CreditStatus }) {
  if (credit.kind === 'unchecked') return null
  if (credit.kind === 'checking')
    return (
      <span className="flex items-center gap-1.5 text-faint">
        <span className="size-3 animate-spin rounded-full border-2 border-info/30 border-t-info" />
        Checking credit
      </span>
    )
  if (credit.kind === 'failed')
    return (
      <span className="text-faint" title={credit.message}>
        Credit unknown
      </span>
    )
  const share =
    credit.keyLimit !== null && credit.keyRemaining !== null && credit.keyLimit > 0
      ? Math.max(0, Math.min(credit.keyRemaining / credit.keyLimit, 1))
      : null
  return (
    <span className="flex min-w-0 items-center gap-2">
      <Icon
        name="account_balance_wallet"
        size={15}
        className={cn(
          'shrink-0 transition-colors',
          SETTLE,
          share === null ? 'text-faint' : share <= 1 - WARN_AT ? 'text-warn' : 'text-ok',
        )}
      />
      <span
        className={num('truncate')}
        title="The key's own spending limit. The account balance needs a management key to read."
      >
        {credit.keyRemaining === null
          ? 'No key limit'
          : `${credits(credit.keyRemaining)} credits left`}
      </span>
    </span>
  )
}

/** Seconds left until a moment, ticking down once a second. */
function useCountdown(until: number | null): number | null {
  const [now, setNow] = useState(() => Date.now())
  useEffect(() => {
    if (until === null) return
    const timer = setInterval(() => setNow(Date.now()), 1000)
    return () => clearInterval(timer)
  }, [until])
  return until === null ? null : Math.max(0, Math.ceil((until - now) / 1000))
}

/** A provider refusal, with a live wait for a rate limit and the reset time for a spending limit. */
export function ProviderFailureNotice({
  failure,
  at,
}: {
  readonly failure: ProviderFailure
  /** When the failure arrived, so the retry wait counts from then. */
  readonly at: number
}) {
  const left = useCountdown(
    failure.retryAfterSeconds === null ? null : at + failure.retryAfterSeconds * 1000,
  )
  const resets = failure.resetAt === null ? null : new Date(failure.resetAt)
  return (
    <div
      role="alert"
      className="flex flex-col gap-1.5 rounded-lg border border-danger/30 bg-danger/[0.06] p-2.5"
    >
      <p className="m-0 text-body text-danger">{failure.message}</p>
      {left === null ? null : (
        <span className={label('flex items-center gap-2 font-normal text-muted')}>
          <Ring share={failure.retryAfterSeconds ? 1 - left / failure.retryAfterSeconds : 1} />
          <span className={num()}>
            {left > 0 ? `Retry in ${clock(left * 1000)}` : 'You can retry now'}
          </span>
        </span>
      )}
      {resets === null || Number.isNaN(resets.getTime()) ? null : (
        <span className={label('font-normal text-muted')}>
          Resets {resets.toLocaleString(undefined, { dateStyle: 'medium', timeStyle: 'short' })}
        </span>
      )}
    </div>
  )
}
