import { readFileSync, readdirSync } from 'node:fs'
import { join } from 'node:path'

export const ICON_NAMES = 'scripts/material-symbols-sharp-names.json'
export const ICON_SUBSET = 'src/assets/fonts/material-symbols-sharp.json'

const LITERAL = /'([a-z0-9_]+)'|"([a-z0-9_]+)"|`([a-z0-9_]+)`/g

function sourceFiles(directory) {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const path = join(directory, entry.name)
    if (entry.isDirectory()) return entry.name === 'generated' ? [] : sourceFiles(path)
    return /\.(ts|tsx)$/.test(entry.name) ? [path] : []
  })
}

export function usedIconNames(root) {
  const names = new Set(JSON.parse(readFileSync(join(root, ICON_NAMES), 'utf8')).names)
  const used = new Set()
  for (const file of sourceFiles(join(root, 'src'))) {
    for (const match of readFileSync(file, 'utf8').matchAll(LITERAL)) {
      const literal = match[1] ?? match[2] ?? match[3]
      if (names.has(literal)) used.add(literal)
    }
  }
  return [...used].sort()
}
