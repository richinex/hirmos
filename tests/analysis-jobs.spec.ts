import { expect, test } from '@playwright/test'
import { chapter } from './examples/support'

test('a sensitivity probe completes after leaving its chapter', async ({ page }, info) => {
  test.setTimeout(90_000)
  await page.addInitScript(() => {
    const post = Worker.prototype.postMessage
    Worker.prototype.postMessage = function (message: unknown, options?: Transferable[] | StructuredSerializeOptions) {
      const send = () => post.call(this, message, Array.isArray(options) ? { transfer: options } : options)
      if (typeof message === 'object' && message !== null && 'kind' in message && message.kind === 'linear-refutation') {
        setTimeout(send, 1500)
        return
      }
      send()
    }
  })
  await page.goto('/app')
  await page.getByRole('button', { name: 'Open A simulated process with a collider', exact: true }).filter({ visible: true }).click()
  await chapter(page, /Sensitivity/)
  const mobile = info.project.name === 'mobile-chromium'
  const history = page.getByRole(mobile ? 'button' : 'heading', { name: /^Probes \(/ })
  const title = mobile ? await history.getAttribute('aria-label') : await history.textContent()
  const before = Number(title?.match(/\d+/)?.[0])
  expect(Number.isFinite(before)).toBe(true)
  await page.getByRole('button', { name: 'Run perturbation and residual probes', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Cancel run', exact: true })).toBeVisible()
  await chapter(page, /DAG workspace/)
  await expect(page.getByRole('heading', { name: 'DAG workspace', exact: true })).toBeVisible()
  await chapter(page, /Sensitivity/)
  if (mobile) await expect(history).toHaveAttribute('aria-label', `Probes (${before + 1})`, { timeout: 60_000 })
  else await expect(history).toHaveText(`Probes (${before + 1})`, { timeout: 60_000 })
  await expect(page.getByRole('button', { name: 'Cancel run', exact: true })).toHaveCount(0)
})

test('replacing prepared data invalidates jobs without remounting the view', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const load = (path: string) => import(new URL(path, location.href).href)
    const { default: React } = await load('/node_modules/.vite/deps/react.js')
    const { default: { createRoot } } = await load('/node_modules/.vite/deps/react-dom_client.js')
    const { default: { flushSync } } = await load('/node_modules/.vite/deps/react-dom.js')
    const { JobsProvider, useJob } = await load('/src/analysis/JobsProvider.tsx')
    let mounts = 0
    let session: ReturnType<typeof useJob>
    function View() {
      React.useState(() => { mounts++; return 0 })
      session = useJob('survival')
      return null
    }
    const host = document.createElement('div')
    document.body.append(host)
    const root = createRoot(host)
    const render = (prepared: string, profile = 'source-one') => flushSync(() => root.render(React.createElement(JobsProvider, { prepared, profile }, React.createElement(View))))
    render('first-dataset')
    let id: string | null = null
    flushSync(() => { id = session.start('analysis', 'Fitting') })
    const started = session.job.kind
    render('second-dataset')
    const replaced = session.job.kind
    const obsolete = id !== null && !session.current(id)
    let sourceRun: string | null = null
    flushSync(() => { sourceRun = session.start('analysis', 'Preparing') })
    render('second-dataset', 'source-two')
    const sourceReplaced = session.job.kind === 'idle' && sourceRun !== null && !session.current(sourceRun)
    flushSync(() => root.unmount())
    host.remove()
    return { mounts, started, replaced, obsolete, sourceReplaced }
  })
  expect(result).toEqual({ mounts: 1, started: 'running', replaced: 'idle', obsolete: true, sourceReplaced: true })
})

test('workflow stores reuse domain transitions without sharing state', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { createWorkflowStore } = await import(new URL('/src/domain/workflowStore.ts', location.href).href)
    const { INITIAL_WORKFLOW, stepWorkflow } = await import(new URL('/src/domain/workflow.ts', location.href).href)
    const first = createWorkflowStore()
    const second = createWorkflowStore()
    const initial = first.getState().workflow
    const untouched = second.getState().workflow
    const dispatch = first.getState().dispatch
    const event = { type: 'project-name-changed', value: 'Independent project' }
    dispatch(event)
    const matches = JSON.stringify(first.getState().workflow) === JSON.stringify(stepWorkflow(INITIAL_WORKFLOW, event))
    const isolated = second.getState().workflow === untouched && JSON.stringify(untouched) === JSON.stringify(INITIAL_WORKFLOW)
    let notifications = 0
    const unsubscribe = first.subscribe(() => { notifications++ })
    dispatch({ type: 'stationarity-evidence-cleared' })
    const unchanged = notifications === 0
    dispatch({ type: 'project-submitted' })
    const created = first.getState().workflow.kind
    dispatch({ type: 'project-closed' })
    unsubscribe()
    return { matches, isolated, unchanged, created, closed: first.getState().workflow === initial, stable: first.getState().dispatch === dispatch }
  })
  expect(result).toEqual({ matches: true, isolated: true, unchanged: true, created: 'awaiting-data', closed: true, stable: true })
})

test('completed and disposed jobs cannot accept late callbacks', async ({ page }) => {
  await page.goto('/app')
  const states = await page.evaluate(async () => {
    const { createJobs } = await import(new URL('/src/analysis/jobs.ts', location.href).href)
    let stops = 0
    const jobs = createJobs(() => { stops++ })
    const first = jobs.start('survival', 'analysis', 'Fitting')
    jobs.finish('survival', first)
    jobs.fail('survival', first, 'Late failure')
    jobs.progress('survival', first, 'Late progress')
    jobs.cancel('survival')
    const completed = jobs.store.getState().jobs.survival
    const second = jobs.start('survival', 'analysis', 'Fitting again')
    jobs.dispose()
    jobs.dispose()
    jobs.finish('survival', second)
    jobs.activate()
    const third = jobs.start('survival', 'analysis', 'New session')
    jobs.finish('survival', second)
    const running = jobs.current('survival', third)
    jobs.fail('survival', third, 'Fitting failed')
    jobs.finish('survival', third)
    return { completed, running, failed: jobs.store.getState().jobs.survival, stops }
  })
  expect(states).toEqual({
    completed: { kind: 'idle' },
    running: true,
    failed: { kind: 'failed', action: 'analysis', detail: 'Fitting failed' },
    stops: 1,
  })
})

test('project jobs reject stale completion and isolate sessions', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { createJobs } = await import(new URL('/src/analysis/jobs.ts', location.href).href)
    let stops = 0
    const first = createJobs(() => { stops++ })
    const second = createJobs(() => {})
    const id = first.start('variance', 'analysis', 'Preparing')
    const blocked = first.start('arrows', 'analysis', 'Preparing')
    first.progress('variance', id, 'Calculating')
    const running = first.store.getState().jobs.variance
    first.cancel('variance')
    first.progress('variance', id, 'Late progress')
    first.finish('variance', id)
    const cancelled = first.store.getState().jobs.variance
    const next = first.start('variance', 'analysis', 'Preparing again')
    first.fail('variance', id, 'Late failure')
    const staleIgnored = first.current('variance', next)
    first.dispose()
    const disposed = !first.current('variance', next) && first.start('arrows', 'analysis', 'Preparing') === null
    return { blocked, stage: running.stage, cancelled, staleIgnored, disposed, stops, isolated: Object.keys(second.store.getState().jobs).length }
  })
  expect(result).toEqual({ blocked: null, stage: 'Calculating', cancelled: { kind: 'cancelled', action: 'analysis' }, staleIgnored: true, disposed: true, stops: 2, isolated: 0 })
})

test('variance run survives analysis tabs and chapter navigation', async ({ page }) => {
  test.setTimeout(90_000)
  await page.goto('/app')
  await page.getByRole('button', { name: 'Open Microservices: why did the website slow down?', exact: true }).filter({ visible: true }).click()
  await chapter(page, /Causal model analysis/)
  await page.getByRole('radio', { name: 'Variance contributions', exact: true }).check()
  await page.getByRole('combobox', { name: 'Influence target' }).click()
  await page.getByRole('option', { name: 'Website', exact: true }).click()
  await page.getByRole('button', { name: 'Run analysis', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Cancel run', exact: true })).toBeVisible()
  await page.getByRole('radio', { name: 'Arrow strengths', exact: true }).check()
  await expect(page.getByRole('button', { name: 'Run analysis', exact: true })).toBeDisabled()
  await page.getByRole('radio', { name: 'Variance contributions', exact: true }).check()
  await expect(page.getByRole('button', { name: 'Cancel run', exact: true })).toBeVisible()
  await chapter(page, /DAG workspace/)
  await chapter(page, /Causal model analysis/)
  await expect(page.getByRole('radio', { name: 'Variance contributions', exact: true })).toBeChecked()
  await expect(page.getByRole('combobox', { name: 'Influence target' })).toContainText('Website')
  await expect(page.getByRole('button', { name: 'Cancel run', exact: true })).toBeVisible()
  await page.getByRole('button', { name: 'Cancel run', exact: true }).click()
  await expect(page.getByRole('alert')).toContainText('The analysis was cancelled.')
  await expect(page.getByRole('alert')).toHaveClass(/color-info/)
  await expect(page.getByRole('button', { name: 'Cancel run', exact: true })).toHaveCount(0)
})
