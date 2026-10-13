import { z } from 'zod'
import { activeTimer } from './activeTimer'
import workerSource from './python.js?raw'
import { PYODIDE_INDEX_URL } from '@/workers/pythonProtocol'

const message = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('ready') }),
  z.object({
    kind: z.literal('done'),
    output: z.string().max(12000),
    stderr: z.string().max(4000),
    exitCode: z.union([z.literal(0), z.literal(1)]),
    locals: z.record(z.string(), z.string()),
  }),
  z.object({ kind: z.literal('error'), error: z.string().max(4000) }),
  z.object({
    kind: z.literal('tool'),
    id: z.number().int().positive(),
    name: z.string().max(80),
    args: z.string().max(16000),
  }),
])
export type HostTool = (name: string, args: unknown) => Promise<unknown>
type Execution = Extract<z.infer<typeof message>, { kind: 'done' }>

export async function pythonSandbox(signal: AbortSignal, tool: HostTool) {
  signal.throwIfAborted()
  const frame = document.createElement('iframe')
  frame.hidden = true
  frame.setAttribute('sandbox', 'allow-scripts')
  frame.setAttribute('aria-hidden', 'true')
  const encode = (value: string) => JSON.stringify(value).replaceAll('<', '\\u003c')
  // Opaque origin denies access to app storage; CSP restricts network to public runtime assets.
  frame.srcdoc = `<meta http-equiv="Content-Security-Policy" content="default-src 'none'; script-src 'unsafe-inline' 'unsafe-eval' blob: ${PYODIDE_INDEX_URL}; worker-src blob:; connect-src ${PYODIDE_INDEX_URL};"><script>
const worker = new Worker(URL.createObjectURL(new Blob([${encode(workerSource)}], {type:'text/javascript'})));
worker.onmessage = e => parent.postMessage(e.data, '*');
worker.onerror = () => parent.postMessage({kind:'error',error:'Python worker could not start.'}, '*');
onmessage = e => { if(e.source === parent) worker.postMessage(e.data); };
worker.postMessage({kind:'init',index:${encode(PYODIDE_INDEX_URL)}});
</script>`
  let settle: { resolve: (value: Execution) => void; reject: (error: Error) => void } | null = null
  let timer: ReturnType<typeof activeTimer> | undefined
  let closed = false
  let calls = 0
  const close = () => {
    if (closed) return
    closed = true
    timer?.stop()
    window.removeEventListener('message', receive)
    signal.removeEventListener('abort', close)
    frame.remove()
    settle?.reject(new Error('Python execution stopped.'))
    settle = null
  }
  const receive = (event: MessageEvent) => {
    if (event.source !== frame.contentWindow || closed) return
    const parsed = message.safeParse(event.data)
    if (!parsed.success) {
      close()
      return
    }
    const data = parsed.data
    if (data.kind === 'tool') {
      if (++calls > 24) {
        close()
        return
      }
      const executionTimer = timer
      executionTimer?.pause()
      void Promise.resolve()
        .then(() => tool(data.name, JSON.parse(data.args)))
        .then(
          (value) => {
            if (!closed)
              frame.contentWindow?.postMessage({ kind: 'reply', id: data.id, ok: true, value }, '*')
          },
          (error: unknown) => {
            if (!closed)
              frame.contentWindow?.postMessage(
                {
                  kind: 'reply',
                  id: data.id,
                  ok: false,
                  error:
                    error instanceof Error && error.message.trim()
                      ? error.message.slice(0, 4000)
                      : 'Tool request refused without a reason.',
                },
                '*',
              )
          },
        )
        .finally(() => executionTimer?.resume())
      return
    }
    timer?.stop()
    const current = settle
    settle = null
    if (data.kind === 'error') current?.reject(new Error(data.error))
    else
      current?.resolve(
        data.kind === 'done'
          ? data
          : { kind: 'done', output: '', stderr: '', exitCode: 0, locals: {} },
      )
  }
  const wait = (milliseconds: number) =>
    new Promise<Execution>((resolve, reject) => {
      settle = { resolve, reject }
      timer = activeTimer(milliseconds, () => {
        reject(new Error('Python execution timed out.'))
        close()
      })
    })
  window.addEventListener('message', receive)
  signal.addEventListener('abort', close, { once: true })
  const ready = wait(120000)
  document.body.append(frame)
  try {
    await ready
  } catch (error) {
    close()
    throw error
  }
  const executeReport = async (code: string) => {
    if (closed || settle) throw new Error('Python runtime is not available.')
    const result = wait(90000)
    frame.contentWindow?.postMessage({ kind: 'run', code }, '*')
    return result
  }
  return {
    executeReport,
    async execute(code: string) {
      const result = await executeReport(code)
      if (result.exitCode !== 0) throw new Error(result.stderr)
      return result.output + result.stderr
    },
    close,
  }
}
