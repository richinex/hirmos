import ELK, { type ELK as ElkEngine } from 'elkjs/lib/elk-api'
import ElkWorker from 'elkjs/lib/elk-worker.min.js?worker'

type Graph = Parameters<ElkEngine['layout']>[0]
type LaidOut = Awaited<ReturnType<ElkEngine['layout']>>
interface Request {
  readonly graph: Graph
  readonly resolve: (graph: LaidOut) => void
  readonly reject: (reason: unknown) => void
  readonly detach: () => void
}

const LIMIT_MS = 8000
const queue: Request[] = []
let engine: ElkEngine | undefined
let generation = 0
let active: Request | undefined
let timer: ReturnType<typeof setTimeout> | undefined

function abandon(reason: unknown): void {
  clearTimeout(timer)
  const requests = active === undefined ? queue.splice(0) : [active, ...queue.splice(0)]
  active = undefined
  generation++
  engine?.terminateWorker()
  engine = undefined
  for (const request of requests) {
    request.detach()
    request.reject(reason)
  }
}

function ensureEngine(): ElkEngine {
  return (engine ??= new ELK({
    workerFactory: () => {
      const worker = new ElkWorker()
      const current = ++generation
      worker.addEventListener('error', () => {
        if (generation === current)
          abandon(new Error('The graph layout worker stopped. Retry the layout.'))
      })
      worker.addEventListener('messageerror', () => {
        if (generation === current)
          abandon(new Error('The graph layout worker returned an unreadable response.'))
      })
      return worker
    },
  }))
}

/** Starts the layout worker ahead of the first layout. */
export function warmLayoutEngine(): void {
  ensureEngine()
}

function drain(): void {
  if (active !== undefined) return
  const request = queue.shift()
  if (request === undefined) return
  active = request
  timer = setTimeout(
    () => abandon(new Error('The graph layout did not finish within 8 seconds. Retry the layout.')),
    LIMIT_MS,
  )
  try {
    const current = ensureEngine()
    void current.layout(request.graph).then(
      (result) => {
        if (active !== request) return
        clearTimeout(timer)
        active = undefined
        request.detach()
        request.resolve(result)
        drain()
      },
      (reason) => {
        if (active === request) abandon(reason)
      },
    )
  } catch (reason) {
    abandon(reason)
  }
}

export function runLayout(graph: Graph, signal?: AbortSignal): Promise<LaidOut> {
  return new Promise((resolve, reject) => {
    if (signal?.aborted) {
      reject(signal.reason)
      return
    }
    const abort = () => {
      const index = queue.indexOf(request)
      if (index !== -1) queue.splice(index, 1)
      request.detach()
      reject(signal?.reason)
    }
    const request: Request = {
      graph,
      resolve,
      reject,
      detach: () => signal?.removeEventListener('abort', abort),
    }
    signal?.addEventListener('abort', abort, { once: true })
    queue.push(request)
    drain()
  })
}
