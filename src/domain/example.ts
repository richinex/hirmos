import type { PersistedProject, SavedProjectHeader } from '@/domain/persistence'
import { assertNever, err, ok, type Result } from '@/domain/dop'

/**
 * The shipped examples: complete walkthroughs, each a project bundle with its source file inside.
 *
 * Each has a fixed project id, so its browser copy occupies one row whichever build produced it.
 * The ledger groups them by the question they answer, and a collection gathers the ones that belong
 * together for a purpose and can be removed as a unit.
 */

export type ExampleQuestion = 'intervention' | 'effect' | 'discovery' | 'set'

export interface ExampleQuestionEntry {
  readonly id: ExampleQuestion
  readonly title: string
  /** When this question is the one being asked, in one sentence. */
  readonly when: string
}

export const EXAMPLE_QUESTIONS: readonly [ExampleQuestionEntry, ...ExampleQuestionEntry[]] = [
  { id: 'intervention', title: 'Did an intervention work?', when: 'Something changed at a known time and you want the size of its effect.' },
  { id: 'effect', title: 'What is the effect of X?', when: 'X varies across units and everything else that varies with it has to be argued away.' },
  { id: 'discovery', title: 'Which variables cause which?', when: 'No graph yet; let the data propose one, then argue with it.' },
  { id: 'set', title: 'What would happen if we set X?', when: 'The graph is settled and the question is a hypothetical, not an observation.' },
]

export type ExampleCollectionId = 'ai-code-quality'

export interface ExampleCollection {
  readonly id: ExampleCollectionId
  readonly title: string
  readonly purpose: string
}

export const EXAMPLE_COLLECTIONS: readonly ExampleCollection[] = [
  { id: 'ai-code-quality', title: 'Impact of AI on code quality', purpose: 'Three designs for the same question, each suited to how adoption happened.' },
]

/** The small drawing beside an example: one fixed picture per kind of study. */
export type ExampleGlyph = 'dag' | 'step' | 'panel' | 'counts' | 'rct' | 'pag' | 'lag' | 'dose'

export interface ShippedExample {
  readonly id: SavedProjectHeader['id']
  readonly name: string
  readonly sourceName: string
  readonly bundleUrl: string
  readonly question: ExampleQuestion
  /** The identification strategy, as the ledger's Design column. */
  readonly design: string
  readonly shape: string
  readonly size: string
  /** Estimates recorded in the shipped bundle; a saved copy reports its own count instead. */
  readonly estimates: number
  readonly glyph: ExampleGlyph
  readonly collection: ExampleCollectionId | null
}

const id = (value: string): SavedProjectHeader['id'] => value as SavedProjectHeader['id']

export const SHIPPED_EXAMPLES: readonly [ShippedExample, ...ShippedExample[]] = [
  {
    id: id('d95e0c7b-44ec-4dee-a929-784cfd923eeb'),
    name: 'Seat-belt law and road deaths',
    sourceName: 'Seatbelts.csv',
    bundleUrl: '/examples/seatbelts.hirmos.json',
    question: 'effect', design: 'Back-door adjustment', shape: 'time series', size: '192 months', estimates: 3, glyph: 'dag', collection: null,
  },
  {
    id: id('7c2f1b3e-5a64-4d1e-9b0a-2e6f8c1d4a71'),
    name: 'AI adoption, company-wide',
    sourceName: 'company-wide-adoption.csv',
    bundleUrl: '/examples/ai-adoption-company-wide.hirmos.json',
    question: 'intervention', design: 'Causal impact against a control series', shape: 'time series', size: '36 months', estimates: 1, glyph: 'step', collection: 'ai-code-quality',
  },
  {
    id: id('a9e4c2d7-8b31-4f5e-b6c0-3d7a9e2f5b82'),
    name: 'AI adoption, March cohort',
    sourceName: 'cohort-march.csv',
    bundleUrl: '/examples/ai-adoption-cohort.hirmos.json',
    question: 'intervention', design: 'Panel DID and synthetic DID', shape: 'panel', size: '6 teams × 24 months', estimates: 1, glyph: 'panel', collection: 'ai-code-quality',
  },
  {
    id: id('c1d8e7f2-3a49-4b6d-8e5f-4f1b2c3d6a93'),
    name: 'AI usage intensity',
    sourceName: 'ai-usage-intensity.csv',
    bundleUrl: '/examples/ai-usage-intensity.hirmos.json',
    question: 'effect', design: 'Adjustment with a mediator to leave alone', shape: 'cross-section', size: '1,200 rows', estimates: 1, glyph: 'dag', collection: 'ai-code-quality',
  },
  {
    id: id('3f6a9c1e-2b7d-4e58-9a01-6c4d8e2f7b13'),
    name: 'NSW job training and 1978 earnings',
    sourceName: 'lalonde.csv',
    bundleUrl: '/examples/lalonde.hirmos.json',
    question: 'effect', design: 'Back-door adjustment over seven covariates', shape: 'cross-section', size: '614 people', estimates: 1, glyph: 'dag', collection: null,
  },
  {
    id: id('6a3e1d9f-5b47-4c82-8d1e-3f7a2b9c5e06'),
    name: 'GPS use and spatial memory',
    sourceName: 'gps-memory.csv',
    bundleUrl: '/examples/gps-memory.hirmos.json',
    question: 'effect', design: 'Front-door through a mediator', shape: 'cross-section', size: '1,000 people', estimates: 1, glyph: 'dag', collection: null,
  },
  {
    id: id('9c5b2e7a-1d38-4f64-a2b9-7e4c1f8d3a25'),
    name: 'A simulated process with a collider',
    sourceName: 'molak-ch7.csv',
    bundleUrl: '/examples/molak-ch7.hirmos.json',
    question: 'effect', design: 'Model, identify, estimate, refute', shape: 'cross-section', size: '1,000 rows', estimates: 2, glyph: 'dag', collection: null,
  },
  {
    id: id('5b2e8d4a-7c19-4f36-b8e2-1d9a3c6e4f57'),
    name: 'Proposition 99 and cigarette sales',
    sourceName: 'prop99-wide.csv',
    bundleUrl: '/examples/prop99.hirmos.json',
    question: 'intervention', design: 'Synthetic control from donor states', shape: 'time series', size: '31 years, 38 donors', estimates: 1, glyph: 'step', collection: null,
  },
  {
    id: id('8e1c4f7b-9a25-4d63-a7f0-2b5c9d8e1a64'),
    name: 'Campylobacter cases and an outbreak step',
    sourceName: 'campylobacter.csv',
    bundleUrl: '/examples/campylobacter.hirmos.json',
    question: 'intervention', design: 'Negative-binomial INGARCH', shape: 'time series', size: '140 periods', estimates: 1, glyph: 'counts', collection: null,
  },
  {
    id: id('2d7f9b3c-4e81-4a5d-b6c3-9f0e1a2d7c48'),
    name: 'Deploys and incidents at a lag',
    sourceName: 'deploys-incidents.csv',
    bundleUrl: '/examples/deploys-incidents.hirmos.json',
    question: 'discovery', design: 'PCMCI+, then a lagged total effect', shape: 'time series', size: '240 weeks', estimates: 1, glyph: 'lag', collection: null,
  },
  {
    id: id('4e8d2a6c-3f71-4b95-9c2d-5a1e7f3b8d92'),
    name: 'FCI with background knowledge',
    sourceName: 'causal-learn-linear-20.csv',
    bundleUrl: '/examples/fci-background-knowledge.hirmos.json',
    question: 'discovery', design: 'FCI with forbidden, required and tiered edges', shape: 'cross-section', size: '10,000 rows × 20', estimates: 0, glyph: 'pag', collection: null,
  },
  {
    id: id('1b9e6c3d-8a52-4d17-b3e4-6c2f9a5d1e78'),
    name: 'Severity, dose and recovery',
    sourceName: 'confounded-dose.csv',
    bundleUrl: '/examples/confounded-dose.hirmos.json',
    question: 'set', design: 'Do-query on a discrete network', shape: 'cross-section', size: '3,000 patients', estimates: 0, glyph: 'dose', collection: null,
  },
]

const SEATBELTS = SHIPPED_EXAMPLES[0]

/** The first example's identity, kept under its old names for the tests and the builder that use them. */
export const EXAMPLE_BUNDLE_URL = SEATBELTS.bundleUrl
export const EXAMPLE_PROJECT_ID = SEATBELTS.id
export const EXAMPLE_PROJECT_NAME = SEATBELTS.name
export const EXAMPLE_SOURCE_NAME = SEATBELTS.sourceName

export const isShippedExampleId = (candidate: SavedProjectHeader['id']): boolean =>
  SHIPPED_EXAMPLES.some((example) => example.id === candidate)

export const shippedExampleById = (candidate: SavedProjectHeader['id']): ShippedExample | null =>
  SHIPPED_EXAMPLES.find((example) => example.id === candidate) ?? null

export type ExampleCopyAssessment =
  | { readonly kind: 'current-release'; readonly snapshot: PersistedProject }
  | { readonly kind: 'replace-with-shipped'; readonly reason: 'missing-stamp' | 'different-release' }
  | { readonly kind: 'invalid-copy'; readonly detail: string }

export type ExampleStampProblem = { readonly kind: 'wrong-project'; readonly actual: SavedProjectHeader['id'] }

/** Bind the adopted copy to the export that produced the shipped bundle. */
export function stampExampleRelease(
  snapshot: PersistedProject,
  exportedAt: string,
  exampleId: SavedProjectHeader['id'] = EXAMPLE_PROJECT_ID,
): Result<PersistedProject, ExampleStampProblem> {
  if (snapshot.project.id !== exampleId) return err({ kind: 'wrong-project', actual: snapshot.project.id })
  return ok({ ...snapshot, origin: { kind: 'shipped-example', exportedAt } })
}

/**
 * Decide what opening a built-in example means; the caller performs the selected storage action.
 * The release identity is the export that produced the bundle: edits keep the stamp, so a copy of
 * this release is kept with them, while a copy of an older release, or one with no stamp at all,
 * is replaced so newly completed chapters are not hidden behind stale browser state.
 */
export function assessExampleCopy(
  stored: PersistedProject,
  shippedExportedAt: string,
  exampleId: SavedProjectHeader['id'] = EXAMPLE_PROJECT_ID,
): ExampleCopyAssessment {
  if (stored.project.id !== exampleId) {
    return { kind: 'invalid-copy', detail: 'The record stored under the example key belongs to another project.' }
  }
  switch (stored.origin.kind) {
    case 'user': return { kind: 'replace-with-shipped', reason: 'missing-stamp' }
    case 'shipped-example':
      return stored.origin.exportedAt === shippedExportedAt
        ? { kind: 'current-release', snapshot: stored }
        : { kind: 'replace-with-shipped', reason: 'different-release' }
    default: return assertNever(stored.origin)
  }
}
