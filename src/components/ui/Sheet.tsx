import { Drawer } from 'vaul'
import { useRef, type ReactNode } from 'react'
import { panelTitle } from './recipes'

/**
 * The phone bottom sheet, from Octopus: Vaul for drag-to-dismiss and spring physics, safe-area
 * padding so content rides above the home indicator.
 *
 * The surface sits on `raised`, not `panel`: on the dark themes a panel-on-stage sheet is a hundredth
 * of a contrast step from the page behind it, so neither the fill nor the scrim reads as a layer.
 * The page is scaled back behind it for the same reason, which does not depend on luminance.
 *
 * `autoFocus` stays on because Vaul otherwise
 * leaves focus on the opener inside the subtree it marks aria-hidden; the focus handlers then return
 * focus to whichever plain button opened the sheet, since no `Drawer.Trigger` is involved.
 */
export function Sheet({
  open,
  onClose,
  title,
  maxH = '78dvh',
  side = 'bottom',
  children,
}: {
  readonly open: boolean
  readonly onClose: () => void
  /** Names the dialog for assistive tech and captions it on screen. */
  readonly title: string
  readonly maxH?: string
  /** 'left' slides in from the leading edge, for navigation; 'bottom' for panels and pickers. */
  readonly side?: 'bottom' | 'left'
  readonly children: ReactNode
}) {
  const opener = useRef<HTMLElement | null>(null)
  return (
    <Drawer.Root
      open={open}
      onOpenChange={(next) => {
        if (!next) onClose()
      }}
      autoFocus
      direction={side}
      shouldScaleBackground={side === 'bottom'}
    >
      <Drawer.Portal>
        <Drawer.Overlay className="fixed inset-0 z-(--z-overlay) bg-black/60" />
        <Drawer.Content
          className={
            side === 'left'
              ? 'fixed inset-y-0 left-0 z-(--z-overlay) flex w-[min(85vw,320px)] flex-col rounded-r-3xl border-r border-edge bg-raised pb-[env(safe-area-inset-bottom)] text-ink outline-none'
              : 'fixed inset-x-0 bottom-0 z-(--z-overlay) flex flex-col rounded-t-3xl border-t border-edge bg-raised pb-[env(safe-area-inset-bottom)] text-ink outline-none'
          }
          style={side === 'left' ? undefined : { maxHeight: maxH }}
          onOpenAutoFocus={() => {
            opener.current = document.activeElement as HTMLElement | null
          }}
          onEscapeKeyDown={(event) => {
            // Let a non-empty search field clear first. Radix handles Escape in capture phase,
            // before FilterField's key handler can prevent the sheet from closing.
            if (
              event.target instanceof HTMLInputElement &&
              event.target.type === 'search' &&
              event.target.value !== ''
            )
              event.preventDefault()
          }}
          onCloseAutoFocus={(event) => {
            const element = opener.current
            if (!element?.isConnected) return
            event.preventDefault()
            element.focus()
          }}
        >
          {side === 'bottom' && (
            <div
              aria-hidden
              className="mx-auto mb-1 mt-2.5 h-1 w-10 shrink-0 rounded-full bg-dim"
            />
          )}
          <Drawer.Title
            className={`shrink-0 border-b border-hair px-4 pb-2.5 ${side === 'left' ? 'pt-4' : 'pt-1.5'} ${panelTitle}`}
          >
            {title}
          </Drawer.Title>
          <div className="panel-scroll min-h-0 flex-1 overflow-y-auto px-4 pb-4 pt-3">
            {children}
          </div>
        </Drawer.Content>
      </Drawer.Portal>
    </Drawer.Root>
  )
}
