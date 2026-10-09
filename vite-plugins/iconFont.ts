import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import type { Plugin } from 'vite'
import { ICON_SUBSET, usedIconNames } from '../scripts/icon-names.mjs'

export const iconFont = (): Plugin => {
  let serving = false
  return {
    name: 'hirmos:icon-font',
    configResolved(config) {
      serving = config.command === 'serve'
    },
    buildStart() {
      const root = process.cwd()
      const subset = new Set(JSON.parse(readFileSync(join(root, ICON_SUBSET), 'utf8')).names)
      const missing = usedIconNames(root).filter((name) => !subset.has(name))
      if (missing.length === 0) return
      const message = `The icon font lacks ${missing.join(', ')}. Run node scripts/subset-icons.mjs to rebuild it.`
      if (serving) this.warn(message)
      else this.error(message)
    },
  }
}
