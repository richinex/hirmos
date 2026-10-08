import { expect, test, type Page } from '@playwright/test'

test('rebuilding a saved delta graph selects it without adding another entry', async ({ page }, info) => {
  test.setTimeout(90_000)
  const mobile = info.project.name === 'mobile-chromium'
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open Severity, dose and recovery', exact: true }).click()
  await page.locator('#data-profile-title').waitFor({ timeout: 60_000 })
  await enter(page, mobile)
  await select(page, 'Intervention variable 1', 'dose')
  await page.getByRole('spinbutton', { name: 'Intervention value 1' }).fill('0')
  await page.getByRole('textbox', { name: 'SWIG assumptions and rationale' }).fill('Algebraic graph-selection check.')
  const configureDelta = async () => {
    await page.getByRole('radio', { name: 'Δ-SWIG', exact: true }).click()
    await select(page, 'Earlier outcome', 'dose')
    await select(page, 'Later outcome', 'recovery')
    for (const title of ['Earlier outcome', 'Later outcome']) {
      const group = page.getByRole('group', { name: title, exact: true })
      await group.getByRole('button', { name: 'Add additive term' }).click()
      await page.getByRole('textbox', { name: `${title} term 1`, exact: true }).fill('shared severity mechanism')
      await group.getByRole('checkbox', { name: 'severity', exact: true }).check()
    }
  }
  const graph = page.getByLabel('Read-only SWIG graph', { exact: true })
  const construct = async (nodes: number) => {
    await page.getByRole('button', { name: 'Construct graph', exact: true }).click()
    await expect(page.getByRole('heading', { name: 'Separation checks' })).toBeVisible()
    await closeInspector(page, mobile)
    await expect(graph.locator('.react-flow__node')).toHaveCount(nodes)
  }
  await configureDelta()
  await construct(5)
  await inspector(page, mobile)
  await page.getByRole('radio', { name: 'New analysis', exact: true }).click()
  await page.getByRole('radio', { name: 'SWIG', exact: true }).click()
  await construct(4)
  await inspector(page, mobile)
  await page.getByRole('radio', { name: 'New analysis', exact: true }).click()
  await configureDelta()
  await construct(5)
  await expect(graph.locator('.react-flow__node').filter({ hasText: '−' })).toHaveCount(1)
  if (mobile) await page.getByRole('button', { name: 'Saved graphs (2)', exact: true }).click()
  await expect(page.getByText('Saved graphs (2)', { exact: true })).toHaveCount(1)
  if (mobile) await page.keyboard.press('Escape')
  await page.screenshot({ path: info.outputPath('rebuilt-delta.png') })
})

test('shared cancellation saves nothing, while navigation preserves a completed run', async ({
  page,
}, info) => {
  test.setTimeout(90_000)
  const mobile = info.project.name === 'mobile-chromium'
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open Severity, dose and recovery', exact: true }).click()
  await page.locator('#data-profile-title').waitFor({ timeout: 60_000 })
  await enter(page, mobile)
  await select(page, 'Intervention variable 1', 'dose')
  await page.getByRole('spinbutton', { name: 'Intervention value 1' }).fill('0')
  await page
    .getByRole('textbox', { name: 'SWIG assumptions and rationale' })
    .fill('Preserve the recorded common cause.')
  // Hold only this command to exercise the actual shared cancellation owner deterministically.
  await page.evaluate(() => {
    const post = Worker.prototype.postMessage
    Worker.prototype.postMessage = function (message, options) {
      const serialization = Array.isArray(options) ? { transfer: options } : options
      if (message?.kind === 'swig-analysis')
        Reflect.set(window, 'releaseSwig', () => post.call(this, message, serialization))
      else post.call(this, message, serialization)
    }
    Reflect.set(window, 'restoreSwigPost', () => {
      Worker.prototype.postMessage = post
    })
  })
  await page.getByRole('button', { name: 'Construct graph', exact: true }).click()
  await page.getByRole('button', { name: 'Cancel run', exact: true }).click()
  await expect(page.getByText('The analysis was cancelled.', { exact: true })).toBeVisible()
  await expect(page.getByRole('heading', { name: 'Separation checks' })).toHaveCount(0)
  await page.getByRole('button', { name: 'Construct graph', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Cancel run', exact: true })).toBeVisible()
  await closeInspector(page, mobile)
  await page.getByRole('button', { name: 'Return to source DAG', exact: true }).click()
  await page.evaluate(() => {
    Reflect.get(window, 'releaseSwig')()
    Reflect.get(window, 'restoreSwigPost')()
  })
  await inspector(page, mobile)
  await page.getByRole('radio', { name: 'SWIG', exact: true }).click()
  await inspector(page, mobile)
  await expect(page.getByRole('heading', { name: 'Separation checks' })).toBeVisible()
  await page.getByRole('radio', { name: 'New analysis', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Construct graph', exact: true })).toBeEnabled()
})

async function select(page: Page, label: string, option: string) {
  await page.getByRole('combobox', { name: label, exact: true }).click()
  await page.getByRole('option', { name: option, exact: true }).click()
}
async function inspector(page: Page, mobile: boolean) {
  if (mobile && !(await page.getByRole('dialog', { name: 'Inspector' }).isVisible()))
    await page.getByRole('button', { name: 'Inspector', exact: true }).click()
}
async function closeInspector(page: Page, mobile: boolean) {
  if (mobile) {
    await page.keyboard.press('Escape')
    await expect(page.getByRole('dialog', { name: 'Inspector' })).toBeHidden()
  }
}
async function enter(page: Page, mobile: boolean) {
  if (mobile) await page.getByRole('button', { name: 'Expand section list' }).click()
  await page
    .getByRole('navigation', { name: 'Workspace sections' })
    .getByRole('button', { name: /DAG workspace/ })
    .click()
  await inspector(page, mobile)
  await page.getByRole('radio', { name: 'SWIG', exact: true }).click()
  await inspector(page, mobile)
}
test('SWIG UI builds, checks separation, restores and preserves the source DAG', async ({
  page,
}, info) => {
  test.setTimeout(120_000)
  const mobile = info.project.name === 'mobile-chromium'
  const errors: string[] = []
  page.on('pageerror', (error) => errors.push(error.message))
  page.on('console', (message) => {
    if (message.text().includes('new nodeTypes or edgeTypes object')) errors.push(message.text())
  })
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open Severity, dose and recovery', exact: true }).click()
  await page.locator('#data-profile-title').waitFor({ timeout: 60_000 })
  await enter(page, mobile)
  await select(page, 'Intervention variable 1', 'dose')
  await page.getByRole('spinbutton', { name: 'Intervention value 1' }).fill('0')
  await page
    .getByRole('textbox', { name: 'SWIG assumptions and rationale' })
    .fill(
      'Severity causes both dose and recovery. Set dose to zero while retaining its natural assignment.',
    )
  await page.getByRole('button', { name: 'Construct graph', exact: true }).click()
  await expect(page.getByRole('heading', { name: 'Separation checks' })).toBeVisible()
  await select(page, 'First separation variable', 'dose')
  await select(page, 'Second separation variable', 'recovery (dose = 0)')
  await page.getByRole('button', { name: 'Check separation', exact: true }).click()
  await expect(page.getByTestId('swig-conclusion')).toContainText(
    'd-connected without conditioning',
  )
  // The query controls keep their values, so the next check only adds the conditioning variable.
  await page
    .getByRole('group', { name: 'Conditioning variables' })
    .getByRole('checkbox', { name: 'severity', exact: true })
    .check()
  await page.getByRole('button', { name: 'Check separation', exact: true }).click()
  await expect(page.getByTestId('swig-conclusion')).toContainText('d-separated given severity')
  await closeInspector(page, mobile)
  const graph = page.getByLabel('Read-only SWIG graph', { exact: true })
  await expect(graph.locator('.react-flow__node')).toHaveCount(4)
  await expect(graph.locator('.react-flow__edge')).toHaveCount(3)
  await expect(graph.locator('.react-flow__node.draggable')).toHaveCount(0)
  for (const theme of ['light', 'dark']) {
    await page.evaluate((t) => document.documentElement.setAttribute('data-theme', t), theme)
    await page.screenshot({ path: info.outputPath(`swig-${theme}.png`) })
    await inspector(page, mobile)
    await page.getByRole('radio', { name: 'New analysis', exact: true }).click()
    await page.getByRole('combobox', { name: 'Intervention variable 1' }).scrollIntoViewIfNeeded()
    await page.screenshot({ path: info.outputPath(`swig-inspector-${theme}.png`) })
    await closeInspector(page, mobile)
  }
  // Wait for the normal debounced save, not a fixed sleep or a test-side write.
  await expect
    .poll(() =>
      page.evaluate(async () => {
        const store = await import(new URL('/src/data/projectStore.ts', location.href).href)
        const saved = await store.loadProject('1b9e6c3d-8a52-4d17-b3e4-6c2f9a5d1e78')
        return saved.ok ? saved.value.swigAnalyses.length : -1
      }),
    )
    .toBe(3)
  // A reload reopens the project from its URL, on the DAG workspace it was showing.
  await page.reload()
  await page.getByRole('heading', { name: 'DAG workspace', exact: true }).waitFor({ timeout: 60_000 })
  await enter(page, mobile)
  await expect(page.getByTestId('swig-conclusion')).toContainText('d-separated given severity')
  await closeInspector(page, mobile)
  await page.getByRole('button', { name: 'Return to source DAG', exact: true }).click()
  await expect(
    page.getByLabel('Causal DAG editor', { exact: true }).locator('.react-flow__node'),
  ).toHaveCount(3)
  expect(errors).toEqual([])
})

test('delta controls require explicit mechanisms and display cancellation', async ({
  page,
}, info) => {
  test.setTimeout(120_000)
  const mobile = info.project.name === 'mobile-chromium'
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open Severity, dose and recovery', exact: true }).click()
  await page.locator('#data-profile-title').waitFor({ timeout: 60_000 })
  await enter(page, mobile)
  await select(page, 'Intervention variable 1', 'dose')
  await page.getByRole('spinbutton', { name: 'Intervention value 1' }).fill('0')
  await page.getByRole('radio', { name: 'Δ-SWIG', exact: true }).click()
  await select(page, 'Earlier outcome', 'dose')
  await select(page, 'Later outcome', 'recovery')
  await page
    .getByRole('textbox', { name: 'SWIG assumptions and rationale' })
    .fill(
      'Algebraic UI test only: both equations contain the same function of severity. These variable names are not a DiD design.',
    )
  await page.getByRole('button', { name: 'Construct graph', exact: true }).click()
  await expect(page.getByRole('alert')).toBeVisible()
  for (const title of ['Earlier outcome', 'Later outcome']) {
    const group = page.getByRole('group', { name: title, exact: true })
    await group.getByRole('button', { name: 'Add additive term' }).click()
    await page
      .getByRole('textbox', { name: `${title} term 1`, exact: true })
      .fill('shared severity mechanism')
    await group.getByRole('checkbox', { name: 'severity', exact: true }).check()
  }
  await page.getByRole('button', { name: 'Construct graph', exact: true }).click()
  await expect(page.getByRole('heading', { name: 'Separation checks' })).toBeVisible()
  await expect(
    page.getByText('Cancelled additive terms: shared severity mechanism.', { exact: true }),
  ).toBeVisible()
  await closeInspector(page, mobile)
  await expect(
    page.getByLabel('Read-only SWIG graph', { exact: true }).locator('.react-flow__node'),
  ).toHaveCount(5)
  await page.screenshot({ path: info.outputPath('delta-swig.png') })
  await inspector(page, mobile)
  await page.getByRole('radio', { name: 'New analysis', exact: true }).click()
  await page.getByRole('button', { name: 'Add difference', exact: true }).click()
  await select(page, 'Earlier outcome 2', 'recovery')
  await select(page, 'Later outcome 2', 'dose')
  for (const title of ['Earlier outcome 2', 'Later outcome 2']) {
    const group = page.getByRole('group', { name: title, exact: true })
    await group.getByRole('button', { name: 'Add additive term' }).click()
    await page
      .getByRole('textbox', { name: title + ' term 1', exact: true })
      .fill('shared severity mechanism')
    await group.getByRole('checkbox', { name: 'severity', exact: true }).check()
  }
  await page.getByRole('button', { name: 'Construct graph', exact: true }).click()
  await expect(
    page.getByText('Cancelled additive terms: shared severity mechanism.', { exact: true }),
  ).toHaveCount(2)
  await closeInspector(page, mobile)
  await expect(
    page.getByLabel('Read-only SWIG graph', { exact: true }).locator('.react-flow__node'),
  ).toHaveCount(6)
})
