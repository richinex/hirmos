import { useCallback, useEffect, useId, useLayoutEffect, useRef, useState, type KeyboardEvent, type ReactNode } from 'react'
import { cn } from '@/lib/utils'
import { well } from './recipes'

/**
 * A single-choice control: one raised knob slides to the chosen option inside a bordered well.
 *
 * Semantics follow the WAI-ARIA radio group pattern. The host is a `radiogroup` and each option owns a
 * native radio input, so form submission, required state and assistive-technology behavior do not have
 * to be recreated with buttons. Exactly one option holds the Tab stop (roving tabindex): the checked
 * option, or the first enabled one when nothing is chosen yet. Arrow keys move focus and choose,
 * wrapping at either end; Home and End jump to the first and last option; Space uses the native radio
 * behavior and Enter is supported as an additional explicit selection gesture.
 *
 * Disabled options are skipped by arrow, Home and End navigation. Their explanatory `title` remains
 * available from the visible option while the native input carries the disabled state.
 *
 * The knob is a measured overlay. It takes the left, top, width and height of the checked option, so
 * text and hit areas never move and the knob stays right when the group wraps onto a second line. The
 * house ease-out carries the slide; reduced motion drops it to a jump. A pointer can also press and
 * slide along the well: the choice follows the pointer live and settles where it lifts, so the well
 * owns its touch gesture and a swipe on it selects rather than scrolls. While the pointer holds the
 * knob it tracks one-to-one with no easing and presses in slightly; the release settles on the house
 * curve.
 *
 * Keep to two to five options with short noun labels while the knob slides. Past five, pass `wrap`:
 * the knob gives way to a per-chip selected surface, so a wrapping catalogue stays calm instead of
 * sending the knob travelling across rows.
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
  /** A wrapping choice grid for longer catalogues: no sliding knob; each chip carries its own selected surface, focus ring, press acknowledgement, and a hatch when disabled. */
  readonly wrap?: boolean
  /**
   * `well` gives the track its own recessed surface; `none` leaves it on the surface it sits on.
   *
   * The knob reads by its own raised fill against whatever is behind it, so a track dropped into a
   * well of the same fill adds a border without adding contrast.
   */
  readonly frame?: 'well' | 'none'
  readonly disabled?: boolean
  /** Native form field name. A stable, instance-local name is generated when this is omitted. */
  readonly name?: string
  /** Applies the native radio-group required constraint when the control participates in a form. */
  readonly required?: boolean
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

/**
 * Wrap-grid chips draw their states on two pseudo-element layers, so selection and keyboard focus
 * never fight: `after` is the selected surface arriving from the centre behind the label, `before`
 * an outset signal ring that converges on focus, replacing the app-wide outline.
 */
const CHIP_LAYERS = [
  'isolate',
  "after:pointer-events-none after:absolute after:inset-0 after:-z-10 after:scale-90 after:rounded-md after:border after:border-edge after:bg-raised after:opacity-0 after:transition-[opacity,transform] after:duration-150 after:content-['']",
  "before:pointer-events-none before:absolute before:-inset-[3px] before:scale-110 before:rounded-lg before:border-2 before:border-signal before:opacity-0 before:transition-[opacity,transform] before:duration-150 before:content-['']",
  'has-[:focus-visible]:outline-none has-[:focus-visible]:before:scale-100 has-[:focus-visible]:before:opacity-100',
].join(' ')

export function SegmentedControl<V extends string>({ value, onChange, options, ariaLabel, className, size = 'md', fill = false, wrap = false, disabled = false, name, required = false, frame = 'well' }: SegmentedControlProps<V>) {
  const host = useRef<HTMLDivElement>(null)
  const [knob, setKnob] = useState<Frame | null>(null)
  const [dragging, setDragging] = useState(false)
  const [motionReady, setMotionReady] = useState(false)
  const generatedName = useId()
  const descriptionBase = useId().replaceAll(':', '')
  const resolvedName = name ?? `segmented-${generatedName}`

  const measure = useCallback(() => {
    if (wrap) { setKnob(null); return }
    const element = host.current
    if (element === null) return
    const checked = element.querySelector<HTMLInputElement>('input[type="radio"]:checked')
    const option = checked?.closest<HTMLLabelElement>('[data-segment-option]')
    if (option === null || option === undefined) { setKnob(null); return }
    // Sub-pixel rects, measured from the host's padding edge, so a fractional padding or font metric cannot leave the knob a pixel off.
    const hostRect = element.getBoundingClientRect()
    const rect = option.getBoundingClientRect()
    const next = { left: rect.left - hostRect.left - element.clientLeft, top: rect.top - hostRect.top - element.clientTop, width: rect.width, height: rect.height }
    setKnob((previous) => (sameFrame(previous, next) ? previous : next))
  }, [wrap])

  useLayoutEffect(measure)

  useLayoutEffect(() => {
    const element = host.current
    if (element === null) return
    const observer = new ResizeObserver(measure)
    observer.observe(element)
    for (const option of element.querySelectorAll('[data-segment-option]')) observer.observe(option)
    void document.fonts.ready.then(measure)
    return () => observer.disconnect()
  }, [measure, options.length])

  // The indicator is present for the first measured paint but cannot animate until its geometry has
  // remained mounted for two frames. This prevents a font or layout measurement from looking like a
  // user-initiated selection change when the control first appears.
  useLayoutEffect(() => {
    let secondFrame = 0
    const firstFrame = window.requestAnimationFrame(() => {
      secondFrame = window.requestAnimationFrame(() => setMotionReady(true))
    })
    return () => {
      window.cancelAnimationFrame(firstFrame)
      window.cancelAnimationFrame(secondFrame)
    }
  }, [])

  useEffect(() => {
    if (!import.meta.env.DEV) return
    if (options.length < 2) console.warn(`SegmentedControl “${ariaLabel}”: a single choice needs at least two options.`)
    if (new Set(options.map((option) => option.value)).size !== options.length) console.warn(`SegmentedControl “${ariaLabel}”: option values repeat; selection follows the first match.`)
  }, [options, ariaLabel])

  const isEnabled = (index: number): boolean => !disabled && options[index]?.disabled !== true
  const checkedIndex = options.findIndex((option) => option.value === value)
  const firstEnabled = options.findIndex((_, index) => isEnabled(index))
  const tabStop = checkedIndex >= 0 && isEnabled(checkedIndex) ? checkedIndex : firstEnabled

  const chooseAt = (index: number) => {
    const option = options[index]
    if (option === undefined || !isEnabled(index)) return
    host.current?.querySelectorAll<HTMLInputElement>('input[type="radio"]')[index]?.focus()
    if (option.value !== value) onChange(option.value)
  }

  const neighbour = (from: number, direction: 1 | -1): number => {
    for (let offset = 1; offset <= options.length; offset += 1) {
      const candidate = (from + direction * offset + options.length) % options.length
      if (isEnabled(candidate)) return candidate
    }
    return -1
  }

  const onKeyDown = (event: KeyboardEvent<HTMLInputElement>, index: number) => {
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
        chooseAt(firstEnabled)
        return
      case 'End':
        event.preventDefault()
        chooseAt(neighbour(0, -1))
        return
      case 'Enter':
        event.preventDefault()
        chooseAt(index)
        return
      default:
        return
    }
  }

  const radios = (): HTMLInputElement[] => Array.from(host.current?.querySelectorAll<HTMLInputElement>('input[type="radio"]') ?? [])
  const optionElements = (): HTMLLabelElement[] => Array.from(host.current?.querySelectorAll<HTMLLabelElement>('[data-segment-option]') ?? [])

  const chooseUnder = (clientX: number, clientY: number) => {
    const index = optionElements().findIndex((option) => {
      const rect = option.getBoundingClientRect()
      return clientX >= rect.left && clientX <= rect.right && clientY >= rect.top && clientY <= rect.bottom
    })
    const option = options[index]
    if (index >= 0 && isEnabled(index) && option !== undefined && option.value !== value) onChange(option.value)
  }

  const dragEnabled = !wrap && !disabled

  return (
    <div
      ref={host}
      role="radiogroup"
      aria-label={ariaLabel}
      aria-disabled={disabled || undefined}
      onPointerDown={dragEnabled ? (event) => {
        if (event.button !== 0) return
        setDragging(true)
        host.current?.setPointerCapture(event.pointerId)
        chooseUnder(event.clientX, event.clientY)
      } : undefined}
      onPointerMove={dragEnabled && dragging ? (event) => chooseUnder(event.clientX, event.clientY) : undefined}
      onPointerUp={dragEnabled ? () => { setDragging(false); radios()[checkedIndex]?.focus() } : undefined}
      onPointerCancel={dragEnabled ? () => setDragging(false) : undefined}
      className={cn(
        frame === 'well' ? well('relative max-w-full flex-wrap gap-1 p-1') : 'relative max-w-full flex-wrap gap-1 p-1',
        !wrap && 'touch-none',
        fill ? 'flex w-full' : 'inline-flex',
        className,
      )}
    >
      {!wrap && knob !== null && (
        <span
          aria-hidden
          className={cn(
            'pointer-events-none absolute left-0 top-0 rounded-md border border-edge bg-raised duration-200 ease-[cubic-bezier(0.2,0.7,0.2,1)] motion-reduce:transition-none',
            !motionReady ? 'transition-none' : dragging ? 'transition-[width,height]' : 'transition-[transform,width,height]',
          )}
          style={{ width: knob.width, height: knob.height, transform: `translate(${knob.left}px, ${knob.top}px)${dragging ? ' scale(0.96)' : ''}` }}
        />
      )}
      {options.map((option, index) => {
        const checked = index === checkedIndex
        const enabled = isEnabled(index)
        const descriptionId = option.title === undefined ? undefined : `${descriptionBase}-${index}`
        return (
          <label
            key={option.value}
            data-segment-option
            title={option.title}
            className={cn(
              'relative grid cursor-pointer place-items-center whitespace-nowrap rounded-md border border-transparent transition-colors duration-150 has-[:focus-visible]:outline-2 has-[:focus-visible]:outline-offset-2 has-[:focus-visible]:outline-signal pointer-coarse:min-h-10',
              SIZE[size],
              fill && 'min-w-0 flex-1 basis-0 truncate text-center',
              wrap && CHIP_LAYERS,
              !enabled
                ? cn('cursor-not-allowed text-faint', wrap && 'hatch')
                : checked
                  ? cn('text-ink', wrap && 'after:scale-100 after:opacity-100')
                  : cn('text-muted hover:text-ink', wrap && 'active:after:scale-100 active:after:opacity-40'),
            )}
          >
            <input
              type="radio"
              className="absolute inset-0 z-20 m-0 h-full w-full cursor-[inherit] appearance-none opacity-0"
              name={resolvedName}
              value={option.value}
              checked={checked}
              disabled={!enabled}
              required={required}
              aria-disabled={enabled ? undefined : true}
              aria-describedby={descriptionId}
              tabIndex={index === tabStop ? 0 : -1}
              onChange={() => { if (enabled && !checked) onChange(option.value) }}
              onKeyDown={(event) => onKeyDown(event, index)}
            />
            <span className="relative z-10 min-w-0">{option.label}</span>
            {descriptionId !== undefined && <span id={descriptionId} className="sr-only">{option.title}</span>}
          </label>
        )
      })}
    </div>
  )
}
