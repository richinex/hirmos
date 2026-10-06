import {
  useCallback,
  useEffect,
  useId,
  useLayoutEffect,
  useRef,
  useState,
  type CSSProperties,
  type KeyboardEvent,
  type ReactNode,
} from 'react'
import { scrollBehavior, useScrollOverflow, type ScrollOverflow } from '@/lib/useScrollOverflow'
import { cn } from '@/lib/utils'
import { well } from './recipes'
import { ScrollArrow } from './ScrollArrow'

/**
 * A single-choice control in three forms: `track` and `line`, chosen by `variant`, and a chip grid, chosen
 * by `wrap`.
 *
 * `track`, the default, is a setting among settings: a track pressed into the panel, a knob lit from above
 * sitting on it, with spacing rather than dividers between options. Labels retain
 * their weight when selection changes. It sits level with the fields around it at the `sm` and `md` sizes.
 * `line` is a view switch: the labels on a hairline, a 2px rule of ink under the chosen one, and a pale
 * surface under the option the mouse is over. Use `line` where the choice changes what the stage shows,
 * `track` where it sets a value.
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
 * The knob, and in the line form the rule, is a measured overlay: it takes the frame of the displayed
 * option, so text and hit areas never move and the knob stays right when a track wraps onto a second line.
 * Its two edges move on separate clocks, the edge on the side of travel on the fast duration and the other
 * on the slow one, so the knob stretches toward the choice and settles without overshoot; reduced motion
 * drops it to a jump. A pointer can also press and slide along a track: a local preview follows the pointer
 * and commits only on release, and the knob presses in slightly while held. Cancelled gestures leave the
 * setting unchanged; vertical touch gestures can scroll the page.
 *
 * A line wider than its column stays on one line and scrolls; each end with more options fades and shows
 * an arrow.
 *
 * Keep to two to five options with short noun labels while the knob slides. Past five, pass `wrap`:
 * the knob gives way to a per-chip selected surface, so a wrapping catalogue stays calm instead of
 * sending the knob travelling across rows.
 */

export interface SegmentOption<V extends string> {
  readonly value: V
  readonly label: ReactNode
  readonly ariaLabel?: string
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
  /** `track` for a setting that carries a value; `line` for a view switch that changes what the stage shows. */
  readonly variant?: 'track' | 'line'
  /** Stretch to the container and give every segment the same width. Use only with labels of similar length. */
  readonly fill?: boolean
  readonly wrap?: boolean
  /**
   * `well` gives the control its own surface: the pressed track behind a sliding knob, or the recessed
   * well behind wrapping chips. `none` leaves the options on the surface they sit on; the knob still
   * reads by its own lifted fill. `line` has no surface either way.
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

interface Layout {
  readonly frames: readonly Frame[]
  readonly width: number
  readonly height: number
}

interface Knob {
  readonly frame: Frame
  readonly forward: boolean
}

interface Hover {
  readonly index: number
  readonly shown: boolean
  readonly placed: boolean
}

type Gesture<V extends string> =
  | { readonly kind: 'idle' }
  | { readonly kind: 'preview'; readonly pointer: number; readonly value: V }

type Form = 'track' | 'line' | 'wrap'

const sameFrame = (a: Frame | null | undefined, b: Frame | null | undefined): boolean =>
  a === b ||
  (a != null &&
    b != null &&
    a.left === b.left &&
    a.top === b.top &&
    a.width === b.width &&
    a.height === b.height)

const sameLayout = (a: Layout | null, b: Layout): boolean =>
  a !== null &&
  a.width === b.width &&
  a.height === b.height &&
  a.frames.length === b.frames.length &&
  a.frames.every((frame, i) => sameFrame(frame, b.frames[i]))

/** Option padding per form and size; the track and line forms are shorter than the chips because the host adds its own. */
const SIZE: Record<Form, Record<'sm' | 'md', string>> = {
  track: { sm: 'px-2 py-0.5 text-label', md: 'px-3 py-1 text-body' },
  line: { sm: 'px-1.5 py-1 text-label', md: 'px-2 py-1.5 text-body' },
  wrap: { sm: 'px-2 py-1 text-label', md: 'px-3 py-1.5 text-body' },
}

const LINE_RULE = 2

const EDGE_FADE = 32

/**
 * Wrap-grid chips draw their states on two pseudo-element layers, so selection and keyboard focus
 * never fight: `after` is the selected surface arriving from the centre behind the label, `before`
 * an outset signal ring that converges on focus, replacing the app-wide outline.
 */
const CHIP_LAYERS = [
  'isolate',
  "after:pointer-events-none after:absolute after:inset-0 after:-z-10 after:scale-90 after:rounded-md after:border after:border-edge after:bg-raised after:opacity-0 after:transition-[opacity,transform] after:duration-(--motion-fast) after:content-['']",
  "before:pointer-events-none before:absolute before:-inset-[3px] before:scale-110 before:rounded-lg before:border-2 before:border-signal before:opacity-0 before:transition-[opacity,transform] before:duration-(--motion-fast) before:content-['']",
  'has-[:focus-visible]:outline-none has-[:focus-visible]:before:scale-100 has-[:focus-visible]:before:opacity-100',
].join(' ')

const optionsIn = (element: HTMLElement): HTMLLabelElement[] =>
  Array.from(element.querySelectorAll<HTMLLabelElement>(':scope > [data-segment-option]'))

const fadeMask = ({ start, end }: ScrollOverflow): CSSProperties => {
  const mask = `linear-gradient(to right, transparent 0, #000 ${start ? EDGE_FADE : 0}px, #000 calc(100% - ${end ? EDGE_FADE : 0}px), transparent 100%)`
  return { maskImage: mask, WebkitMaskImage: mask }
}

export function SegmentedControl<V extends string>({
  value,
  onChange,
  options,
  ariaLabel,
  className,
  size = 'md',
  variant = 'track',
  fill = false,
  wrap = false,
  disabled = false,
  name,
  required = false,
  frame = 'well',
}: SegmentedControlProps<V>) {
  const host = useRef<HTMLDivElement>(null)
  const viewport = useRef<HTMLDivElement>(null)
  const [layout, setLayout] = useState<Layout | null>(null)
  const [knob, setKnob] = useState<Knob | null>(null)
  const [gesture, setGesture] = useState<Gesture<V>>({ kind: 'idle' })
  const [hover, setHover] = useState<Hover | null>(null)
  const [pointerFocus, setPointerFocus] = useState(false)
  const [motionReady, setMotionReady] = useState(false)
  const dragging = gesture.kind === 'preview'
  const displayed = gesture.kind === 'preview' ? gesture.value : value
  const generatedName = useId()
  const descriptionBase = useId().replaceAll(':', '')
  const resolvedName = name ?? `segmented-${generatedName}`
  const form: Form = wrap ? 'wrap' : variant
  const { overflow, page } = useScrollOverflow(viewport, host, form === 'line')
  const scrolls = overflow.start || overflow.end

  const measure = useCallback(() => {
    const element = host.current
    if (element === null) return
    // Sub-pixel rects, measured from the host's padding edge, so a fractional padding or font metric cannot leave the knob a pixel off.
    const box = element.getBoundingClientRect()
    const originX = box.left + element.clientLeft
    const originY = box.top + element.clientTop
    const labels = optionsIn(element)
    const frames = labels.map((label) => {
      const rect = label.getBoundingClientRect()
      return {
        left: rect.left - originX,
        top: rect.top - originY,
        width: rect.width,
        height: rect.height,
      }
    })
    const next = { frames, width: element.clientWidth, height: element.clientHeight }
    setLayout((previous) => (sameLayout(previous, next) ? previous : next))
    const target = wrap
      ? undefined
      : frames[labels.findIndex((label) => label.dataset['segmentPreview'] === 'true')]
    setKnob((previous) => {
      if (target === undefined) return null
      if (previous !== null && sameFrame(previous.frame, target)) return previous
      const forward =
        previous === null ||
        target.left + target.width / 2 >= previous.frame.left + previous.frame.width / 2
      return { frame: target, forward }
    })
  }, [wrap])

  useLayoutEffect(measure)

  useLayoutEffect(() => {
    const element = host.current
    if (element === null) return
    let pending = 0
    let active = true
    const schedule = () => {
      cancelAnimationFrame(pending)
      pending = requestAnimationFrame(() => {
        if (active) measure()
      })
    }
    const observer = new ResizeObserver(schedule)
    observer.observe(element)
    for (const option of optionsIn(element)) observer.observe(option)
    void document.fonts.ready.then(() => {
      if (active) schedule()
    })
    return () => {
      active = false
      observer.disconnect()
      cancelAnimationFrame(pending)
    }
  }, [measure, options.length])

  const signature = JSON.stringify(
    options.map((option) => [option.value, option.disabled === true]),
  )
  const cancelGesture = useCallback(() => {
    setGesture((current) => {
      if (current.kind === 'idle') return current
      return { kind: 'idle' }
    })
  }, [])
  useEffect(cancelGesture, [cancelGesture, signature, disabled, wrap, variant, value])
  useEffect(() => {
    const onVisibility = () => {
      if (document.visibilityState === 'hidden') cancelGesture()
    }
    window.addEventListener('blur', cancelGesture)
    document.addEventListener('visibilitychange', onVisibility)
    return () => {
      window.removeEventListener('blur', cancelGesture)
      document.removeEventListener('visibilitychange', onVisibility)
    }
  }, [cancelGesture])

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

  const hoverUnplaced = hover !== null && !hover.placed
  useEffect(() => {
    if (!hoverUnplaced) return
    const pending = requestAnimationFrame(() =>
      setHover((current) =>
        current === null || current.placed ? current : { ...current, placed: true },
      ),
    )
    return () => cancelAnimationFrame(pending)
  }, [hoverUnplaced])

  useEffect(() => {
    if (!import.meta.env.DEV) return
    if (options.length < 2)
      console.warn(`SegmentedControl “${ariaLabel}”: a single choice needs at least two options.`)
    if (new Set(options.map((option) => option.value)).size !== options.length)
      console.warn(
        `SegmentedControl “${ariaLabel}”: option values repeat; selection follows the first match.`,
      )
  }, [options, ariaLabel])

  const isEnabled = (index: number): boolean => !disabled && options[index]?.disabled !== true
  const checkedIndex = options.findIndex((option) => option.value === value)
  const displayedIndex = options.findIndex((option) => option.value === displayed)
  const firstEnabled = options.findIndex((_, index) => isEnabled(index))
  const tabStop = checkedIndex >= 0 && isEnabled(checkedIndex) ? checkedIndex : firstEnabled

  const revealed = form === 'line' && scrolls ? layout?.frames[checkedIndex] : undefined
  const revealedLeft = revealed?.left
  const revealedRight = revealed === undefined ? undefined : revealed.left + revealed.width
  useEffect(() => {
    const frameElement = viewport.current
    if (frameElement === null || revealedLeft === undefined || revealedRight === undefined) return
    const start = revealedLeft - EDGE_FADE
    const end = revealedRight + EDGE_FADE
    if (start < frameElement.scrollLeft)
      frameElement.scrollTo({ left: Math.max(0, start), behavior: scrollBehavior() })
    else if (end > frameElement.scrollLeft + frameElement.clientWidth)
      frameElement.scrollTo({ left: end - frameElement.clientWidth, behavior: scrollBehavior() })
  }, [revealedLeft, revealedRight])

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
    setPointerFocus(false)
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

  const optionUnder = (clientX: number, clientY: number): number => {
    const element = host.current
    if (element === null) return -1
    const index = optionsIn(element).findIndex((option) => {
      const rect = option.getBoundingClientRect()
      return (
        clientX >= rect.left &&
        clientX <= rect.right &&
        clientY >= rect.top &&
        clientY <= rect.bottom
      )
    })
    return index >= 0 && isEnabled(index) ? index : -1
  }

  const dragEnabled = !wrap && !disabled && variant === 'track'
  // A setting (the track form) that does not already fill its row stretches on a narrow panel; view switches and chip grids keep their width.
  // On a narrow panel a setting spans the row, and so does a short view switch: two or three tabs left
  // at one end read as unfinished. A longer row of tabs keeps its natural width and scrolls.
  const narrowFill = !fill && (form === 'track' || (form === 'line' && options.length <= 3))

  const knobElement =
    form === 'wrap' || knob === null || layout === null ? null : (
      <span
        aria-hidden
        data-motion={motionReady ? undefined : 'off'}
        className={cn(
          'segment-knob pointer-events-none absolute',
          form === 'track'
            ? 'rounded-md bg-knob [background-image:var(--gradient-knob)] shadow-(--shadow-knob)'
            : 'bg-ink',
        )}
        style={
          {
            left: knob.frame.left,
            right: layout.width - knob.frame.left - knob.frame.width,
            top:
              form === 'line' ? knob.frame.top + knob.frame.height - LINE_RULE / 2 : knob.frame.top,
            bottom:
              layout.height -
              knob.frame.top -
              knob.frame.height -
              (form === 'line' ? LINE_RULE / 2 : 0),
            '--segment-left':
              dragging || !knob.forward ? 'var(--motion-fast)' : 'var(--motion-slow)',
            '--segment-right':
              dragging || knob.forward ? 'var(--motion-fast)' : 'var(--motion-slow)',
            transform: dragging ? 'scale(0.96)' : undefined,
          } as CSSProperties
        }
      />
    )

  const hovered = form === 'line' && hover !== null ? layout?.frames[hover.index] : undefined
  const wash =
    hovered === undefined || hover === null ? null : (
      <span
        aria-hidden
        className={cn(
          'pointer-events-none absolute rounded-md bg-raised duration-(--motion-fast)',
          hover.placed ? 'transition-[left,top,width,height,opacity]' : 'transition-opacity',
        )}
        style={{
          left: hovered.left,
          top: hovered.top + 3,
          width: hovered.width,
          height: Math.max(0, hovered.height - 7),
          opacity: hover.shown && hover.index !== displayedIndex ? 1 : 0,
        }}
      />
    )

  const control = (
    <div
      ref={host}
      role="radiogroup"
      aria-label={ariaLabel}
      aria-disabled={disabled || undefined}
      onPointerDown={
        dragEnabled
          ? (event) => {
              if (event.button !== 0) return
              const option = options[optionUnder(event.clientX, event.clientY)]
              if (option === undefined) return
              event.preventDefault()
              setPointerFocus(true)
              host.current?.setPointerCapture(event.pointerId)
              setGesture({ kind: 'preview', pointer: event.pointerId, value: option.value })
            }
          : undefined
      }
      onPointerMove={
        dragEnabled
          ? (event) => {
              if (gesture.kind !== 'preview' || gesture.pointer !== event.pointerId) return
              const option = options[optionUnder(event.clientX, event.clientY)]
              if (option !== undefined && option.value !== gesture.value)
                setGesture({ ...gesture, value: option.value })
            }
          : undefined
      }
      onPointerUp={
        dragEnabled
          ? (event) => {
              if (gesture.kind !== 'preview' || gesture.pointer !== event.pointerId) return
              const index = optionUnder(event.clientX, event.clientY)
              cancelGesture()
              if (event.currentTarget.hasPointerCapture(event.pointerId))
                event.currentTarget.releasePointerCapture(event.pointerId)
              if (index >= 0) chooseAt(index)
            }
          : undefined
      }
      onPointerCancel={cancelGesture}
      onLostPointerCapture={cancelGesture}
      onPointerLeave={
        form === 'line'
          ? () =>
              setHover((current) =>
                current === null || !current.shown ? current : { ...current, shown: false },
              )
          : undefined
      }
      onBlur={
        dragEnabled
          ? (event) => {
              if (!(
                event.relatedTarget instanceof Node &&
                event.currentTarget.contains(event.relatedTarget)
              ))
                setPointerFocus(false)
            }
          : undefined
      }
      className={cn(
        'relative max-w-full',
        form === 'wrap' && cn('flex-wrap', frame === 'well' ? well('gap-1 p-1') : 'gap-1 p-1'),
        form === 'track' &&
          cn(
            'flex-wrap gap-1 rounded-lg p-1',
            frame === 'well' && 'bg-track shadow-(--shadow-track)',
          ),
        form === 'line' && 'w-max max-w-none flex-nowrap gap-1 border-b border-hair',
        dragEnabled && 'touch-pan-y',
        fill ? 'flex w-full' : form === 'line' ? 'flex' : 'inline-flex',
        // On a narrow panel a setting spans the row, so the choice reads as one bar rather than a stub.
        narrowFill && '@max-md/panel:flex @max-md/panel:w-full @max-md/panel:max-w-full',
        className,
      )}
    >
      {knobElement}
      {wash}
      {options.map((option, index) => {
        const checked = index === checkedIndex
        const enabled = isEnabled(index)
        const descriptionId = option.title === undefined ? undefined : `${descriptionBase}-${index}`
        return (
          <label
            key={option.value}
            data-segment-option
            data-segment-preview={option.value === displayed}
            title={option.title}
            onPointerEnter={
              form === 'line'
                ? (event) => {
                    if (event.pointerType === 'mouse')
                      setHover((current) => ({
                        index,
                        shown: true,
                        placed: current !== null && current.shown,
                      }))
                  }
                : undefined
            }
            className={cn(
              'relative grid cursor-pointer place-items-center whitespace-nowrap border border-transparent transition-colors duration-(--motion-fast) pointer-coarse:min-h-11',
              !(form === 'track' && pointerFocus) &&
                'has-[:focus-visible]:outline-2 has-[:focus-visible]:outline-offset-2 has-[:focus-visible]:outline-signal',
              form === 'line' ? 'shrink-0 rounded-sm' : 'rounded-md',
              SIZE[form][size],
              fill &&'min-w-max flex-1 basis-0 text-center',
              // Equal widths while they fit; no option narrower than its own label, so a long set wraps to a second row.
              narrowFill &&
                '@max-md/panel:min-w-max @max-md/panel:flex-1 @max-md/panel:basis-0 @max-md/panel:text-center',
              form !== 'wrap' && 'font-medium',
              wrap && CHIP_LAYERS,
              !enabled
                ? cn('cursor-not-allowed text-faint', wrap && 'hatch')
                : option.value === displayed
                  ? cn(
                      'text-ink',
                      form !== 'wrap' && 'font-medium',
                      wrap && 'after:scale-100 after:opacity-100',
                    )
                  : cn(
                      'text-muted hover:text-ink',
                      wrap && 'active:after:scale-100 active:after:opacity-40',
                    ),
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
              aria-label={option.ariaLabel}
              tabIndex={index === tabStop ? 0 : -1}
              onChange={() => {
                if (enabled && !checked) onChange(option.value)
              }}
              onKeyDown={(event) => onKeyDown(event, index)}
            />
            <span className="relative z-10 min-w-0">{option.label}</span>
            {descriptionId !== undefined && (
              <span id={descriptionId} className="sr-only">
                {option.title}
              </span>
            )}
          </label>
        )
      })}
    </div>
  )

  if (form !== 'line') return control

  return (
    <div className="relative flow-root min-w-0 max-w-full">
      <div
        ref={viewport}
        className={cn(
          'max-w-full',
          scrolls &&
            '-my-1 scroll-px-8 overflow-x-auto overflow-y-hidden py-1 [scrollbar-width:none] [&::-webkit-scrollbar]:hidden',
        )}
        style={scrolls ? fadeMask(overflow) : undefined}
      >
        {control}
      </div>
      {scrolls
        ? (['start', 'end'] as const).map((end) => (
            <span
              key={end}
              className={cn(
                'pointer-events-none absolute inset-y-0 z-20 flex items-center pb-px transition-opacity duration-(--motion-fast) motion-reduce:transition-none',
                end === 'start' ? 'left-0' : 'right-0',
                overflow[end] ? 'opacity-100' : 'invisible opacity-0',
              )}
            >
              <ScrollArrow
                end={end}
                size="sm"
                className="pointer-events-auto"
                label={end === 'end' ? 'Scroll to more options' : 'Scroll back'}
                title={end === 'end' ? 'More options to the right' : 'Options to the left'}
                onClick={() => page(end === 'end' ? 1 : -1)}
              />
            </span>
          ))
        : null}
    </div>
  )
}
