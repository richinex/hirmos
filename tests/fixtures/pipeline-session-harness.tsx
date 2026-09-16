import React from 'react'
import { createRoot } from 'react-dom/client'
import { useStore } from 'zustand'
import { WorkflowProvider } from '../../src/components/WorkflowProvider'
import { PreparationProvider, usePreparationSession } from '../../src/components/data/PreparationProvider'
import { PipelineWorkspace } from '../../src/components/data/pipeline/PipelineWorkspace'
import { prepareSqlInputs } from '../../src/data/sqlPreparation'
import { pipelineBlockId } from '../../src/domain/pipeline'
import type { PipelineResume } from '../../src/domain/workflow'
import type { PipelineSession } from '../../src/data/pipeline'
import type { PipelineController } from '../../src/components/data/pipeline/pipelineSession'

export async function mount() {
    const inputs = await prepareSqlInputs([new File(['x\n1\n2\n3\n'], 'input.csv', { type: 'text/csv' })])
    if (!inputs.ok) throw new Error('Input preparation failed')
    const input = pipelineBlockId('in')
    const script = pipelineBlockId('script')
    const output = pipelineBlockId('output')
    const resume: PipelineResume = { inputs: inputs.value, graph: {
      nodes: [
        { id: input, position: { x: 0, y: 0 }, block: { kind: 'input', file: { kind: 'chosen', alias: inputs.value[0].alias } } },
        { id: script, position: { x: 0, y: 120 }, block: { kind: 'script', code: 'import time\ntime.sleep(3)\nprepared = inputs[0].assign(y=inputs[0]["x"] * 2)' } },
        { id: output, position: { x: 0, y: 240 }, block: { kind: 'output' } },
      ],
      edges: [{ from: input, to: script, port: 0 }, { from: script, to: output, port: 0 }],
    } }
    let controller: PipelineController | null = null
    let live: PipelineSession | null = null
    let openings = 0
    const h = React.createElement
    function Status({ current }: { current: PipelineController }) {
      const state = useStore(current.store)
      const python = useStore(current.python.store)
      if (state.session.kind === 'ready' && state.session.live !== live) { live = state.session.live; openings++ }
      const outcome = state.run.kind === 'ran' ? state.run.result.outcomes.get(script) : undefined
      return h('output', { 'data-testid': 'session-state' }, JSON.stringify({ openings, running: python.active !== null, rows: outcome?.kind === 'ran' ? outcome.rowCount : null }))
    }
    function Editor() {
      const owner = usePreparationSession()
      const entry = useStore(owner.store, state => state.entry.kind === 'pipeline' ? state.entry : null)
      if (entry !== null) controller = entry.controller
      const [visible, show] = React.useState(true)
      return h(React.Fragment, null,
        h('button', { onClick: () => show(!visible) }, visible ? 'Hide test editor' : 'Show test editor'),
        entry !== null && h(Status, { current: entry.controller }),
        visible && h('div', { style: { height: 600 } }, h(PipelineWorkspace, { resume, onPrepared: () => {} })))
    }
    function Harness() {
      const [opened, setOpened] = React.useState(true)
      return h(React.Fragment, null,
        h('button', { onClick: () => setOpened(false) }, 'Close test project'),
        opened ? h(WorkflowProvider, null, h(PreparationProvider, null, h(Editor))) : h('button', { onClick: () => {
          if (controller === null) throw new Error('The pipeline did not open')
          document.querySelector('[data-testid="closed-session"]')!.textContent = JSON.stringify({ closed: live?.lifecycle.current, active: controller.python.store.getState().active })
          document.querySelector('[data-testid="closed-run"]')!.textContent = controller.store.getState().run.kind
        } }, 'Inspect closed session'),
        h('output', { 'data-testid': 'closed-session' }),
        h('output', { 'data-testid': 'closed-run' }))
    }
    const host = document.createElement('div')
    host.id = 'pipeline-session-test'
    host.style.cssText = 'position:fixed;inset:0;z-index:99999;background:white;overflow:auto'
    document.body.append(host)
    createRoot(host).render(h(Harness))
}
