import { expect, test, type Locator, type Page } from '@playwright/test'
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { choose as pick, fixture, prepare as prepareDataset } from './examples/support'
import { expectVerdict } from './dagitty-verdict'

const fixtureDir = fileURLToPath(new URL('fixtures/dagitty/', import.meta.url))

/**
 * dagitty's example DAGs entered by pasting their text, then judged the way dagitty-parity.spec.ts
 * judges the same graphs built arrow by arrow: same arrows in the ledger, same roles bound, and
 * the same adjustment verdict. A pasted graph and a drawn one must be the same graph.
 */

interface Fixture {
  readonly label: string
  readonly slug: string
  readonly dot: string
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

const chosen = async (trigger: Locator): Promise<string> => (await trigger.innerText()).replace(/\s*expand_more\s*$/, '').trim()

const prepare = async (page: Page, fixture: Fixture) => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill(`paste · ${fixture.label}`)
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type="file"]').setInputFiles(`${fixtureDir}${fixture.slug}.csv`)
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await page.getByRole('radio', { name: /Independent observations/ }).click()
  for (const node of fixture.nodes) {
    if (node.kind === 'observed') await page.getByRole('checkbox', { name: node.name, exact: true }).check()
  }
  await page.getByRole('button', { name: /Create prepared/ }).click()
  await expect(page.getByRole('status').filter({ hasText: /^Cross-section, / })).toBeVisible({ timeout: 30_000 })
  await page.getByRole('button', { name: /Build a DAG/ }).click()
  await page.getByRole('button', { name: /Substantive knowledge/ }).click()
  await page.getByLabel('DAG name').fill(fixture.label)
  await page.getByRole('button', { name: /Create DAG/ }).click()
}

const paste = async (page: Page, text: string) => {
  await page.getByRole('button', { name: 'From text' }).click()
  await page.getByRole('textbox', { name: 'Graph text' }).fill(text)
  await page.getByRole('button', { name: 'Convert to DAG' }).click()
}

for (const fixture of selected) {
  test(`pasted dagitty graph: ${fixture.label}`, async ({ page }, testInfo) => {
    test.skip(testInfo.project.name !== 'chromium', 'The parity corpus runs once; mobile layout has separate workflow tests.')
    test.setTimeout(90_000 + fixture.edges.length * 2_000)
    await prepare(page, fixture)
    await paste(page, fixture.dot)

    // The form closes on success, and every arrow of the fixture is in the ledger, once.
    await expect(page.getByRole('textbox', { name: 'Graph text' })).toHaveCount(0)
    const ledger = page.getByRole('table', { name: 'Arrows' })
    for (const edge of fixture.edges) {
      await expect(ledger.getByText(`${edge.from} → ${edge.to}`, { exact: true })).toHaveCount(1)
    }
    await expect(page.getByText(`${fixture.edges.length} arrows`, { exact: true })).toBeVisible()
    await expect(page.getByText(`Needs rationale ${fixture.edges.length}`, { exact: false })).toBeVisible()

    // [exposure] and [outcome] bind the study without another click.
    const binding = page.getByRole('group', { name: 'Study binding' })
    expect(await chosen(binding.getByRole('combobox', { name: 'Treatment' }))).toBe(fixture.exposure)
    expect(await chosen(binding.getByRole('combobox', { name: 'Outcome' }))).toBe(fixture.outcome)

    // The same verdict the click-built graph receives in dagitty-parity.spec.ts.
    const adjustment = page.getByLabel('Adjustment')
    await expect(adjustment).toBeVisible()
    await expectVerdict(adjustment, fixture.expected)
  })
}

test('text the graph cannot take is refused by name, and nothing is added', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'One layout is enough for a refusal.')
  const fixture = fixtures.find((entry) => entry.slug === 'the-m-bias-graph')!
  await prepare(page, fixture)

  await paste(page, 'dag { morale -> E }')
  const alert = page.getByRole('alert').filter({ hasText: 'morale' })
  await expect(alert).toContainText('morale is not a column of the prepared data')
  await expect(page.getByText('0 arrows', { exact: true })).toBeVisible()

  await page.getByRole('textbox', { name: 'Graph text' }).fill('dag { E -> D D -> E }')
  await page.getByRole('button', { name: 'Convert to DAG' }).click()
  await expect(page.getByRole('alert').filter({ hasText: 'D → E' })).toContainText('directed cycle')
  await expect(page.getByText('0 arrows', { exact: true })).toBeVisible()
})

/**
 * Two shipped examples cover what dagitty's corpus never uses: an unmeasured variable declared
 * outright, and arrows across time. Each is entered as text and must match what the editor draws,
 * with its refusal twin: a lag on cross-section data, and a same-period self-loop.
 */

const openExample = async (page: Page, name: string, csv: string) => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill(name)
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type="file"]').setInputFiles(fixture(csv))
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await expect(page.getByText('Choose the observation structure')).toBeVisible({ timeout: 90_000 })
}

const startDag = async (page: Page, name: string) => {
  await page.getByRole('button', { name: /Build a DAG/ }).click()
  await page.getByRole('button', { name: /Substantive knowledge/ }).click()
  await page.getByLabel('DAG name').fill(name)
  await page.getByRole('button', { name: /Create DAG/ }).click()
}

test('an unmeasured variable declared in the text is drawn as one', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'One layout is enough.')
  await openExample(page, 'paste · GPS and memory', 'gps-memory.csv')
  await prepareDataset(page, { structure: 'cross-section', columns: 'all' })
  await startDag(page, 'GPS use and spatial memory')

  await paste(page, 'dag { X [exposure] Y [outcome] U [latent] X -> Z Z -> Y U -> X U -> Y }')
  await expect(page.getByRole('textbox', { name: 'Graph text' })).toHaveCount(0)
  const ledger = page.getByRole('table', { name: 'Arrows' })
  for (const arrow of ['X → Z', 'Z → Y', 'U → X', 'U → Y']) {
    await expect(ledger.getByText(arrow, { exact: true })).toHaveCount(1)
  }
  await expect(page.getByText('4 arrows', { exact: true })).toBeVisible()
  await expect(page.getByText('Needs rationale 4', { exact: false })).toBeVisible()
  // U is unmeasured, so it is offered as neither treatment nor outcome, and the study binds X and Y.
  const binding = page.getByRole('group', { name: 'Study binding' })
  expect(await chosen(binding.getByRole('combobox', { name: 'Treatment' }))).toBe('X')
  expect(await chosen(binding.getByRole('combobox', { name: 'Outcome' }))).toBe('Y')
  await binding.getByRole('combobox', { name: 'Treatment' }).click()
  await expect(page.getByRole('listbox').getByRole('option', { name: 'U', exact: true })).toHaveCount(0)
  await page.keyboard.press('Escape')
  // The back-door path runs through U, and U is unmeasured: the front-door example's point, stated by the workspace.
  await expect(page.locator('main').getByText('open through U (unmeasured)', { exact: false })).toBeVisible()

  // A lag has no meaning without time order, and the refusal says which arrow asked for one.
  await paste(page, 'dag { X -> Y [lag=1] }')
  await expect(page.getByRole('alert').filter({ hasText: 'X → Y' })).toBeVisible()
  await expect(page.getByText('4 arrows', { exact: true })).toBeVisible()
})

test('lagged arrows and lagged self-loops read from text as the editor draws them', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'One layout is enough.')
  await openExample(page, 'paste · deploys and incidents', 'deploys-incidents.csv')
  await prepareDataset(page, { structure: 'time series', time: 'week', columns: 'all' })
  await startDag(page, 'Deploys and incidents at a lag')

  await paste(page, 'dag { deploys -> incidents [lag=3] incidents -> incidents [lag=1] deploys -> deploys [lag=1] }')
  await expect(page.getByRole('textbox', { name: 'Graph text' })).toHaveCount(0)
  const ledger = page.getByRole('table', { name: 'Arrows' })
  const row = (arrow: string) => ledger.getByRole('row').filter({ has: page.getByText(arrow, { exact: true }) })
  await expect(row('deploys → incidents')).toHaveCount(1)
  await expect(row('deploys → incidents')).toContainText('3')
  await expect(row('incidents → incidents')).toHaveCount(1)
  await expect(row('incidents → incidents')).toContainText('1')
  await expect(row('deploys → deploys')).toHaveCount(1)
  await expect(page.getByText('3 arrows', { exact: true })).toBeVisible()
  await expect(page.getByText('Needs rationale 3', { exact: false })).toBeVisible()

  // The same arrow at another lag is another arrow; the same arrow at the same lag is a repeat.
  await paste(page, 'dag { deploys -> incidents [lag=1] }')
  await expect(page.getByText('4 arrows', { exact: true })).toBeVisible()
  await paste(page, 'dag { deploys -> incidents [lag=1] }')
  await expect(page.getByRole('alert').filter({ hasText: 'deploys → incidents' })).toBeVisible()
  await expect(page.getByText('4 arrows', { exact: true })).toBeVisible()

  // A variable cannot cause itself in the same period; only across time.
  await page.getByRole('textbox', { name: 'Graph text' }).fill('dag { incidents -> incidents }')
  await page.getByRole('button', { name: 'Convert to DAG' }).click()
  await expect(page.getByRole('alert').filter({ hasText: 'incidents → incidents' })).toBeVisible()
  await expect(page.getByText('4 arrows', { exact: true })).toBeVisible()
})
