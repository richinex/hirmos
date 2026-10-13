let python
let inspectLocals
let next = 0
const pending = new Map()
self.hostCall = (name, args) =>
  new Promise((resolve, reject) => {
    const id = ++next
    pending.set(id, { resolve, reject })
    self.postMessage({ kind: 'tool', id, name, args })
  })
self.onmessage = async ({ data }) => {
  if (data.kind === 'reply') {
    const call = pending.get(data.id)
    pending.delete(data.id)
    if (call)
      data.ok ? call.resolve(JSON.stringify(data.value)) : call.reject(new Error(data.error))
    return
  }
  try {
    if (data.kind === 'init') {
      const { loadPyodide } = await import(data.index + 'pyodide.mjs')
      python = await loadPyodide({ indexURL: data.index })
      await python.runPythonAsync(`
import json
from js import hostCall
async def tool(name, args=None):
    return json.loads(await hostCall(name, json.dumps(args or {})))

async def lm_query(prompt):
    results = await tool('delegate', {'tasks': [prompt]})
    if results[0]['kind'] != 'answered':
        raise RuntimeError(results[0]['reason'])
    return results[0]['answer']

async def lm_query_batched(prompts):
    return await tool('delegate', {'tasks': prompts})

async def FINAL_VAR(text, citations):
    return await tool('finish', {'answer': text, 'evidence': citations})

async def DAG_HISTORY(kind=None, limit=None, turn_id=None):
    return await tool('DAG_HISTORY', {'kind': kind, 'limit': limit, 'turn_id': turn_id})
async def DAG_GREP(pattern):
    return await tool('DAG_GREP', {'pattern': pattern})
async def DAG_EXPAND(node_id):
    return await tool('DAG_EXPAND', {'node_id': node_id})
async def DAG_DESCRIBE():
    return await tool('DAG_DESCRIBE', {})
async def DAG_SEARCH(query):
    return await tool('DAG_SEARCH', {'query': query})

def _assistant_locals(namespace=globals(), type_of=type, dumps=json.dumps, names=set(globals()), list_of=list):
    return dumps({name: type_of(value).__name__ for name, value in list_of(namespace.items())
                  if name not in names and not name.startswith('_')})
`)
      inspectLocals = python.globals.get('_assistant_locals')
      self.postMessage({ kind: 'ready' })
    } else if (data.kind === 'run' && python) {
      let output = ''
      let stderr = ''
      const append = (text) => {
        output = (output + text + '\n').slice(0, 12000)
      }
      python.setStdout({ batched: append })
      python.setStderr({
        batched: (text) => {
          stderr = (stderr + text + '\n').slice(0, 4000)
        },
      })
      let exitCode = 0
      try {
        const result = await python.runPythonAsync(data.code)
        if (result?.destroy) result.destroy()
      } catch (error) {
        exitCode = 1
        stderr = (stderr + String(error)).slice(0, 4000)
      }
      const locals = JSON.parse(inspectLocals())
      self.postMessage({ kind: 'done', output, stderr, exitCode, locals })
    }
  } catch (error) {
    self.postMessage({ kind: 'error', error: String(error).slice(0, 4000) })
  }
}
