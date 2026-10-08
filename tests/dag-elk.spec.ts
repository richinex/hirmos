import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'
import { prepare, createDag, chapter } from './examples/support'

const corpus = JSON.parse(readFileSync(new URL('./fixtures/dagitty/examples.json', import.meta.url), 'utf8'))

test('arrows remain visible after importing into an already laid-out DAG and tidying', async ({ page }) => {
  test.setTimeout(60_000)
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Persistent arrows')
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type="file"]').setInputFiles({
    name: 'controls.csv', mimeType: 'text/csv',
    buffer: Buffer.from('X,Y,Z,M,W\n0,0,1,0,1\n0,1,0,1,0\n1,0,0,0,1\n1,1,1,1,0\n'),
  })
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await prepare(page, { structure: 'cross-section', columns: 'all' })
  await createDag(page, 'Controls')
  await expect(page.locator('.react-flow__node')).toHaveCount(5)
  // Let the empty graph finish measuring before its cards are rearranged.
  await page.waitForTimeout(1000)
  await page.getByRole('button', { name: 'From text', exact: true }).click()
  await page.getByRole('textbox', { name: 'Graph text' }).fill('dag { X [exposure] Y [outcome] Z -> X Z -> Y X -> Y }')
  await page.getByRole('button', { name: 'Convert to DAG', exact: true }).click()
  await expect(page.locator('.react-flow__edge-path')).toHaveCount(3)
  // The regression briefly showed arrows, then lost them after the animation.
  await page.waitForTimeout(1000)
  await expect(page.locator('.react-flow__edge-path')).toHaveCount(3)
  await page.getByRole('button', { name: 'Tidy graph', exact: true }).click()
  await page.waitForTimeout(1000)
  await expect(page.locator('.react-flow__edge-path')).toHaveCount(3)
})

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
  page.on('console', message => { if (message.text().includes('new nodeTypes or edgeTypes object')) errors.push(message.text()) })
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
  await expect(page.locator('[data-sketch-stroke]')).toHaveCount(6)
  await expect(page.getByRole('button', { name: 'Use clean drawing', exact: true })).toHaveAttribute('aria-pressed', 'true')
  await page.getByRole('button', { name: 'Use clean drawing', exact: true }).click()
  await expect(page.locator('[data-sketch-stroke]')).toHaveCount(0)
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
  await expect(page.getByRole('toolbar', { name: 'Canvas', exact: true }).getByRole('button', { name: 'Use hand-drawn style', exact: true })).toBeVisible()
  await page.getByRole('button', { name: 'Use hand-drawn style', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Use clean drawing', exact: true })).toHaveAttribute('aria-pressed', 'true')
  await expect(page.locator('[data-sketch-stroke]')).toHaveCount(6)
  // The sketch must replace clean strokes with a visibly rough, two-pass path,
  // rather than merely mounting a renderer whose output still looks straight.
  const sketchGeometry = await page.locator('[data-sketch-stroke^="edge:"] path').evaluateAll(paths => paths.map(element => {
    const path = element as SVGPathElement
    const start = path.getPointAtLength(0)
    const end = path.getPointAtLength(path.getTotalLength())
    const length = Math.hypot(end.x - start.x, end.y - start.y)
    let deviation = 0
    for (let i = 1; i < 100; i++) {
      const p = path.getPointAtLength(path.getTotalLength() * i / 100)
      deviation = Math.max(deviation, Math.abs((end.x - start.x) * (start.y - p.y) - (start.x - p.x) * (end.y - start.y)) / length)
    }
    return { moves: (path.getAttribute('d')?.match(/M/g) ?? []).length, deviation }
  }))
  expect(sketchGeometry.every(path => path.moves >= 2)).toBe(true)
  expect(Math.max(...sketchGeometry.map(path => path.deviation))).toBeGreaterThan(1)
  const sketchPaths = await page.locator('[data-sketch-stroke] path').evaluateAll(paths => paths.map(p => p.getAttribute('d')))
  for (const theme of ['light', 'dark']) {
    await page.evaluate(t => document.documentElement.setAttribute('data-theme', t), theme)
    await page.screenshot({ path: info.outputPath(`triangle-sketch-${theme}.png`) })
    expect(await page.locator('[data-sketch-stroke] path').evaluateAll(paths => paths.map(p => p.getAttribute('d')))).toEqual(sketchPaths)
    expect(await page.locator('.react-flow__node').evaluateAll(nodes => nodes.map(n => n.getAttribute('style')))).toEqual(beforeTheme)
  }
  await page.getByRole('button', { name: 'Use clean drawing', exact: true }).click()
  await expect(page.locator('[data-sketch-stroke]')).toHaveCount(0)
  const card = page.locator('.react-flow__node').first()
  const original = await card.evaluate(node => (node as HTMLElement).style.transform)
  const grip = await card.locator('.dag-card-grip').boundingBox()
  expect(grip).not.toBeNull()
  await page.mouse.move(grip!.x + grip!.width / 2, grip!.y + grip!.height / 2)
  await page.mouse.down()
  await page.mouse.move(grip!.x + grip!.width / 2 + 100, grip!.y + grip!.height / 2 + 70, { steps: 12 })
  await page.mouse.up()
  await expect.poll(() => card.evaluate(node => (node as HTMLElement).style.transform)).not.toBe(original)
  await page.getByRole('button', { name: 'Tidy graph' }).click()
  await expect.poll(() => card.evaluate(node => (node as HTMLElement).style.transform)).toBe(original)
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
  // Adding an arrow after moving a card must not rearrange any existing card.
  const heldCard = page.locator('.react-flow__node[aria-label="Observed variable: Prize"]')
  const heldGrip = await heldCard.locator('.dag-card-grip').boundingBox()
  await page.mouse.move(heldGrip!.x + heldGrip!.width / 2, heldGrip!.y + heldGrip!.height / 2)
  await page.mouse.down()
  await page.mouse.move(heldGrip!.x + heldGrip!.width / 2 + 40, heldGrip!.y + heldGrip!.height / 2 + 20, { steps: 8 })
  await page.mouse.up()
  const heldPositions = await page.locator('.react-flow__node').evaluateAll(nodes => nodes.map(node => ({ id: node.getAttribute('data-id'), transform: (node as HTMLElement).style.transform })))
  const causeBox = await heldCard.boundingBox()
  const effectBox = await page.locator('.react-flow__node[aria-label="Observed variable: Opened"]').boundingBox()
  await page.mouse.move(causeBox!.x + causeBox!.width / 2, causeBox!.y + causeBox!.height - 7)
  await page.mouse.down()
  await page.mouse.move(effectBox!.x + effectBox!.width / 2, effectBox!.y + effectBox!.height - 7, { steps: 10 })
  await page.mouse.up()
  await expect(page.locator('.react-flow__edge-path')).toHaveCount(3)
  expect(await page.locator('.react-flow__node').evaluateAll(nodes => nodes.map(node => ({ id: node.getAttribute('data-id'), transform: (node as HTMLElement).style.transform })))).toEqual(heldPositions)
  await page.screenshot({ path: info.outputPath('triangle-held-new-arrow.png') })
  await selectArrow('Prize', 'Opened')
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

test('Proposition 99 can hide disconnected variables without changing its DAG', async ({ page }, info) => {
  test.setTimeout(90_000)
  const errors: string[] = []
  page.on('pageerror', error => errors.push(error.message))
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open Proposition 99 and cigarette sales', exact: true }).click()
  await expect(page.locator('#data-profile-title')).toBeVisible()
  await chapter(page, /^DAG workspace/)
  if (info.project.name === 'mobile-chromium') await page.keyboard.press('Escape')
  const unexpandedEditor = page.getByLabel('Causal DAG editor', { exact: true })
  await expect(page.locator('.react-flow__node')).toHaveCount(40)
  await unexpandedEditor.scrollIntoViewIfNeeded()
  await expect.poll(() => page.locator('.react-flow__node').evaluateAll(nodes => nodes.some(n => {
    const b = n.getBoundingClientRect(); const canvas = n.closest('[aria-label="Causal DAG editor"]')!.getBoundingClientRect()
    return b.left >= canvas.left && b.right <= canvas.right && b.top >= canvas.top && b.bottom <= canvas.bottom
  }))).toBe(true)
  await page.screenshot({ path: info.outputPath('prop99-unexpanded.png') })
  const editorBox = await unexpandedEditor.boundingBox()
  const controls = await page.getByRole('toolbar', { name: 'Canvas', exact: true }).boundingBox()
  expect(controls!.y).toBeGreaterThanOrEqual(editorBox!.y)
  expect(controls!.y + controls!.height).toBeLessThanOrEqual(editorBox!.y + editorBox!.height)
  await expect(page.getByRole('button', { name: 'Expand graph', exact: true })).toBeVisible()
  await page.getByRole('button', { name: 'Expand graph', exact: true }).click()
  await expect(page.locator('.react-flow__node')).toHaveCount(40)
  await page.screenshot({ path: info.outputPath('prop99-all-variables.png') })
  await page.getByRole('button', { name: 'Hide disconnected variables', exact: true }).click()
  await expect(page.getByRole('button', { name: /Show disconnected variables/ })).toHaveAttribute('aria-pressed', 'true')
  await expect(page.getByRole('button', { name: 'Graph display', exact: true })).toHaveCount(0)
  await expect(page.locator('.react-flow__node')).toHaveCount(2)
  await expect(page.locator('.react-flow__edge-path')).toHaveCount(1)
  await page.getByRole('button', { name: 'Tidy graph', exact: true }).click()
  await expect.poll(() => page.locator('.react-flow__node').evaluateAll(nodes => nodes.every(n => {
    const b = n.getBoundingClientRect(); return b.left >= 0 && b.right <= innerWidth && b.top >= 0 && b.bottom <= innerHeight
  }))).toBe(true)
  for (const theme of ['light', 'dark']) {
    await page.evaluate(t => document.documentElement.setAttribute('data-theme', t), theme)
    await page.screenshot({ path: info.outputPath(`prop99-connected-${theme}.png`) })
  }
  await page.getByRole('button', { name: 'Close the floating window', exact: true }).click()
  await unexpandedEditor.scrollIntoViewIfNeeded()
  await expect.poll(() => page.locator('.react-flow__node').evaluateAll(nodes => nodes.every(n => {
    const b = n.getBoundingClientRect(); const canvas = n.closest('[aria-label="Causal DAG editor"]')!.getBoundingClientRect()
    return b.left >= canvas.left && b.right <= canvas.right && b.top >= canvas.top && b.bottom <= canvas.bottom
  }))).toBe(true)
  await page.screenshot({ path: info.outputPath('prop99-unexpanded-connected.png') })
  const toolbar = await page.getByRole('toolbar', { name: 'Canvas', exact: true }).boundingBox()
  expect(toolbar!.x).toBeGreaterThanOrEqual(0)
  expect(toolbar!.x + toolbar!.width).toBeLessThanOrEqual(page.viewportSize()!.width)
  await page.getByRole('button', { name: /Show disconnected variables/ }).click()
  await expect(page.getByRole('button', { name: 'Hide disconnected variables', exact: true })).toHaveAttribute('aria-pressed', 'false')
  await expect(page.locator('.react-flow__node')).toHaveCount(40)
  await expect(page.locator('.react-flow__edge-path')).toHaveCount(1)
  expect(errors).toEqual([])
})

test('fixed-position routing avoids obstacles and labels clear the cards', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const layoutPath = '/src/components/dag/elkLayout.ts'
    const fixedPath = '/src/components/dag/fixedRouting.ts'
    const labelPath = '/src/components/dag/routeLabels.ts'
    const { layoutDag } = await import(/* @vite-ignore */ layoutPath)
    const { routeFixedDag } = await import(/* @vite-ignore */ fixedPath)
    const { placeRouteLabels, routeLabelSize } = await import(/* @vite-ignore */ labelPath)
    const graph = { nodes: ['A', 'B', 'C'].map(id => ({ id })), edges: [{ id: 'ab', cause: 'A', effect: 'B', timing: { kind: 'contemporaneous' }, support: { kind: 'unstated' } }] }
    const size = { width: 164, height: 58, nameLines: 1 }
    const automatic = await layoutDag(graph, 'across', size)
    if (!automatic.ok) return { problem: automatic.error }
    const positions = new Map([['A', { x: 0, y: 120 }], ['B', { x: 500, y: 120 }], ['C', { x: 250, y: 120 }]])
    const fixed = await routeFixedDag(graph, positions, size, automatic.value)
    if (!fixed.ok) return { problem: fixed.error }
    const points = fixed.value.routes.get('ab').points
    let crossed = false
    for (let i = 1; i < points.length; i++) for (let step = 0; step <= 100; step++) {
      const t = step / 100
      const x = points[i - 1].x * (1 - t) + points[i].x * t
      const y = points[i - 1].y * (1 - t) + points[i].y * t
      if (x > 250 && x < 414 && y > 120 && y < 178) crossed = true
    }
    const label = placeRouteLabels(graph, fixed.value, size).get('ab')
    const dimensions = routeLabelSize(graph.edges[0])
    return { crossed, labelClear: label?.kind === 'clear', fixed: [...fixed.value.nodes], labels: dimensions }
  })
  expect(result).not.toHaveProperty('problem')
  expect(result.crossed).toBe(false)
  expect(result.labelClear).toBe(true)
  expect(result.fixed).toEqual([['A', { x: 0, y: 120 }], ['B', { x: 500, y: 120 }], ['C', { x: 250, y: 120 }]])
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
