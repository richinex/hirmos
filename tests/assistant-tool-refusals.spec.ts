import { expect, test } from '@playwright/test'

test('Python receives the finish validation reason and can correct the evidence list', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { pythonSandbox } = await import(new URL('/src/assistant/sandbox.ts', location.href).href)
    const { answerSchema } = await import(new URL('/src/assistant/agent.ts', location.href).href)
    const runtime = await pythonSandbox(
      new AbortController().signal,
      async (_name: string, args: unknown) => answerSchema.parse(args),
    )
    try {
      const failed = await runtime.executeReport(
        'await tool("finish", {"answer":"Ready", "evidence":[]})',
      )
      const corrected = await runtime.executeReport(
        'print(await tool("finish", {"answer":"Ready", "evidence":["user-question"]}))',
      )
      return { failed, corrected }
    } finally {
      runtime.close()
    }
  })
  expect(result.failed.exitCode).toBe(1)
  expect(result.failed.stderr).toContain('evidence')
  expect(result.failed.stderr).toContain('>=1')
  expect(result.failed.stderr).not.toContain('Tool request refused.')
  expect(result.corrected.exitCode).toBe(0)
  expect(result.corrected.output).toContain('user-question')
})

test('workspace lookup lists available keys and distinguishes pages without workspace tools', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { readWorkspaceSettings } = await import(
      new URL('/src/assistant/workspaceConfiguration.tsx', location.href).href
    )
    const capabilities = {
      survival: {
        title: 'Survival',
        current: { kind: 'nonparametric' },
        columns: [],
        schema: { type: 'object' },
        describe: () => [{ key: 'kind', label: 'Analysis type', value: 'Kaplan–Meier' }],
      },
    }
    return {
      available: readWorkspaceSettings(capabilities, {}),
      missing: readWorkspaceSettings(capabilities, { key: 'estimation' }),
      empty: readWorkspaceSettings({}, { key: 'estimation' }),
      inherited: readWorkspaceSettings(capabilities, { key: 'toString' }),
      found: readWorkspaceSettings(capabilities, { key: 'survival' }),
    }
  })
  expect(result.available).toEqual([{ key: 'survival', title: 'Survival' }])
  expect(result.missing.reason).toContain('Available keys: survival')
  expect(result.missing.available).toEqual(result.available)
  expect(result.empty.reason).toContain('No workspace settings tools are available on this page')
  expect(result.empty.reason).toContain('separate estimator proposal tools')
  expect(result.inherited.kind).toBe('refused')
  expect(result.found.current).toEqual({ kind: 'nonparametric' })
  expect(result.found.settings).toEqual([
    { key: 'kind', label: 'Analysis type', value: 'Kaplan–Meier' },
  ])
})
