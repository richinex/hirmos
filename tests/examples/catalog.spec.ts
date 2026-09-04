import { expect, test } from '@playwright/test'
import { readFile } from 'node:fs/promises'
import { parseBundle } from '../../src/domain/bundle'
import { SHIPPED_EXAMPLES } from '../../src/domain/example'
import { bundleTarget } from './support'

/**
 * The catalog and the bundles it points at must agree, or a card opens the wrong project or none:
 * every shipped example has a bundle on disk that parses, carries the catalog's id and name, and
 * brings its own source file, and no two examples share an id.
 */

test('every shipped example has a parseable bundle carrying its own id, name and source', async () => {
  for (const example of SHIPPED_EXAMPLES) {
    const parsed = parseBundle(await readFile(bundleTarget(example), 'utf8'))
    expect(parsed.ok, `${example.bundleUrl} parses`).toBe(true)
    if (!parsed.ok) continue
    expect(parsed.value.project.project.id, `${example.bundleUrl} id`).toBe(example.id)
    expect(parsed.value.project.project.name, `${example.bundleUrl} name`).toBe(example.name)
    expect(parsed.value.data.kind, `${example.bundleUrl} brings its source`).toBe('source-file')
  }
})

test('shipped example ids and bundle paths are distinct', () => {
  expect(new Set(SHIPPED_EXAMPLES.map((example) => example.id)).size).toBe(SHIPPED_EXAMPLES.length)
  expect(new Set(SHIPPED_EXAMPLES.map((example) => example.bundleUrl)).size).toBe(SHIPPED_EXAMPLES.length)
})
