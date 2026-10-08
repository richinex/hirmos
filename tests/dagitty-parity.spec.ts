import { expect, test, type Locator, type Page } from '@playwright/test'
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { expectVerdict } from './dagitty-verdict'

const fixtureDir = fileURLToPath(new URL('fixtures/dagitty/', import.meta.url))

/**
 * dagitty's example DAGs, rebuilt through the DAG Workspace's own controls, then Hirmos's back-door
 * verdict checked against dagitty's: whether a back-door path is open, and whether the adjustment set
 * Hirmos names is one dagitty accepts. Fixtures come from tests/fixtures/dagitty/build.mjs.
 */

interface Fixture {
  readonly label: string
  readonly slug: string
  readonly exposure: string
  readonly outcome: string
  readonly nodes: readonly { readonly name: string; readonly kind: 'observed' | 'latent' }[]
  readonly edges: readonly { readonly from: string; readonly to: string }[]
  readonly expected: {
    readonly msas: readonly (readonly string[])[]
    readonly backdoorOpen: boolean
    readonly canonical: readonly string[]
    readonly canonicalValid: boolean
  }
}

const fixtures: readonly Fixture[] = JSON.parse(readFileSync(`${fixtureDir}examples.json`, 'utf8'))
const heavy = process.env.DAGITTY_HEAVY === '1'
const selected = fixtures.filter((fixture) => heavy || fixture.edges.length <= 24)

const choose = async (trigger: Locator, label: string) => {
  await trigger.click()
  await trigger
    .page()
    .getByRole('listbox')
    .getByRole('option', { name: label, exact: true })
    .click()
}

const prepare = async (page: Page, fixture: Fixture) => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill(`dagitty · ${fixture.label}`)
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type="file"]').setInputFiles(`${fixtureDir}${fixture.slug}.csv`)
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await page.getByRole('radio', { name: /Independent observations/ }).click()
  for (const node of fixture.nodes) {
    if (node.kind === 'observed')
      await page.getByRole('checkbox', { name: node.name, exact: true }).check()
  }
  await page.getByRole('button', { name: /Create prepared/ }).click()
  await expect(page.getByRole('status').filter({ hasText: /^Cross-section, / })).toBeVisible({
    timeout: 30_000,
  })
}

const buildDag = async (page: Page, fixture: Fixture) => {
  await page.getByRole('button', { name: /Build a DAG/ }).click()
  await page.getByRole('button', { name: /Substantive knowledge/ }).click()
  await page.getByLabel('DAG name').fill(fixture.label)
  await page.getByRole('button', { name: /Create DAG/ }).click()
  for (const node of fixture.nodes) {
    if (node.kind !== 'latent') continue
    await page.getByRole('button', { name: /Unmeasured variable/ }).click()
    await page.getByLabel(/Unmeasured variable name/).fill(node.name)
    await page.getByRole('button', { name: /Add to DAG/ }).click()
  }
  const ledger = page.getByRole('table', { name: 'Arrows' })
  for (const edge of fixture.edges) {
    await choose(page.getByLabel('Proposed cause'), edge.from)
    await choose(page.getByLabel('Proposed effect'), edge.to)
    await page.getByLabel(/Rationale|Substantive basis/).fill(`dagitty example: ${fixture.label}`)
    await page.getByRole('button', { name: /Add the arrow|Add edge/ }).click()
    await expect(ledger.getByText(`${edge.from} → ${edge.to}`, { exact: true })).toBeVisible()
  }
  await choose(page.getByLabel('Treatment'), fixture.exposure)
  await choose(page.getByLabel('Outcome'), fixture.outcome)
}

for (const fixture of selected) {
  test(`dagitty parity: ${fixture.label}`, async ({ page }, testInfo) => {
    test.skip(
      testInfo.project.name !== 'chromium',
      'The numerical parity corpus runs once; mobile layout has separate workflow tests.',
    )
    test.setTimeout(120_000 + fixture.edges.length * 8_000)
    await prepare(page, fixture)
    await buildDag(page, fixture)
    if (fixture.slug === 'small-model-with-mediator') {
      await page.getByRole('button', { name: 'Run checks' }).click()
      await expect(page.getByText('Conditional-independence results', { exact: true })).toBeVisible(
        { timeout: 60_000 },
      )
      await page.getByText('Conditional-independence results', { exact: true }).click()
      await expect(page.getByRole('columnheader', { name: 'Holm p' })).toBeVisible()
      await expect(page.getByText('Relabeled-graph comparison', { exact: true })).toBeVisible()
    }
    const adjustment = page.getByLabel('Adjustment')
    await expect(adjustment).toBeVisible()
    // Evidence for the side-by-side report: Hirmos's canvas and its verdict, beside dagitty's drawing and verdict.
    const out = 'test-results/dagitty-parity'
    mkdirSync(out, { recursive: true })
    await page.getByRole('button', { name: /Tidy graph/ }).click()
    await page.waitForTimeout(400)
    await page.getByLabel('Causal DAG editor').screenshot({ path: `${out}/${fixture.slug}.png` })
    writeFileSync(
      `${out}/${fixture.slug}.json`,
      JSON.stringify({ verdict: (await adjustment.innerText()).replace(/\n+/g, ' ').trim() }),
    )
    if (!(await expectVerdict(adjustment, fixture.expected))) return
    expect(
      fixture.expected.canonicalValid,
      'dagitty must accept the canonical set Hirmos names',
    ).toBe(true)

    if (fixture.slug === 'extended-confounding-triangle') {
      await page.getByRole('button', { name: 'Use for study' }).click()
      await page.getByRole('radio', { name: 'Observed choice' }).click()
      await page
        .getByLabel('Assignment sentence')
        .fill('Treatment arose from observed unit characteristics.')
      await page.getByRole('button', { name: 'Identify the effect' }).click()

      await expect(
        page.getByRole('heading', { name: 'Choose a valid adjustment set' }),
      ).toBeVisible()
      await expect(page.getByRole('radio', { name: /^Minimal set 1\s*A, Z$/ })).toBeVisible()
      await expect(page.getByRole('radio', { name: /^Minimal set 2\s*B, Z$/ })).toBeVisible()
      await expect(page.getByRole('radio', { name: /^Canonical set\s*A, B, Z$/ })).toBeVisible()

      await page.getByRole('radio', { name: /^Minimal set 1\s*A, Z$/ }).check()
      await page.getByRole('button', { name: 'Record this set' }).click()
      const canvas = page.getByTestId('canvas')
      await expect(canvas.getByText('Minimal adjustment set 1', { exact: false })).toBeVisible()
      await expect(canvas.getByText('Canonical set: A, B, Z.', { exact: true })).toBeVisible()
    }
  })
}

test('O-set is recorded as a supplied set and survives project reload', async ({ page }, info) => {
  test.skip(
    info.project.name !== 'chromium',
    'Parity setup is desktop-only; the bite-check test covers the O-set chooser on mobile.',
  )
  test.setTimeout(120000)
  const fixture = fixtures.find((f) => f.slug === 'extended-confounding-triangle')!
  await prepare(page, fixture)
  await buildDag(page, fixture)
  await page.getByRole('button', { name: 'Use for study' }).click()
  await page.getByRole('radio', { name: 'Observed choice' }).click()
  await page
    .getByLabel('Assignment sentence')
    .fill('Treatment arose from observed unit characteristics.')
  await page.getByRole('button', { name: 'Identify the effect' }).click()
  const choose = page.getByRole('radio', { name: /^Recommended O-set/ })
  await expect(choose).toBeChecked()
  await choose.scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath('o-set-choice.png') })
  await page.getByRole('button', { name: 'Record this set' }).click()
  await expect(
    page.getByText('Recommended adjustment set (O-set)', { exact: false }).first(),
  ).toBeVisible()
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open dagitty · ' + fixture.label, exact: true }).click()
  const needs = page.getByRole('heading', { name: 'Choose the data file again', exact: true })
  await expect(needs.or(page.locator('#data-profile-title'))).toBeVisible()
  if (await needs.isVisible())
    await page.locator('input[type=file]').setInputFiles(fixtureDir + fixture.slug + '.csv')
  await page.getByRole('button', { name: /^Study design/ }).click()
  await expect(
    page.getByText('Recommended adjustment set (O-set)', { exact: false }).first(),
  ).toBeVisible()
})
