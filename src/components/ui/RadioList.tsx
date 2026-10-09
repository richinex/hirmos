import { useCallback, useId, useRef, type KeyboardEvent, type ReactNode } from 'react'
import { fieldLabel, well } from '@/components/ui/recipes'
import { cn } from '@/lib/utils'
import { ParameterHelp } from '@/components/ui/ParameterLabel'

/**
 * A single choice where each option needs a line of explanation, or where labels would wrap a
 * segmented control: stacked native radios, one row each, the chosen row raised. Use the segmented
 * knob for two or three short options that always fit one line; use this when they do not.
 */

export interface RadioOption<V extends string> {
  readonly value: V
  readonly label: string
  readonly hint?: string
  readonly help?: string
  readonly disabled?: boolean
  /** Shown on hover; use it for the reason an option is disabled. */
  readonly title?: string
}

export function RadioList<V extends string>({
  legend,
  legendHidden = false,
  value,
  onChange,
  options,
  className,
  columns = 1,
  frame = 'well',
}: {
  readonly legend: ReactNode
  /** Keep the legend for assistive technology only, when a heading directly above already names the choice. */
  readonly legendHidden?: boolean
  readonly value: V | null
  readonly onChange: (next: V) => void
  readonly options: readonly RadioOption<V>[]
  readonly className?: string
  /** Two columns keep a long list of short options from stretching across a wide panel. */
  readonly columns?: 1 | 2
  /**
   * `well` draws the group its own surface; `none` leaves it on the surface it sits on.
   *
   * The surface says "these options are one control". Where a heading, field label or rule directly
   * above already says that, drawing it again puts a box inside a box and the border stops carrying
   * meaning. The chosen row keeps its own surface either way, since that is what marks the selection.
   */
  readonly frame?: 'well' | 'none'
}) {
  const name = useId()
  const group = useRef<HTMLFieldSetElement | null>(null)

  // Browsers move a radio group with the arrow keys but not with Home and End, which a list of six
  // methods is long enough to want. Disabled options are skipped, as the arrow keys already skip them.
  const jumpToEnd = useCallback(
    (event: KeyboardEvent<HTMLFieldSetElement>) => {
      if (event.key !== 'Home' && event.key !== 'End') return
      const radios = [
        ...(group.current?.querySelectorAll<HTMLInputElement>(
          'input[type="radio"]:not(:disabled)',
        ) ?? []),
      ]
      const target = event.key === 'Home' ? radios[0] : radios.at(-1)
      if (target === undefined) return
      event.preventDefault()
      target.focus()
      if (!target.checked) onChange(target.value as V)
    },
    [onChange],
  )

  return (
    <fieldset
      ref={group}
      role="radiogroup"
      onKeyDown={jumpToEnd}
      className={cn('m-0 min-w-0 border-0 p-0', className)}
    >
      <legend className={legendHidden ? 'sr-only' : fieldLabel}>{legend}</legend>
      <div
        className={cn(
          'grid gap-1',
          frame === 'well' && well('p-1'),
          columns === 2 && '@3xl/panel:grid-cols-2',
          !legendHidden && 'mt-1',
        )}
      >
        {options.map((option) => {
          const chosen = option.value === value
          const labelId = `${name}-${option.value}`
          const hintId = `${labelId}-hint`
          return (
            <div key={option.value} className="relative min-w-0">
              <label
                title={option.title}
                className={cn(
                  'relative grid grid-cols-[auto_1fr] items-start gap-x-2.5 rounded-md border px-2.5 py-1.5 transition-colors duration-(--motion-fast) pointer-coarse:min-h-11',
                  option.disabled ? 'cursor-not-allowed text-faint' : 'cursor-pointer',
                  chosen ? 'border-edge bg-raised' : 'border-transparent hover:border-hair',
                  option.help !== undefined && 'pr-9 pointer-coarse:pr-12',
                )}
              >
                <input
                  type="radio"
                  name={name}
                  value={option.value}
                  checked={chosen}
                  disabled={option.disabled}
                  aria-labelledby={labelId}
                  aria-describedby={option.hint === undefined ? undefined : hintId}
                  onChange={() => {
                    if (!chosen) onChange(option.value)
                  }}
                  className="mt-1 accent-signal"
                />
                <span className="min-w-0">
                  <span
                    id={labelId}
                    className={cn('block text-body', option.disabled ? 'text-faint' : 'text-ink')}
                  >
                    {option.label}
                  </span>
                  {option.hint !== undefined && (
                    <span id={hintId} className="block text-label text-faint">
                      {option.hint}
                    </span>
                  )}
                </span>
              </label>
              {option.help !== undefined && (
                <span className="absolute right-1 top-1 pointer-coarse:top-0">
                  <ParameterHelp label={option.label} help={option.help} />
                </span>
              )}
            </div>
          )
        })}
      </div>
    </fieldset>
  )
}
