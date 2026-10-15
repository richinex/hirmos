import { expect, test } from '@playwright/test'

test('discovery review preserves saved constraints, endpoint directions and dates', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { readRun } = await import(new URL('/src/assistant/runRead.ts', location.href).href)
    const { parseSnapshotValue } = await import(
      new URL('/src/domain/persistence.ts', location.href).href
    )
    const bundle = await (await fetch('/examples/fci-background-knowledge.hirmos.json')).json()
    const parsed = parseSnapshotValue(bundle.project)
    if (!parsed.ok) throw new Error(JSON.stringify(parsed.error))
    const workflow = { ...parsed.value, kind: 'profiled' }
    const run = workflow.discoveryRuns.at(-1)
    const before = JSON.stringify(workflow)
    const args = { workspace: 'discovery', id: run.id, view: 'review' }
    const review = readRun(workflow, args)
    const unchanged = before === JSON.stringify(workflow)
    workflow.prepared = { ...workflow.prepared, id: 'different-data' }
    return {
      review,
      unchanged,
      stale: readRun(workflow, args),
      unknown: readRun(workflow, { ...args, id: 'unknown' }),
    }
  })
  expect(result.unchanged).toBe(true)
  expect(result.review.kind).toBe('saved-run')
  expect(result.review.createdAt).toBe('2026-09-09T12:34:49.351Z')
  const background = result.review.review.backgroundKnowledge.settings
  expect(background).toEqual(
    expect.arrayContaining([
      expect.objectContaining({ key: 'forbidden', value: 'X1 → X4; X4 → X1' }),
      expect.objectContaining({ key: 'required', value: 'X8 → X18' }),
      expect.objectContaining({ key: 'tiers', value: 'X1: 1; X20: 0' }),
    ]),
  )
  expect(result.review.review.endpoints.edges).toEqual(
    expect.arrayContaining([
      { left: 'X1', marks: '-->', right: 'X9', display: 'X1 --> X9' },
      { left: 'X1', marks: '<--', right: 'X20', display: 'X1 <-- X20' },
    ]),
  )
  expect(result.stale.review).toEqual(result.review.review)
  expect(result.stale.provenance.preparedDataset).toBe('unavailable')
  expect(result.unknown.kind).toBe('refused')
})

test('temporal reviews use saved method evidence rather than a generic effect checklist', async ({
  page,
}) => {
  await page.goto('/app')
  const reviews = await page.evaluate(async () => {
    const { readRun } = await import(new URL('/src/assistant/runRead.ts', location.href).href)
    const { parseSnapshotValue } = await import(
      new URL('/src/domain/persistence.ts', location.href).href
    )
    const bundle = await (await fetch('/examples/seatbelts.hirmos.json')).json()
    const parsed = parseSnapshotValue(bundle.project)
    if (!parsed.ok) throw new Error(JSON.stringify(parsed.error))
    const workflow = { ...parsed.value, kind: 'profiled' }
    return workflow.discoveryRuns.map((run: { id: string }) =>
      readRun(workflow, { workspace: 'discovery', id: run.id, view: 'review' }),
    )
  })
  expect(reviews).toHaveLength(2)
  for (const read of reviews) {
    expect(read.review.kind).toBe('available')
    expect(read.review.backgroundKnowledge).toBeNull()
    expect(read.review.findings.maximumLag).toBeGreaterThan(0)
    expect(read.review.requirements.length).toBeGreaterThan(0)
  }
  expect(
    reviews.find((r: { model: string }) => r.model === 'lpcmci-run').review.findings.interpretation,
  ).toContain('latent confounding')
})
