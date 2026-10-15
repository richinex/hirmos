import { expect, test } from '@playwright/test'
import { chapter } from './examples/support'

test('propensity review separates recorded findings, absent balance and stale provenance without changing the run', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { reviewAnalysis } = await import(
      new URL('/src/domain/analysisReview.ts', location.href).href
    )
    const { readRun } = await import(new URL('/src/assistant/runRead.ts', location.href).href)
    const { project: p } = await (await fetch('/examples/lalonde.hirmos.json')).json()
    const base = p.estimationRuns[0]
    const run = {
      ...base,
      kind: 'doubly-robust-run',
      configuration: {
        kind: 'doubly-robust',
        model: 'newton',
        maxIter: 1000,
        uncertainty: { kind: 'none' },
      },
      evidence: {
        kind: 'doublyRobust',
        target: 'att',
        observations: 4,
        parameters: 2,
        estimate: 1,
        treatedTerm: 2,
        controlTerm: 1,
        converged: false,
        interval: null,
        propensity: [0.3, 0.6, 0.2, 0.7],
        treated: [true, true, false, false],
        balance: {
          kind: 'available',
          reference: 'treated',
          labels: ['age'],
          rows: [
            { column: 0, before: 0.4, after: 0.05, denominator: 1, usedFullSampleSpread: false },
          ],
        },
      },
    }
    const context = {
      studies: p.studies,
      identifications: p.identifications,
      documents: p.dagDocuments,
      preparedDataset: p.prepared.id,
      sensitivityRuns: [],
    }
    const before = JSON.stringify(run)
    const review = reviewAnalysis(run, context)
    const missing = reviewAnalysis(
      { ...run, evidence: { ...run.evidence, balance: { kind: 'notRecorded' } } },
      context,
    )
    const unavailable = reviewAnalysis(
      {
        ...run,
        evidence: {
          ...run.evidence,
          balance: { kind: 'unavailable', reason: 'Zero reference spread' },
        },
      },
      context,
    )
    const empty = reviewAnalysis(
      {
        ...run,
        evidence: {
          ...run.evidence,
          balance: { kind: 'available', reference: 'treated', labels: [], rows: [] },
        },
      },
      context,
    )
    const stale = reviewAnalysis(run, {
      ...context,
      preparedDataset: 'changed',
      documents: p.dagDocuments.map((d: any) => ({
        ...d,
        current: { ...d.current, id: 'new-revision' },
      })),
    })
    const workflow = { ...p, kind: 'profiled', estimationRuns: [run] }
    return {
      review,
      missing,
      unavailable,
      empty,
      stale,
      unchanged: JSON.stringify(run) === before,
      read: readRun(workflow, { id: run.id, view: 'review' }),
      unknown: readRun(workflow, { id: 'unknown', view: 'review' }),
      other: readRun(workflow, { id: run.id, workspace: 'survival', view: 'review' }),
      unsupported: reviewAnalysis(base, context),
    }
  })
  const item = (review: any, id: string) => review.items.find((i: any) => i.id === id)
  expect(item(result.review, 'population').reading).toContain('2 treated rows')
  expect(item(result.review, 'convergence').evidence.concern).toBe(true)
  expect(item(result.review, 'balance').evidence.kind).toBe('available')
  expect(item(result.review, 'balance').reading).toContain('not the outcome regression')
  expect(item(result.missing, 'balance').evidence.kind).toBe('not-recorded')
  expect(item(result.unavailable, 'balance').evidence.kind).toBe('unavailable')
  expect(item(result.empty, 'balance').evidence.kind).toBe('not-applicable')
  expect(result.stale.provenance.join(' ')).toContain('earlier DAG revision')
  expect(result.stale.provenance.join(' ')).toContain('current prepared data')
  expect(result.unchanged).toBe(true)
  expect(result.read.review).toEqual(result.review)
  expect(result.unknown.kind).toBe('refused')
  expect(result.other.kind).toBe('refused')
  expect(result.unsupported.kind).toBe('not-supported')
})

test('DiD review distinguishes a two-period limitation, pre-treatment intervals and mismatched sensitivity evidence', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { reviewAnalysis } = await import(
      new URL('/src/domain/analysisReview.ts', location.href).href
    )
    const { project: p } = await (await fetch('/examples/ai-adoption-cohort.hirmos.json')).json()
    const base = p.estimationRuns.find((r: any) => r.evidence.kind === 'panelDid')
    const context = {
      studies: p.studies,
      identifications: p.identifications,
      documents: p.dagDocuments,
      preparedDataset: p.prepared.id,
      sensitivityRuns: [],
    }
    const two = reviewAnalysis(
      { ...base, evidence: { ...base.evidence, nPre: 1, nPost: 1 } },
      context,
    )
    const specification = {
      controls: 'never-treated',
      baseline: 'universal',
      anticipation: 0,
      firstEvent: null,
      lastEvent: null,
      balance: null,
      confidence: 0.95,
      inference: { kind: 'analytical' },
    }
    const intervals = [
      { kind: 'estimated', estimate: 0.3, standardError: 0.1, lower: 0.1, upper: 0.5 },
      { kind: 'reference' },
      { kind: 'estimated', estimate: 1, standardError: 0.1, lower: 0.8, upper: 1.2 },
    ]
    const staggered = {
      ...base,
      configuration: {
        kind: 'panel-intervention',
        primary: 'staggered',
        covariates: [],
        specification,
      },
      evidence: {
        kind: 'staggeredDid',
        specification,
        observations: 40,
        retainedObservations: 36,
        changes: [{ kind: 'noUntreatedComparison', period: 5 }],
        smallCohorts: [],
        fits: [],
        events: {
          keys: [-2, -1, 0],
          intervals,
          coverage: { kind: 'pointwise' },
          analyticalCovariance: [
            [0.01, 0, 0],
            [0, 0, 0],
            [0, 0, 0.01],
          ],
        },
      },
    }
    const review = reviewAnalysis(staggered, context)
    const mismatch = reviewAnalysis(staggered, {
      ...context,
      sensitivityRuns: [
        {
          kind: 'honest-did-run',
          id: 'invalid',
          source: { kind: 'estimation', run: base.id },
          preparedDataset: 'other-data',
          evidence: {},
        },
      ],
    })
    const unrelated = reviewAnalysis(staggered, {
      ...context,
      sensitivityRuns: [
        {
          kind: 'honest-did-run',
          id: 'unrelated',
          source: { kind: 'estimation', run: 'another-run' },
          evidence: {},
        },
      ],
    })
    const synthetic = reviewAnalysis(p.estimationRuns[0], context)
    const availablePlacebo = reviewAnalysis(
      {
        ...p.estimationRuns[0],
        evidence: {
          ...p.estimationRuns[0].evidence,
          syntheticDidPlacebo: { kind: 'available', replications: 100, standardError: 0.2 },
        },
      },
      context,
    )
    const honest = {
      kind: 'honest-did-run',
      id: crypto.randomUUID(),
      preparedDataset: base.preparedDataset,
      createdAt: '2026-10-15T00:00:00Z',
      source: { kind: 'estimation', run: base.id },
      evidence: {
        kind: 'honestDid',
        version: 1,
        estimate: 1,
        standardError: 0.1,
        conventional: [0.8, 1.2],
        request: {
          eventStudy: {
            pre: 1,
            post: 1,
            events: [-2, 0],
            coefficients: [0.3, 1],
            covariance: [
              [0.01, 0],
              [0, 0.01],
            ],
            contrast: [1],
          },
          configuration: {
            confidence: 0.95,
            bounds: [0.1],
            seed: 0,
            restriction: { kind: 'smoothness', points: 100 },
          },
        },
        results: [
          {
            bound: 0.1,
            interval: {
              kind: 'fixedLength',
              lower: 0.5,
              upper: 1.5,
              coefficients: [0, 1],
              halfLength: 0.5,
              search: 'grid',
              accuracy: 'solved',
            },
          },
        ],
      },
    }
    const matched = reviewAnalysis(staggered, { ...context, sensitivityRuns: [honest] })
    const altered = reviewAnalysis(staggered, {
      ...context,
      sensitivityRuns: [
        {
          ...honest,
          evidence: {
            ...honest.evidence,
            request: {
              ...honest.evidence.request,
              eventStudy: { ...honest.evidence.request.eventStudy, coefficients: [0.4, 1] },
            },
          },
        },
      ],
    })
    const sun = reviewAnalysis(
      {
        ...base,
        kind: 'panel-intervention-run',
        configuration: {
          kind: 'panel-intervention',
          primary: 'sunAbraham',
          referencePeriods: [-1],
          referenceCohorts: [],
          confidence: 0.95,
        },
        evidence: {
          kind: 'sunAbraham',
          retainedRows: [0, 1],
          removedRows: [2],
          clusters: 2,
          omitted: [],
          request: { referencePeriods: [-1], referenceCohorts: [] },
          events: [{ key: -2, interval: { lower: 0.1, upper: 0.5 } }],
        },
      },
      context,
    )
    const standalone = reviewAnalysis(
      {
        ...base,
        kind: 'synthetic-control-run',
        configuration: {
          kind: 'synthetic-control',
          start: { kind: 'from-treatment' },
          donors: [],
          crossFitFolds: 3,
          alpha: 0.05,
        },
        evidence: {
          nPre: 10,
          nPost: 4,
          weights: [0.2, 0.8],
          donorPlacebo: { kind: 'unavailable', reason: 'Insufficient comparable donors.' },
        },
      },
      context,
    )
    return {
      two,
      review,
      mismatch,
      unrelated,
      synthetic,
      availablePlacebo,
      matched,
      altered,
      sun,
      standalone,
    }
  })
  const item = (review: any, id: string) => review.items.find((i: any) => i.id === id)
  expect(item(result.two, 'pre-trends').evidence.kind).toBe('not-assessable')
  expect(item(result.review, 'pre-trends').reading).toContain('1 exclude zero')
  expect(item(result.review, 'pre-trends').evidence.concern).toBe(true)
  expect(item(result.review, 'pre-trends').next).toContain('not a joint pre-trend test')
  expect(item(result.review, 'parallel-trends').evidence.kind).toBe('not-recorded')
  expect(item(result.mismatch, 'parallel-trends').evidence.kind).toBe('mismatched')
  expect(item(result.unrelated, 'parallel-trends').evidence.kind).toBe('not-recorded')
  expect(item(result.synthetic, 'placebo-uncertainty').evidence.kind).toBe('unavailable')
  expect(item(result.availablePlacebo, 'placebo-uncertainty').reading).toContain('standard error')
  expect(item(result.synthetic, 'in-time-placebo').reading).toContain('in-time placebo estimate')
  expect(item(result.matched, 'parallel-trends').evidence.kind).toBe('available')
  expect(item(result.matched, 'parallel-trends').reading).toContain('0.500 to 1.50')
  expect(item(result.altered, 'parallel-trends').evidence.kind).toBe('mismatched')
  expect(item(result.sun, 'comparison').reading).toContain('1 removed')
  expect(item(result.sun, 'pre-trends').evidence.concern).toBe(true)
  expect(item(result.sun, 'parallel-trends').evidence.kind).toBe('not-assessable')
  expect(item(result.standalone, 'placebos').evidence.kind).toBe('unavailable')
  expect(item(result.standalone, 'placebos').reading).toBe('Insufficient comparable donors.')
})

test('RD and IV reviews report diagnostic gaps without applying the propensity checklist', async ({
  page,
}) => {
  await page.goto('/app')
  const reviews = await page.evaluate(async () => {
    const { reviewAnalysis } = await import(
      new URL('/src/domain/analysisReview.ts', location.href).href
    )
    const { project: p } = await (await fetch('/examples/lalonde.hirmos.json')).json()
    const base = p.estimationRuns[0]
    const context = {
      studies: p.studies,
      identifications: p.identifications,
      documents: p.dagDocuments,
      preparedDataset: p.prepared.id,
      sensitivityRuns: [],
    }
    return [
      reviewAnalysis(
        {
          ...base,
          kind: 'sharp-rd-run',
          configuration: { kind: 'sharp-rd' },
          evidence: {
            cutoff: 0,
            bandwidth: 1,
            effectiveObservations: [20, 30],
            robust: { interval: [-1, 2] },
            conventional: { value: 1 },
            biasCorrected: { value: 0.5 },
          },
        },
        context,
      ),
      reviewAnalysis(
        {
          ...base,
          kind: 'instrumental-variable-run',
          configuration: {
            kind: 'instrumental-variable',
            simulations: 100,
            sampleSizeFraction: 1,
            level: 0.95,
            seed: 1,
          },
          evidence: { instruments: [2], observations: 50 },
        },
        context,
      ),
    ]
  })
  expect(reviews[0].items.find((i: any) => i.id === 'local-sample').reading).toContain(
    '20 observations below and 30',
  )
  expect(reviews[0].items.find((i: any) => i.id === 'manipulation').evidence.kind).toBe(
    'not-recorded',
  )
  expect(reviews[1].items.find((i: any) => i.id === 'strength').evidence.kind).toBe('not-recorded')
  for (const id of ['manipulation', 'bandwidth']) {
    expect(reviews[0].items.find((i: any) => i.id === id).capability.kind).toBe('not-offered')
    expect(reviews[0].items.find((i: any) => i.id === id).next).toBeNull()
  }
  expect(reviews[1].items.find((i: any) => i.id === 'exclusion').evidence.kind).toBe(
    'not-assessable',
  )
  for (const review of reviews)
    expect(review.items.some((i: any) => i.id === 'balance')).toBe(false)
})

test('Results does not add an agent review panel', async ({ page }) => {
  await page.emulateMedia({ reducedMotion: 'reduce' })
  await page.goto('/app')
  await page.setViewportSize({ width: 1280, height: 1100 })
  await page.getByRole('button', { name: 'Open AI adoption, March cohort', exact: true }).click()
  await chapter(page, /Results/)
  await expect(page.getByTestId('analysis-review')).toHaveCount(0)
  await expect(page.getByRole('heading', { name: /^Analysis review/ })).toHaveCount(0)
})
