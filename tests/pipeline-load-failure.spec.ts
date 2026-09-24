import { expect, test } from '@playwright/test'

const newProject = async (page: import('@playwright/test').Page) => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Pipeline')
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.getByRole('radio', { name: 'Build a pipeline' }).click({ force: true })
}

/** A VPN or firewall blocks the download; the workbench must explain itself rather than sit inert. */
test('a pipeline editor that will not download says so', async ({ page }) => {
  test.setTimeout(120_000)
  let blocked = 0
  await page.route('**/PipelineWorkspace*', (route) => { blocked += 1; return route.abort('failed') })

  await newProject(page)
  await page.getByRole('button', { name: 'Open the editor' }).click()

  expect(blocked, 'the editor chunk was never intercepted').toBeGreaterThan(0)
  await expect(page.getByTestId('editor-problem')).toContainText('could not be downloaded', { timeout: 30_000 })
  await expect(page.getByRole('navigation', { name: 'Workspace chapters' })).toBeVisible()
})

/** Pyodide behind a VPN neither arrives nor fails, so the wait needs its own deadline. */
test('a Python runtime that never arrives fails with an explanation instead of loading forever', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { createPythonRuntime } = await import(new URL('/src/data/pythonRuntime.ts', location.href).href)
    // A worker that acknowledges the start and then goes quiet, as a blocked download does.
    class SilentWorker extends EventTarget {
      onmessage: ((event: MessageEvent<unknown>) => void) | null = null
      onmessageerror: (() => void) | null = null
      postMessage() { queueMicrotask(() => this.onmessage?.({ data: { kind: 'loading', detail: 'Loading Python' } } as MessageEvent<unknown>)) }
      terminate() {}
    }
    const runtime = createPythonRuntime(() => new SilentWorker() as unknown as Worker, 150)
    runtime.warm()
    await new Promise((resolve) => setTimeout(resolve, 60))
    const whileWaiting = runtime.store.getState().runtime
    await new Promise((resolve) => setTimeout(resolve, 400))
    const afterDeadline = runtime.store.getState().runtime
    return { whileWaiting, afterDeadline }
  })
  expect(result.whileWaiting.kind).toBe('loading')
  expect(result.afterDeadline.kind).toBe('failed')
  expect(result.afterDeadline.detail).toContain('VPN')
  expect(result.afterDeadline.detail).toContain('reload the page')
})
