import { useEffect, useRef, useState } from 'react'
import type { EChartsCoreOption, EChartsType } from 'echarts/core'
import { EMPTY_ZOOM_HISTORY, popZoom, pushZoom, rangeOf, WHOLE_AXIS, type ZoomHistory, type ZoomTarget } from './zoomHistory'
import { visibleWindow, type VisibleWindow } from './window'

/**
 * The one chart host. The registry loads lazily on first mount so chart code stays out of the initial
 * bundle; the option is re-applied whenever its identity changes, which the theme hook guarantees on a
 * theme switch because builders take the theme as an argument.
 *
 * A builder that declares `brush` gets range selection: a drag along x zooms to the drawn range, and
 * a double click steps back one level. Fine pointers only; touch keeps pinch and drag from the zoom
 * pair, because a finger cannot tell a brush from a pan.
 */
/** The root of an option, under `baseOption` when the builder made it responsive. */
const rootOf = (option: EChartsCoreOption): Record<string, unknown> => {
  const base = Reflect.get(option, 'baseOption')
  return (base !== null && typeof base === 'object' ? base : option) as Record<string, unknown>
}

/** The description the builder wrote into the option. */
const describe = (option: EChartsCoreOption): string | undefined => {
  const aria = Reflect.get(rootOf(option), 'aria')
  const label = aria !== null && typeof aria === 'object' ? Reflect.get(aria, 'label') : undefined
  const description = label !== null && typeof label === 'object' ? Reflect.get(label, 'description') : undefined
  return typeof description === 'string' ? description : undefined
}

const selectsRange = (option: EChartsCoreOption): boolean => Reflect.get(rootOf(option), 'brush') !== undefined

const finePointer = (): boolean => window.matchMedia('(pointer: fine)').matches

/** Ask the chart to show a range, or everything. */
const applyZoom = (chart: EChartsType, target: ZoomTarget): void => {
  if (target === WHOLE_AXIS) chart.dispatchAction({ type: 'dataZoom', dataZoomIndex: 0, start: 0, end: 100 })
  else chart.dispatchAction({ type: 'dataZoom', dataZoomIndex: 0, startValue: target.start, endValue: target.end })
}

const takeBrushCursor = (chart: EChartsType): void =>
  chart.dispatchAction({ type: 'takeGlobalCursor', key: 'brush', brushOption: { brushType: 'lineX', brushMode: 'single' } })

export function EChart({ option, label, className = 'h-[260px]', style, testId, onReady, onWindow, window: wanted }: {
  readonly option: EChartsCoreOption
  readonly label: string
  readonly className?: string
  /** Explicit pixel size for charts with a natural size, such as the lag grid. */
  readonly style?: React.CSSProperties
  readonly testId?: string
  /** The live instance, for a caller that exports or inspects the drawing. */
  readonly onReady?: (chart: EChartsType) => void
  /** The visible x range after every zoom, or null for the whole axis. */
  readonly onWindow?: (window: VisibleWindow | null) => void
  /** A window to show, so a chart can follow another that shares its axis; null for the whole axis. */
  readonly window?: VisibleWindow | null
}) {
  const host = useRef<HTMLDivElement>(null)
  const chart = useRef<EChartsType | null>(null)
  const latestOption = useRef(option)
  const latest = useRef({ onReady, onWindow })
  const history = useRef<ZoomHistory>(EMPTY_ZOOM_HISTORY)
  const [failed, setFailed] = useState(false)
  const [mounted, setMounted] = useState(false)
  latestOption.current = option
  latest.current = { onReady, onWindow }

  useEffect(() => {
    let cancelled = false
    let resize: ResizeObserver | null = null
    void import('./registry').then(({ createChart }) => {
      const element = host.current
      if (cancelled || !element) return
      const sized = () => element.clientWidth > 0 && element.clientHeight > 0
      const mount = () => {
        const instance = createChart(element)
        chart.current = instance
        instance.setOption(latestOption.current, { notMerge: true })
        if (selectsRange(latestOption.current) && finePointer()) takeBrushCursor(instance)
        instance.on('datazoom', () => latest.current.onWindow?.(visibleWindow(instance)))
        instance.on('brushEnd', (event) => {
          const areas = Reflect.get(event as object, 'areas')
          const first = Array.isArray(areas) ? areas[0] : undefined
          const bounds = first !== undefined ? Reflect.get(first, 'coordRange') : undefined
          instance.dispatchAction({ type: 'brush', areas: [] })
          if (!Array.isArray(bounds) || bounds.length !== 2) return
          const range = rangeOf([Number(bounds[0]), Number(bounds[1])], 0)
          if (range === null) return
          history.current = pushZoom(history.current, visibleWindow(instance) ?? WHOLE_AXIS)
          applyZoom(instance, range)
        })
        setMounted(true)
        latest.current.onReady?.(instance)
      }
      // A host in a hidden pane has no size yet; the chart mounts when it first gets one.
      resize = new ResizeObserver(() => {
        if (!sized()) return
        if (chart.current === null) mount()
        else chart.current.resize()
      })
      resize.observe(element)
      if (sized()) mount()
      // Every label decision — overlap hiding, truncation, the grid's shrink-to-fit — is computed from
      // text measured through an offscreen canvas, and cached. Measured before the webfont arrives, those
      // are fallback metrics that are never revisited, so re-lay out once the real face has loaded.
      void document.fonts?.ready.then(() => {
        if (!cancelled && chart.current !== null) chart.current.resize()
      })
    }).catch(() => { if (!cancelled) setFailed(true) })
    return () => {
      cancelled = true
      resize?.disconnect()
      chart.current?.dispose()
      chart.current = null
    }
  }, [])

  useEffect(() => {
    const instance = chart.current
    if (instance === null) return
    instance.setOption(option, { notMerge: true })
    // A fresh option drops the brush cursor with everything else, so it is taken again.
    if (selectsRange(option) && finePointer()) takeBrushCursor(instance)
  }, [option])

  // Following another chart: only a window the chart is not already showing is applied, so two charts
  // that follow each other settle rather than ping-pong.
  useEffect(() => {
    const instance = chart.current
    if (instance === null || wanted === undefined) return
    const shown = visibleWindow(instance)
    const same = wanted === null ? shown === null : shown !== null && shown.start === wanted.start && shown.end === wanted.end
    if (!same) applyZoom(instance, wanted ?? WHOLE_AXIS)
  }, [wanted])

  const stepBack = () => {
    const instance = chart.current
    if (instance === null || !selectsRange(latestOption.current)) return
    const { history: remaining, target } = popZoom(history.current)
    history.current = remaining
    applyZoom(instance, target)
  }

  if (failed) return <p role="alert" className="grid min-h-40 place-items-center text-body text-danger">The chart could not be loaded.</p>
  return <div ref={host} role="img" aria-label={label} aria-description={describe(option)} data-testid={testId} style={style} aria-busy={mounted ? undefined : true} onDoubleClick={stepBack} className={`min-w-0 w-full ${className}${mounted ? '' : ' skeleton hold-appear rounded-lg'}`} />
}
