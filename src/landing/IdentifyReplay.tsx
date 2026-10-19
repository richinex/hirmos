import { useLayoutEffect, useRef } from 'react'
import { createTimeline, spring, stagger, svg, utils } from 'animejs'

/**
 * The NSW example's identification, replayed: the graph draws in, each back-door path shows open and is
 * closed by its confounder, and the record line lands. Every string is the DAG workspace's own, from the
 * shipped example; the colours are the canvas's flow colours. No controls: it plays while in view, fades
 * out before it rewinds, and reduced motion gets the settled record.
 */

const CONFOUNDERS = ['age', 'educ', 'black', 'hispan', 'married', 'nodegree', 're75'] as const
const RECORD =
  'Adjusting for age, educ, black, hispan, married, nodegree, re75 blocks all represented back-door paths.'

const TREAT = { x: 92, y: 188 }
const OUTCOME = { x: 268, y: 188 }
const ROLE_R = 21
const NODE_R = 10
const SPOTS = CONFOUNDERS.map((name, i) => {
  const t = i / (CONFOUNDERS.length - 1)
  return { name, x: 28 + t * 304, y: 74 - Math.sin(Math.PI * t) * 44 }
})

/** A segment from the edge of one circle to just short of another, with its arrowhead. */
function arrow(from: { x: number; y: number }, to: { x: number; y: number }, r0: number, r1: number) {
  const dx = to.x - from.x
  const dy = to.y - from.y
  const length = Math.hypot(dx, dy)
  const ux = dx / length
  const uy = dy / length
  const x1 = from.x + ux * (r0 + 2)
  const y1 = from.y + uy * (r0 + 2)
  const x2 = to.x - ux * (r1 + 3)
  const y2 = to.y - uy * (r1 + 3)
  const head = `${x2},${y2} ${x2 - ux * 7 - uy * 3.5},${y2 - uy * 7 + ux * 3.5} ${x2 - ux * 7 + uy * 3.5},${y2 - uy * 7 - ux * 3.5}`
  return { d: `M${x1.toFixed(1)} ${y1.toFixed(1)}L${x2.toFixed(1)} ${y2.toFixed(1)}`, head }
}

const CAUSAL = arrow(TREAT, OUTCOME, ROLE_R, ROLE_R)

export function IdentifyReplay() {
  const rootRef = useRef<HTMLDivElement>(null)

  useLayoutEffect(() => {
    const root = rootRef.current
    if (root === null) return
    const all = (selector: string) => Array.from(root.querySelectorAll<HTMLElement | SVGElement>(selector))
    const one = (selector: string) => root.querySelector<HTMLElement | SVGElement>(selector)
    const pane = one('.replay-pane')
    if (pane === null) return
    if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) return

    const nodes = all('.id-node')
    const roles = all('.id-role')
    const lines = all('.id-line')
    const heads = all('.id-head')
    const rows = all('.id-row')
    const states = all('.id-state')
    const record = one('.id-record')
    const causal = all('.id-causal')

    const reset = () => {
      utils.set([...nodes, ...roles], { opacity: 0, scale: 0.6 })
      utils.set([...heads, ...rows, ...states, ...causal], { opacity: 0 })
      if (record) utils.set(record, { opacity: 0, translateY: 6 })
      for (const group of all('.id-path')) group.setAttribute('data-state', 'open')
    }
    reset()

    const land = spring({ stiffness: 170, damping: 15 })
    const tl = createTimeline({ loop: true, defaults: { ease: 'out(3)' }, onLoop: reset })
    tl.add(nodes, { opacity: [0, 1], scale: [0.6, 1], ease: land, duration: 700, delay: stagger(70) }, 200)
    tl.add(roles, { opacity: [0, 1], scale: [0.6, 1], ease: land, duration: 700, delay: stagger(120) }, 500)
    tl.add(svg.createDrawable(lines as SVGPathElement[]), { draw: ['0 0', '0 1'], duration: 700, delay: stagger(30), ease: 'inOut(2)' }, 900)
    tl.add(heads, { opacity: [0, 1], duration: 240 }, 1550)
    tl.add([rows[0], ...causal], { opacity: [0, 1], duration: 320 }, 1900)
    CONFOUNDERS.forEach((_, i) => {
      const at = 2500 + i * 620
      tl.add(rows[i + 1], { opacity: [0, 1], translateX: [-6, 0], duration: 320 }, at)
      tl.call(() => root.querySelector(`.id-path[data-i="${i}"]`)?.setAttribute('data-state', 'closed'), at + 300)
      tl.add(states[i], { opacity: [0, 1], duration: 260 }, at + 300)
    })
    if (record) tl.add(record, { opacity: [0, 1], translateY: [6, 0], duration: 420 }, 2500 + CONFOUNDERS.length * 620 + 200)
    tl.add(pane, { opacity: 0, duration: 420, ease: 'inOut(2)' }, '+=3800')
    tl.add(pane, { opacity: 1, duration: 0 }, '+=60')

    tl.pause()
    const io = new IntersectionObserver(([entry]) => (entry?.isIntersecting ? tl.play() : tl.pause()), { threshold: 0.3 })
    io.observe(root)
    return () => {
      io.disconnect()
      tl.revert()
    }
  }, [])

  return (
    <div ref={rootRef} className="replay">
      <div className="replay-pane replay-pane--split">
        <svg className="id-figure" viewBox="0 0 360 236" role="img" aria-labelledby="id-figure-title">
          <title id="id-figure-title">
            NSW training and 1978 earnings: seven confounders point to treat and re78, and adjusting for them
            closes every back-door path
          </title>
          {SPOTS.map((spot, i) => (
            <g key={spot.name} className="id-path" data-i={i} data-state="closed">
              {[TREAT, OUTCOME].map((target) => {
                const { d, head } = arrow(spot, target, NODE_R, ROLE_R)
                return (
                  <g key={target.x}>
                    <path className="id-line" d={d} />
                    <polygon className="id-head" points={head} />
                  </g>
                )
              })}
            </g>
          ))}
          <path className="id-causal id-causal-line" d={CAUSAL.d} />
          <polygon className="id-causal id-causal-head" points={CAUSAL.head} />
          {SPOTS.map((spot) => (
            <g key={spot.name} className="id-node">
              <circle cx={spot.x} cy={spot.y} r={NODE_R} />
              <text x={spot.x} y={spot.y - NODE_R - 6}>
                {spot.name}
              </text>
            </g>
          ))}
          {[
            { at: TREAT, name: 'treat', role: 'Treatment', tone: 'treatment' },
            { at: OUTCOME, name: 're78', role: 'Outcome', tone: 'outcome' },
          ].map(({ at, name, role, tone }) => (
            <g key={name} className={`id-role id-role--${tone}`}>
              <circle cx={at.x} cy={at.y} r={ROLE_R} />
              <text className="id-role-name" x={at.x} y={at.y + 5}>
                {name}
              </text>
              <text className="id-role-label" x={at.x} y={at.y + ROLE_R + 17}>
                {role}
              </text>
            </g>
          ))}
        </svg>
        <div className="id-ledger">
          <p className="replay-caption">Back-door paths</p>
          <ul className="id-rows">
            <li className="id-row">
              <span>treat → re78</span>
              <span className="id-word id-word--causal">causal</span>
            </li>
            {CONFOUNDERS.map((name) => (
              <li key={name} className="id-row">
                <span>treat ← {name} → re78</span>
                <span className="id-state id-word">closed by {name}</span>
              </li>
            ))}
          </ul>
          <p className="id-record">{RECORD}</p>
        </div>
      </div>
    </div>
  )
}
