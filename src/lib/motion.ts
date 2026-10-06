/** Canvas durations share CSS tokens. Read at the action, including delayed fit callbacks,
 * so changing the OS preference does not require remounting a canvas. */
export function canvasMotion(kind: 'zoom' | 'fit' | 'arrange'): { duration: number } {
  if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) return { duration: 0 }
  const value = getComputedStyle(document.documentElement)
    .getPropertyValue('--motion-canvas-' + kind)
    .trim()
  // The production CSS minifier writes 320ms as .32s, so a leading digit is optional.
  const match = /^(\d*\.?\d+)(ms|s)$/.exec(value)
  if (match === null) throw new Error('Missing or invalid canvas motion token: ' + kind)
  return { duration: Number(match[1]) * (match[2] === 's' ? 1000 : 1) }
}
