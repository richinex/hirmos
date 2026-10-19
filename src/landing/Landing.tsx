import { Fragment, useEffect, useRef, useState, type CSSProperties } from 'react'
import { HirmosMark } from '@/components/HirmosMark'
import { Icon } from '@/components/Icon'
import { SourceNotice } from '@/components/SourceNotice'
import { InternalLink } from '@/components/ui/InternalLink'
import { SHIPPED_EXAMPLES } from '@/domain/example'
import { CHAPTER_METADATA, chapterPath, type ChapterId } from '@/domain/navigation'
import { choreographLanding } from './choreography'
import { DeskReplay } from './DeskReplay'
import { EstimateReplay } from './EstimateReplay'
import { IdentifyReplay } from './IdentifyReplay'
import type { HirmosGraph } from './HirmosGraph'

// Fetched with the page and mounted the moment it arrives. Not React.lazy: a Suspense boundary holds
// its fallback for React's reveal window, and the fallback here is the same empty placeholder the
// static shell in index.html already paints, so nothing may appear and move in between.
const graphChunk = import('./HirmosGraph')
// The textbook example, opened with its bundled data at its graph.
const NSW_EXAMPLE = SHIPPED_EXAMPLES.find((example) => example.sourceName === 'lalonde.csv')

/** The hero headline, one slot per phrase so the reveal is markup, not a script; a slot never breaks. */
const HEADLINE = ['The causal inference', 'app'] as const

const WORKFLOW = [
  {
    number: '01',
    title: 'Prepare',
    copy: 'Profile the source, define the observation structure, resolve missingness, and record transformations.',
  },
  {
    number: '02',
    title: 'Examine',
    copy: 'Run data diagnostics and, when useful, compare discovery methods that propose candidate edges.',
  },
  {
    number: '03',
    title: 'Model',
    copy: 'Develop the causal graph from domain knowledge, study design, and relevant empirical evidence.',
  },
  {
    number: '04',
    title: 'Identify',
    copy: 'Define the treatment, outcome, intervention, target population, and effect measure. Determine whether the graph identifies that effect and which adjustment strategies are valid.',
  },
  {
    number: '05',
    title: 'Estimate',
    copy: 'Fit eligible estimators and compare effect estimates, uncertainty, and model diagnostics.',
  },
  {
    number: '06',
    title: 'Assess',
    copy: 'Examine sensitivity to assumptions, apply appropriate refutation checks, and evaluate counterfactual questions when the fitted causal model supports them.',
  },
] as const

function Scene() {
  const [Graph, setGraph] = useState<typeof HirmosGraph | null>(null)
  useEffect(() => {
    let mounted = true
    void graphChunk.then((module) => {
      if (mounted) setGraph(() => module.HirmosGraph)
    })
    return () => {
      mounted = false
    }
  }, [])
  return (
    <div className="landing-scene" aria-hidden="true">
      {Graph !== null && <Graph className="landing-scene__canvas" />}
    </div>
  )
}

/** A chapter's tab as the workbench draws it: the chapter's icon and its short name. */
function ChapterTab({ chapter }: { readonly chapter: ChapterId }) {
  const { icon, shortName } = CHAPTER_METADATA[chapter]
  return (
    <p className="landing-tab">
      <Icon name={icon} size={18} fill />
      {shortName}
    </p>
  )
}

export function Landing() {
  const root = useRef<HTMLElement>(null)
  useEffect(() => {
    document.title = 'Hirmos - causal inference app'
    return root.current === null ? undefined : choreographLanding(root.current)
  }, [])
  useEffect(() => {
    const element = root.current
    if (element === null) return
    const mark = () => element.style.setProperty('--scrolled', window.scrollY > 4 ? '1' : '0')
    mark()
    window.addEventListener('scroll', mark, { passive: true })
    return () => window.removeEventListener('scroll', mark)
  }, [])
  return (
    <main ref={root} className="landing-root">
      <header className="landing-width landing-top">
        <InternalLink className="landing-wordmark" href="/" aria-label="Hirmos home">
          <HirmosMark className="landing-mark" size={28} />
          <span>hirmos</span>
        </InternalLink>
      </header>

      <div className="landing-width landing-drawers">
        <section id="workflow" className="landing-drawer" aria-labelledby="landing-title">
          <div className="landing-folder">
            <div className="landing-band">
              <Scene />
              <div className="landing-scrim" aria-hidden="true" />
              <div className="landing-hero-copy">
                <h1 id="landing-title" className="landing-headline">
                  {HEADLINE.map((word, index) => (
                    <Fragment key={word}>
                      <span className="landing-slot" style={{ '--slot': index } as CSSProperties}>
                        <span className="landing-slot-word">{word}</span>
                      </span>
                      {index + 1 < HEADLINE.length ? ' ' : null}
                    </Fragment>
                  ))}
                </h1>
              </div>
            </div>
            <section className="landing-sheet workflow-section" aria-labelledby="workflow-title">
              <div className="section-intro">
                <h2 id="workflow-title">Go from raw data to causal estimate in six steps</h2>
                <div className="landing-actions">
                  <InternalLink className="landing-primary" href="/app">
                    Start an analysis
                  </InternalLink>
                  {NSW_EXAMPLE === undefined ? null : (
                    <InternalLink
                      className="landing-secondary"
                      href={chapterPath('dag', NSW_EXAMPLE.id)}
                    >
                      Open the NSW example
                    </InternalLink>
                  )}
                </div>
              </div>
              <ol className="workflow-list">
                {WORKFLOW.map((step) => (
                  <li key={step.number}>
                    <span>{step.number}</span>
                    <h3>{step.title}</h3>
                    <p>{step.copy}</p>
                  </li>
                ))}
              </ol>
            </section>
          </div>
        </section>

        <section id="identification" className="landing-drawer" aria-labelledby="identify-title">
          <ChapterTab chapter="study" />
          <div className="landing-folder landing-folder--story">
            <div className="landing-folder-copy">
              <h2 id="identify-title" className="landing-folder-title">
                Check the adjustment set before estimation
              </h2>
              <p>
                Hirmos checks whether the recorded DAG supplies a measured back-door adjustment set.
                It reports the selected set, mediators, colliders, and open paths, and states which
                identification strategies were not assessed.
              </p>
            </div>
            <div className="landing-sheet landing-story">
              {NSW_EXAMPLE === undefined ? null : <p className="replay-example">{NSW_EXAMPLE.name}</p>}
              <IdentifyReplay />
            </div>
          </div>
        </section>

        <section id="estimate" className="landing-drawer" aria-labelledby="estimate-title">
          <ChapterTab chapter="estimation" />
          <div className="landing-folder landing-folder--story landing-folder--flip">
            <div className="landing-folder-copy">
              <h2 id="estimate-title" className="landing-folder-title">
                Choose an estimator for the identified effect and run the analysis
              </h2>
              <p>
                Identification determines how to express the causal question using observed data.
                Estimation applies a statistical method to that expression.
              </p>
            </div>
            <div className="landing-sheet landing-story">
              {NSW_EXAMPLE === undefined ? null : <p className="replay-example">{NSW_EXAMPLE.name}</p>}
              <EstimateReplay />
            </div>
          </div>
        </section>
        <section id="results" className="landing-drawer" aria-label="Results">
          <ChapterTab chapter="results" />
          <div className="landing-folder">
            <div className="landing-sheet landing-story">
              {NSW_EXAMPLE === undefined ? null : <p className="replay-example">{NSW_EXAMPLE.name}</p>}
              <DeskReplay />
            </div>
          </div>
        </section>
      </div>

      <footer className="landing-width landing-footer">
        <SourceNotice />
      </footer>
    </main>
  )
}
