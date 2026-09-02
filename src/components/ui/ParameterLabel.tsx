import { Icon } from '@/components/Icon'
import { TapNote, Tooltip } from '@/components/ui/Tooltip'
import { useMediaQuery } from '@/lib/useMediaQuery'
import { cn } from '@/lib/utils'

export function ParameterHelp({ label, help }: { readonly label: string; readonly help: string }) {
  // A finger cannot hover, and the hover note closes itself on the touch that opened it. Where the
  // pointer is coarse the same text is opened by a tap instead, and stays until it is dismissed.
  const coarse = useMediaQuery('(pointer: coarse)')
  const Note = coarse ? TapNote : Tooltip
  return (
    <Note text={help}>
      {/* A button, so the control carries a role: a focusable span with only an aria-label is announced
          without any sign that it does something. The cursor is left to the app's rule for buttons. */}
      <button
        type="button"
        className="grid size-5 shrink-0 place-items-center rounded text-faint transition-colors hover:text-ink pointer-coarse:size-9"
        aria-label={`About ${label}`}
      >
        <Icon name="info" size={14} />
      </button>
    </Note>
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
