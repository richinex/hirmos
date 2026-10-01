import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'
import { prepare, createDag } from './examples/support'

const corpus = JSON.parse(readFileSync(new URL('./fixtures/dagitty/examples.json', import.meta.url), 'utf8'))

test('ELK routes the dagitty corpus without crossing cards, in both orientations', async ({ page }) => {
  test.setTimeout(90_000)
  await page.goto('/app')
  const failures = await page.evaluate(async fixtures => {
    const path = '/src/components/dag/elkLayout.ts'
    const { layoutDag } = await import(/* @vite-ignore */ path)
    const failures: string[] = []
    const special = [
      { label: 'Empty graph', nodes: [], edges: [] },
      { label: 'Isolated cards', nodes: [{ name: 'A' }, { name: 'B' }], edges: [] },
      { label: 'Lagged self-loop and parallel arrows', nodes: [{ name: 'A' }, { name: 'B' }], edges: [{ from: 'A', to: 'A' }, { from: 'A', to: 'B' }, { from: 'A', to: 'B' }, { from: 'B', to: 'A' }] },
    ]
    for (const orientation of ['across', 'down']) {
      for (const fixture of [...fixtures, ...special]) {
        const graph = {
          nodes: fixture.nodes.map((n: { name: string }) => ({ id: n.name })),
          edges: fixture.edges.map((e: { from: string; to: string }, i: number) => ({ id: `e${i}`, cause: e.from, effect: e.to })),
        }
        const result = await layoutDag(graph, orientation, { width: 164, height: 58, nameLines: 1 })
        if (!result.ok) { failures.push(`${fixture.label}/${orientation}: ${result.error.kind}`); continue }
        for (const edge of graph.edges) {
          const points = result.value.routes.get(edge.id).points
          for (let i = 1; i < points.length; i++) {
            for (const node of graph.nodes) {
              if (node.id === edge.cause || node.id === edge.effect) continue
              const box = result.value.nodes.get(node.id)
              for (let step = 0; step <= 100; step++) {
                const t = step / 100
                const x = points[i - 1].x * (1 - t) + points[i].x * t
                const y = points[i - 1].y * (1 - t) + points[i].y * t
                if (x > box.x + .01 && x < box.x + 164 - .01 && y > box.y + .01 && y < box.y + 58 - .01) {
                  failures.push(`${fixture.label}/${orientation}: ${edge.cause} -> ${edge.effect} crosses ${node.id}`)
                  break
                }
              }
            }
          }
        }
      }
    }
    return failures
  }, corpus)
  expect(failures).toEqual([])
})

test('triangle layout, labels, arrow selection and Tidy work in the canvas', async ({ page }, info) => {
  test.setTimeout(90_000)
  const errors: string[] = []
  page.on('pageerror', error => errors.push(error.message))
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('ELK triangle')
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type="file"]').setInputFiles({ name: 'triangle.csv', mimeType: 'text/csv', buffer: Buffer.from('Prize,Choice,Opened\n0,0,1\n0,1,0\n1,0,0\n1,1,1\n0,0,0\n1,1,0\n') })
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await prepare(page, { structure: 'cross-section', columns: 'all' })
  await createDag(page, 'Triangle')
  if (info.project.name === 'mobile-chromium') await page.keyboard.press('Escape')
  await page.getByRole('button', { name: 'From text' }).click()
  await page.getByRole('textbox', { name: 'Graph text' }).fill('dag { Prize -> Choice Choice -> Opened Prize -> Opened }')
  await page.getByRole('button', { name: 'Convert to DAG' }).click()
  await expect(page.locator('.react-flow__edge-path')).toHaveCount(3)
  await page.getByRole('button', { name: 'Expand graph', exact: true }).click()
  await page.getByRole('button', { name: 'Tidy graph' }).click()
  await expect(page.locator('.react-flow__node')).toHaveCount(3)
  await expect.poll(() => page.locator('.react-flow__edge-path').evaluateAll(paths => paths.every(path => !/[QC]/.test(path.getAttribute('d') ?? '')))).toBe(true)
  await page.getByRole('button', { name: 'Show arrow labels' }).click()
  await expect(page.getByText('needs rationale', { exact: true }).first()).toBeVisible()
  await page.getByRole('button', { name: 'Hide arrow labels' }).click()
  // Wait for the existing layout animation, rather than capturing moving cards.
  let previous = ''
  let stable = 0
  await expect.poll(async () => {
    const current = await page.locator('.react-flow__node').evaluateAll(nodes => nodes.map(n => n.getAttribute('style')).join('|'))
    stable = current === previous ? stable + 1 : 0
    previous = current
    return stable
  }).toBeGreaterThanOrEqual(2)
  await page.screenshot({ path: info.outputPath('triangle-light.png') })
  const beforeTheme = await page.locator('.react-flow__node').evaluateAll(nodes => nodes.map(n => n.getAttribute('style')))
  await page.evaluate(() => document.documentElement.setAttribute('data-theme', 'dark'))
  await page.screenshot({ path: info.outputPath('triangle-dark.png') })
  expect(await page.locator('.react-flow__node').evaluateAll(nodes => nodes.map(n => n.getAttribute('style')))).toEqual(beforeTheme)
  const card = page.locator('.react-flow__node').first()
  const original = await card.getAttribute('style')
  const grip = await card.locator('.dag-card-grip').boundingBox()
  expect(grip).not.toBeNull()
  await page.mouse.move(grip!.x + grip!.width / 2, grip!.y + grip!.height / 2)
  await page.mouse.down()
  await page.mouse.move(grip!.x + grip!.width / 2 + 100, grip!.y + grip!.height / 2 + 70, { steps: 12 })
  await page.mouse.up()
  await expect(card).not.toHaveAttribute('style', original!)
  await page.getByRole('button', { name: 'Tidy graph' }).click()
  await expect(card).toHaveAttribute('style', original!)
  const selectArrow = async (cause: string, effect: string) => {
    let previous = ''
    let stable = 0
    await expect.poll(async () => {
      const current = await page.locator('.react-flow__node').evaluateAll(nodes => nodes.map(n => n.getAttribute('style')).join('|'))
      stable = current === previous ? stable + 1 : 0
      previous = current
      return stable
    }).toBeGreaterThanOrEqual(2)
    const location = await page.locator(`.react-flow__edge[aria-label^="${cause} causes ${effect} "] .react-flow__edge-path`).evaluate(element => {
      const path = element as SVGPathElement
      const p = path.getPointAtLength(path.getTotalLength() / 2)
      const screen = new DOMPoint(p.x, p.y).matrixTransform(path.getScreenCTM()!)
      return { x: screen.x, y: screen.y }
    })
    await page.mouse.click(location.x, location.y)
  }
  await selectArrow('Prize', 'Opened')
  await expect(page.getByRole('button', { name: /Reverse/ }).first()).toBeVisible()
  await expect(page.getByRole('button', { name: 'Reconnect cause endpoint' })).toBeVisible()
  await expect(page.getByRole('button', { name: 'Reconnect effect endpoint' })).toBeVisible()
  await page.getByRole('button', { name: 'Remove selected arrow' }).click()
  await expect(page.locator('.react-flow__edge-path')).toHaveCount(2)
  await selectArrow('Prize', 'Choice')
  let knob = await page.getByRole('button', { name: 'Reconnect cause endpoint' }).boundingBox()
  await expect.poll(async () => {
    knob = await page.getByRole('button', { name: 'Reconnect cause endpoint' }).boundingBox()
    return page.evaluate(({ x, y }) => document.elementFromPoint(x, y)?.outerHTML, { x: knob!.x + knob!.width / 2, y: knob!.y + knob!.height / 2 })
  }).toContain('aria-label="Reconnect cause endpoint"')
  const destination = await page.locator('.react-flow__node').filter({ hasText: 'Opened' }).boundingBox()
  await page.mouse.move(knob!.x + knob!.width / 2, knob!.y + knob!.height / 2)
  await page.mouse.down()
  await page.mouse.move(destination!.x + destination!.width / 2, destination!.y + destination!.height / 2, { steps: 15 })
  await expect(page.locator('.react-flow__connection')).toHaveCount(1)
  await page.mouse.up()
  // Opened -> Choice would make a same-period cycle, so the domain must refuse it.
  await expect(page.getByRole('status').filter({ hasText: /cycle/ })).toBeVisible()
  await expect(page.locator('.react-flow__edge[aria-label^="Prize causes Choice "]')).toHaveCount(1)
  await page.screenshot({ path: info.outputPath('triangle-selected.png') })
  expect(errors).toEqual([])
})

test('larger bound graph retains causal colours and readable routes', async ({ page }, info) => {
  test.setTimeout(90_000)
  const fixture = corpus.find((f: { label: string }) => f.label === 'Extended confounding triangle')
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('ELK larger graph')
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type="file"]').setInputFiles(new URL(`./fixtures/dagitty/${fixture.slug}.csv`, import.meta.url).pathname)
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await prepare(page, { structure: 'cross-section', columns: 'all' })
  await createDag(page, 'Confounding triangle')
  if (info.project.name === 'mobile-chromium') await page.keyboard.press('Escape')
  await page.getByRole('button', { name: 'From text' }).click()
  await page.getByRole('textbox', { name: 'Graph text' }).fill(fixture.dot)
  await page.getByRole('button', { name: 'Convert to DAG' }).click()
  await expect(page.locator('.react-flow__edge-path')).toHaveCount(fixture.edges.length)
  await page.getByRole('button', { name: 'Expand graph', exact: true }).click()
  await page.getByRole('button', { name: 'Tidy graph' }).click()
  let previous = ''
  let stable = 0
  await expect.poll(async () => {
    const current = await page.locator('.react-flow__node').evaluateAll(nodes => nodes.map(n => n.getAttribute('style')).join('|'))
    stable = current === previous ? stable + 1 : 0
    previous = current
    return stable
  }).toBeGreaterThanOrEqual(2)
  await expect(page.getByLabel('Arrow legend')).toBeVisible()
  for (const theme of ['light', 'dark']) {
    await page.evaluate(t => document.documentElement.setAttribute('data-theme', t), theme)
    await page.screenshot({ path: info.outputPath(`larger-${theme}.png`) })
  }
})
