import { execFileSync } from 'node:child_process'
import { mkdirSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'
import { ICON_NAMES, ICON_SUBSET, usedIconNames } from './icon-names.mjs'

const root = process.cwd()
const font = join(root, 'node_modules/material-symbols/material-symbols-sharp.woff2')
const directory = join(root, 'src/assets/fonts')
const manifest = join(root, ICON_SUBSET)
mkdirSync(directory, { recursive: true })

const python = (...args) =>
  execFileSync(
    'uv',
    ['run', '--quiet', '--no-project', '--with', 'fonttools==4.60.1', '--with', 'brotli==1.1.0',
      'python', join(root, 'scripts/subset-icons.py'), ...args],
    { stdio: 'inherit' },
  )

python('names', font, join(root, ICON_NAMES))
const names = usedIconNames(root)
writeFileSync(manifest, `${JSON.stringify({ names }, null, 2)}\n`)
python('subset', manifest, font, join(directory, 'material-symbols-sharp.woff2'))
console.log(`Material Symbols Sharp: ${names.length} icons`)
