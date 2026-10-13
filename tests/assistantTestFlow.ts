import { expect, type Page } from '@playwright/test'
import { openRouterStream } from './openRouterStream'

export async function connectAssistant(page: Page, blocks: readonly string[]) {
  let requests = 0
  await page.route('https://openrouter.ai/api/v1/key', (route) => route.fulfill({ json: { data: { limit: null, limit_remaining: null } } }))
  await page.route('https://openrouter.ai/api/v1/chat/completions', (route) => {
    const code = blocks[requests++] ?? 'raise RuntimeError("Unexpected model retry")'
    return route.fulfill(openRouterStream({ choices: [{ message: { content: '```python\n' + code + '\n```' } }] }))
  })
  await page.getByRole('button', { name: 'Open the assistant', exact: true }).click()
  await page.getByLabel('OpenRouter API key').fill('test-key')
  await page.getByLabel('OpenRouter model ID').fill('test/model')
  await page.getByRole('button', { name: 'Privacy', exact: true }).click()
  await page.getByRole('button', { name: 'I agree', exact: true }).click()
  await page.getByRole('button', { name: 'Use these settings', exact: true }).click()
  return () => requests
}

export async function ask(page: Page, question: string) {
  await page.getByLabel('Ask a question').fill(question)
  await page.getByRole('button', { name: 'Send', exact: true }).click()
}

export async function approveRun(page: Page, question: string, answer = 'Completed') {
  await ask(page, question)
  const approval = page.getByRole('region', { name: 'Analysis approval' }).last()
  await expect(approval).toBeVisible({ timeout: 30000 })
  await approval.getByRole('button', { name: 'Run and share summary', exact: true }).click()
  await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(answer, { timeout: 45000 })
}

export async function snapshot(page: Page, name: string) {
  return page.evaluate(async (name) => {
    const { listProjects, loadProject } = await import(new URL('/src/data/projectStore.ts', location.href).href)
    const header = (await listProjects()).find((h: { name: string }) => h.name === name)
    if (!header) return null
    const saved = await loadProject(header.id)
    return saved.ok ? saved.value : null
  }, name)
}
