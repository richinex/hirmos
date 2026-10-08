import { animate, cubicBezier, stagger, utils } from 'animejs'

/**
 * The landing page's motion, in the house register: one ease-out for everything, every reveal once,
 * nothing that loops except the dashed evidence links. Durations come from the landing's motion tokens in
 * the stylesheet. The hero is not touched: its word reveal is CSS and has painted before this runs. Under reduced motion nothing here runs and the page shows its final
 * state, because the stylesheet never hides content on its own.
 */

const EASE = cubicBezier(0.2, 0.7, 0.2, 1)

/** A duration from the stylesheet's motion tokens, so the numbers live in one place. */
const tokenMs = (name: string, fallback: number): number => {
  const value = Number.parseFloat(getComputedStyle(document.documentElement).getPropertyValue(name))
  return Number.isFinite(value) ? value : fallback
}

const groups: readonly { readonly selector: string; readonly children: string }[] = [
  { selector: '.workflow-section .section-intro', children: ':scope > *' },
  { selector: '.workflow-list', children: ':scope > li' },
  {
    selector: '.feature-section--evidence',
    children: ':scope > .feature-copy > *, :scope > .evidence-figure',
  },
  {
    selector: '.feature-section--identify',
    children: ':scope > .backdoor-figure, :scope > .feature-copy > *',
  },
  { selector: '.structures-section .section-intro', children: ':scope > *' },
  { selector: '.structure-grid', children: ':scope > article' },
]

const drawOn = (path: SVGPathElement, duration: number, delay: number): void => {
  const length = path.getTotalLength()
  path.style.strokeDasharray = `${length}`
  path.style.strokeDashoffset = `${length}`
  animate(path, { strokeDashoffset: 0, duration, delay, ease: EASE })
}

export function choreographLanding(root: HTMLElement): () => void {
  if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) return () => {}
  const reveal = tokenMs('--motion-reveal', 640)
  const draw = tokenMs('--motion-draw', 900)
  const step = tokenMs('--motion-stagger', 70)

  const observer = new IntersectionObserver(
    (entries) => {
      for (const entry of entries) {
        if (!entry.isIntersecting) continue
        observer.unobserve(entry.target)
        const group = entry.target as HTMLElement
        const children = Array.from(
          group.querySelectorAll<HTMLElement>(group.dataset['reveal'] ?? ':scope > *'),
        )
        animate(children, {
          opacity: 1,
          translateY: 0,
          duration: reveal,
          delay: stagger(step),
          ease: EASE,
        })
        for (const path of group.querySelectorAll<SVGPathElement>(
          '.backdoor-edge--signal, .dag-link--signal',
        ))
          drawOn(path, draw, 260)
      }
    },
    { rootMargin: '0px 0px -12% 0px', threshold: 0.05 },
  )

  for (const { selector, children } of groups) {
    for (const group of root.querySelectorAll<HTMLElement>(selector)) {
      group.dataset['reveal'] = children
      utils.set(Array.from(group.querySelectorAll<HTMLElement>(children)), {
        opacity: 0,
        translateY: 18,
      })
      observer.observe(group)
    }
  }

  return () => {
    observer.disconnect()
  }
}
