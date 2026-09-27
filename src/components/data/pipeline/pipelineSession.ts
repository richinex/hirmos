import { createStore } from 'zustand/vanilla'
import { blockLabel, type PipelineBlockId, type PipelineGraph } from '@/domain/pipeline'
import { selectDerivedSource, type PipelineResume, type SelectedSource } from '@/domain/workflow'
import type { SqlPreparationInput } from '@/domain/sourceInputs'
import { addPipelineInput, closePipeline, describePipelineRunProblem, materializePipeline, openPipeline, removePipelineInput, runPipeline, type PipelineRun, type PipelineSession } from '@/data/pipeline'
import { createPythonRuntime } from '@/data/pythonRuntime'
import { configureBlock, initialGraph, removeBlock } from './pipelineWorkspaceModel'

type Session = { readonly kind: 'opening' } | { readonly kind: 'ready'; readonly live: PipelineSession } | { readonly kind: 'failed'; readonly detail: string }
type Run = { readonly kind: 'idle' } | { readonly kind: 'ran'; readonly result: PipelineRun } | { readonly kind: 'refused'; readonly detail: string }
type Action = { readonly kind: 'idle' } | { readonly kind: 'busy' } | { readonly kind: 'refused'; readonly detail: string }
interface State {
  readonly graph: PipelineGraph
  readonly files: readonly SqlPreparationInput[]
  readonly session: Session
  readonly run: Run
  readonly choosing: Action
  readonly materializing: Action
}

const semantics = (graph: PipelineGraph) => JSON.stringify({ nodes: graph.nodes.map(node => [node.id, node.block]), edges: graph.edges })

/** One project-owned graph and DuckDB session. All table mutations share its queue. */
export function createPipelineSession(resume: PipelineResume | null) {
  const python = createPythonRuntime()
  const store = createStore<State>(() => ({ graph: resume?.graph ?? initialGraph([]), files: resume?.inputs ?? [], session: { kind: 'opening' }, run: { kind: 'idle' }, choosing: { kind: 'idle' }, materializing: { kind: 'idle' } }))
  let closed = true
  let epoch = 0
  let revision = 0
  let timer: ReturnType<typeof setTimeout> | undefined
  let ready: Promise<PipelineSession | null> = Promise.resolve(null)
  let queue: Promise<unknown> = Promise.resolve()
  const valid = (generation: number) => !closed && epoch === generation
  const nameOf = (id: PipelineBlockId): string => {
    const block = store.getState().graph.nodes.find(node => node.id === id)?.block
    if (block === undefined) return id
    return block.kind === 'input' && block.file.kind === 'chosen' ? `${blockLabel('input')} ${block.file.alias}` : blockLabel(block.kind)
  }
  const enqueue = <T,>(work: (live: PipelineSession, generation: number) => Promise<T>): Promise<T | null> => {
    const generation = epoch
    const opening = ready
    const next = queue.then(async () => {
      const live = await opening
      if (live === null || !valid(generation)) return null
      return work(live, generation)
    })
    queue = next.catch(() => undefined)
    return next
  }
  const schedule = () => {
    if (closed) return
    clearTimeout(timer)
    const version = ++revision
    timer = setTimeout(() => {
      void enqueue(async (live, generation) => {
        if (version !== revision) return
        const result = await runPipeline(live, store.getState().graph)
        if (!valid(generation) || version !== revision) return
        store.setState({ run: result.ok ? { kind: 'ran', result: result.value } : { kind: 'refused', detail: describePipelineRunProblem(result.error, nameOf) } })
      }).catch(cause => { if (!closed && version === revision) store.setState({ run: { kind: 'refused', detail: String(cause) } }) })
    }, 250)
  }
  const setGraph = (update: PipelineGraph | ((graph: PipelineGraph) => PipelineGraph)) => {
    if (closed) return
    const previous = store.getState().graph
    const graph = typeof update === 'function' ? update(previous) : update
    if (graph === previous) return
    store.setState({ graph })
    if (semantics(graph) !== semantics(previous)) schedule()
  }
  const open = () => {
    if (!closed) return
    closed = false
    const generation = ++epoch
    python.activate()
    store.setState({ session: { kind: 'opening' }, run: { kind: 'idle' }, choosing: { kind: 'idle' }, materializing: { kind: 'idle' } })
    ready = openPipeline(store.getState().files, python.scripts).then(async result => {
      if (!valid(generation)) { if (result.ok) await closePipeline(result.value); return null }
      if (!result.ok) { store.setState({ session: { kind: 'failed', detail: describePipelineRunProblem(result.error, nameOf) } }); return null }
      store.setState({ session: { kind: 'ready', live: result.value } })
      return result.value
    }).catch(cause => { if (valid(generation)) store.setState({ session: { kind: 'failed', detail: String(cause) } }); return null })
    schedule()
  }
  const dispose = () => {
    if (closed) return
    closed = true
    epoch++
    revision++
    clearTimeout(timer)
    python.dispose()
    const session = store.getState().session
    if (session.kind === 'ready') void closePipeline(session.live).catch(() => undefined)
    store.setState({ session: { kind: 'opening' }, run: { kind: 'idle' }, choosing: { kind: 'idle' }, materializing: { kind: 'idle' } })
  }
  const chooseFile = async (id: PipelineBlockId, file: File) => {
    if (closed || store.getState().choosing.kind === 'busy') return
    const generation = epoch
    store.setState({ choosing: { kind: 'busy' } })
    try {
      await enqueue(async live => {
        const previous = store.getState().graph.nodes.find(node => node.id === id)?.block
        if (previous?.kind !== 'input') return
        const added = await addPipelineInput(live, file)
        if (!valid(generation)) return
        if (!added.ok) throw new Error(describePipelineRunProblem(added.error, nameOf))
        const current = store.getState().graph.nodes.find(node => node.id === id)?.block
        if (current?.kind !== 'input') { await removePipelineInput(live, added.value.alias); return }
        setGraph(graph => configureBlock(graph, id, { kind: 'input', file: { kind: 'chosen', alias: added.value.alias } }))
        if (previous.file.kind === 'chosen') await removePipelineInput(live, previous.file.alias)
        if (valid(generation)) store.setState({ files: [...live.inputs] })
      })
      if (valid(generation)) store.setState({ choosing: store.getState().session.kind === 'failed' ? { kind: 'refused', detail: 'The data engine could not start. The file cannot be read.' } : { kind: 'idle' } })
    } catch (cause) { if (valid(generation)) store.setState({ choosing: { kind: 'refused', detail: String(cause) } }) }
  }
  const dropBlock = (id: PipelineBlockId) => {
    if (closed) return
    const generation = epoch
    const node = store.getState().graph.nodes.find(node => node.id === id)
    setGraph(graph => removeBlock(graph, id))
    const file = node?.block.kind === 'input' ? node.block.file : null
    if (file?.kind !== 'chosen') return
    void enqueue(async (live, generation) => {
      await removePipelineInput(live, file.alias)
      if (valid(generation)) store.setState({ files: [...live.inputs] })
    }).catch(cause => { if (valid(generation)) store.setState({ choosing: { kind: 'refused', detail: String(cause) } }) })
  }
  const adopt = async (): Promise<SelectedSource | null> => {
    if (closed || store.getState().materializing.kind === 'busy') return null
    const generation = epoch
    const version = revision
    const graph = store.getState().graph
    store.setState({ materializing: { kind: 'busy' } })
    try {
      const source = await enqueue(async live => {
        const output = await materializePipeline(live, graph)
        if (!valid(generation)) return null
        if (version !== revision) throw new Error('The pipeline has changed. Use the updated result as the source.')
        if (!output.ok) throw new Error(describePipelineRunProblem(output.error, nameOf))
        const source = selectDerivedSource(output.value.file, output.value.recipe)
        if (!source.ok) throw new Error(`The output could not be used as a source: ${source.error.kind}`)
        return source.value
      })
      if (!valid(generation)) return null
      store.setState({ materializing: { kind: 'idle' } })
      return source
    } catch (cause) {
      if (valid(generation)) store.setState({ materializing: { kind: 'refused', detail: String(cause) } })
      return null
    }
  }
  return { store, python, open, dispose, setGraph, chooseFile, dropBlock, adopt, active: () => !closed }
}
export type PipelineController = ReturnType<typeof createPipelineSession>
