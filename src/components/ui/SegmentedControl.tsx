import { useCallback, useLayoutEffect, useRef, useState, type KeyboardEvent, type ReactNode } from 'react'
import { cn } from '@/lib/utils'

/**
 * A single-choice control: one raised knob slides to the chosen option inside a bordered well.
 *
 * Semantics follow the WAI-ARIA radio group pattern. The host is a `radiogroup`, each option a `radio`
 * carrying `aria-checked`, and exactly one option holds the Tab stop (roving tabindex): the checked
 * option, or the first enabled one when nothing is chosen yet. Arrow keys move focus and choose,
 * wrapping at either end; Home and End jump to the first and last option; Space and Enter choose the
 * focused option through the button's own click.
 *
 * A disabled option stays in the arrow-key order so a keyboard user can reach the `title` that says
 * why, but it never takes the Tab stop and choosing it is a no-op.
 *
 * The knob is a measured overlay. It takes the left, top, width and height of the checked option, so
 * text and hit areas never move and the knob stays right when the group wraps onto a second line. The
 * house ease-out carries the slide; reduced motion drops it to a jump.
 *
 * Keep to two to five options with short noun labels. Past five, or when labels will not fit on one
 * line, use `Select` instead of letting the group wrap.
 */

export interface SegmentOption<V extends string> {
  readonly value: V
  readonly label: ReactNode
  readonly disabled?: boolean
  /** Shown on hover and read by screen readers as the option's description; use it for the reason an option is disabled. */
  readonly title?: string
}

export interface SegmentedControlProps<V extends string> {
  readonly value: V | null
  readonly onChange: (next: V) => void
  readonly options: readonly SegmentOption<V>[]
  readonly ariaLabel: string
  readonly className?: string
  /** `sm` is the 24px chrome size for toolbars; `md` the 30px field size, matching `button()`. */
  readonly size?: 'sm' | 'md'
  /** Stretch to the container and give every segment the same width. Use only with labels of similar length. */
  readonly fill?: boolean
  readonly disabled?: boolean
}

interface Frame {
  readonly left: number
  readonly top: number
  readonly width: number
  readonly height: number
}

const sameFrame = (a: Frame | null, b: Frame | null): boolean =>
  a === b || (a !== null && b !== null && a.left === b.left && a.top === b.top && a.width === b.width && a.height === b.height)

const SIZE: Record<'sm' | 'md', string> = {
  sm: 'px-2 py-1 text-label',
  md: 'px-3 py-1.5 text-body',
}

export function SegmentedControl<V extends string>({ value, onChange, options, ariaLabel, className, size = 'md', fill = false, disabled = false }: SegmentedControlProps<V>) {
  const host = useRef<HTMLDivElement>(null)
  const [knob, setKnob] = useState<Frame | null>(null)

  const measure = useCallback(() => {
    const element = host.current
    if (element === null) return
    const checked = element.querySelector<HTMLButtonElement>('[role="radio"][aria-checked="true"]')
    if (checked === null) { setKnob(null); return }
    // Sub-pixel rects, measured from the host's padding edge, so a fractional padding or font metric cannot leave the knob a pixel off.
    const hostRect = element.getBoundingClientRect()
    const rect = checked.getBoundingClientRect()
    const next = { left: rect.left - hostRect.left - element.clientLeft, top: rect.top - hostRect.top - element.clientTop, width: rect.width, height: rect.height }
    setKnob((previous) => (sameFrame(previous, next) ? previous : next))
  }, [])

  useLayoutEffect(measure)

  useLayoutEffect(() => {
    const element = host.current
    if (element === null) return
    const observer = new ResizeObserver(measure)
    observer.observe(element)
    for (const radio of element.querySelectorAll('[role="radio"]')) observer.observe(radio)
    void document.fonts.ready.then(measure)
    return () => observer.disconnect()
  }, [measure, options.length])

  const isEnabled = (index: number): boolean => !disabled && options[index]?.disabled !== true
  const checkedIndex = options.findIndex((option) => option.value === value)
  const firstEnabled = options.findIndex((_, index) => isEnabled(index))
  const tabStop = checkedIndex >= 0 && isEnabled(checkedIndex) ? checkedIndex : firstEnabled

  const chooseAt = (index: number) => {
    const option = options[index]
    if (option === undefined) return
    host.current?.querySelectorAll<HTMLButtonElement>('[role="radio"]')[index]?.focus()
    if (isEnabled(index) && option.value !== value) onChange(option.value)
  }

  const neighbour = (from: number, direction: 1 | -1): number => (from + direction + options.length) % options.length

  const onKeyDown = (event: KeyboardEvent<HTMLButtonElement>, index: number) => {
    switch (event.key) {
      case 'ArrowRight':
      case 'ArrowDown':
        event.preventDefault()
        chooseAt(neighbour(index, 1))
        return
      case 'ArrowLeft':
      case 'ArrowUp':
        event.preventDefault()
        chooseAt(neighbour(index, -1))
        return
      case 'Home':
        event.preventDefault()
        chooseAt(0)
        return
      case 'End':
        event.preventDefault()
        chooseAt(options.length - 1)
        return
      default:
        return
    }
  }

  return (
    <div
      ref={host}
      role="radiogroup"
      aria-label={ariaLabel}
      aria-disabled={disabled || undefined}
      className={cn(
        'relative max-w-full flex-wrap gap-1 rounded-lg border border-hair bg-well p-1',
        fill ? 'flex w-full' : 'inline-flex',
        className,
      )}
    >
      {knob !== null && (
        <span
          aria-hidden
          className="pointer-events-none absolute left-0 top-0 rounded-md border border-edge bg-raised transition-[transform,width,height] duration-200 ease-[cubic-bezier(0.2,0.7,0.2,1)] motion-reduce:transition-none"
          style={{ width: knob.width, height: knob.height, transform: `translate(${knob.left}px, ${knob.top}px)` }}
        />
      )}
      {options.map((option, index) => {
        const checked = index === checkedIndex
        const enabled = isEnabled(index)
        return (
          <button
            key={option.value}
            type="button"
            role="radio"
            aria-checked={checked}
            aria-disabled={enabled ? undefined : true}
            tabIndex={index === tabStop ? 0 : -1}
            title={option.title}
            onClick={() => { if (enabled && !checked) onChange(option.value) }}
            onKeyDown={(event) => onKeyDown(event, index)}
            className={cn(
              'relative whitespace-nowrap rounded-md border border-transparent transition-colors duration-150 pointer-coarse:min-h-10',
              SIZE[size],
              fill && 'min-w-0 flex-1 basis-0 truncate text-center',
              !enabled ? 'cursor-not-allowed text-faint' : checked ? 'text-ink' : 'text-muted hover:text-ink',
            )}
          >
            {option.label}
          </button>
        )
      })}
    </div>
  )
}
