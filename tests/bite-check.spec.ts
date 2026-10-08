import { test, expect } from '@playwright/test'
import { chapter, identifyEffect } from './examples/support'

test('bite drafts inherit their parent and reject changed questions or graph revisions', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { parseSnapshotValue } = await import(new URL('/src/domain/persistence.ts', location.href).href)
    const { biteStudyDraft, biteVariables, matchesBiteDraft, readyStudySpecification } = await import(new URL('/src/domain/study.ts', location.href).href)
    const { stepWorkflow } = await import(new URL('/src/domain/workflow.ts', location.href).href)
    const parsed = parseSnapshotValue((await (await fetch('/examples/ai-usage-intensity.hirmos.json')).json()).project)
    if (!parsed.ok) throw Error('fixture')
    const snapshot = parsed.value
    const parent = { ...snapshot.studies[0], estimand: { kind: 'average-treatment-effect-on-treated', scale: 'additive', treatedValue: 1 } }
    const variable = biteVariables(parent)[0]
    const draft = biteStudyDraft(parent, variable.node)
    const ready = (value: unknown, docs = snapshot.dagDocuments) => readyStudySpecification(value, docs, snapshot.prepared, [parent])
    const changed = [
      { ...draft, treatment: parent.outcome.node },
      { ...draft, outcome: parent.outcome.node },
      { ...draft, estimand: 'average-treatment-effect' },
      { ...draft, assignment: { ...draft.assignment, description: 'Different assignment' } },
    ]
    const stale = snapshot.dagDocuments.map((doc: any) => ({ ...doc, current: { ...doc.current, id: 'new-revision' } }))
    const workflow = { ...snapshot, kind: 'profiled', studies: [parent], studyDraft: draft }
    return {
      target: draft.estimand,
      valid: ready(draft).ok,
      invalid: changed.map(value => ready(value).error?.kind),
      stale: ready(draft, stale).error?.kind,
      retained: stepWorkflow(workflow, { type: 'study-draft-changed', draft }).studyDraft.biteOf === parent.id,
      cleared: changed.map(value => stepWorkflow(workflow, { type: 'study-draft-changed', draft: value }).studyDraft.biteOf),
      missingParent: readyStudySpecification(draft, snapshot.dagDocuments, snapshot.prepared, []).error?.kind,
      matches: matchesBiteDraft(draft, parent, snapshot.dagDocuments),
    }
  })
  expect(result.target).toBe('average-treatment-effect-on-treated')
  expect(result.valid).toBe(true)
  expect(result.matches).toBe(true)
  expect(result.retained).toBe(true)
  expect(result.invalid).toEqual(Array(4).fill('bite-specification-mismatch'))
  expect(result.cleared).toEqual(Array(4).fill(null))
  expect(result.stale).toBe('bite-specification-mismatch')
  expect(result.missingParent).toBe('bite-specification-mismatch')
})

test('bite variables are the measured mediators the treatment points at directly', async ({
  page,
}) => {
  await page.goto('/app')
  const names = await page.evaluate(async () => {
    const { parseSnapshotValue } = await import(
      new URL('/src/domain/persistence.ts', location.href).href
    )
    const { biteVariables } = await import(new URL('/src/domain/study.ts', location.href).href)
    const read = async (bundle: string) => {
      const parsed = parseSnapshotValue((await (await fetch(bundle)).json()).project)
      if (!parsed.ok) throw Error(JSON.stringify(parsed.error))
      return parsed.value.studies.map((study: unknown) =>
        biteVariables(study).map((variable: { name: string }) => variable.name),
      )
    }
    return {
      usage: await read('/examples/ai-usage-intensity.hirmos.json'),
      lalonde: await read('/examples/lalonde.hirmos.json'),
    }
  })
  expect(names.usage).toEqual([['code_volume_kloc']])
  for (const study of names.lalonde) expect(study).toEqual([])
})

test('a bite check is recorded against its study, estimated, and paired in Results', async ({
  page,
}) => {
  test.setTimeout(240_000)
  await page.goto('/app')
  await page.getByRole('button', { name: 'Open AI usage intensity', exact: true }).click()
  await chapter(page, /^Study design/)

  await page.getByText('Check the treatment’s bite', { exact: true }).click()
  await page.getByRole('button', { name: 'Study as the outcome' }).click()
  await expect(page.getByRole('combobox', { name: 'Outcome' })).toHaveText(/code_volume_kloc/)
  await identifyEffect(page, /^Recommended O-set/)
  const card = page.getByRole('article', { name: /code_volume_kloc identification$/ })
  await expect(card).toContainText('Bite check for: Average effect of ai_usage on bugs_per_kloc.')
  await expect(card).toContainText('Recommended adjustment set (O-set)')

  await card.getByRole('button', { name: 'Continue to estimation' }).click()
  await expect(page.getByRole('combobox', { name: 'Identified study' })).toHaveText(
    /Average effect of ai_usage on code_volume_kloc/,
  )
  await page.getByRole('button', { name: 'Run adjusted linear regression' }).click()
  await expect(page.getByText(/on code volume kloc/).first()).toBeVisible({ timeout: 180_000 })

  await chapter(page, /^Results/)
  const pick = async (estimate: RegExp) => {
    await page.getByRole('combobox', { name: 'Estimate', exact: true }).click()
    await page.getByRole('option', { name: estimate }).click()
  }
  await pick(/on bugs_per_kloc/)
  const bite = page.getByRole('region', { name: 'Bite of ai_usage' })
  await expect(bite).toContainText('Effect on code_volume_kloc')
  await expect(bite).toContainText('95%')
  await expect(bite).toContainText('Difference in')
  await pick(/on code_volume_kloc/)
  const study = page.getByRole('region', { name: 'Study', exact: true })
  await expect(study).toContainText('Bite check for')
  await expect(study).toContainText('Average effect of ai_usage on bugs_per_kloc')

  // Sticky routing reopens the project on reload; the link is part of the saved study.
  await page.reload()
  await chapter(page, /^Study design/)
  await expect(
    page.getByText('Bite check for: Average effect of ai_usage on bugs_per_kloc.').first(),
  ).toBeVisible()
})
