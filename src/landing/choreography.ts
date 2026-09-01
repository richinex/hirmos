import { animate, cubicBezier, stagger, utils } from 'animejs'

/**
 * The landing page's motion, in the house register: one ease-out for everything, every reveal once,
 * nothing that loops except the dashed evidence links. The hero is not touched: it ships as static HTML
 * and has painted before this runs. Under reduced motion nothing here runs and the page shows its final
 * state, because the stylesheet never hides content on its own.
 */

const EASE = cubicBezier(0.2, 0.7, 0.2, 1)

const groups: readonly { readonly selector: string; readonly children: string }[] = [
  { selector: '.workflow-section .section-intro', children: ':scope > *' },
  { selector: '.workflow-list', children: ':scope > li' },
  { selector: '.feature-section--evidence', children: ':scope > .feature-copy > *, :scope > .evidence-figure' },
  { selector: '.feature-section--identify', children: ':scope > .backdoor-figure, :scope > .feature-copy > *' },
  { selector: '.structures-section .section-intro', children: ':scope > *' },
  { selector: '.structure-grid', children: ':scope > article' },
  { selector: '.final-cta', children: ':scope > *' },
]

const drawOn = (path: SVGPathElement, delay: number): void => {
  const length = path.getTotalLength()
  path.style.strokeDasharray = `${length}`
  path.style.strokeDashoffset = `${length}`
  animate(path, { strokeDashoffset: 0, duration: 900, delay, ease: EASE })
}

export function choreographLanding(root: HTMLElement): () => void {
  if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) return () => {}

  const observer = new IntersectionObserver((entries) => {
    for (const entry of entries) {
      if (!entry.isIntersecting) continue
      observer.unobserve(entry.target)
      const group = entry.target as HTMLElement
      const children = Array.from(group.querySelectorAll<HTMLElement>(group.dataset['reveal'] ?? ':scope > *'))
      animate(children, { opacity: 1, translateY: 0, duration: 640, delay: stagger(70), ease: EASE })
      for (const path of group.querySelectorAll<SVGPathElement>('.backdoor-edge--signal, .dag-link--signal')) drawOn(path, 260)
    }
  }, { rootMargin: '0px 0px -12% 0px', threshold: 0.05 })

  for (const { selector, children } of groups) {
    for (const group of root.querySelectorAll<HTMLElement>(selector)) {
      group.dataset['reveal'] = children
      utils.set(Array.from(group.querySelectorAll<HTMLElement>(children)), { opacity: 0, translateY: 18 })
      observer.observe(group)
    }
  }

  return () => { observer.disconnect() }
}
