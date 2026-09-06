// The company-wide AI adoption series, generated with a fixed seed so the shipped example and its
// walkthrough can be rebuilt to the same numbers.
//
//   node tests/examples/generate-company-wide-adoption.mjs
//
// 72 monthly rows. The legacy portfolio is an AR(1) defect rate around 2.0 defects per KLOC. Product
// code tracks it, `bugs = 0.90 * legacy + 0.16 + noise`, and falls by 0.40 from the rollout in month 49,
// so the truth is −0.40 per month over 24 post-rollout months, a cumulative −9.60. The length is
// chosen so a constraint-based time-series method at lag 2 has the rows it asks for.
import { writeFileSync } from 'node:fs'
import { resolve, dirname } from 'node:path'
import { fileURLToPath } from 'node:url'

const HERE = dirname(fileURLToPath(import.meta.url))
const MONTHS = 72
const ROLLOUT = 49
const EFFECT = -0.4

// mulberry32, then Box–Muller: a small seeded generator is enough for a fixture.
const seeded = (seed) => () => {
  seed = (seed + 0x6d2b79f5) | 0
  let t = seed
  t = Math.imul(t ^ (t >>> 15), t | 1)
  t ^= t + Math.imul(t ^ (t >>> 7), t | 61)
  return ((t ^ (t >>> 14)) >>> 0) / 4294967296
}
const uniform = seeded(20260904)
const normal = () => {
  const u = Math.max(uniform(), 1e-12)
  const v = uniform()
  return Math.sqrt(-2 * Math.log(u)) * Math.cos(2 * Math.PI * v)
}

const rows = [['month', 'ai_active', 'legacy_bugs_per_kloc', 'bugs_per_kloc']]
let legacyDeviation = 0
for (let month = 1; month <= MONTHS; month += 1) {
  legacyDeviation = 0.6 * legacyDeviation + 0.08 * normal()
  const legacy = 2.0 + legacyDeviation
  const active = month >= ROLLOUT ? 1 : 0
  const bugs = 0.9 * legacy + 0.16 + 0.05 * normal() + EFFECT * active
  rows.push([month, active, legacy.toFixed(4), bugs.toFixed(4)])
}
const csv = rows.map((row) => row.join(',')).join('\n') + '\n'
for (const target of [
  resolve(HERE, '../fixtures/company-wide-adoption.csv'),
  resolve(HERE, '../../docs/2026-09-04-ai-adoption-company-wide/data/company-wide-adoption.csv'),
]) writeFileSync(target, csv)
console.log(`${MONTHS} rows, rollout at month ${ROLLOUT}, effect ${EFFECT} per month`)
