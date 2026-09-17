import type { ReactNode } from 'react'
import { button } from './recipes'
import { Dialog } from './Dialog'

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
  return (
    <Dialog open={open} role="alertdialog" title={title} onClose={onClose}>
      <p className="mb-0 mt-2 text-body text-muted">{message}</p>
      <div className="mt-4 flex justify-end gap-2">
        <button type="button" autoFocus className={button('outline')} onClick={onClose}>Cancel</button>
        <button type="button" className={button(danger ? 'danger' : 'signal')} onClick={() => { onConfirm(); onClose() }}>{confirmLabel}</button>
      </div>
    </Dialog>
  )
}
