// Builds the dagitty parity fixtures: each dagitty example DAG with one exposure and one outcome,
// rewritten in Hirmos's terms (a bidirected arrow becomes an unmeasured common cause), with dagitty's
// own verdicts: the minimal sufficient adjustment sets, whether any back-door path is open, and
// whether the set Hirmos's canonical rule would name is a valid adjustment set.
// Run: node tests/fixtures/dagitty/build.mjs <path to dagitty-master>
import { readFileSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'
import vm from 'node:vm'

const root = process.argv[2]
if (!root) throw new Error('give the dagitty-master path')
const ctx = { console }
ctx.self = ctx
vm.createContext(ctx)
vm.runInContext(readFileSync(join(root, 'r/inst/js/dagitty-alg.js'), 'utf8'), ctx)
vm.runInContext(readFileSync(join(root, 'gui/js/example-dags.js'), 'utf8'), ctx)
const { GraphParser, GraphAnalyzer, Graph } = ctx.DAGitty

const FLAG = { E: 'exposure', O: 'outcome', U: 'latent', A: 'adjusted', 1: null }

/** The legacy `e`/`v` form: `e` lines are adjacency lists, `v` lines are `name flag @x,y`. */
const legacyToDot = (example) => {
  const nodes = example.v.trim().split('\n').map((line) => {
    const [name, flag, at] = line.trim().split(/\s+/)
    const pos = at && at.startsWith('@') ? at.slice(1) : null
    return { name, flag: FLAG[flag], pos }
  })
  const rows = example.e.trim().split('\n').map((line) => line.trim().split(/\s+/))
  // Two legacy forms: adjacency lists (`from to to …`), or a 0/1 adjacency matrix in vertex order.
  const matrix = rows.length === nodes.length && rows.every((row) => row.length === nodes.length && row.every((cell) => cell === '0' || cell === '1'))
  const edges = matrix
    ? rows.flatMap((row, from) => row.flatMap((cell, to) => (cell === '1' ? [`${nodes[from].name} -> ${nodes[to].name}`] : [])))
    : rows.flatMap(([from, ...to]) => to.map((target) => `${from} -> ${target}`))
  const attrs = (node) => [node.flag, node.pos ? `pos="${node.pos}"` : null].filter(Boolean)
  return `dag { ${nodes.map((node) => `${node.name}${attrs(node).length ? ` [${attrs(node).join(',')}]` : ''}`).join(' ')} ${edges.join(' ')} }`
}

const slug = (text) => text.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '')

const fixtures = []
for (const example of ctx.examples) {
  try {
  const dot = example.d ?? legacyToDot(example)
  const g = GraphParser.parseGuess(dot)
  const sources = g.getSources().map((v) => v.id)
  const targets = g.getTargets().map((v) => v.id)
  if (sources.length !== 1 || targets.length !== 1) continue
  const [exposure] = sources
  const [outcome] = targets
  const latent = new Set(g.getLatentNodes().map((v) => v.id))
  const nodes = g.getVertices().map((v) => ({ name: v.id, kind: latent.has(v.id) ? 'latent' : 'observed', x: v.layout_pos_x ?? null, y: v.layout_pos_y ?? null }))
  const edges = []
  let bidirected = 0
  for (const edge of g.getEdges()) {
    if (edge.directed === Graph.Edgetype.Directed) edges.push({ from: edge.v1.id, to: edge.v2.id })
    else if (edge.directed === Graph.Edgetype.Bidirected) {
      bidirected += 1
      const name = `U_${edge.v1.id}${edge.v2.id}`
      nodes.push({ name, kind: 'latent', x: null, y: null })
      edges.push({ from: name, to: edge.v1.id }, { from: name, to: edge.v2.id })
    } else throw new Error(`${example.l}: unsupported edge type ${edge.directed}`)
  }
  // Hirmos's canonical rule, on the Hirmos graph: observed ancestors of X or Y, minus X, Y, causal-path nodes and their descendants.
  const h = GraphParser.parseGuess(`dag { ${nodes.map((n) => `${n.name}${n.name === exposure ? ' [exposure]' : n.name === outcome ? ' [outcome]' : n.kind === 'latent' ? ' [latent]' : ''}`).join(' ')} ${edges.map((e) => `${e.from} -> ${e.to}`).join(' ')} }`)
  const ids = (vs) => vs.map((v) => v.id)
  const X = h.getVertex(exposure), Y = h.getVertex(outcome)
  const descX = new Set(ids(h.descendantsOf([X])))
  const ancY = new Set(ids(h.ancestorsOf([Y])))
  const causal = [...descX].filter((id) => id === outcome || ancY.has(id))
  const forbidden = new Set([exposure, outcome, ...causal])
  for (const id of causal) for (const d of ids(h.descendantsOf([h.getVertex(id)]))) forbidden.add(d)
  const candidates = [...new Set([...ids(h.ancestorsOf([X])), ...ids(h.ancestorsOf([Y]))])].filter((id) => !forbidden.has(id) && !nodes.some((n) => n.name === id && n.kind === 'latent')).sort()
  // Verdicts on the Hirmos twin (bidirected arrows as unmeasured common causes, no forced adjustments), so they match what the UI can express.
  const msas = GraphAnalyzer.listMsasTotalEffect(h).map((set) => set.map((v) => v.id).sort())
  fixtures.push({
    label: example.l,
    slug: slug(example.l),
    exposure, outcome,
    nodes, edges, bidirected,
    expected: {
      msas,
      backdoorOpen: !GraphAnalyzer.isAdjustmentSet(h, []),
      canonical: candidates,
      canonicalValid: GraphAnalyzer.isAdjustmentSet(h, candidates),
    },
  })
  // one numeric column per observed node; the graph, not the data, decides identification
  let seed = 7
  const rand = () => { seed = (seed * 48271) % 2147483647; return seed / 2147483647 }
  const observed = nodes.filter((n) => n.kind === 'observed').map((n) => n.name)
  const rows = Array.from({ length: 80 }, () => observed.map(() => (rand() * 10).toFixed(3)).join(','))
  writeFileSync(join('tests/fixtures/dagitty', `${slug(example.l)}.csv`), [observed.join(','), ...rows].join('\n') + '\n')
  } catch (error) {
    console.log(`skip ${example.l}: ${error instanceof Error ? error.message.slice(0, 160) : String(error).slice(0, 160)}`)
  }
}
writeFileSync('tests/fixtures/dagitty/examples.json', JSON.stringify(fixtures, null, 2) + '\n')
for (const f of fixtures) console.log(`${f.label} | nodes ${f.nodes.length} (${f.nodes.filter((n) => n.kind === 'latent').length} latent) | edges ${f.edges.length} | X ${f.exposure} Y ${f.outcome} | back-door open ${f.expected.backdoorOpen} | MSAS ${f.expected.msas.length === 0 ? 'none' : f.expected.msas.map((s) => `{${s.join(',')}}`).join(' ')} | canonical {${f.expected.canonical.join(',')}} valid ${f.expected.canonicalValid}`)
