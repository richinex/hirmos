import * as RadixSelect from '@radix-ui/react-select'
import { Children, isValidElement, type ReactElement, type ReactNode } from 'react'
import { Icon } from '@/components/Icon'
import { cn } from '@/lib/utils'

/**
 * Radix Select behind the native `<select>` surface the panels already use: `value`, `onChange` with a
 * target value, and `<option>` children. The options are data; Radix renders the trigger, the list and
 * the keyboard model, so every theme gets the same list instead of the platform's.
 *
 * Radix refuses an item whose value is the empty string, so an `<option value="">` becomes the
 * placeholder and, when it is not disabled, a first item that resets the value to ''.
 */

const EMPTY = '__empty__'

interface OptionProps {
  readonly value?: string | number
  readonly disabled?: boolean
  readonly children?: ReactNode
}

interface Option {
  readonly value: string
  readonly label: string
  readonly disabled: boolean
}

const textOf = (node: ReactNode): string => {
  if (node === null || node === undefined || typeof node === 'boolean') return ''
  if (typeof node === 'string' || typeof node === 'number') return String(node)
  if (Array.isArray(node)) return node.map(textOf).join('')
  if (isValidElement<{ readonly children?: ReactNode }>(node)) return textOf(node.props.children)
  return ''
}

const collect = (children: ReactNode, into: Option[]): void => {
  Children.forEach(children, (child) => {
    if (!isValidElement(child)) return
    if (child.type === 'option') {
      const props = (child as ReactElement<OptionProps>).props
      into.push({
        value: props.value === undefined ? textOf(props.children) : String(props.value),
        label: textOf(props.children),
        disabled: props.disabled === true,
      })
      return
    }
    const nested = (child as ReactElement<{ readonly children?: ReactNode }>).props.children
    if (nested !== undefined) collect(nested, into)
  })
}

export function Select({
  value,
  onChange,
  children,
  className,
  disabled,
  id,
  trigger,
  heading,
  'aria-label': ariaLabel,
  'aria-labelledby': ariaLabelledBy,
}: {
  readonly value: string | number
  readonly onChange: (event: { readonly target: { readonly value: string } }) => void
  readonly children?: ReactNode
  readonly className?: string
  readonly disabled?: boolean
  readonly id?: string
  /** Shown in the trigger instead of the chosen label and the chevron, for a compact control such as a type glyph. */
  readonly trigger?: ReactNode
  /** A line above the options that names what is being chosen, for a trigger that shows no label. */
  readonly heading?: string
  readonly 'aria-label'?: string
  readonly 'aria-labelledby'?: string
}) {
  const options: Option[] = []
  collect(children, options)
  const current = String(value)
  const placeholder = options.find((option) => option.value === '')
  const items = options.filter((option) => option.value !== '' || !option.disabled)
  const selected = options.find((option) => option.value === current)
  return (
    <RadixSelect.Root
      value={current}
      onValueChange={(next) => onChange({ target: { value: next === EMPTY ? '' : next } })}
      disabled={disabled}
    >
      <RadixSelect.Trigger
        id={id}
        aria-label={ariaLabel}
        aria-labelledby={ariaLabelledBy}
        className={cn(
          'inline-flex max-w-full items-center justify-between gap-2 text-left data-[placeholder]:text-faint',
          className,
        )}
      >
        {trigger ?? (
          <>
            <span className="min-w-0 flex-1 truncate">
              <RadixSelect.Value placeholder={placeholder?.label ?? ''}>
                {selected !== undefined && selected.value !== '' ? selected.label : undefined}
              </RadixSelect.Value>
            </span>
            <RadixSelect.Icon aria-hidden className="flex shrink-0 text-faint">
              <Icon name="expand_more" size={14} />
            </RadixSelect.Icon>
          </>
        )}
      </RadixSelect.Trigger>
      <RadixSelect.Portal>
        <RadixSelect.Content
          position="popper"
          align="start"
          sideOffset={4}
          className={cn(
            'float z-(--z-popover) max-h-[min(20rem,var(--radix-select-content-available-height))] min-w-[var(--radix-select-trigger-width)] overflow-hidden rounded-lg border border-edge bg-panel text-body text-ink',
            trigger !== undefined && 'min-w-52',
          )}
        >
          <RadixSelect.ScrollUpButton aria-hidden className="flex justify-center py-0.5 text-faint">
            <Icon name="expand_less" size={14} />
          </RadixSelect.ScrollUpButton>
          <RadixSelect.Viewport className="p-1">
            {heading !== undefined && (
              <RadixSelect.Group>
                <RadixSelect.Label className="mb-1 border-b border-line px-2 pb-1.5 pt-1 text-label text-muted">
                  {heading}
                </RadixSelect.Label>
              </RadixSelect.Group>
            )}
            {items.map((option) => (
              <RadixSelect.Item
                key={option.value === '' ? EMPTY : option.value}
                value={option.value === '' ? EMPTY : option.value}
                disabled={option.disabled}
                className="relative flex cursor-pointer select-none items-center gap-2 rounded-md py-1.5 pl-2 pr-7 outline-none data-[disabled]:cursor-not-allowed data-[disabled]:text-dim data-[highlighted]:bg-well data-[state=checked]:text-ink"
              >
                <RadixSelect.ItemText>{option.label}</RadixSelect.ItemText>
                <RadixSelect.ItemIndicator
                  aria-hidden
                  className="absolute right-2 text-signal-text"
                >
                  <Icon name="check" size={13} />
                </RadixSelect.ItemIndicator>
              </RadixSelect.Item>
            ))}
          </RadixSelect.Viewport>
          <RadixSelect.ScrollDownButton
            aria-hidden
            className="flex justify-center py-0.5 text-faint"
          >
            <Icon name="expand_more" size={14} />
          </RadixSelect.ScrollDownButton>
        </RadixSelect.Content>
      </RadixSelect.Portal>
    </RadixSelect.Root>
  )
}
