let installed = false

export function trackActivePane(): void {
  if (installed) return
  installed = true
  let active: Element | null = null
  const mark = (event: Event) => {
    if (event.target instanceof Element && event.target.closest('[data-pane-neutral]')) return
    const pane = event.target instanceof Element ? event.target.closest('[data-pane]') : null
    if (pane === active) return
    active?.removeAttribute('data-pane-active')
    pane?.setAttribute('data-pane-active', '')
    active = pane
  }
  document.addEventListener('pointerdown', mark, true)
  document.addEventListener('focusin', mark, true)
}
