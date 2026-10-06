import { useState, type ReactNode } from 'react'
import { Icon } from '@/components/Icon'
import { FloatingWindow } from '@/components/ui/FloatingWindow'

/**
 * A figure that can be lifted into a floating window.
 *
 * A figure is read at two scales: a glance inside the panel, and a close look where one lag, node or
 * time point matters. The header takes a row of actions, exports as a rule, beside the close button.
 */
export function FloatingFigure({
  label,
  defaultWidth = 900,
  defaultHeight = 560,
  actions,
  notice = null,
  figure,
  children,
}: {
  readonly label: string
  readonly defaultWidth?: number
  readonly defaultHeight?: number
  /** Controls for the window's header, left of the close button. */
  readonly actions?: ReactNode
  /** A line shown above the lifted figure, for a failed export. */
  readonly notice?: string | null
  /** The figure as drawn in the window, sized to fill it. */
  readonly figure: ReactNode
  /** The figure in place, given the button that opens the window, or null while it is open. */
  readonly children: (openButton: ReactNode) => ReactNode
}) {
  const [lifted, setLifted] = useState(false)

  const openButton = lifted ? null : (
    <button
      type="button"
      className="grid h-6 w-6 shrink-0 place-items-center rounded-md border border-hair bg-well text-faint transition-colors hover:border-edge hover:text-ink"
      aria-label={`Open ${label} in a floating window`}
      title="Open in a floating window"
      onClick={() => setLifted(true)}
    >
      <Icon name="open_in_full" size={13} />
    </button>
  )

  return (
    <>
      {children(openButton)}
      {lifted && (
        <FloatingWindow
          label={label}
          onClose={() => setLifted(false)}
          defaultWidth={defaultWidth}
          defaultHeight={defaultHeight}
          actions={actions}
        >
          {notice !== null && (
            <p role="alert" className="mb-2 mt-0 text-body text-danger">
              {notice}
            </p>
          )}
          {figure}
        </FloatingWindow>
      )}
    </>
  )
}
