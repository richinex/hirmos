import { assertNever } from '@/domain/dop'
import type { ExampleGlyph as Kind } from '@/domain/example'

/**
 * The small drawing beside an example in the ledger: one fixed picture per kind of study, so the
 * row says what the example is before the name is read. Drawn in the palette's line colours, never
 * a screenshot, which is what keeps it legible at this size.
 */
export function ExampleGlyph({ kind }: { readonly kind: Kind }) {
  return (
    <svg viewBox="0 0 56 30" width={48} height={26} aria-hidden focusable="false" className="block shrink-0">
      {shape(kind)}
    </svg>
  )
}

const node = (x: number, y: number) => <circle cx={x} cy={y} r={3.6} className="fill-panel stroke-bone" strokeWidth={1.2} />
const edge = (x1: number, y1: number, x2: number, y2: number) => {
  const dx = x2 - x1
  const dy = y2 - y1
  const length = Math.hypot(dx, dy)
  const ux = dx / length
  const uy = dy / length
  const sx = x1 + ux * 4.5
  const sy = y1 + uy * 4.5
  const ex = x2 - ux * 5.5
  const ey = y2 - uy * 5.5
  return (
    <>
      <line x1={sx} y1={sy} x2={ex} y2={ey} className="stroke-muted" strokeWidth={1.1} />
      <polygon points={`${ex},${ey} ${ex - ux * 3.5 - uy * 2},${ey - uy * 3.5 + ux * 2} ${ex - ux * 3.5 + uy * 2},${ey - uy * 3.5 - ux * 2}`} className="fill-muted" />
    </>
  )
}
const marker = (x: number) => <line x1={x} y1={3} x2={x} y2={27} className="stroke-signal" strokeWidth={1} strokeDasharray="2 2" />
const series = (points: readonly (readonly [number, number])[], dashed = false) => (
  <polyline points={points.map(([x, y]) => `${x},${y}`).join(' ')} fill="none" className={dashed ? 'stroke-muted' : 'stroke-bone'} strokeWidth={1.2} strokeDasharray={dashed ? '3 2' : undefined} />
)

function shape(kind: Kind) {
  switch (kind) {
    case 'dag': {
      // Treatment and outcome on the baseline, two confounders above; every arrow drawn.
      const n = [[8, 22], [48, 22], [18, 7], [38, 7]] as const
      return (
        <>
          {edge(...n[2], ...n[0])}{edge(...n[2], ...n[1])}{edge(...n[3], ...n[1])}{edge(...n[0], ...n[1])}{edge(...n[3], ...n[0])}
          {n.map(([x, y]) => <g key={`${x}-${y}`}>{node(x, y)}</g>)}
        </>
      )
    }
    case 'step': {
      const pre = [16, 15, 17, 14, 16, 15, 17, 16, 15, 16]
      const post = [12, 11, 12, 10, 11, 12]
      return <>{marker(34)}{series([...pre, ...post].map((y, i) => [4 + i * 3.2, y] as const))}</>
    }
    case 'panel': {
      const unit = (y0: number, y1: number, offset: number) =>
        series(Array.from({ length: 10 }, (_, i) => [4 + i * 5.4, (i < 5 ? y0 : y1) + ((i + offset) % 2)] as const))
      return <>{marker(28)}{unit(9, 13, 0)}{unit(16, 20, 1)}{unit(23, 23, 0)}</>
    }
    case 'counts': {
      const heights = [6, 9, 7, 11, 8, 10, 7, 9, 16, 19, 17, 20, 18, 21]
      return <>{marker(32)}{heights.map((h, i) => <rect key={i} x={4 + i * 3.6} y={28 - h} width={2.2} height={h} className="fill-muted" />)}</>
    }
    case 'rct':
      // Two arms and the difference in their means.
      return (
        <>
          <line x1={14} y1={26} x2={14} y2={12} className="stroke-muted" strokeWidth={1.1} />
          <line x1={42} y1={26} x2={42} y2={8} className="stroke-muted" strokeWidth={1.1} />
          {node(14, 12)}{node(42, 8)}
          <line x1={4} y1={26} x2={52} y2={26} className="stroke-signal" strokeWidth={1} strokeDasharray="2 2" />
        </>
      )
    case 'pag':
      // Discovered edges with circle ends and one arrowhead.
      return (
        <>
          <line x1={12} y1={8} x2={30} y2={22} className="stroke-muted" strokeWidth={1.1} />
          <line x1={30} y1={22} x2={48} y2={8} className="stroke-muted" strokeWidth={1.1} />
          <line x1={12} y1={8} x2={48} y2={8} className="stroke-muted" strokeWidth={1.1} />
          <circle cx={12} cy={8} r={2.4} fill="none" className="stroke-muted" strokeWidth={1.1} />
          <circle cx={48} cy={8} r={2.4} fill="none" className="stroke-muted" strokeWidth={1.1} />
          <polygon points="30,22 26,18.5 28,17" className="fill-muted" />
          {node(30, 24)}
        </>
      )
    case 'lag':
      return (
        <>
          {series([[4, 20], [12, 18], [20, 22], [28, 16], [36, 19], [44, 12], [52, 15]])}
          {series([[4, 10], [12, 8], [20, 12], [28, 6], [36, 9], [44, 4], [52, 7]], true)}
          <polygon points="30,14 27,11 32,11" className="fill-muted" />
        </>
      )
    case 'dose':
      // The observed slope, dashed, against the interventional one.
      return (
        <>
          <line x1={6} y1={12} x2={50} y2={18} className="stroke-muted" strokeWidth={1.1} strokeDasharray="2 2" />
          <line x1={6} y1={22} x2={50} y2={6} className="stroke-bone" strokeWidth={1.2} />
          <polygon points="50,6 45.5,6.5 47.5,9.5" className="fill-muted" />
        </>
      )
    default:
      return assertNever(kind)
  }
}
