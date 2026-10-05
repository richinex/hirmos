import { expect, test, type Page } from '@playwright/test'
import { prepare } from './examples/support'
async function select(page: Page, label: string, value: string) {
  await page.getByRole('combobox', { name: label, exact: true }).click()
  await page.getByRole('option', { name: value, exact: true }).click()
}
async function inspector(page: Page, mobile: boolean) {
  if (mobile && !(await page.getByRole('dialog', { name: 'Inspector' }).isVisible()))
    await page.getByRole('button', { name: 'Inspector', exact: true }).click()
}
async function close(page: Page, mobile: boolean) {
  if (mobile) {
    await page.keyboard.press('Escape')
    await expect(page.getByRole('dialog', { name: 'Inspector' })).toBeHidden()
  }
}
test('qualified DiD accepts an optional rationale and restores its assessment', async ({
  page,
}, info) => {
  test.setTimeout(150000)
  const mobile = info.project.name === 'mobile-chromium'
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('DiD graph integration')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.locator('input[type="file"]').setInputFiles({
    name: 'did.csv',
    mimeType: 'text/csv',
    buffer: Buffer.from('D,Y0,Y1\n0,1,2\n1,2,4\n0,2,3\n1,3,5\n'),
  })
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await prepare(page, { structure: 'cross-section', columns: 'all' })
  await page.getByRole('button', { name: /Build a DAG/ }).click()
  await page.getByRole('button', { name: /Substantive knowledge/ }).click()
  await page.getByLabel('DAG name').fill('Explicit two-period DiD')
  await page.getByRole('button', { name: /Create DAG/ }).click()
  await page.getByRole('button', { name: 'From text', exact: true }).click()
  await page
    .getByRole('textbox', { name: 'Graph text' })
    .fill(
      'dag { U [latent] e0 [latent] e1 [latent] U -> D U -> Y0 U -> Y1 D -> Y1 e0 -> Y0 e1 -> Y1 }',
    )
  await page.getByRole('button', { name: 'Convert to DAG' }).click()
  await inspector(page, mobile)
  await page.getByRole('radio', { name: 'SWIG', exact: true }).click()
  await inspector(page, mobile)
  await page.getByRole('radio', { name: 'DiD adjustment', exact: true }).click()
  for (const [name, role] of [
    ['U', 'Common causes'],
    ['D', 'Treatments'],
    ['Y0', 'Outcomes'],
    ['Y1', 'Outcomes'],
    ['e0', 'Exogenous disturbances'],
    ['e1', 'Exogenous disturbances'],
  ])
    await page
      .getByRole('group', { name: role, exact: true })
      .getByRole('checkbox', { name, exact: true })
      .check()
  await page.getByRole('spinbutton', { name: 'Period for Y1', exact: true }).fill('1')
  await page.getByRole('textbox', { name: 'DiD graph rationale' }).fill('')
  await page.getByRole('button', { name: 'Assess DiD adjustment', exact: true }).click()
  const assessment = page.getByRole('region', { name: 'DiD adjustment assessment' })
  await expect(assessment).toContainText('supports an adjustment family, subject to overlap')
  await expect(assessment).toContainText(/Required controls\s*None/)
  await close(page, mobile)
  await expect(
    page.getByLabel('Read-only SWIG graph', { exact: true }).locator('.react-flow__node'),
  ).toHaveCount(8)
  for (const theme of ['light', 'dark']) {
    await page.evaluate((t) => document.documentElement.setAttribute('data-theme', t), theme)
    await page.screenshot({ path: info.outputPath('did-' + theme + '.png') })
    await inspector(page, mobile)
    await page.getByRole('radio', { name: 'New analysis', exact: true }).click()
    await expect(page.getByRole('textbox', { name: 'DiD graph rationale' })).toHaveValue('')
    await page.getByRole('textbox', { name: 'DiD graph rationale' }).scrollIntoViewIfNeeded()
    await page.screenshot({ path: info.outputPath('did-controls-' + theme + '.png') })
    await close(page, mobile)
  }
  await expect
    .poll(() =>
      page.evaluate(async () => {
        const store = await import(new URL('/src/data/projectStore.ts', location.href).href)
        const projects = await store.listProjects()
        const project = projects.find((p: { name: string }) => p.name === 'DiD graph integration')
        if (!project) return 0
        const saved = await store.loadProject(project.id)
        return saved.ok ? saved.value.swigAnalyses.length : 0
      }),
    )
    .toBe(1)
  await page.reload()
  await page.getByRole('button', { name: 'Open DiD graph integration', exact: true }).click()
  await expect(
    page.getByRole('heading', { name: 'Choose the data file again', exact: true }),
  ).toBeVisible()
  await page.locator('input[type="file"]').setInputFiles({
    name: 'did.csv',
    mimeType: 'text/csv',
    buffer: Buffer.from('D,Y0,Y1\n0,1,2\n1,2,4\n0,2,3\n1,3,5\n'),
  })
  await page.locator('#data-profile-title').waitFor({ timeout: 60000 })
  if (mobile) await page.getByRole('button', { name: 'Expand section list' }).click()
  await page
    .getByRole('navigation', { name: 'Workspace sections' })
    .getByRole('button', { name: /DAG workspace/ })
    .click()
  await inspector(page, mobile)
  await page.getByRole('radio', { name: 'SWIG', exact: true }).click()
  await inspector(page, mobile)
  await expect(assessment).toContainText(/Required controls\s*None/)
  await close(page, mobile)
  // Navigation also preserves the selected saved result.
  await page.getByRole('button', { name: 'Return to source DAG' }).click()
  await inspector(page, mobile)
  await page.getByRole('radio', { name: 'SWIG', exact: true }).click()
  await inspector(page, mobile)
  await expect(assessment).toContainText(/Required controls\s*None/)
  await close(page, mobile)
})
test('temporal expansion requires a boundary assumption and creates period-labelled nodes', async ({
  page,
}, info) => {
  test.setTimeout(90000)
  const mobile = info.project.name === 'mobile-chromium'
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open Severity, dose and recovery', exact: true }).click()
  await page.locator('#data-profile-title').waitFor({ timeout: 60000 })
  if (mobile) await page.getByRole('button', { name: 'Expand section list' }).click()
  await page
    .getByRole('navigation', { name: 'Workspace sections' })
    .getByRole('button', { name: /DAG workspace/ })
    .click()
  await inspector(page, mobile)
  await page.getByRole('radio', { name: 'SWIG', exact: true }).click()
  await inspector(page, mobile)
  await select(page, 'Source graph representation', 'Expand a time-series template')
  await expect(page.getByRole('button', { name: 'Construct graph', exact: true })).toBeDisabled()
  await page.getByRole('checkbox', { name: /I assume the displayed initial-history/ }).check()
  await select(page, 'Intervention variable 1', 'dose [t=1]')
  await page.getByRole('spinbutton', { name: 'Intervention value 1' }).fill('0')
  await page
    .getByRole('textbox', { name: 'SWIG assumptions and rationale' })
    .fill(
      'Each period is an independent copy in this illustrative template. No omitted history causes.',
    )
  await page.getByRole('button', { name: 'Construct graph', exact: true }).click()
  await expect(page.getByRole('heading', { name: 'Separation checks' })).toBeVisible()
  await page.getByText('Recorded specification', { exact: true }).click()
  await expect(
    page.getByText(
      'Time expansion: periods 0 to 1. Initial-history closure was explicitly assumed.',
      { exact: true },
    ),
  ).toBeVisible()
  await close(page, mobile)
  await expect(
    page.getByLabel('Read-only SWIG graph', { exact: true }).locator('.react-flow__node'),
  ).toHaveCount(7)
  await page.screenshot({ path: info.outputPath('temporal-swig.png') })
})
