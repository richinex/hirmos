/**
 * The zoom a reader has drawn on a chart, and the way back out of it. A target is a pair of axis
 * values or the whole axis; the history is the targets left behind on the way in, newest last, so
 * stepping back is a pop. Twenty levels are more than any reader drills, and the bound keeps a
 * runaway brush from growing the stack without limit.
 */
export interface ZoomRange {
  readonly start: number
  readonly end: number
}

/** The whole axis: what a chart shows before any range is drawn, and what the last step back returns to. */
export const WHOLE_AXIS = 'whole-axis'
export type ZoomTarget = ZoomRange | typeof WHOLE_AXIS

export type ZoomHistory = readonly ZoomTarget[]

const DEPTH = 20

export const EMPTY_ZOOM_HISTORY: ZoomHistory = []

/** Record the view being left; the oldest levels fall away beyond the depth. */
export const pushZoom = (history: ZoomHistory, leaving: ZoomTarget): ZoomHistory => [...history, leaving].slice(-DEPTH)

/** Step back one level: the view to show again, and the history without it. */
export const popZoom = (history: ZoomHistory): { readonly history: ZoomHistory; readonly target: ZoomTarget } =>
  history.length === 0
    ? { history, target: WHOLE_AXIS }
    : { history: history.slice(0, -1), target: history[history.length - 1] }

/** A brush is a zoom only when it spans something; a click or a hair-width drag is not a request to zoom. */
export const rangeOf = (bounds: readonly [number, number], minimumSpan: number): ZoomRange | null => {
  const start = Math.min(bounds[0], bounds[1])
  const end = Math.max(bounds[0], bounds[1])
  return end - start < minimumSpan ? null : { start, end }
}
