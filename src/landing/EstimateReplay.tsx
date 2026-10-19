import { useLayoutEffect, useRef } from 'react'
import { createTimeline, splitText, stagger, utils } from 'animejs'
import { ReplayCursor } from './ReplayCursor'

/**
 * The NSW example's estimate, replayed: the run is pressed, the effect counts up to its value, the
 * interval draws across zero, and the reading lands. Every string and number is the Estimation and
 * Results screens' own for run 133cff07 of the shipped example. Plays while in view, fades out before it
 * rewinds, and reduced motion gets the settled result.
 */

const ESTIMATE = 1.22
const LOWER = -0.34
const UPPER = 2.78
const AXIS = { min: -1, max: 3 }
const at = (value: number) => `${((value - AXIS.min) / (AXIS.max - AXIS.min)) * 100}%`

const INCLUDES_ZERO = 'The interval includes zero: the data do not rule out no effect.'
const BOTTOM_LINE =
  'After accounting for the 7 variables of the identified adjustment set, setting treat to 1 rather than 0 is associated with an estimated increase of 1.22 in re78 on average over the prepared study population.'

export function EstimateReplay() {
  const rootRef = useRef<HTMLDivElement>(null)

  useLayoutEffect(() => {
    const root = rootRef.current
    if (root === null) return
    const one = <T extends Element>(selector: string) => root.querySelector<T>(selector)
    const pane = one<HTMLElement>('.replay-pane')
    const run = one<HTMLElement>('.es-run')
    const figure = one<HTMLElement>('.es-figure')
    const meta = one<HTMLElement>('.es-meta')
    const band = one<HTMLElement>('.es-band')
    const point = one<HTMLElement>('.es-point')
    const zero = one<HTMLElement>('.es-zero')
    const zeroNote = one<HTMLElement>('.es-includes')
    const reading = one<HTMLElement>('.es-reading')
    const cursor = one<HTMLElement>('.replay-cursor')
    if (!pane || !run || !figure || !meta || !band || !point || !zero || !zeroNote || !reading || !cursor) return
    if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) return

    const split = splitText(reading, { words: true })
    const words = split.words
    const count = { value: 0 }
    const press = { left: run.offsetLeft + run.offsetWidth * 0.6, top: run.offsetTop + run.offsetHeight * 0.5 }
    const reset = () => {
      utils.set(cursor, { opacity: 0, left: press.left + 140, top: press.top + 90, scale: 1 })
      count.value = 0
      figure.textContent = '0.00'
      run.removeAttribute('data-pressed')
      utils.set([meta, zeroNote, zero, point], { opacity: 0 })
      utils.set(band, { scaleX: 0, opacity: 0 })
      utils.set(words, { opacity: 0, translateY: 4 })
    }
    reset()

    const tl = createTimeline({ loop: true, defaults: { ease: 'out(3)' }, onLoop: reset })
    tl.add(cursor, { opacity: [0, 1], duration: 160 }, 60)
    tl.add(cursor, { ...press, duration: 420, ease: 'inOut(3)' }, 60)
    tl.add(cursor, { scale: [1, 0.86, 1], duration: 260 }, 500)
    tl.call(() => run.setAttribute('data-pressed', ''), 500)
    tl.call(() => run.removeAttribute('data-pressed'), 760)
    tl.add(cursor, { opacity: 0, duration: 260 }, 900)
    tl.add(count, {
      value: [0, ESTIMATE],
      duration: 1100,
      ease: 'out(4)',
      onUpdate: () => { figure.textContent = count.value.toFixed(2) },
    }, 900)
    tl.add(meta, { opacity: [0, 1], duration: 360 }, 1800)
    tl.add(zero, { opacity: [0, 1], duration: 300 }, 2100)
    tl.add(point, { opacity: [0, 1], duration: 200 }, 2200)
    tl.add(band, { opacity: { to: 1, duration: 100 }, scaleX: [0, 1], duration: 900, ease: 'inOut(3)' }, 2300)
    tl.add(zeroNote, { opacity: [0, 1], duration: 420 }, 3400)
    tl.add(words, { opacity: [0, 1], translateY: [4, 0], duration: 320, delay: stagger(26) }, 4200)
    tl.add(pane, { opacity: 0, duration: 420, ease: 'inOut(2)' }, '+=3800')
    tl.add(pane, { opacity: 1, duration: 0 }, '+=60')

    tl.pause()
    const io = new IntersectionObserver(([entry]) => (entry?.isIntersecting ? tl.play() : tl.pause()), { threshold: 0.3 })
    io.observe(root)
    return () => {
      io.disconnect()
      tl.revert()
      split.revert()
    }
  }, [])

  return (
    <div ref={rootRef} className="replay">
      <div className="replay-pane es-pane">
        <ReplayCursor />
        <span className="es-run">Run adjusted linear regression</span>
        <p className="es-sentence">Effect of changing treat on re78</p>
        <p className="es-figure" aria-label="1.22; 95% confidence interval minus 0.34 to 2.78">
          {ESTIMATE.toFixed(2)}
        </p>
        <div className="es-strip" aria-hidden>
          <span className="es-zero" style={{ left: at(0) }} />
          <span className="es-zero-label" style={{ left: at(0) }}>
            0
          </span>
          <span className="es-band" style={{ left: at(LOWER), width: `calc(${at(UPPER)} - ${at(LOWER)})` }} />
          <span className="es-point" style={{ left: at(ESTIMATE) }} />
        </div>
        <p className="es-meta">
          <span>[−0.34, 2.78] 95% CI</span>
          <span>SE 0.79</span>
          <span>n = 614</span>
        </p>
        <p className="es-includes">{INCLUDES_ZERO}</p>
        <p className="es-reading">{BOTTOM_LINE}</p>
      </div>
    </div>
  )
}
