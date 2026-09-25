import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { parseDagText } from '../src/domain/dagText'

/**
 * Hirmos reads the graph description language itself rather than shipping dagitty's reader, so this
 * checks the reading against dagitty's own on every example graph it publishes. The fixtures hold
 * both sides: the text dagitty was given, and what dagitty made of it, recorded by
 * tests/fixtures/dagitty/build.mjs running GraphParser.parseGuess.
 */

interface Fixture {
  readonly label: string
  readonly slug: string
  readonly dot: string
  readonly exposure: string
  readonly outcome: string
  readonly nodes: readonly { readonly name: string; readonly kind: 'observed' | 'latent'; readonly x: number | null; readonly y: number | null }[]
  readonly edges: readonly { readonly from: string; readonly to: string }[]
  readonly bidirected: number
}

const fixtures: readonly Fixture[] = JSON.parse(
  readFileSync(fileURLToPath(new URL('fixtures/dagitty/examples.json', import.meta.url)), 'utf8'),
)

const sorted = (edges: readonly { readonly from: string; readonly to: string }[]) =>
  [...edges].map((edge) => `${edge.from}->${edge.to}`).sort()

test('every dagitty example graph reads the same way here', () => {
  expect(fixtures.length).toBeGreaterThan(0)
  for (const fixture of fixtures) {
    const parsed = parseDagText(fixture.dot)
    expect(parsed.ok, `${fixture.label}: ${parsed.ok ? '' : JSON.stringify(parsed.error)}`).toBe(true)
    if (!parsed.ok) continue

    expect(parsed.value.exposures, `${fixture.label} exposure`).toEqual([fixture.exposure])
    expect(parsed.value.outcomes, `${fixture.label} outcome`).toEqual([fixture.outcome])
    expect(parsed.value.bidirected, `${fixture.label} bidirected arrows`).toBe(fixture.bidirected)

    // Same arrows, and the same count, so a duplicate or a dropped arrow both fail.
    expect(sorted(parsed.value.edges), `${fixture.label} arrows`).toEqual(sorted(fixture.edges))
    expect(parsed.value.edges.length, `${fixture.label} arrow count`).toBe(fixture.edges.length)

    const ours = new Map(parsed.value.nodes.map((node) => [node.name, node]))
    const theirs = new Map(fixture.nodes.map((node) => [node.name, node]))
    expect([...ours.keys()].sort(), `${fixture.label} variables`).toEqual([...theirs.keys()].sort())
    for (const [name, node] of theirs) {
      expect(ours.get(name)?.kind, `${fixture.label}: ${name} measured or not`).toBe(node.kind)
      expect(ours.get(name)?.x ?? null, `${fixture.label}: ${name} x`).toBe(node.x ?? null)
      expect(ours.get(name)?.y ?? null, `${fixture.label}: ${name} y`).toBe(node.y ?? null)
    }
  }
})

test('the largest published graph reads completely', () => {
  const largest = [...fixtures].sort((a, b) => b.edges.length - a.edges.length)[0]!
  const parsed = parseDagText(largest.dot)
  expect(parsed.ok).toBe(true)
  if (!parsed.ok) return
  expect(largest.edges.length).toBeGreaterThanOrEqual(69)
  expect(parsed.value.edges.length).toBe(largest.edges.length)
  expect(parsed.value.nodes.length).toBe(largest.nodes.length)
})

interface Probe {
  readonly label: string
  readonly dot: string
  readonly refused?: string
  readonly nodes?: readonly { readonly name: string; readonly kind: 'observed' | 'latent'; readonly x: number | null; readonly y: number | null }[]
  readonly edges?: readonly { readonly from: string; readonly to: string }[]
  readonly bidirected?: number
  readonly adjusted?: readonly string[]
  readonly exposures?: readonly string[]
  readonly outcomes?: readonly string[]
}

const probes: readonly Probe[] = JSON.parse(
  readFileSync(fileURLToPath(new URL('fixtures/dagitty/grammar-probes.json', import.meta.url)), 'utf8'),
)

/**
 * One deliberate difference. dagitty's wrapper test is
 * `^(digraph|graph|dag|pdag|mag|pag)(\s+\w+)?\s*\{…\}$`, which has no room for the `strict`
 * that graphviz allows, so dagitty wraps the whole text again and reads `strict`, `digraph` and the
 * graph's name as three variables. Hirmos reads the header, because a reader who pastes valid
 * graphviz means the two variables and the one arrow.
 */
const DELIBERATE: ReadonlyMap<string, string> = new Map([
  ['strict with a name', 'dagitty reads the header words as variables; Hirmos reads the header'],
  ['two lags on one pair', 'dagitty has no lag, so two lags on one pair are one arrow there and two here'],
])

test('the grammar reads as dagitty reads it, beyond the forms the examples use', () => {
  expect(probes.length).toBeGreaterThan(15)
  for (const probe of probes) {
    if (DELIBERATE.has(probe.label)) continue
    const parsed = parseDagText(probe.dot)

    if (probe.refused !== undefined) {
      expect(parsed.ok, `${probe.label}: dagitty refuses this, so Hirmos must too`).toBe(false)
      continue
    }

    expect(parsed.ok, `${probe.label}: ${parsed.ok ? '' : JSON.stringify(parsed.error)}`).toBe(true)
    if (!parsed.ok) continue
    expect(sorted(parsed.value.edges), `${probe.label} arrows`).toEqual(sorted(probe.edges ?? []))
    expect(parsed.value.edges.length, `${probe.label} arrow count`).toBe((probe.edges ?? []).length)
    expect(parsed.value.bidirected, `${probe.label} bidirected`).toBe(probe.bidirected ?? 0)
    expect([...parsed.value.exposures], `${probe.label} exposures`).toEqual([...(probe.exposures ?? [])])
    expect([...parsed.value.outcomes], `${probe.label} outcomes`).toEqual([...(probe.outcomes ?? [])])
    expect([...parsed.value.adjusted].sort(), `${probe.label} adjusted`).toEqual([...(probe.adjusted ?? [])].sort())

    const ours = new Map(parsed.value.nodes.map((node) => [node.name, node]))
    const theirs = new Map((probe.nodes ?? []).map((node) => [node.name, node]))
    expect([...ours.keys()].sort(), `${probe.label} variables`).toEqual([...theirs.keys()].sort())
    for (const [name, node] of theirs) {
      expect(ours.get(name)?.kind, `${probe.label}: ${name} measured or not`).toBe(node.kind)
      expect(ours.get(name)?.x ?? null, `${probe.label}: ${name} x`).toBe(node.x ?? null)
      expect(ours.get(name)?.y ?? null, `${probe.label}: ${name} y`).toBe(node.y ?? null)
    }
  }
})

const randoms: readonly Probe[] = JSON.parse(
  readFileSync(fileURLToPath(new URL('fixtures/dagitty/grammar-random.json', import.meta.url)), 'utf8'),
).map((entry: Omit<Probe, 'label'>, index: number) => ({ ...entry, label: `random ${index}` }))

test('random graphs over the whole grammar read as dagitty reads them', () => {
  expect(randoms.length).toBeGreaterThan(300)
  for (const probe of randoms) {
    const parsed = parseDagText(probe.dot)
    expect(parsed.ok, `${probe.label}: ${parsed.ok ? '' : JSON.stringify(parsed.error)}\n${probe.dot}`).toBe(true)
    if (!parsed.ok) continue
    const where = `${probe.label}\n${probe.dot}`
    expect(sorted(parsed.value.edges), `${where}\narrows`).toEqual(sorted(probe.edges ?? []))
    expect(parsed.value.edges.length, `${where}\narrow count`).toBe((probe.edges ?? []).length)
    expect(parsed.value.bidirected, `${where}\nbidirected`).toBe(probe.bidirected ?? 0)
    expect([...parsed.value.exposures], `${where}\nexposures`).toEqual([...(probe.exposures ?? [])])
    expect([...parsed.value.outcomes], `${where}\noutcomes`).toEqual([...(probe.outcomes ?? [])])
    expect([...parsed.value.adjusted].sort(), `${where}\nadjusted`).toEqual([...(probe.adjusted ?? [])].sort())
    const ours = new Map(parsed.value.nodes.map((node) => [node.name, node]))
    const theirs = new Map((probe.nodes ?? []).map((node) => [node.name, node]))
    expect([...ours.keys()].sort(), `${where}\nvariables`).toEqual([...theirs.keys()].sort())
    for (const [name, node] of theirs) {
      expect(ours.get(name)?.kind, `${where}\n${name} measured or not`).toBe(node.kind)
      expect(ours.get(name)?.x ?? null, `${where}\n${name} x`).toBe(node.x ?? null)
      expect(ours.get(name)?.y ?? null, `${where}\n${name} y`).toBe(node.y ?? null)
    }
  }
})

interface DagittyTest extends Probe {
  readonly source: string
  readonly nonDag?: boolean
}

const dagittyTests: readonly DagittyTest[] = JSON.parse(
  readFileSync(fileURLToPath(new URL('fixtures/dagitty/dagitty-tests.json', import.meta.url)), 'utf8'),
)

/**
 * dagitty's own test inputs: the strings its parser tests feed it, its validation and graph-type
 * cases, and its shared test graphs written out by its serializer. A graph with undirected or
 * partially directed arrows is not a DAG and is refused here by the operator's name; every other
 * input must read as dagitty reads it, including a graph of variables with no arrows at all.
 */
test("dagitty's own test inputs read as dagitty reads them", () => {
  expect(dagittyTests.length).toBeGreaterThan(60)
  expect(dagittyTests.filter((entry) => entry.nonDag).length).toBeGreaterThan(5)
  for (const entry of dagittyTests) {
    const where = `${entry.source}: ${entry.label}\n${entry.dot}`
    const parsed = parseDagText(entry.dot)
    if (entry.refused !== undefined) {
      expect(parsed.ok, `${where}\ndagitty refuses this`).toBe(false)
      continue
    }
    if (entry.nonDag) {
      expect(parsed.ok, `${where}\nnot a DAG, so refused here`).toBe(false)
      if (!parsed.ok) expect(parsed.error.kind, where).toBe('unsupported-edge')
      continue
    }
    expect(parsed.ok, `${where}\n${parsed.ok ? '' : JSON.stringify(parsed.error)}`).toBe(true)
    if (!parsed.ok) continue
    expect(sorted(parsed.value.edges), `${where}\narrows`).toEqual(sorted(entry.edges ?? []))
    expect(parsed.value.edges.length, `${where}\narrow count`).toBe((entry.edges ?? []).length)
    expect(parsed.value.bidirected, `${where}\nbidirected`).toBe(entry.bidirected ?? 0)
    expect([...parsed.value.exposures], `${where}\nexposures`).toEqual([...(entry.exposures ?? [])])
    expect([...parsed.value.outcomes], `${where}\noutcomes`).toEqual([...(entry.outcomes ?? [])])
    expect([...parsed.value.adjusted].sort(), `${where}\nadjusted`).toEqual([...(entry.adjusted ?? [])].sort())
    const ours = new Map(parsed.value.nodes.map((node) => [node.name, node]))
    const theirs = new Map((entry.nodes ?? []).map((node) => [node.name, node]))
    expect([...ours.keys()].sort(), `${where}\nvariables`).toEqual([...theirs.keys()].sort())
    for (const [name, node] of theirs) {
      expect(ours.get(name)?.kind, `${where}\n${name} measured or not`).toBe(node.kind)
      expect(ours.get(name)?.x ?? null, `${where}\n${name} x`).toBe(node.x ?? null)
      expect(ours.get(name)?.y ?? null, `${where}\n${name} y`).toBe(node.y ?? null)
    }
  }
})

test('a graphviz header is read, where dagitty reads its words as variables', () => {
  const probe = probes.find((entry) => entry.label === 'strict with a name')
  expect(probe, 'the probe should still be recorded so the difference stays visible').toBeDefined()
  expect(DELIBERATE.has('strict with a name')).toBe(true)
  // What dagitty makes of it, kept so the difference is a decision rather than a surprise.
  expect((probe?.nodes ?? []).map((node) => node.name)).toEqual(['strict', 'digraph', 'name', 'A', 'B'])

  const parsed = parseDagText('strict digraph name { A -> B }')
  expect(parsed.ok).toBe(true)
  if (!parsed.ok) return
  expect(parsed.value.nodes.map((node) => node.name)).toEqual(['A', 'B'])
  expect(parsed.value.edges).toEqual([{ from: 'A', to: 'B', lag: null }])
})

test('a lag on an arrow is its own arrow, which dagitty has no notion of', () => {
  const probe = probes.find((entry) => entry.label === 'two lags on one pair')
  expect(probe).toBeDefined()
  expect((probe?.edges ?? []).length, 'dagitty keeps one').toBe(1)

  const parsed = parseDagText('dag { A -> B [lag=1] A -> B [lag=3] B <- A [lag=1] A -> A [lag=2] }')
  expect(parsed.ok).toBe(true)
  if (!parsed.ok) return
  expect(parsed.value.edges).toEqual([
    { from: 'A', to: 'B', lag: 1 },
    { from: 'A', to: 'B', lag: 3 },
    { from: 'A', to: 'A', lag: 2 },
  ])
  expect(parseDagText('dag { A -> B [lag=0] }').ok).toBe(false)
  expect(parseDagText('dag { A -> B [lag=x] }').ok).toBe(false)
  expect(parseDagText('dag { A <-> B [lag=1] }').ok).toBe(false)
})

test('text the workspace cannot hold is refused by name', () => {
  for (const operator of ['--', '@-@', '@->', '<-@', '@--', '--@']) {
    const refused = parseDagText(`dag { A ${operator} B }`)
    expect(refused.ok, `${operator} should be refused`).toBe(false)
    if (!refused.ok) expect(refused.error).toEqual({ kind: 'unsupported-edge', operator })
  }
  const reserved = parseDagText('dag { graph -> B }')
  expect(reserved.ok).toBe(false)

  expect(parseDagText('   ').ok).toBe(false)
  expect(parseDagText('dag { A -> "unclosed }').ok).toBe(false)
  // A graph of variables and no arrows reads, as it does in dagitty; drawing nothing is the import's refusal.
  const bare = parseDagText('dag { A [exposure] }')
  expect(bare.ok && bare.value.edges.length === 0 && bare.value.exposures[0] === 'A').toBe(true)
})
