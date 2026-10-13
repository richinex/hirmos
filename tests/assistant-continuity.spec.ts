import { expect, test } from '@playwright/test'
import { ask, connectAssistant } from './assistantTestFlow'

test('accepted-source reads survive editor disposal and discard replaced-source responses', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { createAssistantBridge } = await import(
      new URL('/src/assistant/applicationTools.tsx', location.href).href
    )
    const { profileAcceptedSource } = await import(
      new URL('/src/assistant/sourceProfile.ts', location.href).href
    )
    const { selectSource } = await import(new URL('/src/domain/workflow.ts', location.href).href)
    const bridge = createAssistantBridge()
    const selected = selectSource(
      new File(['period,value\n1,10\n2,20\n3,\n'], 'source.csv', { type: 'text/csv' }),
    )
    if (!selected.ok) throw new Error('Invalid source')
    const request = {
      table: 'source',
      columns: ['period', 'value'],
      checks: [{ kind: 'time', column: 'period', units: [], step: 1 }],
    }
    const signal = AbortSignal.timeout(30000)
    const source = {
      kind: 'source',
      revision: 'accepted',
      inspection: {
        kind: 'source',
        name: 'source.csv',
        tables: ['source'],
        meaning: 'Before preprocessing',
      },
      profile: (request: unknown, signal: AbortSignal) =>
        profileAcceptedSource(selected.value, null, request, signal),
    }
    bridge.publishSourceData(source)
    const close = bridge.publishPreparation({
      kind: 'sql',
      revision: 'editor',
      inspection: { kind: 'sql', tables: ['draft'] },
      profile: () => {
        throw new Error('Disposed editor called')
      },
    })
    close()
    const profiled = await bridge.profile('accepted', request, signal)
    let release!: (value: unknown) => void
    const delayed = {
      ...source,
      revision: 'delayed',
      profile: () =>
        new Promise((resolve) => {
          release = resolve
        }),
    }
    bridge.publishSourceData(delayed)
    const pending = bridge.profile('delayed', request, signal)
    bridge.publishSourceData({ ...source, revision: 'replacement' })
    release(profiled.receipt.profile)
    const stale = await pending
    let mismatch = ''
    try {
      await profileAcceptedSource(selected.value, 'wrong-fingerprint', request, signal)
    } catch (error) {
      mismatch = String(error)
    }
    return { profiled, stale, mismatch }
  })
  expect(result.profiled.kind).toBe('profiled')
  expect(result.profiled.receipt.profile.rows).toBe(3)
  expect(result.profiled.receipt.profile.summaries[0]).toMatchObject({ minimum: '1', maximum: '3' })
  expect(result.profiled.receipt.profile.summaries[1].nulls).toBe(1)
  expect(result.stale.kind).toBe('refused')
  expect(result.mismatch).toContain('source file changed')
})

test('restored failed and stopped questions remain in recent context without carrying approvals', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { saveDisplayTurn, loadDisplayTurns } = await import(
      new URL('/src/assistant/displayHistory.ts', location.href).href
    )
    const { conversationRecords } = await import(
      new URL('/src/assistant/conversation.ts', location.href).href
    )
    const session = crypto.randomUUID()
    const base = { context: 'old page', trace: [] }
    await saveDisplayTurn('project', session, 1, {
      ...base,
      id: 'one',
      question: 'Use date.',
      outcome: { kind: 'answered', answer: { answer: 'Noted', evidence: ['user-question'] } },
    })
    await saveDisplayTurn('project', session, 2, {
      ...base,
      id: 'two',
      question: 'Correction: use period as the time column.',
      outcome: { kind: 'failed', message: 'Timed out' },
    })
    await saveDisplayTurn('project', session, 3, {
      ...base,
      id: 'three',
      question: 'Keep the monthly frequency.',
      outcome: { kind: 'cancelled' },
    })
    return conversationRecords(await loadDisplayTurns(session))
  })
  expect(result.map((item: { question: string }) => item.question)).toEqual([
    'Use date.',
    'Correction: use period as the time column.',
    'Keep the monthly frequency.',
  ])
  expect(result[1].answer).toContain('Check current records')
  expect(result[2].answer).toContain('stopped')
})

test('assistant profiles upload and inspected data, then opens and closes the pipeline without replacing data', async ({
  page,
}, info) => {
  test.setTimeout(120000)
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Data continuity')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.locator('input[type=file]').setInputFiles({
    name: 'measurements.csv',
    mimeType: 'text/csv',
    buffer: Buffer.from('period,value\n1,10\n2,20\n3,30\n'),
  })
  const profile =
    'p = await tool("project")\nr = await tool("profile_table", {"table":"source", "columns":["period"]})\nassert r["kind"] == "profiled", str(r)\nassert r["receipt"]["profile"]["summaries"][0]["maximum"] == "3"\nawait FINAL_VAR("The source has three periods.", ["data-summary"])'
  const propose =
    'p = await tool("project")\ns = await tool("workspace_settings", {"key":"data-editor"})\nr = await tool("propose_workspace", {"key":"data-editor", "configuration":{"editor":"pipeline"}, "reason":"Reopen the source for editing."})\nprint(r)\nawait FINAL_VAR("Review the editor proposal. No data has changed.", ["project-summary"])'
  const edit =
    'p = await tool("project")\nassert p["preparation"]["kind"] == "pipeline", str(p["preparation"])\ng = p["preparation"]["graph"]\ni = next(n for n in g["nodes"] if n["block"]["kind"] == "input")\no = next(n for n in g["nodes"] if n["block"]["kind"] == "output")\ng["edges"] = [{"from":i["id"], "to":o["id"], "port":0}]\nr = await tool("propose_pipeline", {"graph":g, "summary":"Connect the input to the output.", "checks":["Confirm the three input rows remain."]})\nassert r["kind"] != "refused", str(r)\nawait FINAL_VAR("Review the proposed connection.", ["project-summary"])'
  const calls = await connectAssistant(page, [profile, profile, propose, edit, propose, profile])
  await ask(page, 'Use period as the time column. Check its range.')
  await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
    'three periods',
    { timeout: 45000 },
  )
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
  await expect(page.locator('#data-profile-title')).toHaveText('measurements.csv', {
    timeout: 30000,
  })
  const request = page.waitForRequest((request) => request.url().includes('/chat/completions'))
  await ask(page, 'Check that range after inspection.')
  expect(JSON.stringify((await request).postDataJSON())).toContain('Use period as the time column')
  await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
    'three periods',
    { timeout: 45000 },
  )
  await ask(page, 'Open the pipeline so we can edit the source.')
  await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
    'Review the editor',
    { timeout: 30000 },
  )
  await page.getByRole('button', { name: 'Open editor', exact: true }).click()
  await expect(page.getByTestId('block-input-measurements')).toContainText('3 rows, 2 columns', {
    timeout: 30000,
  })
  await page.screenshot({
    path: info.outputPath('assistant-reopened-pipeline.png'),
    fullPage: true,
  })
  await ask(page, 'Connect the input to the output.')
  await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
    'Review the proposed connection',
    { timeout: 30000 },
  )
  await page.getByRole('button', { name: 'Apply and evaluate pipeline', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Use as source', exact: true })).toBeEnabled()
  await page.getByRole('button', { name: 'Use as source', exact: true }).click()
  await page
    .getByRole('alertdialog', { name: 'Replace dataset?' })
    .getByRole('button', { name: 'Replace dataset', exact: true })
    .click()
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Edit data', exact: true })).toBeVisible()
  await ask(page, 'Reopen the pipeline that made this source.')
  await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
    'Review the editor',
    { timeout: 30000 },
  )
  await page.getByRole('button', { name: 'Open editor', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Use as source', exact: true })).toBeEnabled({
    timeout: 30000,
  })
  await page.getByRole('button', { name: 'Cancel editing', exact: true }).click()
  await ask(page, 'Check the accepted source after closing the editor.')
  await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
    'three periods',
    { timeout: 45000 },
  )
  expect(calls()).toBe(6)
})
