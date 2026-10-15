/** A `--motion-*` duration token in milliseconds, or 0 under reduced motion. Read at the action, including
 * delayed fit callbacks, so changing the OS preference does not require remounting anything. */
export function motionDuration(token: string): number {
  if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) return 0
  const value = getComputedStyle(document.documentElement)
    .getPropertyValue('--motion-' + token)
    .trim()
  // The production CSS minifier writes 320ms as .32s, so a leading digit is optional.
  const match = /^(\d*\.?\d+)(ms|s)$/.exec(value)
  if (match === null) throw new Error('Missing or invalid motion token: ' + token)
  return Number(match[1]) * (match[2] === 's' ? 1000 : 1)
}

export function canvasMotion(kind: 'zoom' | 'fit' | 'arrange'): { duration: number } {
  return { duration: motionDuration('canvas-' + kind) }
}
