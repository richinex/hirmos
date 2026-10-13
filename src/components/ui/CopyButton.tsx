import { useEffect, useRef, useState } from 'react'
import { Icon } from '../Icon'
import { cn } from '@/lib/utils'
import { iconControl } from './recipes'

/**
 * Copy one string, with the icon-swap acknowledgement.
 *
 * A copy is silent, so without a confirmation the reader cannot tell a success from a dead button and
 * clicks again. The check shows for 1.6s, the same gesture and duration AssistantCard already uses.
 *
 * Failure is reported, not swallowed: `writeText` rejects on an insecure origin or a denied permission.
 */
export function CopyButton({
  text, label = 'Copy', size = 'compact', className,
}: {
  text: string
  label?: string
  /** Dense code blocks keep their 22px overlay; answer/tool chrome uses the shared 32px target. */
  size?: 'compact' | 'chrome'
  className?: string
}) {
  const [state, setState] = useState<'idle' | 'copied' | 'failed'>('idle')
  // Cleared on unmount: the execution log unmounts cells as it re-renders, and a timer that outlives its
  // component would set state on something that is gone.
  const timer = useRef<ReturnType<typeof setTimeout>>(undefined)
  useEffect(() => () => clearTimeout(timer.current), [])

  const copy = () => {
    clearTimeout(timer.current)
    void navigator.clipboard.writeText(text)
      .then(() => setState('copied'))
      .catch(() => setState('failed'))
      .finally(() => { timer.current = setTimeout(() => setState('idle'), 1600) })
  }

  const icon = state === 'copied' ? 'check' : state === 'failed' ? 'close' : 'content_copy'
  const tone = state === 'copied' ? 'text-ok' : state === 'failed' ? 'text-danger' : undefined
  return (
    <button
      type="button"
      onClick={copy}
      aria-label={state === 'failed' ? `${label} failed` : label}
      title={state === 'failed' ? 'Could not reach the clipboard' : label}
      className={cn(
        size === 'chrome'
          ? iconControl(state === 'copied' ? 'selected' : 'quiet')
          : 'grid h-[22px] w-[22px] shrink-0 place-items-center rounded-md border border-transparent text-faint transition-colors duration-150 hover:text-ink',
        className,
      )}
    >
      <Icon name={icon} size={14} className={tone} />
    </button>
  )
}
