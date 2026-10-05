import { useEffect, useRef, type ReactNode } from 'react'
import { Icon } from '@/components/Icon'
import { iconControl } from './recipes'

/** Shared native-modal surface: focus containment and Escape are supplied by the browser. */
export function Dialog({
  open,
  title,
  onClose,
  role = 'dialog',
  dismissible = false,
  children,
}: {
  readonly open: boolean
  readonly title: string
  readonly onClose: () => void
  readonly role?: 'dialog' | 'alertdialog'
  readonly dismissible?: boolean
  readonly children: ReactNode
}) {
  const host = useRef<HTMLDialogElement>(null)
  const backdropPress = useRef(false)
  const outside = (element: HTMLDialogElement, x: number, y: number) => {
    const bounds = element.getBoundingClientRect()
    return x < bounds.left || x > bounds.right || y < bounds.top || y > bounds.bottom
  }
  useEffect(() => {
    const dialog = host.current
    if (dialog === null) return
    if (open && !dialog.open) dialog.showModal()
    if (!open && dialog.open) dialog.close()
  }, [open])
  return (
    <dialog
      ref={host}
      role={role}
      aria-label={title}
      onClose={onClose}
      onPointerDown={(event) => {
        backdropPress.current =
          event.target === event.currentTarget &&
          outside(event.currentTarget, event.clientX, event.clientY)
      }}
      onClick={(event) => {
        if (
          dismissible &&
          backdropPress.current &&
          event.target === event.currentTarget &&
          outside(event.currentTarget, event.clientX, event.clientY)
        )
          onClose()
        backdropPress.current = false
      }}
      className="float pop m-auto max-h-[90dvh] w-[min(92vw,470px)] overflow-y-auto rounded-3xl border border-line bg-panel p-4 text-ink outline-none backdrop:bg-black/55 backdrop:backdrop-blur-sm"
    >
      <div className="mb-2 flex items-center justify-between gap-3">
        <h2 className="m-0 text-subtitle font-medium text-ink">{title}</h2>
        {dismissible && (
          <button
            type="button"
            autoFocus
            aria-label={`Close ${title.toLowerCase()}`}
            className={iconControl('quiet')}
            onClick={onClose}
          >
            <Icon name="close" size={18} />
          </button>
        )}
      </div>
      {children}
    </dialog>
  )
}
