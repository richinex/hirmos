import { Fragment, useEffect, useRef, useState, type CSSProperties } from 'react'
import { HirmosMark } from '@/components/HirmosMark'
import { InternalLink } from '@/components/ui/InternalLink'
import { choreographLanding } from './choreography'
import type { HirmosGraph } from './HirmosGraph'

// Fetched with the page and mounted the moment it arrives. Not React.lazy: a Suspense boundary holds
// its fallback for React's reveal window, and the fallback here is the same empty placeholder the
// static shell in index.html already paints, so nothing may appear and move in between.
const graphChunk = import('./HirmosGraph')

/** The hero headline, one slot per word so the reveal is markup, not a script. */
const HEADLINE = 'A scratchpad for causal inference.'.split(' ')

const WORKFLOW = [
  { number: '01', title: 'Prepare', copy: 'Profile the source, define the observation structure, resolve missingness, and record transformations.' },
  { number: '02', title: 'Examine', copy: 'Run data diagnostics and, when useful, compare discovery methods that propose candidate edges.' },
  { number: '03', title: 'Model', copy: 'Develop the causal graph from domain knowledge, study design, and relevant empirical evidence.' },
  { number: '04', title: 'Identify', copy: 'Define the treatment, outcome, intervention, target population, and effect measure. Determine whether the graph identifies that effect and which adjustment strategies are valid.' },
  { number: '05', title: 'Estimate', copy: 'Fit eligible estimators and compare effect estimates, uncertainty, and model diagnostics.' },
  { number: '06', title: 'Assess', copy: 'Examine sensitivity to assumptions, apply appropriate refutation checks, and evaluate counterfactual questions when the fitted causal model supports them.' },
] as const

function Scene() {
  const [Graph, setGraph] = useState<typeof HirmosGraph | null>(null)
  useEffect(() => {
    let mounted = true
    void graphChunk.then((module) => { if (mounted) setGraph(() => module.HirmosGraph) })
    return () => { mounted = false }
  }, [])
  return (
    <div className="landing-scene" aria-hidden="true">
      {Graph !== null && <Graph className="landing-scene__canvas" />}
    </div>
  )
}

function BackdoorFigure() {
  return (
    <svg className="backdoor-figure" viewBox="0 0 620 280" role="img" aria-labelledby="backdoor-title backdoor-desc">
      <title id="backdoor-title">A treatment, outcome, and common cause</title>
      <desc id="backdoor-desc">The common cause points to treatment and outcome. Treatment also points to outcome. Conditioning on the common cause closes the back-door path.</desc>
      <defs>
        <marker id="landing-arrow" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto" markerUnits="strokeWidth">
          <path d="M0,0 L8,4 L0,8 Z" className="backdoor-arrow" />
        </marker>
      </defs>
      <path className="backdoor-edge backdoor-edge--muted" d="M305 72 C245 94 201 125 166 179" markerEnd="url(#landing-arrow)" />
      <path className="backdoor-edge backdoor-edge--muted" d="M321 72 C382 94 432 126 466 179" markerEnd="url(#landing-arrow)" />
      <path className="backdoor-edge backdoor-edge--signal" d="M203 211 C278 236 362 236 436 211" markerEnd="url(#landing-arrow)" />
      <g className="backdoor-node"><circle cx="313" cy="54" r="30" /><text x="313" y="60">C</text></g>
      <g className="backdoor-node"><circle cx="150" cy="207" r="34" /><text x="150" y="213">X</text></g>
      <g className="backdoor-node"><circle cx="470" cy="207" r="34" /><text x="470" y="213">Y</text></g>
      <text className="backdoor-label" x="313" y="17">common cause</text>
      <text className="backdoor-label" x="150" y="262">treatment</text>
      <text className="backdoor-label" x="470" y="262">outcome</text>
    </svg>
  )
}

function EvidenceFigure() {
  return (
    <div className="evidence-figure" aria-label="Discovery evidence remains separate from the authored DAG">
      <section>
        <header><span>Discovery evidence</span><small>candidate relations</small></header>
        <svg viewBox="0 0 300 220" aria-hidden="true">
          <path className="evidence-link evidence-link--one" d="M55 59 C115 25 176 43 239 86" />
          <path className="evidence-link evidence-link--two" d="M59 62 C104 118 169 151 246 153" />
          <path className="evidence-link evidence-link--three" d="M93 174 C130 125 167 99 236 88" />
          <circle cx="52" cy="58" r="25" /><circle cx="88" cy="177" r="25" /><circle cx="243" cy="87" r="25" /><circle cx="247" cy="154" r="25" />
          <text x="52" y="63">A</text><text x="88" y="182">B</text><text x="243" y="92">C</text><text x="247" y="159">D</text>
        </svg>
      </section>
      <div className="evidence-divider" aria-hidden="true"><span>inform</span><b>→</b></div>
      <section>
        <header><span>Authored DAG</span><small>stated assumptions</small></header>
        <svg viewBox="0 0 300 220" aria-hidden="true">
          <defs><marker id="evidence-arrow" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto"><path d="M0 0L8 4L0 8Z" /></marker></defs>
          <path className="dag-link" d="M66 65 C104 72 135 87 165 104" markerEnd="url(#evidence-arrow)" />
          <path className="dag-link" d="M183 111 C217 117 233 130 242 150" markerEnd="url(#evidence-arrow)" />
          <path className="dag-link dag-link--signal" d="M68 68 C113 122 167 151 227 159" markerEnd="url(#evidence-arrow)" />
          <circle cx="52" cy="58" r="25" /><circle cx="179" cy="106" r="25" /><circle cx="247" cy="163" r="25" />
          <text x="52" y="63">A</text><text x="179" y="111">C</text><text x="247" y="168">D</text>
        </svg>
      </section>
    </div>
  )
}

export function Landing() {
  const root = useRef<HTMLElement>(null)
  useEffect(() => {
    document.title = 'Hirmos, browser causal inference workbench'
    return root.current === null ? undefined : choreographLanding(root.current)
  }, [])
  return (
    <main ref={root} className="landing-root">
      {/* The band: header and hero on the blue. The scene fills the band behind the copy and takes the
          band's tokens, so the observations and the graph draw in the band's ink on the band's ground. */}
      <div className="landing-band band-texture">
        <Scene />
        <div className="landing-scrim" aria-hidden="true" />
        <header className="landing-header">
          <div className="landing-width landing-header__row">
            <InternalLink className="landing-wordmark" href="/" aria-label="Hirmos home">
              <HirmosMark className="landing-mark" size={28} />
              <span>hirmos</span>
            </InternalLink>
          </div>
        </header>

        <div className="landing-width landing-hero-shell">
          <section className="landing-hero-copy" aria-labelledby="landing-title">
            <h1 id="landing-title" className="landing-headline">
              {HEADLINE.map((word, index) => (
                <Fragment key={word}>
                  <span className="landing-slot" style={{ '--slot': index } as CSSProperties}><span className="landing-slot-word">{word}</span></span>
                  {index + 1 < HEADLINE.length ? ' ' : null}
                </Fragment>
              ))}
            </h1>
            <div className="landing-actions">
              <InternalLink className="landing-primary" href="/app">Start an analysis</InternalLink>
            </div>
          </section>
        </div>
      </div>

      <div className="landing-content">
        <section id="workflow" className="landing-width workflow-section" aria-labelledby="workflow-title">
          <div className="section-intro">
            <p className="landing-eyebrow">The workbench</p>
            <h2 id="workflow-title">Go from raw data to causal estimate in six steps</h2>
            <p>Hirmos provides a versioned artifact at each stage of the analysis.</p>
          </div>
          <ol className="workflow-list">
            {WORKFLOW.map((step) => (
              <li key={step.number}>
                <span>{step.number}</span>
                <div><h3>{step.title}</h3><p>{step.copy}</p></div>
              </li>
            ))}
          </ol>
        </section>

        <section id="graph" className="landing-width feature-section feature-section--evidence" aria-labelledby="evidence-title">
          <div className="feature-copy">
            <p className="landing-eyebrow">Discovery and specification</p>
            <h2 id="evidence-title">Compare discovery results with the causal model</h2>
            <p>Discovery methods report candidate contemporaneous and lagged relations. Hirmos displays these results beside the editable DAG. Graph revisions record which arrows enter the model.</p>
            <ul className="feature-points">
              <li>compare results from more than one discovery method</li>
              <li>inspect lag marks and uncertainty</li>
              <li>check cycles, temporal order, and edge grammar</li>
            </ul>
          </div>
          <EvidenceFigure />
        </section>

        <section id="identification" className="landing-width feature-section feature-section--identify" aria-labelledby="identify-title">
          <BackdoorFigure />
          <div className="feature-copy">
            <p className="landing-eyebrow">Identification before estimation</p>
            <h2 id="identify-title">Check the adjustment set before estimation</h2>
            <p>Hirmos checks whether the recorded DAG supplies a measured back-door adjustment set. It reports the selected set, mediators, colliders, and open paths, and states which identification strategies were not assessed.</p>
            <div className="adjustment-record">
              <span>Requested effect</span><b>X → Y</b>
              <span>Valid adjustment set</span><b>{'{ C }'}</b>
              <span>Identification</span><b className="record-ok">Back-door criterion satisfied</b>
            </div>
          </div>
        </section>

        <section id="structures" className="landing-width structures-section" aria-labelledby="structures-title">
          <div className="section-intro">
            <p className="landing-eyebrow">Observation structures</p>
            <h2 id="structures-title">Choose the correct observation structure</h2>
            <p>The observation structure records temporal order, repeated units, or independent observations. This determines method eligibility.</p>
          </div>
          <div className="structure-grid">
            <article><span className="structure-art structure-art--series" aria-hidden="true" /><h3>Time series</h3><p>Set regular or irregular time, lags, stationarity evidence, breaks, and time-aware discovery.</p></article>
            <article><span className="structure-art structure-art--panel" aria-hidden="true" /><h3>Panel</h3><p>Set unit and time keys for difference-in-differences, synthetic control, and synthetic DID.</p></article>
            <article><span className="structure-art structure-art--cross" aria-hidden="true" /><h3>Cross-sectional</h3><p>Use independent observations for DAG identification and compatible estimators.</p></article>
          </div>
        </section>

        <section className="landing-width final-cta" aria-labelledby="final-title">
          <HirmosMark className="landing-mark" size={28} />
          <h2 id="final-title">Start a new causal analysis</h2>
          <InternalLink className="landing-primary" href="/app">Open Hirmos</InternalLink>
        </section>

        <footer className="landing-width landing-footer">
          <InternalLink className="landing-wordmark" href="/"><HirmosMark className="landing-mark" /><span>hirmos</span></InternalLink>
        </footer>
      </div>
    </main>
  )
}
