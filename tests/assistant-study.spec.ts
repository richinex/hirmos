import { openRouterStream } from './openRouterStream'
import { expect, test } from '@playwright/test'

test('study-bound runs refuse missing, different and misnamed subjects', async ({ page }) => {
  await page.goto('/app')
  const results = await page.evaluate(async () => {
    const { runSubjectProblem } = await import(
      new URL('/src/assistant/analysisRun.ts', location.href).href
    )
    const subject = {
      kind: 'saved-study',
      study: 'kms-study',
      treatment: 'kms',
      outcome: 'DriversKilled',
    }
    const inspection = {
      section: 'Estimation',
      method: 'Regression',
      specification: '{}',
      sharedResult: '',
      study: { subject, question: '', settings: [] },
    }
    return [
      undefined,
      { ...subject, study: 'law-study', treatment: 'law' },
      { ...subject, treatment: 'law' },
      { ...subject, outcome: 'Other' },
      subject,
    ].map((requested) => runSubjectProblem(inspection, requested))
  })
  expect(results.slice(0, 4).every((message) => typeof message === 'string')).toBe(true)
  expect(results[4]).toBeNull()
})

test('study proposals reuse domain validation and cannot overwrite a changed draft', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { proposeStudy, studyProposalEvent } = await import(
      new URL('/src/assistant/studyProposal.ts', location.href).href
    )
    const { stepWorkflow } = await import(new URL('/src/domain/workflow.ts', location.href).href)
    const bundle = await (await fetch('/examples/ai-usage-intensity.hirmos.json')).json()
    const { parseSnapshotValue } = await import(
      new URL('/src/domain/persistence.ts', location.href).href
    )
    const snapshot = parseSnapshotValue(bundle.project)
    if (!snapshot.ok) throw new Error(JSON.stringify(snapshot.error))
    const workflow = { ...snapshot.value, kind: 'profiled' }
    const document = workflow.dagDocuments[0]
    const base = {
      document: document.id,
      treatment: 'ai_usage',
      outcome: 'bugs_per_kloc',
      target: { kind: 'average-treatment-effect' },
      assignment: {
        kind: 'observed-choice',
        description: 'Users choose their AI usage intensity.',
      },
      consistencyRationale: null,
      noInterferenceRationale: null,
      summary: 'Study AI usage and defects.',
      checks: ['Review assignment and assumptions.'],
    }
    const before = JSON.stringify(workflow)
    const proposed = proposeStudy(workflow, base)
    if (!proposed.ok) throw new Error(proposed.error)
    const unchanged = before === JSON.stringify(workflow)
    const event = studyProposalEvent(workflow, proposed.value)
    if (!event.ok) throw new Error(event.error)
    const applied = stepWorkflow(workflow, event.value)
    const invalid = [
      { outcome: 'missing' },
      { outcome: 'ai_usage' },
      { assignment: { kind: 'randomised', description: 'Invented randomisation.' } },
      { target: { kind: 'conditional-average-treatment-effect' } },
      {
        target: {
          kind: 'conditional-average-treatment-effect',
          modifier: 'code_volume_kloc',
          grouping: { kind: 'levels' },
        },
      },
      {
        target: {
          kind: 'conditional-average-treatment-effect',
          modifier: 'team_experience',
          grouping: { kind: 'quantiles', bins: 1 },
        },
      },
      { target: { kind: 'average-treatment-effect', cutoff: 2 } },
      { target: { kind: 'local-cutoff-effect', running: 'team_experience', cutoff: Infinity } },
    ].map((change) => proposeStudy(workflow, { ...base, ...change }).ok)
    const targets = [
      ...[
        'average-treatment-effect',
        'average-treatment-effect-on-treated',
        'average-treatment-effect-on-controls',
        'overlap-weighted-average-treatment-effect',
        'average-partial-effect',
        'variance-weighted-average-partial-effect',
      ].map((kind) => ({ kind })),
      {
        kind: 'conditional-average-treatment-effect',
        modifier: 'team_experience',
        grouping: { kind: 'quantiles', bins: 3 },
      },
      { kind: 'conditional-average-treatment-effect-per-row', modifiers: ['team_experience'] },
      { kind: 'conditional-partial-effect-per-row', modifiers: [] },
      { kind: 'local-cutoff-effect', running: 'team_experience', cutoff: 0 },
    ].map((target) => proposeStudy(workflow, { ...base, target }).ok)
    return {
      unchanged,
      invalid,
      targets,
      blankRationales: [
        applied.studyDraft.consistencyRationale,
        applied.studyDraft.noInterferenceRationale,
      ],
      savedStudiesUnchanged: JSON.stringify(applied.studies) === JSON.stringify(workflow.studies),
      savedIdentificationsUnchanged:
        JSON.stringify(applied.identifications) === JSON.stringify(workflow.identifications),
      staleDraft: studyProposalEvent(
        {
          ...workflow,
          studyDraft: { ...workflow.studyDraft, consistencyRationale: 'Manual edit' },
        },
        proposed.value,
      ).ok,
      staleData: studyProposalEvent(
        { ...workflow, prepared: { ...workflow.prepared, id: 'new-version' } },
        proposed.value,
      ).ok,
      staleGraph: studyProposalEvent(
        {
          ...workflow,
          dagDocuments: [{ ...document, current: { ...document.current, id: 'new-revision' } }],
        },
        proposed.value,
      ).ok,
    }
  })
  expect(result).toMatchObject({
    unchanged: true,
    blankRationales: ['', ''],
    savedStudiesUnchanged: true,
    savedIdentificationsUnchanged: true,
    staleDraft: false,
    staleData: false,
    staleGraph: false,
  })
  expect(result.invalid).toEqual(Array(8).fill(false))
  expect(result.targets).toEqual(Array(10).fill(true))
})

test('reviewed study draft reaches identification and estimation through existing controls', async ({
  page,
  isMobile,
}) => {
  test.setTimeout(120000)
  let requests = 0
  await page.route('https://openrouter.ai/api/v1/chat/completions', (route) => {
    requests++
    const code =
      requests === 1
        ? [
            'p = await tool("project")',
            'print(await tool("study_schema"))',
            'print(await tool("propose_study", {"document":p["graphs"][0]["id"],"treatment":"ai_usage","outcome":"bugs_per_kloc","target":{"kind":"average-treatment-effect"},"assignment":{"kind":"observed-choice","description":"Users choose their AI usage intensity; review this assignment description."},"consistencyRationale":None,"noInterferenceRationale":None,"summary":"Study AI usage and defect rates.","checks":["Confirm the assignment description.","Review whether consistency and no interference are credible."]}))',
            'await tool("finish", {"answer":"Review the draft before applying it. No study has been identified.","evidence":["project-summary"]})',
          ].join('\n')
        : requests === 4
          ? [
              'p = await tool("project")',
              'choice = p["studySelection"]["choices"][-1]',
              'print(await tool("propose_study_selection", {"identification":choice["identification"],"reason":"Select the newly identified study, not the earlier one."}))',
              'await tool("finish", {"answer":"Review the study selection. No selection or run has happened yet.","evidence":["project-summary"]})',
            ].join('\n')
          : [
              'p = await tool("project")',
              requests === 2
                ? 'subject = p["runs"]["identification"]["study"]["subject"]'
                : 's = p["studies"][-1]\nassert s["adjustment"]["kind"] == "o-set"\nr = await tool("study", {"id":s["id"]})\nassert r["kind"] == "study-record" and r["study"] == s["id"]\nassert r["identifications"][0]["adjustment"]["kind"] == "o-set"\nsubject = {"kind":"saved-study","study":s["id"],"treatment":s["treatment"]["name"],"outcome":s["outcome"]["name"]}',
              `result = await tool("run_analysis", {"key":"${requests === 2 ? 'identification' : 'estimation'}","subject":subject,"reason":"Run the new study, not the earlier saved study."})`,
              'print(result)',
              'await tool("finish", {"answer":result["kind"] + ": " + result.get("runId", result.get("reason", "")),"evidence":["project-summary"]})',
            ].join('\n')
    return route.fulfill(
      openRouterStream({ choices: [{ message: { content: '```python\n' + code + '\n```' } }] }),
    )
  })
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open AI usage intensity', exact: true }).click()
  const chapters = page.getByRole('navigation', { name: 'Workspace sections' })
  if (isMobile) await page.getByRole('button', { name: 'Expand section list', exact: true }).click()
  await chapters.getByRole('button', { name: /Study design/ }).click()
  const assignment = page.getByRole('textbox', { name: 'Assignment sentence', exact: true })
  const before = await assignment.inputValue()
  await page.getByRole('button', { name: 'Open the assistant', exact: true }).first().click()
  await page.getByLabel('OpenRouter API key').fill('test-key')
  await page.getByLabel('OpenRouter model ID').fill('test/model')
  await page.getByRole('button', { name: 'Privacy', exact: true }).click()
  await page.getByRole('button', { name: 'I agree', exact: true }).click()
  await page.getByRole('button', { name: 'Use these settings', exact: true }).click()
  const ask = async (question: string) => {
    await page.getByLabel('Ask a question').fill(question)
    await page.getByRole('button', { name: 'Send', exact: true }).click()
  }
  await ask(
    'Propose the average effect of AI usage on defects; users choose their intensity. Leave the assumption rationales blank.',
  )
  const proposal = page.getByRole('region', { name: 'Study proposal' })
  await expect(proposal).toBeVisible({ timeout: 30000 })
  await expect(page.getByRole('region', { name: 'Assistant answer' })).toBeVisible()
  await expect(assignment).toHaveValue(before)
  await proposal.getByRole('button', { name: 'Apply study draft', exact: true }).click()
  await expect(proposal).toContainText('Draft applied.')
  await expect(assignment).toHaveValue(
    'Users choose their AI usage intensity; review this assignment description.',
  )
  await expect(
    page.getByRole('textbox', { name: 'Consistency rationale', exact: true }),
  ).toHaveValue('')
  await page.screenshot({ path: '/tmp/hirmos-study-proposal.png' })
  await ask('Identify the current study; leave adjustment selection to me.')
  const approval = page.getByRole('region', { name: 'Analysis approval' })
  await expect(approval.last()).toBeVisible({ timeout: 30000 })
  await approval.last().getByRole('button', { name: 'Run and share summary', exact: true }).click()
  await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
    'review-required',
    { timeout: 30000 },
  )
  await page.getByRole('radio', { name: /^Recommended O-set/ }).check()
  await page.getByRole('button', { name: 'Record this set', exact: true }).click()
  await expect(page.getByRole('heading', { name: 'Choose a valid adjustment set' })).toHaveCount(0)
  if (isMobile) await page.getByRole('button', { name: 'Expand section list', exact: true }).click()
  await chapters.getByRole('button', { name: /Estimation/ }).click()
  await ask('Run the newly saved study, not the earlier saved study.')
  await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
    'refused:',
    { timeout: 30000 },
  )
  await expect(approval).toHaveCount(1)
  await ask('Propose selecting the newly identified study in Estimation.')
  const selection = page.getByRole('region', { name: 'Study selection proposal' })
  await expect(selection).toBeVisible({ timeout: 30000 })
  await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
    'No selection or run',
  )
  await expect(approval).toHaveCount(1)
  await selection.screenshot({ path: '/tmp/hirmos-study-selection.png' })
  await selection.getByRole('button', { name: 'Select study', exact: true }).click()
  await expect(selection).toContainText('Study selected. No analysis was run.')
  await expect(approval).toHaveCount(1)
  await expect(page.getByRole('button', { name: /^Run adjusted linear regression/ })).toBeEnabled({
    timeout: 30000,
  })
  await ask('Estimate the current study after I approve the run.')
  await expect(
    approval.last().getByRole('button', { name: 'Run and share summary', exact: true }),
  ).toBeVisible({ timeout: 30000 })
  await expect(approval.last()).toContainText(
    'Average treatment effect of ai_usage on bugs_per_kloc',
  )
  await expect(approval.last()).toContainText('Identified adjustment set')
  await approval.last().screenshot({ path: '/tmp/hirmos-study-approval.png' })
  await approval.last().getByRole('button', { name: 'Run and share summary', exact: true }).click()
  await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
    'completed:',
    { timeout: 30000 },
  )
  await page.screenshot({ path: '/tmp/hirmos-study-estimation.png' })
})
