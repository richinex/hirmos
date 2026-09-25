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
  const g = GraphParser.parseDot(dot)
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
    // The text the reader pastes, kept so a parser can be checked against dagitty's own reading.
    dot,
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

// Grammar probes: inputs chosen from dagitty's own GraphDotParser.pegjs rather than from the
// example graphs, so the reader is checked against dagitty on forms no example happens to use.
const probes = [
  ['chain and reversed arrows', 'graph { X [exposure] Y [outcome] X <- A -> M <- B -> Y X -> Y }'],
  ['group on both sides', 'dag { {A B} -> {C D} }'],
  ['group chained onward', 'dag { {A B} -> C -> {D E} }'],
  ['single letter aliases', 'dag { X [e] Y [o] Z [u] W [a] X -> Y Z -> X W -> Y }'],
  ['long aliases', 'dag { X [source] Y [target] Z [unobserved] W [adjusted] X -> Y Z -> X W -> Y }'],
  ['legacy aliases', 'dag { X [exposure] Y [outcome] L [latent] L -> X X -> Y }'],
  ['semicolons and commas', 'dag { A -> B; B -> C, C -> D; }'],
  ['no wrapper', 'a.1 -> b_2 b_2 -> 3c'],
  ['strict with a name', 'strict digraph name { A -> B }'],
  ['quoted names', 'dag { "long name" -> b }'],
  ['quoted with escape', 'dag { "say \\"hi\\"" -> b }'],
  ['bidirected', 'dag { A [exposure] B [outcome] A <-> B A -> B }'],
  ['two bidirected', 'dag { A <-> B B <-> C }'],
  ['declared after use', 'dag { A -> B B [outcome] A [exposure] }'],
  ['repeated arrow', 'dag { A -> B A -> B }'],
  ['repeated bidirected', 'dag { A <-> B A <-> B }'],
  ['bidirected written both ways', 'dag { A <-> B B <-> A }'],
  ['arrows in both directions', 'dag { A -> B B -> A }'],
  ['repeated arrow written in reverse', 'dag { A -> B B <- A }'],
  ['repeated node declaration', 'dag { A [exposure] A [exposure] A -> B }'],
  ['directed and bidirected between the same pair', 'dag { A -> B A <-> B }'],
  ['lag as an edge attribute', 'dag { A -> B [lag=1] }'],
  ['lagged self loop', 'dag { A -> A [lag=1] }'],
  ['two lags on one pair', 'dag { A -> B [lag=1] A -> B [lag=3] }'],
  ['contemporaneous self loop', 'dag { A -> A }'],
  ['bounding box and positions', 'dag { bb="-3,-0.5,2,1.2" A [pos="1.5,-2.25"] B [pos="0,0"] A -> B }'],
  ['label attribute', 'dag { A [label="An A"] A -> B }'],
  ['unknown attribute kept out of the way', 'dag { A [colour=red] A -> B }'],
  ['uppercase graph type', 'DAG { A -> B }'],
  ['digit and dot names', 'dag { 1.5 -> 2.5 }'],
  ['attribute without a value', 'dag { A [exposure] B [outcome] A -> B }'],
]
const probed = []
for (const [label, dot] of probes) {
  try {
    const g = GraphParser.parseDot(dot)
    const latent = new Set(g.getLatentNodes().map((v) => v.id))
    const adjusted = g.getAdjustedNodes().map((v) => v.id).sort()
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
      } else throw new Error(`unsupported edge type ${edge.directed}`)
    }
    const sources = g.getSources().map((v) => v.id)
    const targets = g.getTargets().map((v) => v.id)
    probed.push({ label, dot, nodes, edges, bidirected, adjusted,
      exposures: sources, outcomes: targets,
      exposure: sources.length === 1 ? sources[0] : null,
      outcome: targets.length === 1 ? targets[0] : null })
  } catch (error) {
    probed.push({ label, dot, refused: String(error instanceof Error ? error.message : error).slice(0, 120) })
  }
}
writeFileSync('tests/fixtures/dagitty/grammar-probes.json', JSON.stringify(probed, null, 2) + '\n')
console.log(`\n${probed.length} grammar probes, ${probed.filter((p) => p.refused).length} refused by dagitty`)


// Random graphs over the whole grammar surface, read by dagitty, so the reader is checked against
// far more shapes than a hand-written probe list covers.
let rs = 20260925
const rnd = () => { rs = (rs * 48271) % 2147483647; return rs / 2147483647 }
const pick = (xs) => xs[Math.floor(rnd() * xs.length)]
const NAMES = ['A','B','C','D','E','X','Y','Z','M','W','v1','n_2','a.b','x0','Long_Name','q9']
const randomDot = () => {
  const names = []
  const count = 2 + Math.floor(rnd() * 6)
  while (names.length < count) { const n = pick(NAMES); if (!names.includes(n)) names.push(n) }
  const lines = []
  if (rnd() < 0.4) lines.push(`bb="${(rnd()*4-2).toFixed(3)},${(rnd()*4-2).toFixed(3)},${(rnd()*4-2).toFixed(3)},${(rnd()*4-2).toFixed(3)}"`)
  for (const n of names) {
    const attrs = []
    if (rnd() < 0.25) attrs.push(pick(['exposure','e','source']))
    else if (rnd() < 0.25) attrs.push(pick(['outcome','o','target']))
    if (rnd() < 0.2) attrs.push(pick(['latent','l','unobserved','u']))
    if (rnd() < 0.15) attrs.push(pick(['adjusted','a']))
    if (rnd() < 0.5) attrs.push(`pos="${(rnd()*4-2).toFixed(3)},${(rnd()*4-2).toFixed(3)}"`)
    if (rnd() < 0.1) attrs.push(`label="${pick(['one','two','a b'])}"`)
    lines.push(attrs.length > 0 ? `${n} [${attrs.join(',')}]` : n)
  }
  const arrows = 1 + Math.floor(rnd() * 8)
  for (let i = 0; i < arrows; i += 1) {
    const op = () => pick(['->','->','->','<-','<->'])
    const side = () => {
      if (rnd() < 0.2) {
        const group = []
        const size = 1 + Math.floor(rnd() * 2)
        while (group.length < size) { const n = pick(names); if (!group.includes(n)) group.push(n) }
        return `{${group.join(' ')}}`
      }
      return pick(names)
    }
    let line = `${side()} ${op()} ${side()}`
    if (rnd() < 0.25) line += ` ${op()} ${side()}`
    lines.push(line)
  }
  const type = pick(['dag','graph','digraph','DAG',''])
  const body = lines.join(rnd() < 0.5 ? '\n\t' : ' ')
  return type === '' ? body : `${type} { ${body} }`
}
const randoms = []
for (let i = 0; i < 400; i += 1) {
  const dot = randomDot()
  try {
    const g = GraphParser.parseDot(dot)
    const latent = new Set(g.getLatentNodes().map((v) => v.id))
    const nodes = g.getVertices().map((v) => ({ name: v.id, kind: latent.has(v.id) ? 'latent' : 'observed', x: v.layout_pos_x ?? null, y: v.layout_pos_y ?? null }))
    const edges = []
    let bidirected = 0
    let unsupported = false
    for (const edge of g.getEdges()) {
      if (edge.directed === Graph.Edgetype.Directed) edges.push({ from: edge.v1.id, to: edge.v2.id })
      else if (edge.directed === Graph.Edgetype.Bidirected) {
        bidirected += 1
        const name = `U_${edge.v1.id}${edge.v2.id}`
        nodes.push({ name, kind: 'latent', x: null, y: null })
        edges.push({ from: name, to: edge.v1.id }, { from: name, to: edge.v2.id })
      } else unsupported = true
    }
    if (unsupported) continue
    const sources = g.getSources().map((v) => v.id)
    const targets = g.getTargets().map((v) => v.id)
    randoms.push({ dot, nodes, edges, bidirected,
      adjusted: g.getAdjustedNodes().map((v) => v.id).sort(),
      exposures: sources, outcomes: targets,
      exposure: sources.length === 1 ? sources[0] : null,
      outcome: targets.length === 1 ? targets[0] : null })
  } catch { /* dagitty refuses it; the reader is checked on what dagitty accepts */ }
}
writeFileSync('tests/fixtures/dagitty/grammar-random.json', JSON.stringify(randoms, null, 1) + '\n')
console.log(`${randoms.length} random graphs recorded`)


// dagitty's own test inputs: every string its parser tests feed it, its shared test graphs written
// back out by its serializer, and its validation and graph-type cases. Recorded as dagitty reads
// them, so the reader is checked against the cases dagitty's authors chose, not only ours.
const tests = []
const readOne = (source, label, dot) => {
  try {
    const g = GraphParser.parseDot(dot)
    const latent = new Set(g.getLatentNodes().map((v) => v.id))
    const nodes = g.getVertices().map((v) => ({ name: v.id, kind: latent.has(v.id) ? 'latent' : 'observed', x: v.layout_pos_x ?? null, y: v.layout_pos_y ?? null }))
    const edges = []
    let bidirected = 0
    const nonDag = []
    for (const edge of g.getEdges()) {
      if (edge.directed === Graph.Edgetype.Directed) edges.push({ from: edge.v1.id, to: edge.v2.id })
      else if (edge.directed === Graph.Edgetype.Bidirected) {
        bidirected += 1
        const name = `U_${edge.v1.id}${edge.v2.id}`
        nodes.push({ name, kind: 'latent', x: null, y: null })
        edges.push({ from: name, to: edge.v1.id }, { from: name, to: edge.v2.id })
      } else nonDag.push(String(edge.directed))
    }
    const sources = g.getSources().map((v) => v.id)
    const targets = g.getTargets().map((v) => v.id)
    tests.push({ source, label, dot, nodes, edges, bidirected, adjusted: g.getAdjustedNodes().map((v) => v.id).sort(),
      exposures: sources, outcomes: targets, nonDag: nonDag.length > 0 })
  } catch (error) {
    tests.push({ source, label, dot, refused: String(error instanceof Error ? error.message : error).slice(0, 120) })
  }
}
// parser.js: the argument of every $p(...), GraphParser.parseDot(...) and new Graph(...), evaluated
// in the sandbox because a few are string concatenations.
const parserSource = readFileSync(join(root, 'test/test/parser.js'), 'utf8')
let seen = 0
for (const match of parserSource.matchAll(/(\$p\(|GraphParser\.parseDot\(|new Graph\()/g)) {
  let i = match.index + match[0].length, depth = 1, j = i
  while (depth > 0 && j < parserSource.length) { const c = parserSource[j]; if (c === '(') depth += 1; else if (c === ')') depth -= 1; j += 1 }
  const expression = parserSource.slice(i, j - 1).trim()
  let text
  try { text = vm.runInContext(`(${expression})`, ctx) } catch { continue }
  if (typeof text !== 'string') continue
  seen += 1
  readOne('parser.js', `parser.js input ${seen}`, text)
}
// graph-validation.js and graph-types.js: the same extraction.
for (const file of ['test/test/graph-validation.js', 'test/test/graph-types.js']) {
  const text = readFileSync(join(root, file), 'utf8')
  let n = 0
  for (const match of text.matchAll(/new Graph\(/g)) {
    let i = match.index + match[0].length, depth = 1, j = i
    while (depth > 0 && j < text.length) { const c = text[j]; if (c === '(') depth += 1; else if (c === ')') depth -= 1; j += 1 }
    let dot
    try { dot = vm.runInContext(`(${text.slice(i, j - 1).trim()})`, ctx) } catch { continue }
    if (typeof dot !== 'string') continue
    n += 1
    readOne(file.split('/').pop(), `${file.split('/').pop()} input ${n}`, dot)
  }
}
// test-graphs.js: dagitty's shared graphs, serialized by dagitty and read back, so the serializer's
// own output is also an input the reader must take.
for (const k of ['GraphParser', 'Graph', 'GraphSerializer', 'GraphAnalyzer', 'GraphTransformer']) ctx[k] = ctx.DAGitty[k]
ctx.window = ctx
vm.runInContext(readFileSync(join(root, 'test/test-graphs.js'), 'utf8'), ctx)
for (const name of Object.keys(ctx.TestGraphs)) {
  if (name === 'findExample') continue
  const value = ctx.TestGraphs[name]
  let dot
  try {
    const made = typeof value === 'function' ? value() : value
    dot = typeof made === 'string' ? made : ctx.DAGitty.GraphSerializer.toDot(made)
  } catch (error) { tests.push({ source: 'test-graphs.js', label: name, dot: '', refused: `not serializable: ${String(error).slice(0, 80)}` }); continue }
  readOne('test-graphs.js', name, dot)
}
writeFileSync('tests/fixtures/dagitty/dagitty-tests.json', JSON.stringify(tests, null, 1) + '\n')
console.log(`${tests.length} dagitty test inputs: ${tests.filter((t) => t.refused).length} refused by dagitty, ${tests.filter((t) => t.nonDag).length} with non-DAG arrows, ${tests.filter((t) => !t.refused && !t.nonDag && t.edges.length === 0).length} with no arrows`)

writeFileSync('tests/fixtures/dagitty/examples.json', JSON.stringify(fixtures, null, 2) + '\n')
for (const f of fixtures) console.log(`${f.label} | nodes ${f.nodes.length} (${f.nodes.filter((n) => n.kind === 'latent').length} latent) | edges ${f.edges.length} | X ${f.exposure} Y ${f.outcome} | back-door open ${f.expected.backdoorOpen} | MSAS ${f.expected.msas.length === 0 ? 'none' : f.expected.msas.map((s) => `{${s.join(',')}}`).join(' ')} | canonical {${f.expected.canonical.join(',')}} valid ${f.expected.canonicalValid}`)
