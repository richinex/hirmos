import { test, expect } from '@playwright/test'
import { readFile } from 'node:fs/promises'

test('recipe recovery preserves provenance and still validates the rest of the project', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { parseSnapshotValue } = await import(
      new URL('/src/domain/persistence.ts', location.href).href
    )
    const { project } = await (await fetch('/examples/ai-usage-intensity.hirmos.json')).json()
    const baseline = parseSnapshotValue(project)
    if (!baseline.ok) throw new Error(JSON.stringify(baseline.error))
    const original = { kind: 'sql-derived', statement: null }
    const damaged = { ...project, source: { ...project.source, recipe: original } }
    const recovered = parseSnapshotValue(damaged, true)
    if (!recovered.ok) throw new Error(JSON.stringify(recovered.error))
    return {
      strict: parseSnapshotValue(damaged).ok,
      recipe: recovered.value.source.recipe,
      sameProfile:
        JSON.stringify(recovered.value.profile) === JSON.stringify(baseline.value.profile),
      sameStudies:
        JSON.stringify(recovered.value.studies) === JSON.stringify(baseline.value.studies),
      sameRuns:
        JSON.stringify(recovered.value.estimationRuns) ===
        JSON.stringify(baseline.value.estimationRuns),
      missingProfile: parseSnapshotValue({ ...damaged, profile: null }, true).ok,
      invalidProfile: parseSnapshotValue(
        {
          ...damaged,
          profile: {
            ...project.profile,
            source: { ...project.profile.source, fingerprint: 'wrong' },
          },
        },
        true,
      ).ok,
      invalidStudy: parseSnapshotValue({ ...damaged, studies: [null] }, true).ok,
    }
  })
  expect(result).toMatchObject({
    strict: false,
    recipe: { kind: 'unreadable-recipe', original: { kind: 'sql-derived', statement: null } },
    sameProfile: true,
    sameStudies: true,
    sameRuns: true,
    missingProfile: false,
    invalidProfile: false,
    invalidStudy: false,
  })
})

test('recovery blocks replacement saves and exports the original record unchanged', async ({
  page,
}) => {
  await page.goto('/app')
  const seeded = await page.evaluate(async () => {
    const persistence = await import(new URL('/src/domain/persistence.ts', location.href).href)
    const store = await import(new URL('/src/data/projectStore.ts', location.href).href)
    const { project } = await (await fetch('/examples/ai-usage-intensity.hirmos.json')).json()
    const valid = persistence.parseSnapshotValue(project)
    if (!valid.ok) throw new Error(JSON.stringify(valid.error))
    const snapshot = {
      ...valid.value,
      origin: { kind: 'user' },
      project: { ...valid.value.project, name: 'Recover recipe', id: crypto.randomUUID() },
    }
    await store.saveProject(snapshot)
    const body = persistence.serialiseSnapshot({
      ...snapshot,
      source: { ...snapshot.source, recipe: { kind: 'sql-derived', statement: null } },
    })
    const db = await new Promise<IDBDatabase>((resolve, reject) => {
      const r = indexedDB.open('hirmos', 1)
      r.onsuccess = () => resolve(r.result)
      r.onerror = () => reject(r.error)
    })
    await new Promise<void>((resolve, reject) => {
      const tx = db.transaction('projects', 'readwrite')
      tx.objectStore('projects').put(
        { header: persistence.headerOf(snapshot), body },
        snapshot.project.id,
      )
      tx.oncomplete = () => resolve()
      tx.onerror = () => reject(tx.error)
    })
    db.close()
    const recovered = await store.loadProject(snapshot.project.id, true)
    if (!recovered.ok) throw new Error(JSON.stringify(recovered.error))
    const save = await store.saveProject(recovered.value)
    const replacement = await store.saveProject(snapshot)
    const raw = await store.readProjectRecord(snapshot.project.id)
    return {
      body,
      save: save.ok,
      replacement: replacement.ok,
      unchanged: raw.ok && raw.value.body === body,
    }
  })
  expect(seeded).toMatchObject({ save: false, replacement: false, unchanged: true })
  await page.goto('/app/projects')
  await page.reload()
  const download = page.waitForEvent('download')
  await page.getByRole('button', { name: 'Export Recover recipe', exact: true }).click()
  expect(await readFile((await (await download).path())!, 'utf8')).toBe(seeded.body)
  await page.getByRole('button', { name: 'Open Recover recipe', exact: true }).click()
  await expect(page.getByRole('alert').filter({ hasText: 'Recovery mode:' })).toBeVisible()
  await expect(page.getByRole('button', { name: 'Download original record' })).toBeVisible()
  const input = page.locator('input[type=file]').filter({ hasNot: page.locator('[multiple]') })
  await input
    .first()
    .setInputFiles({ name: 'wrong.csv', mimeType: 'text/csv', buffer: Buffer.from('x\n1\n') })
  await expect(
    page.getByText('This is not the file the project was built from.', { exact: false }),
  ).toBeVisible()
  const bundle = await (await page.request.get('/examples/ai-usage-intensity.hirmos.json')).json()
  await input.first().setInputFiles({
    name: bundle.data.name,
    mimeType: bundle.data.mediaType,
    buffer: Buffer.from(bundle.data.base64, 'base64'),
  })
  await expect(page.getByRole('heading', { name: 'Choose the data file again' })).toHaveCount(0)
  await expect(page.getByRole('alert').filter({ hasText: 'Recovery mode:' })).toBeVisible()
  await expect(page.getByRole('button', { name: 'Edit data', exact: true })).toHaveCount(0)
  await page.screenshot({ path: '/tmp/hirmos-recipe-recovery.png', fullPage: true })
  const unchanged = await page.evaluate(async (body) => {
    const store = await import(new URL('/src/data/projectStore.ts', location.href).href)
    const header = (await store.listProjects()).find(
      (p: { name: string }) => p.name === 'Recover recipe',
    )
    const raw = await store.readProjectRecord(header.id)
    return raw.ok && raw.value.body === body
  }, seeded.body)
  expect(unchanged).toBe(true)
})

test('an invalid JSON project can still be downloaded', async ({ page }) => {
  await page.goto('/app')
  await page.evaluate(async () => {
    const persistence = await import(new URL('/src/domain/persistence.ts', location.href).href)
    const store = await import(new URL('/src/data/projectStore.ts', location.href).href)
    const { project } = await (await fetch('/examples/ai-usage-intensity.hirmos.json')).json()
    const parsed = persistence.parseSnapshotValue(project)
    if (!parsed.ok) throw new Error(JSON.stringify(parsed.error))
    const snapshot = {
      ...parsed.value,
      origin: { kind: 'user' },
      project: { ...parsed.value.project, name: 'Unreadable JSON', id: crypto.randomUUID() },
    }
    await store.saveProject(snapshot)
    const db = await new Promise<IDBDatabase>((resolve) => {
      const r = indexedDB.open('hirmos', 1)
      r.onsuccess = () => resolve(r.result)
    })
    await new Promise<void>((resolve) => {
      const tx = db.transaction('projects', 'readwrite')
      tx.objectStore('projects').put(
        { header: persistence.headerOf(snapshot), body: '{broken original' },
        snapshot.project.id,
      )
      tx.oncomplete = () => resolve()
    })
    db.close()
  })
  await page.goto('/app/projects')
  await page.reload()
  const download = page.waitForEvent('download')
  await page.getByRole('button', { name: 'Export Unreadable JSON', exact: true }).click()
  expect(await readFile((await (await download).path())!, 'utf8')).toBe('{broken original')
})
