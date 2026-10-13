import { expect, test } from '@playwright/test'

test('proposal availability and graph summaries distinguish the relevant states', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { proposalAvailability } = await import(
      new URL('/src/assistant/proposalAvailability.ts', location.href).href
    )
    const { describeGraphOperation } = await import(
      new URL('/src/assistant/graphProposal.ts', location.href).href
    )
    const { identificationChoices } = await import(
      new URL('/src/assistant/useIdentificationTool.ts', location.href).href
    )
    const document = {
      current: {
        graph: {
          nodes: [
            { id: 'x', name: 'Exposure' },
            { id: 'y', name: 'Outcome' },
          ],
          edges: [{ id: 'edge', cause: 'x', effect: 'y', timing: { kind: 'lagged', lag: 2 } }],
        },
      },
    }
    return {
      availability: [false, true].flatMap((running) =>
        [false, true].map((disabled) => proposalAvailability(running, disabled).kind),
      ),
      removedLag: describeGraphOperation(
        {
          kind: 'edit-arrow',
          edge: 'edge',
          timing: { kind: 'contemporaneous' },
          rationale: 'Immediate mechanism',
        },
        document,
      ).title,
      added: describeGraphOperation(
        {
          kind: 'add-arrow',
          cause: 'Exposure',
          effect: 'Outcome',
          timing: { kind: 'contemporaneous' },
          rationale: null,
        },
        document,
      ).title,
      empty: identificationChoices([[]], ['Age']),
      adjusted: identificationChoices([[0]], ['Age']),
      unavailable: identificationChoices([], ['Age']),
    }
  })
  expect(result.availability).toEqual(['ready', 'blocked', 'blocked', 'blocked'])
  expect(result.removedLag).toContain('(lag 2 removed)')
  expect(result.added).toBe('Add Exposure → Outcome')
  expect(result.empty.noAdjustmentNeeded).toBe(true)
  expect(result.empty.meaning).toContain('No adjustment is needed')
  expect(result.adjusted.minimalSets).toEqual([['Age']])
  expect(result.adjusted).not.toHaveProperty('meaning')
  expect(result.unavailable.noAdjustmentNeeded).toBe(false)
  expect(result.unavailable).not.toHaveProperty('meaning')
})
