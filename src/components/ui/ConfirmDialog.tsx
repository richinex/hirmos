import { useEffect, useRef, type ReactNode } from 'react'
import { button } from './recipes'

/**
 * Confirmation for an action that cannot be undone. Built on the native dialog element, whose
 * `showModal` supplies the focus trap, Escape handling and scroll containment. Cancel comes first
 * and takes the initial focus, so the safe choice is one keypress away.
 */
export function ConfirmDialog({ open, title, message, confirmLabel, danger = false, onConfirm, onClose }: {
  readonly open: boolean
  readonly title: string
  readonly message: ReactNode
  readonly confirmLabel: string
  readonly danger?: boolean
  readonly onConfirm: () => void
  readonly onClose: () => void
}) {
  const host = useRef<HTMLDialogElement>(null)
  useEffect(() => {
    const dialog = host.current
    if (dialog === null) return
    if (open && !dialog.open) dialog.showModal()
    if (!open && dialog.open) dialog.close()
  }, [open])
  return (
    <dialog
      ref={host}
      role="alertdialog"
      aria-label={title}
      onClose={onClose}
      className="m-auto w-[min(92vw,26rem)] rounded-xl border border-line bg-panel p-4 text-ink backdrop:bg-black/40 backdrop:backdrop-blur-sm"
    >
      <h2 className="m-0 text-title font-medium text-ink">{title}</h2>
      <p className="mb-0 mt-2 text-body text-muted">{message}</p>
      <div className="mt-4 flex justify-end gap-2">
        <button type="button" autoFocus className={button('outline')} onClick={onClose}>Cancel</button>
        <button type="button" className={button(danger ? 'danger' : 'signal')} onClick={() => { onConfirm(); onClose() }}>{confirmLabel}</button>
      </div>
    </dialog>
  )
}
