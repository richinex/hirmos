import { expect, test } from '@playwright/test'

test('Python sessions isolate cancellation and reject discarded worker messages', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { createPythonRuntime } = await import(new URL('/src/data/pythonRuntime.ts', location.href).href)
    const workers: any[] = []
    const factory = () => {
      const worker = { terminated: false, onmessage: null as any, onerror: null as any, onmessageerror: null as any, messages: [] as any[], terminate() { this.terminated = true }, postMessage(message: unknown) { this.messages.push(message) } }
      workers.push(worker)
      return worker
    }
    const first = createPythonRuntime(factory)
    const second = createPythonRuntime(factory)
    const step = { kind: 'script', id: 'same-block', inputs: [], code: '', view: 'output' }
    const one = first.scripts.run(step, {})
    const two = second.scripts.run(step, {})
    await Promise.resolve()
    const old = workers[0]
    const oldRequest = old.messages[0].request
    const otherRequest = workers[1].messages[0].request
    first.cancel()
    const replacement = workers[2]
    const retry = first.scripts.run(step, {})
    await Promise.resolve()
    const retryRequest = first.store.getState().active.request
    const cancelled = await one
    old.onmessage({ data: { kind: 'ready', python: 'old-worker' } })
    old.onerror({ message: 'old error' })
    const retained = first.store.getState().active.request === retryRequest
    const staleIgnored = first.store.getState().runtime.kind === 'loading'
    const isolated = second.store.getState().active.request === otherRequest
    const concurrent = await first.scripts.run(step, {})
    replacement.onmessage({ data: { kind: 'run-failed', request: retryRequest, detail: 'script error', stdout: '' } })
    const retried = await retry
    second.dispose()
    const disposed = await two
    const afterClose = await second.scripts.run(step, {})
    first.dispose()
    return { cancelled: !cancelled.ok, retained, staleIgnored, isolated, unique: retryRequest !== oldRequest, concurrent: !concurrent.ok, retried: retried.error.detail, disposed: !disposed.ok, afterClose: !afterClose.ok, terminated: workers.every(worker => worker.terminated) }
  })
  expect(result).toEqual({ cancelled: true, retained: true, staleIgnored: true, isolated: true, unique: true, concurrent: true, retried: 'script error', disposed: true, afterClose: true, terminated: true })
})

test('Python disposal during Arrow input transfer prevents worker creation', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { createPythonRuntime } = await import(new URL('/src/data/pythonRuntime.ts', location.href).href)
    let created = 0
    const session = createPythonRuntime(() => { created++; throw new Error('Must not start') })
    let finish!: (bytes: Uint8Array) => void
    const connection = { useUnsafe: () => new Promise<Uint8Array>(resolve => { finish = resolve }) }
    const pending = session.scripts.run({ kind: 'script', id: 'a', inputs: ['input'], code: '', view: 'out' }, connection)
    session.dispose()
    session.activate()
    finish(new Uint8Array([1]))
    const result = await pending
    session.dispose()
    return { created, refused: !result.ok, active: session.store.getState().active }
  })
  expect(result).toEqual({ created: 0, refused: true, active: null })
})

test('Python cancellation during result staging leaves the previous output untouched', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { createPythonRuntime } = await import(new URL('/src/data/pythonRuntime.ts', location.href).href)
    const worker: any = { onmessage: null, terminate() {}, postMessage() {} }
    const session = createPythonRuntime(() => worker)
    const queries: string[] = []
    let staged!: () => void
    const connection = {
      insertArrowFromIPCStream: () => new Promise<void>(resolve => { staged = resolve }),
      query: async (sql: string) => { queries.push(sql) },
    }
    const pending = session.scripts.run({ kind: 'script', id: 'a', inputs: [], code: '', view: 'out' }, connection)
    await Promise.resolve()
    const request = session.store.getState().active.request
    worker.onmessage({ data: { kind: 'ran', request, rows: 1, columns: ['x'], stdout: '', shape: { kind: 'table' }, prepared: { format: 'arrow-stream', bytes: new Uint8Array() } } })
    await Promise.resolve()
    session.cancel()
    staged()
    const result = await pending
    session.dispose()
    return { refused: !result.ok, writes: queries.filter(sql => !sql.startsWith('DROP TABLE IF EXISTS')), cleaned: queries.some(sql => sql.startsWith('DROP TABLE IF EXISTS')) }
  })
  expect(result).toEqual({ refused: true, writes: [], cleaned: true })
})
