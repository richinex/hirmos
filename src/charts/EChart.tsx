import { useEffect, useRef, useState } from 'react'
import type { EChartsCoreOption, EChartsType } from 'echarts/core'

/**
 * The one chart host. The registry loads lazily on first mount so chart code stays out of the initial
 * bundle; the option is re-applied whenever its identity changes, which the theme hook guarantees on a
 * theme switch because builders take the theme as an argument.
 */
/** The description the builder wrote into the option; a responsive option keeps it under `baseOption`. */
const describe = (option: EChartsCoreOption): string | undefined => {
  const root = Reflect.get(option, 'baseOption') ?? option
  const aria = root !== null && typeof root === 'object' ? Reflect.get(root, 'aria') : undefined
  const label = aria !== null && typeof aria === 'object' ? Reflect.get(aria, 'label') : undefined
  const description = label !== null && typeof label === 'object' ? Reflect.get(label, 'description') : undefined
  return typeof description === 'string' ? description : undefined
}

export function EChart({ option, label, className = 'h-[260px]', style, testId }: {
  readonly option: EChartsCoreOption
  readonly label: string
  readonly className?: string
  /** Explicit pixel size for charts with a natural size, such as the lag grid. */
  readonly style?: React.CSSProperties
  readonly testId?: string
}) {
  const host = useRef<HTMLDivElement>(null)
  const chart = useRef<EChartsType | null>(null)
  const latestOption = useRef(option)
  const [failed, setFailed] = useState(false)
  const [mounted, setMounted] = useState(false)
  latestOption.current = option

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
        setMounted(true)
      }
      // A host in a hidden pane has no size yet; the chart mounts when it first gets one.
      resize = new ResizeObserver(() => {
        if (!sized()) return
        if (chart.current === null) mount()
        else chart.current.resize()
      })
      resize.observe(element)
      if (sized()) mount()
    }).catch(() => { if (!cancelled) setFailed(true) })
    return () => {
      cancelled = true
      resize?.disconnect()
      chart.current?.dispose()
      chart.current = null
    }
  }, [])

  useEffect(() => { chart.current?.setOption(option, { notMerge: true }) }, [option])

  if (failed) return <p role="alert" className="grid min-h-40 place-items-center text-body text-danger">The chart could not be loaded.</p>
  return <div ref={host} role="img" aria-label={label} aria-description={describe(option)} data-testid={testId} style={style} aria-busy={mounted ? undefined : true} className={`min-w-0 w-full ${className}${mounted ? '' : ' skeleton hold-appear rounded-lg'}`} />
}
