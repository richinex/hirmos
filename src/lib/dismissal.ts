// One Escape, one layer. Window-level Escape handlers share this gate so a keypress already
// consumed by a Radix layer (dialog, popover, palette), or typed into a field, never also
// dismisses the surface underneath and discards its state.

/** Panel ids in mount order; only the last one may answer a window-level Escape. */
const openLayers: string[] = []

export const pushLayer = (id: string): (() => void) => {
  openLayers.push(id)
  return () => {
    const index = openLayers.lastIndexOf(id)
    if (index >= 0) openLayers.splice(index, 1)
  }
}

export const anyLayerOpen = (): boolean => openLayers.length > 0

const typingIn = (target: EventTarget | null): boolean => {
  const el = target as HTMLElement | null
  return !!el && (el.tagName === 'INPUT' || el.tagName === 'TEXTAREA' || el.isContentEditable)
}

/** True when this Escape belongs to someone else: already handled, typed into a field, or a
 * Radix layer (dialog, popover, palette) is open and owns the dismissal. */
export const escapeClaimed = (e: KeyboardEvent): boolean =>
  e.defaultPrevented ||
  typingIn(e.target) ||
  document.querySelector(
    '[role="dialog"][data-state="open"], [data-radix-popper-content-wrapper]',
  ) !== null

/** The gate for a window-level Escape handler on a stacked surface. */
export const escapeFor = (id: string, e: KeyboardEvent): boolean => {
  if (escapeClaimed(e)) return false
  if (openLayers.length > 0 && openLayers.at(-1) !== id) return false
  e.preventDefault()
  return true
}
