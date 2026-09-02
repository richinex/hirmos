import { Icon } from '@/components/Icon'
import { Tooltip } from '@/components/ui/Tooltip'
import { cn } from '@/lib/utils'

export function ParameterHelp({ label, help }: { readonly label: string; readonly help: string }) {
  return (
    <Tooltip text={help}>
      {/* A focusable span carrying an explicit role, not a <button>: assistive technology needs a role
          to announce beside the label, but Radix's trigger stops opening on pointer hover when the
          element is a button, and hover is how most readers reach this note. Focus opens it too, so a
          keyboard reader gets the text by tabbing here; there is nothing to activate. */}
      <span
        role="button"
        tabIndex={0}
        className="grid size-5 shrink-0 cursor-help place-items-center rounded text-faint transition-colors hover:text-ink"
        aria-label={`About ${label}`}
      >
        <Icon name="info" size={14} />
      </span>
    </Tooltip>
  )
}

export function ParameterLabel({
  label,
  help,
  htmlFor,
  className,
}: {
  readonly label: string
  readonly help: string
  readonly htmlFor?: string
  readonly className?: string
}) {
  const text = htmlFor === undefined
    ? <span>{label}</span>
    : <label htmlFor={htmlFor}>{label}</label>

  return (
    <span className={cn(className, 'flex w-fit items-center gap-1')}>
      {text}
      <ParameterHelp label={label} help={help} />
    </span>
  )
}
