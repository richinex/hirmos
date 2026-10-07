import { useState, type ChangeEvent, type InputHTMLAttributes, type KeyboardEvent } from 'react'

type NumberInputProps = Omit<InputHTMLAttributes<HTMLInputElement>, 'type' | 'inputMode' | 'role'>

/** The field's number, or NaN when it is blank or unreadable. */
export const numberValue = (target: HTMLInputElement): number =>
  target.value.trim() === '' ? NaN : Number(target.value)

const read = (text: string): number => (text.trim() === '' ? NaN : Number(text))
const same = (a: number, b: number): boolean => a === b || (Number.isNaN(a) && Number.isNaN(b))
const decimals = (value: number): number => (String(value).split('.')[1] ?? '').length

/**
 * A number field that always writes a decimal point. A native number input follows the
 * operating system's region, so in many regions it shows 0,05 for 0.05.
 */
export function NumberInput({ onChange, onKeyDown, min, max, step, value, ...rest }: NumberInputProps) {
  const shown = value === undefined || (typeof value === 'number' && !Number.isFinite(value)) ? '' : String(value)
  const [draft, setDraft] = useState(shown)
  // Text such as "0." or "1e" is kept while it still reads as the value the parent holds.
  const text = same(read(draft), read(shown)) || draft === shown ? draft : shown

  const change = (event: ChangeEvent<HTMLInputElement>) => {
    event.currentTarget.value = event.currentTarget.value.replace(',', '.')
    setDraft(event.currentTarget.value)
    onChange?.(event)
  }

  const nudge = (event: KeyboardEvent<HTMLInputElement>) => {
    onKeyDown?.(event)
    if (event.defaultPrevented || (event.key !== 'ArrowUp' && event.key !== 'ArrowDown')) return
    const target = event.currentTarget
    const increment = typeof step === 'number' ? step : Number(step) || 1
    const current = numberValue(target)
    const next = (Number.isFinite(current) ? current : 0) + (event.key === 'ArrowUp' ? increment : -increment)
    const bounded = Math.min(Number(max ?? Infinity), Math.max(Number(min ?? -Infinity), next))
    event.preventDefault()
    // React tracks the input's value, so the native setter is used for the change to reach onChange.
    const setter = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')?.set
    setter?.call(target, String(Number(bounded.toFixed(Math.max(decimals(current), decimals(increment))))))
    target.dispatchEvent(new Event('input', { bubbles: true }))
  }

  const numeric = read(text)
  return (
    <input
      {...rest}
      type="text"
      inputMode="decimal"
      role="spinbutton"
      autoComplete="off"
      spellCheck={false}
      aria-valuenow={Number.isFinite(numeric) ? numeric : undefined}
      aria-valuemin={min === undefined ? undefined : Number(min)}
      aria-valuemax={max === undefined ? undefined : Number(max)}
      value={text}
      onChange={change}
      onKeyDown={nudge}
    />
  )
}
